use async_trait::async_trait;

use crate::{Arc, StreamDeck};
use crate::events::received::{
    AppearEvent, ApplicationEvent, DeviceDidConnectEvent, DeviceDidDisconnectEvent,
    DidReceiveGlobalSettingsEvent, DidReceiveSettingsEvent, KeyEvent, PropertyInspectorAppearEvent,
    SendToPluginEvent, SystemDidWakeUpEvent, TitleParametersDidChangeEvent,
};

#[async_trait]
pub trait Action: Sync {
    fn uuid(&self) -> &str;
    async fn on_key_down(&self, _e: KeyEvent, _sd: Arc<StreamDeck>) {}
    async fn on_key_up(&self, _e: KeyEvent, _sd: Arc<StreamDeck>) {}
    async fn on_appear(&self, _e: AppearEvent, _sd: Arc<StreamDeck>) {}
    async fn on_disappear(&self, _e: AppearEvent, _sd: Arc<StreamDeck>) {}
    async fn on_title_parameters_changed(
        &self,
        _e: TitleParametersDidChangeEvent,
        _sd: Arc<StreamDeck>,
    ) {}
    async fn on_device_connect(&self, _e: DeviceDidConnectEvent, _sd: Arc<StreamDeck>) {}
    async fn on_device_disconnect(&self, _e: DeviceDidDisconnectEvent, _sd: Arc<StreamDeck>) {}
    async fn on_application_launch(&self, _e: ApplicationEvent, _sd: Arc<StreamDeck>) {}
    async fn on_application_terminate(&self, _e: ApplicationEvent, _sd: Arc<StreamDeck>) {}
    async fn on_system_wake_up(&self, _e: SystemDidWakeUpEvent, _sd: Arc<StreamDeck>) {}
    async fn on_property_inspector_appear(
        &self,
        _e: PropertyInspectorAppearEvent,
        _sd: Arc<StreamDeck>,
    ) {}
    async fn on_property_inspector_disappear(
        &self,
        _e: PropertyInspectorAppearEvent,
        _sd: Arc<StreamDeck>,
    ) {}
    async fn on_send_to_plugin(&self, _e: SendToPluginEvent, _sd: Arc<StreamDeck>) {}
    async fn on_settings_changed(&self, _e: DidReceiveSettingsEvent, _sd: Arc<StreamDeck>) {}
    async fn on_global_settings_changed(
        &self,
        _e: DidReceiveGlobalSettingsEvent,
        _sd: Arc<StreamDeck>,
    ) {}
    // custom tap events
    async fn on_single_tap(&self, _e: KeyEvent, _sd: Arc<StreamDeck>) {}
    async fn on_double_tap(&self, _e: KeyEvent, _sd: Arc<StreamDeck>) {}
    async fn on_long_tap(&self, _e: KeyEvent, _sd: Arc<StreamDeck>) {}
}
