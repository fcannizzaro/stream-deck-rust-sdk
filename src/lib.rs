use core::time::Duration;
use futures_channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures_util::{FutureExt, StreamExt};
use serde::de::value::MapDeserializer;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

use url::Url;

use crate::action::Action;
use crate::action_manager::ActionManager;
use crate::args::{parse_args, StreamDeckArgs};
use crate::events::events::{ActionState, InputEvent, RegistrationEvent, StreamDeckTarget};
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

pub fn get_settings<T: serde::de::DeserializeOwned>(settings: HashMap<String, Value>) -> Option<T> {
    let deserialized = T::deserialize(MapDeserializer::new(settings.into_iter()));
    if deserialized.is_ok() {
        Some(deserialized.unwrap())
    } else {
        None
    }
}

pub struct Init {
    pub stream_deck: StreamDeck,
    args: (
        WebSocketStream<MaybeTlsStream<TcpStream>>,
        UnboundedReceiver<Message>,
    ),
    manager: Arc<ActionManager>,
}

impl Init {
    pub async fn connect(self) {
        let stream_deck = self.stream_deck.clone();
        let (ws, rx) = self.args;
        let (ws_w, ws_r) = ws.split();
        let fwd_to_ws = rx.map(Ok).forward(ws_w).fuse();

        #[cfg(feature = "logging")]
        println!(" > plugin registered");

        let actions = self.manager.clone();
        let press_events = Arc::new(Mutex::new(HashMap::<String, (SystemTime, bool)>::new()));

        let ws_read = ws_r
            .for_each(|r_msg| async {
                let msg = r_msg.expect("read error");
                let data = msg.into_text().unwrap();

                #[cfg(feature = "logging")]
                println!("Received: {}", data);

                let input: InputEvent = serde_json::from_str(&data).unwrap();
                let manager = actions.clone();
                let events_arc = press_events.clone();
                let sd = stream_deck.clone();

                tokio::spawn(async move {
                    match input {
                        InputEvent::DidReceiveSettings(e) => {
                            let settings_value =
                                serde_json::to_value(e.clone().payload.settings).unwrap();
                            let update =
                                serde_json::from_value::<HashMap<String, Value>>(settings_value)
                                    .unwrap();
                            sd.update_instances_settings(e.context.clone(), update)
                                .await;
                            manager
                                .get(&e.action)
                                .on_settings_changed(e.clone(), sd)
                                .await;
                        }
                        InputEvent::DidReceiveGlobalSettings(e) => {
                            let settings_value =
                                serde_json::to_value(e.clone().payload.settings).unwrap();
                            let update =
                                serde_json::from_value::<HashMap<String, Value>>(settings_value)
                                    .unwrap();

                            sd.update_global_settings(update, None).await;

                            for (_k, action) in manager.actions.iter() {
                                action
                                    .on_global_settings_changed(e.clone(), sd.clone())
                                    .await;
                            }
                        }
                        InputEvent::KeyDown(e) => {
                            let action = manager.get(&e.action);
                            let now = SystemTime::now();
                            let timeout = action.long_timeout();
                            sd.update_instances_settings(
                                e.context.clone(),
                                e.payload.settings.clone(),
                            )
                            .await;

                            action.on_key_down(e.clone(), sd.clone()).await;

                            if timeout > 0.0 {
                                let mut interval =
                                    tokio::time::interval(Duration::from_millis(100));
                                // check every 100ms if the key is still pressed and if the timeout is reached
                                loop {
                                    interval.tick().await;
                                    let elapsed = now.elapsed().unwrap().as_millis() as f32;
                                    let mut events = events_arc.lock().await;
                                    let latest_event = events.get(&e.context.clone());
                                    // on key up was called before the long timeout
                                    if latest_event.is_some() {
                                        let (prev, _) = latest_event.unwrap();
                                        if prev > &now {
                                            break;
                                        }
                                    }
                                    // check if the elapsed time is greater than the long timeout
                                    if elapsed >= timeout {
                                        action.on_long_press(e.clone(), timeout, sd.clone()).await;
                                        events.insert(e.context.clone(), (SystemTime::now(), true));
                                        drop(events);
                                        return;
                                    }
                                }
                            }
                        }
                        InputEvent::KeyUp(mut e) => {
                            let mut events = events_arc.lock().await;
                            let latest_event = events.get(&e.context.clone());
                            let mut should_skip = false;
                            // check if elapsed time between two "onKeyUp" events is less than 500ms
                            if let Some((prev, skip)) = latest_event {
                                should_skip = *skip;
                                let elapse_time = prev.elapsed().unwrap().as_millis();
                                if elapse_time < 500 {
                                    e.is_double_tap = true;
                                }
                            }
                            // update the latest event time
                            events.insert(e.context.clone(), (SystemTime::now(), false));
                            drop(events);
                            // trigger the event modified or not (if needed)
                            if !should_skip {
                                manager.get(&e.action).on_key_up(e.clone(), sd).await;
                            }
                        }
                        InputEvent::TouchTap(e) => {
                            manager.get(&e.action).on_touch_tap(e, sd).await;
                        }
                        InputEvent::DialPress(e) => {
                            manager.get(&e.action).on_dial_press(e, sd).await;
                        }
                        InputEvent::DialRotate(e) => {
                            manager.get(&e.action).on_dial_rotate(e, sd).await;
                        }
                        InputEvent::WillAppear(e) => {
                            let id = e.action.clone();
                            let arc_contexts = sd.contexts.clone();
                            let mut contexts = arc_contexts.lock().await;
                            sd.update_instances_settings(
                                e.context.clone(),
                                e.payload.settings.clone(),
                            )
                            .await;
                            contexts
                                .entry(id)
                                .or_insert(Vec::new())
                                .push(e.context.clone());
                            drop(contexts);
                            manager.get(&e.action).on_appear(e.clone(), sd).await;
                        }
                        InputEvent::WillDisappear(e) => {
                            let id = e.action.clone();
                            let arc_contexts = sd.contexts.clone();
                            let mut contexts = arc_contexts.lock().await;
                            let contexts = contexts.entry(id).or_default();
                            contexts.retain(|element| *element != e.context);
                            manager.get(&e.action).on_disappear(e.clone(), sd).await;
                        }
                        InputEvent::TitleParametersDidChange(e) => {
                            manager
                                .get(&e.action)
                                .on_title_parameters_changed(e.clone(), sd)
                                .await;
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
                            manager
                                .get(&e.action)
                                .on_property_inspector_appear(e.clone(), sd)
                                .await;
                        }
                        InputEvent::PropertyInspectorDidDisappear(e) => {
                            manager
                                .get(&e.action)
                                .on_property_inspector_disappear(e.clone(), sd)
                                .await;
                        }
                        InputEvent::SendToPlugin(e) => {
                            let shared = manager.shared();

                            if shared.is_some() {
                                let shared = shared.unwrap();
                                shared.on_send_to_plugin(e.clone(), sd.clone()).await;
                            }

                            manager
                                .get(&e.action)
                                .on_send_to_plugin(e.clone(), sd)
                                .await;
                        }
                    }
                });
            })
            .fuse();

        stream_deck.clone().register().await;

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
}

pub async fn init(manager: ActionManager, ext_tx: Option<UnboundedSender<String>>) -> Init {
    let args = parse_args();
    let port = &args.port;
    let url = Url::parse(&format!("ws://localhost:{port}")).unwrap();

    let (ws, _r) = connect_async(url.clone()).await.expect("cannot connect");

    #[cfg(feature = "logging")]
    println!(" > connected");

    let (tx, rx) = futures::channel::mpsc::unbounded();
    let stream_deck = StreamDeck::new(args.clone(), tx.clone(), ext_tx);

    return Init {
        stream_deck,
        args: (ws, rx),
        manager: Arc::new(manager),
    };
}
