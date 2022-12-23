use async_trait::async_trait;

use crate::events::received::{
    AppearEvent, ApplicationEvent, DeviceDidConnectEvent, DeviceDidDisconnectEvent, DialPressEvent,
    DialRotateEvent, DidReceiveGlobalSettingsEvent, DidReceiveSettingsEvent, KeyEvent,
    PropertyInspectorAppearEvent, SendToPluginEvent, SystemDidWakeUpEvent,
    TitleParametersDidChangeEvent, TouchTapEvent,
};
use crate::stream_deck::StreamDeck;

#[async_trait]
#[allow(unused)]
pub trait Action: Sync {
    fn uuid(&self) -> &str;
    fn long_timeout(&self) -> f32 {
        0.0
    }
    async fn on_appear(&self, e: AppearEvent, sd: StreamDeck) {}
    async fn on_disappear(&self, e: AppearEvent, sd: StreamDeck) {}
    async fn on_key_down(&self, e: KeyEvent, sd: StreamDeck) {}
    async fn on_key_up(&self, e: KeyEvent, sd: StreamDeck) {}
    // custom actions
    async fn on_long_press(&self, e: KeyEvent, timeout: f32, sd: StreamDeck) {}
    // settings
    async fn on_settings_changed(&self, e: DidReceiveSettingsEvent, sd: StreamDeck) {}
    async fn on_global_settings_changed(&self, e: DidReceiveGlobalSettingsEvent, sd: StreamDeck) {}
    // dial
    async fn on_dial_rotate(&self, e: DialRotateEvent, sd: StreamDeck) {}
    async fn on_dial_press(&self, e: DialPressEvent, sd: StreamDeck) {}
    // touch
    async fn on_touch_tap(&self, e: TouchTapEvent, sd: StreamDeck) {}
    // other events
    async fn on_title_parameters_changed(&self, e: TitleParametersDidChangeEvent, sd: StreamDeck) {}
    async fn on_device_connect(&self, e: DeviceDidConnectEvent, sd: StreamDeck) {}
    async fn on_device_disconnect(&self, e: DeviceDidDisconnectEvent, sd: StreamDeck) {}
    async fn on_application_launch(&self, e: ApplicationEvent, sd: StreamDeck) {}
    async fn on_application_terminate(&self, e: ApplicationEvent, sd: StreamDeck) {}
    async fn on_system_wake_up(&self, e: SystemDidWakeUpEvent, sd: StreamDeck) {}
    async fn on_property_inspector_appear(&self, e: PropertyInspectorAppearEvent, sd: StreamDeck) {}
    async fn on_property_inspector_disappear(
        &self,
        e: PropertyInspectorAppearEvent,
        sd: StreamDeck,
    ) {
    }
    async fn on_send_to_plugin(&self, e: SendToPluginEvent, sd: StreamDeck) {}
}
