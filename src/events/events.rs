#![allow(dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::Serialize_repr;

use crate::args::DeviceInfo;

#[derive(Serialize_repr, Clone)]
#[repr(u16)]
pub enum StreamDeckTarget {
    All = 0,
    Hardware = 1,
    Software = 2,
}

#[derive(Serialize_repr, Clone)]
#[repr(u16)]
pub enum ActionState {
    First = 0,
    Second = 1,
}

#[derive(Serialize, Clone)]
pub struct RegistrationEvent {
    pub(crate) event: String,
    pub(crate) uuid: String,
}

#[derive(Serialize, Clone)]
pub struct ShowActionEvent {
    pub(crate) event: String,
    pub(crate) context: String,
}

#[derive(Serialize, Clone)]
pub struct SetStatePayload {
    pub(crate) state: i32,
}

#[derive(Serialize, Clone)]
pub struct SetStateEvent {
    pub(crate) event: String,
    pub(crate) context: String,
    pub(crate) payload: SetStatePayload,
}

#[derive(Serialize, Clone)]
pub struct SetFeedbackEvent {
    pub(crate) event: String,
    pub(crate) context: String,
    pub(crate) payload: Value,
}

#[derive(Serialize, Clone)]
pub struct SetFeedbackLayoutPayload {
    pub(crate) layout: String,
}

#[derive(Serialize, Clone)]
pub struct SetFeedbackLayoutEvent {
    pub(crate) event: String,
    pub(crate) context: String,
    pub(crate) payload: SetFeedbackLayoutPayload,
}

#[derive(Serialize, Clone)]
pub struct LogMessagePayload {
    pub(crate) message: String,
}

#[derive(Serialize, Clone)]
pub struct LogMessageEvent {
    pub(crate) event: String,
    pub(crate) payload: LogMessagePayload,
}

#[derive(Serialize, Clone)]
pub struct OpenUrlPayload {
    pub(crate) url: String,
}

#[derive(Serialize, Clone)]
pub struct OpenUrlEvent {
    pub(crate) event: String,
    pub(crate) payload: OpenUrlPayload,
}

#[derive(Serialize, Clone)]
pub struct SetSettingsEvent<T> {
    pub(crate) event: String,
    pub(crate) context: String,
    pub(crate) payload: T,
}

#[derive(Serialize, Clone)]
pub struct GetSettingsEvent {
    pub(crate) event: String,
    pub(crate) context: String,
}

#[derive(Serialize, Clone)]
pub struct SwitchToProfilePayload {
    pub(crate) profile: String,
}

#[derive(Serialize, Clone)]
pub struct SwitchToProfileEvent {
    pub(crate) event: String,
    pub(crate) context: String,
    pub(crate) device: String,
    pub(crate) payload: SwitchToProfilePayload,
}

#[derive(Serialize, Clone)]
pub struct SendToPropertyInspectorEvent {
    pub(crate) event: String,
    pub(crate) action: String,
    pub(crate) context: String,
    pub(crate) payload: HashMap<String, Value>,
}

#[derive(Serialize, Clone)]
pub struct SetTitleImagePayload {
    pub(crate) title: Option<String>,
    pub(crate) image: Option<String>,
    pub(crate) target: Option<StreamDeckTarget>,
    pub(crate) state: Option<ActionState>,
}

#[derive(Serialize, Clone)]
pub struct SetTitleImageEvent {
    pub(crate) event: String,
    pub(crate) context: String,
    pub(crate) payload: SetTitleImagePayload,
}

#[derive(Deserialize, Clone)]
pub struct PayloadCoordinates {
    pub column: i32,
    pub row: i32,
}

#[derive(Deserialize, Clone)]
pub struct EmptySettings {}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DidReceiveSettingsPayload {
    pub settings: HashMap<String, Value>,
    pub coordinates: Option<PayloadCoordinates>,
    pub is_in_multi_action: bool,
}

