use std::collections::HashMap;
use std::sync::Arc;

use futures_channel::mpsc::UnboundedSender;
use futures_util::SinkExt;
use tokio::sync::Mutex;
use tungstenite::Message;

use crate::{
    ActionState, get_settings_event, log_message, register, send_to_property_inspector,
    set_global_settings, set_image, set_settings, set_state, set_title, show_alert, show_ok,
    StreamDeckArgs, StreamDeckTarget, switch_to_profile,
};
use crate::events::sent::{get_global_settings_event, open_url};

#[derive(Clone)]
pub struct StreamDeck {
    pub contexts: Arc<Mutex<HashMap<String, Vec<String>>>>,
    args: StreamDeckArgs,
    tx: UnboundedSender<Message>,
}

impl StreamDeck {
    pub fn to_arc(&self) -> Arc<StreamDeck> {
        Arc::new(self.clone())
    }

    pub fn new(args: StreamDeckArgs, tx: UnboundedSender<Message>) -> Self {
        Self {
            contexts: Arc::new(Mutex::new(HashMap::new())),
            args,
            tx,
        }
    }

    pub(crate) async fn send(&self, content: String) {
        self.tx
            .clone()
            .send(Message::Text(content))
            .await
            .expect("Cannot send message");
    }

    pub async fn register(&self) {
        self.send(register(
            self.args.register_event.clone(),
            self.args.plugin_uuid.clone(),
        ))
            .await
    }

    pub async fn set_title(&self, context: String, title: Option<String>) {
        #[cfg(feature = "logging")]
        println!(" > set_title: {:?}", title);
        self.send(set_title(context, title, None, None)).await;
    }

    pub async fn set_title_extra(
        &self,
        context: String,
        title: Option<String>,
        target: Option<StreamDeckTarget>,
        state: Option<ActionState>,
    ) {
        self.send(set_title(context, title, target, state)).await;
    }

    pub async fn set_image_b64(&self, context: String, base64: Option<String>) {
        #[cfg(feature = "logging")]
        println!(" > set_image: {:?}", base64);
        self.send(set_image(context, base64, None, None)).await;
    }

    pub async fn show_ok(&self, context: String) {
        self.send(show_ok(context)).await;
    }

    pub async fn log(&self, message: String) {
        self.send(log_message(message)).await;
    }

    pub async fn show_alert(&self, context: String) {
        self.send(show_alert(context)).await;
    }

    pub async fn switch_to_profile(&self, device: String, profile: String) {
        self.send(switch_to_profile(
            self.args.plugin_uuid.clone(),
            device,
            profile,
        ))
            .await;
    }

    pub async fn send_to_property_inspector(
        &self,
        action: String,
        context: String,
        payload: HashMap<String, serde_json::Value>,
    ) {
        self.send(send_to_property_inspector(action, context, payload))
            .await;
    }

    pub async fn set_state(&self, context: String, state: i32) {
        self.send(set_state(context, state)).await;
    }

    pub async fn open_url(&self, url: String) {
        self.send(open_url(url)).await;
    }

    pub async fn set_settings<Settings: serde::ser::Serialize>(
        &self,
        context: String,
        settings: Settings,
    ) {
        self.send(set_settings(&context, settings)).await;
        self.send(get_settings_event(context)).await;
    }

    pub async fn set_global_settings<GlobalSettings: serde::ser::Serialize>(
        &self,
        settings: GlobalSettings,
    ) {
        self.send(set_global_settings(self.args.plugin_uuid.clone(), settings))
            .await;
        self.send(get_global_settings_event(self.args.plugin_uuid.clone()))
            .await;
    }
}
