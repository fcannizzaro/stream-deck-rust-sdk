use std::collections::HashMap;
use std::sync::Arc;

use futures_channel::mpsc::UnboundedSender;
use futures_util::SinkExt;
use serde::de::value::MapDeserializer;
use serde_json::Value;
use tokio::sync::Mutex;
use tungstenite::Message;

use crate::events::sent::{get_global_settings_event, open_url, set_feedback};
use crate::{
    get_settings_event, log_message, register, send_to_property_inspector, set_global_settings,
    set_image, set_settings, set_state, set_title, show_alert, show_ok, switch_to_profile,
    ActionState, StreamDeckArgs, StreamDeckTarget,
};

#[derive(Clone)]
pub struct StreamDeck {
    pub contexts: Arc<Mutex<HashMap<String, Vec<String>>>>,
    args: StreamDeckArgs,
    pub(crate) global_settings: Arc<Mutex<HashMap<String, Value>>>,
    pub(crate) instances_settings: Arc<Mutex<HashMap<String, HashMap<String, Value>>>>,
    tx: UnboundedSender<Message>,
    ext_tx: Option<UnboundedSender<String>>,
}

impl StreamDeck {
    pub fn to_arc(&self) -> Arc<StreamDeck> {
        Arc::new(self.clone())
    }

    pub fn new(
        args: StreamDeckArgs,
        tx: UnboundedSender<Message>,
        ext_tx: Option<UnboundedSender<String>>,
    ) -> Self {
        Self {
            contexts: Arc::new(Mutex::new(HashMap::new())),
            args,
            tx,
            global_settings: Arc::new(Mutex::new(HashMap::new())),
            instances_settings: Arc::new(Mutex::new(HashMap::new())),
            ext_tx,
        }
    }

    pub async fn global_settings<T: serde::de::DeserializeOwned>(&self) -> T {
        T::deserialize(MapDeserializer::new(
            self.global_settings.lock().await.clone().into_iter(),
        ))
        .unwrap()
    }

    pub async fn settings<T: serde::de::DeserializeOwned>(&self, context: String) -> Option<T> {
        let all_settings = self.instances_settings.lock().await;
        let settings = all_settings.get(&context);
        match settings {
            Some(settings) => {
                Some(T::deserialize(MapDeserializer::new(settings.clone().into_iter())).unwrap())
            }
            None => None,
        }
    }

    pub async fn external(&self, data: String) {
        self.ext_tx.clone().unwrap().send(data).await.unwrap();
    }

    pub(crate) async fn send(&self, content: String) {
        self.tx
            .clone()
            .send(Message::Text(content))
            .await
            .expect("Cannot send message");
    }

    pub async fn register(&self) {
        let uuid = self.args.plugin_uuid.clone();
        self.send(register(self.args.register_event.clone(), uuid.clone()))
            .await;
        self.send(get_global_settings_event(uuid)).await
    }

    pub async fn set_title(&self, context: String, title: Option<String>) {
        #[cfg(feature = "logging")]
        println!(" > set_title: {:?}", title);
        self.send(set_title(context, title, None, None)).await;
    }

    pub async fn contexts_of(&self, uuid: &str) -> Vec<String> {
        let contexts = self.contexts.lock().await;
        if contexts.contains_key(uuid) {
            contexts.get(uuid).unwrap().clone()
        } else {
            vec![]
        }
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
        payload: HashMap<String, Value>,
    ) {
        self.send(send_to_property_inspector(action, context, payload))
            .await;
    }

    pub async fn set_state(&self, context: String, state: i32) {
        self.send(set_state(context, state)).await;
    }

    pub async fn set_feedback(
        &self,
        context: String,
        value: i32,
        indicator: i32,
        title: Option<String>,
        opacity: Option<i32>,
    ) {
        self.send(set_feedback(context, value, indicator, title, opacity))
            .await;
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

    pub async fn update_global_settings(
        &self,
        settings: HashMap<String, Value>,
        update: Option<bool>,
    ) {
        let mut locked = self.global_settings.lock().await;
        settings.iter().for_each(|(k, v)| {
            locked.insert(k.clone(), v.clone());
        });
        if update.is_some() {
            self.set_global_settings(locked.clone()).await;
        }
    }

    pub(crate) async fn update_instances_settings(
        &self,
        context: String,
        settings: HashMap<String, Value>,
    ) {
        let mut locked = self.instances_settings.lock().await;
        locked.insert(context, settings);
    }

    pub async fn set_global_settings<GlobalSettings: serde::ser::Serialize + Clone>(
        &self,
        settings: GlobalSettings,
    ) {
        self.send(set_global_settings(self.args.plugin_uuid.clone(), settings))
            .await;
        self.send(get_global_settings_event(self.args.plugin_uuid.clone()))
            .await;
    }
}