#[derive(Deserialize, Clone)]
pub struct DidReceiveSettingsEvent {
    pub action: String,
    pub context: String,
    pub device: String,
    pub payload: DidReceiveSettingsPayload,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DidReceiveGlobalSettingsPayload {
    pub settings: HashMap<String, Value>,
}

#[derive(Deserialize, Clone)]
pub struct DidReceiveGlobalSettingsEvent {
    pub payload: DidReceiveGlobalSettingsPayload,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyEventPayload {
    pub state: Option<i32>,
    pub user_desired_state: Option<i32>,
    pub is_in_multi_action: bool,
    pub coordinates: Option<PayloadCoordinates>,
    pub settings: HashMap<String, Value>,
}

#[derive(Deserialize, Clone)]
pub struct KeyEvent {
    pub action: String,
    pub context: String,
    pub device: String,
    pub payload: KeyEventPayload,
    #[serde(skip_deserializing, skip_serializing)]
    pub is_double_tap: bool,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DialRotateEventPayload {
    pub ticks: i32,
    pub pressed: bool,
    pub settings: HashMap<String, Value>,
}

#[derive(Deserialize, Clone)]
pub struct DialRotateEvent {
    pub action: String,
    pub context: String,
    pub device: String,
    pub payload: DialRotateEventPayload,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DialPressEventPayload {
    pub pressed: Option<bool>,
    pub coordinates: Option<PayloadCoordinates>,
    pub settings: HashMap<String, Value>,
}

#[derive(Deserialize, Clone)]
pub struct DialPressEvent {
    pub action: String,
    pub context: String,
    pub device: String,
    pub payload: DialPressEventPayload,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TouchTapEventPayload {
    pub hold: bool,
    pub coordinates: PayloadCoordinates,
    pub tap_pos: [i32; 2],
    pub settings: HashMap<String, Value>,
}

#[derive(Deserialize, Clone)]
pub struct TouchTapEvent {
    pub action: String,
    pub context: String,
    pub device: String,
    pub payload: TouchTapEventPayload,
}

#[derive(Deserialize, Clone)]
pub enum Controller {
    Keypad,
    Encoder,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppearEventPayload {
    pub state: Option<i32>,
    pub is_in_multi_action: bool,
    pub coordinates: Option<PayloadCoordinates>,
    pub settings: HashMap<String, Value>,
    pub controller: Controller,
}

#[derive(Deserialize, Clone)]
pub struct AppearEvent {
    pub action: String,
    pub context: String,
    pub device: String,
    pub payload: AppearEventPayload,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TitleAlignment {
    Bottom,
    Top,
    Middle,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TitleParameters {
    pub font_family: String,
    pub font_size: i32,
    pub font_style: String,
    pub font_underline: bool,
    pub show_title: bool,
    pub title_alignment: TitleAlignment,
    pub title_color: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TitleParametersDidChangeEventPayload {
    pub state: i32,
    pub title: String,
    pub title_parameters: TitleParameters,
    pub coordinates: Option<PayloadCoordinates>,
    pub settings: HashMap<String, Value>,
}

#[derive(Deserialize, Clone)]
pub struct TitleParametersDidChangeEvent {
    pub action: String,
    pub context: String,
    pub device: String,
    pub payload: TitleParametersDidChangeEventPayload,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDidConnectEvent {
    pub device: String,
    pub device_info: DeviceInfo,
}

#[derive(Deserialize, Clone)]
pub struct DeviceDidDisconnectEvent {
    pub device: String,
}

#[derive(Deserialize, Clone)]
pub struct ApplicationEvent {
    pub application: String,
}

#[derive(Deserialize, Clone)]
pub struct SystemDidWakeUpEvent {}

#[derive(Deserialize, Clone)]
pub struct PropertyInspectorAppearEvent {
    pub action: String,
    pub context: String,
    pub device: String,
}

#[derive(Deserialize, Clone)]
pub struct SendToPluginEvent {
    pub action: String,
    pub context: String,
    pub payload: HashMap<String, Value>,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "event")]
pub enum InputEvent {
    #[serde(rename = "didReceiveSettings")]
    DidReceiveSettings(DidReceiveSettingsEvent),
    #[serde(rename = "didReceiveGlobalSettings")]
    DidReceiveGlobalSettings(DidReceiveGlobalSettingsEvent),
    #[serde(rename = "keyDown")]
    KeyDown(KeyEvent),
    #[serde(rename = "keyUp")]
    KeyUp(KeyEvent),
    #[serde(rename = "dialRotate")]
    DialRotate(DialRotateEvent),
    #[serde(rename = "dialPress")]
    DialPress(DialPressEvent),
    #[serde(rename = "touchTap")]
    TouchTap(TouchTapEvent),
    #[serde(rename = "willAppear")]
    WillAppear(AppearEvent),
    #[serde(rename = "willDisappear")]
    WillDisappear(AppearEvent),
    #[serde(rename = "titleParametersDidChange")]
    TitleParametersDidChange(TitleParametersDidChangeEvent),
    #[serde(rename = "deviceDidConnect")]
    DeviceDidConnect(DeviceDidConnectEvent),
    #[serde(rename = "deviceDidDisconnect")]
    DeviceDidDisconnect(DeviceDidDisconnectEvent),
    #[serde(rename = "applicationDidLaunch")]
    ApplicationDidLaunch(ApplicationEvent),
    #[serde(rename = "applicationDidTerminate")]
    ApplicationDidTerminate(ApplicationEvent),
    #[serde(rename = "systemDidWakeUp")]
    SystemDidWakeUp(SystemDidWakeUpEvent),
    #[serde(rename = "propertyInspectorDidAppear")]
    PropertyInspectorDidAppear(PropertyInspectorAppearEvent),
    #[serde(rename = "propertyInspectorDidDisappear")]
    PropertyInspectorDidDisappear(PropertyInspectorAppearEvent),
    #[serde(rename = "sendToPlugin")]
    SendToPlugin(SendToPluginEvent),
}
