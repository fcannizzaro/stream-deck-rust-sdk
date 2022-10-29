use std::collections::HashMap;
use std::sync::Arc;

use futures_channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures_util::{FutureExt, StreamExt};
use serde::de::value::MapDeserializer;
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;
use url::Url;

use crate::action::Action;
use crate::action_manager::ActionManager;
use crate::args::{parse_args, StreamDeckArgs};
use crate::events::received::{ActionState, InputEvent, RegistrationEvent, StreamDeckTarget};
use crate::events::sent::{
    get_settings_event, log_message, register, send_to_property_inspector, set_global_settings,
    set_image, set_settings, set_state, set_title, show_alert, show_ok, switch_to_profile,
};
use crate::stream_deck::StreamDeck;

pub mod action;
pub mod action_manager;
mod args;
pub mod events;
pub mod stream_deck;

#[cfg(feature = "images")]
pub mod images;

#[cfg(feature = "download")]
pub mod download;

pub fn get_settings<T: serde::de::DeserializeOwned>(settings: HashMap<String, Value>) -> T {
    T::deserialize(MapDeserializer::new(settings.into_iter())).unwrap()
}

pub async fn init(
    ext_tx: Option<UnboundedSender<String>>,
) -> (
    StreamDeck,
    WebSocketStream<MaybeTlsStream<TcpStream>>,
    UnboundedReceiver<Message>,
) {
    let args = parse_args();
    let port = &args.port;
    let url = Url::parse(&format!("ws://localhost:{port}")).unwrap();

    let (ws, _r) = connect_async(url.clone()).await.expect("cannot connect");

    #[cfg(feature = "logging")]
    println!(" > connected");

    let (tx, rx) = futures::channel::mpsc::unbounded();
    let stream_deck = StreamDeck::new(args.clone(), tx.clone(), ext_tx);

    return (stream_deck.clone(), ws, rx);
}

pub async fn connect(
    args: (
        StreamDeck,
        WebSocketStream<MaybeTlsStream<TcpStream>>,
        UnboundedReceiver<Message>,
    ),
    manager: ActionManager,
) {
    let (stream_deck, ws, rx) = args;
    let (ws_w, ws_r) = ws.split();
    let fwd_to_ws = rx.map(Ok).forward(ws_w).fuse();

    stream_deck.clone().register().await;

    #[cfg(feature = "logging")]
    println!(" > plugin registered");

    let ws_read = ws_r
        .for_each(|r_msg| async {
            let msg = r_msg.expect("read error");
            let data = msg.into_text().unwrap();

            #[cfg(feature = "logging")]
            println!("Received: {}", data);

            let input: InputEvent = serde_json::from_str(&data).unwrap();

            let sd = stream_deck.to_arc();

            match input {
                InputEvent::DidReceiveSettings(e) => {
                    let action = manager.get(&e.action);
                    let settings_value = serde_json::to_value(e.clone().payload.settings).unwrap();
                    let update =
                        serde_json::from_value::<HashMap<String, Value>>(settings_value).unwrap();
                    sd.update_instances_settings(e.context.clone(), update)
                        .await;
                    action.on_settings_changed(e.clone(), sd).await;
                }
                InputEvent::DidReceiveGlobalSettings(e) => {
                    let settings_value = serde_json::to_value(e.clone().payload.settings).unwrap();
                    let update =
                        serde_json::from_value::<HashMap<String, Value>>(settings_value).unwrap();

                    sd.update_global_settings(update).await;

                    for (_k, action) in manager.actions.iter() {
                        action
                            .on_global_settings_changed(e.clone(), sd.clone())
                            .await;
                    }
                }
                InputEvent::KeyDown(e) => {
                    let action = manager.get(&e.action);
                    action.on_key_down(e.clone(), sd).await;
                }
                InputEvent::KeyUp(e) => {
                    let action = manager.get(&e.action);
                    action.on_key_up(e.clone(), sd).await;
                }
                InputEvent::WillAppear(e) => {
                    let action = manager.get(&e.action);
                    let mut contexts = stream_deck.contexts.lock().await;
                    let id = e.action.clone();
                    contexts
                        .entry(id)
                        .or_insert(Vec::new())
                        .push(e.context.clone());
                    action.on_appear(e.clone(), sd).await;
                }
                InputEvent::WillDisappear(e) => {
                    let action = manager.get(&e.action);
                    let mut contexts = stream_deck.contexts.lock().await;
                    let id = e.action.clone();
                    let contexts = contexts.entry(id).or_default();
                    contexts.retain(|element| *element != e.context);
                    action.on_disappear(e.clone(), sd).await;
                }
                InputEvent::TitleParametersDidChange(e) => {
                    let action = manager.get(&e.action);
                    action.on_title_parameters_changed(e.clone(), sd).await;
                }
                InputEvent::DeviceDidConnect(e) => {
                    for (_k, action) in manager.actions.iter() {
                        action.on_device_connect(e.clone(), sd.clone()).await;
                    }
                }
                InputEvent::DeviceDidDisconnect(e) => {
                    for (_k, action) in manager.actions.iter() {
                        action.on_device_disconnect(e.clone(), sd.clone()).await;
                    }
                }
                InputEvent::ApplicationDidLaunch(e) => {
                    for (_k, action) in manager.actions.iter() {
                        action.on_application_launch(e.clone(), sd.clone()).await;
                    }
                }
                InputEvent::ApplicationDidTerminate(e) => {
                    for (_k, action) in manager.actions.iter() {
                        action.on_application_terminate(e.clone(), sd.clone()).await;
                    }
                }
                InputEvent::SystemDidWakeUp(e) => {
                    for (_k, action) in manager.actions.iter() {
                        action.on_system_wake_up(e.clone(), sd.clone()).await;
                    }
                }
                InputEvent::PropertyInspectorDidAppear(e) => {
                    let action = manager.get(&e.action);
                    action.on_property_inspector_appear(e.clone(), sd).await;
                }
                InputEvent::PropertyInspectorDidDisappear(e) => {
                    let action = manager.get(&e.action);
                    action.on_property_inspector_disappear(e.clone(), sd).await;
                }
                InputEvent::SendToPlugin(e) => {
                    let action = manager.get(&e.action);
                    action.on_send_to_plugin(e.clone(), sd).await;
                }
            }
        })
        .fuse();

    tokio::pin!(ws_read, fwd_to_ws);

    tokio::select! {
        _ = ws_read => println!("done"),
        fwd_r = fwd_to_ws => {
             match fwd_r {
                 Ok(_) => println!("no error"),
                 Err(e) => println!("error: {}", e),
             }
        }
    }
}
