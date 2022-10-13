use std::collections::HashMap;

use crate::events::received::{
    ActionState, GetSettingsEvent, LogMessageEvent, LogMessagePayload, OpenUrlEvent,
    OpenUrlPayload, SendToPropertyInspectorEvent, SetSettingsEvent, SetStateEvent,
    SetTitleImageEvent, SetTitleImagePayload, ShowActionEvent, StreamDeckTarget,
    SwitchToProfileEvent, SwitchToProfilePayload,
};
use crate::RegistrationEvent;

pub fn set_title(
    context: String,
    title: Option<String>,
    target: Option<StreamDeckTarget>,
    state: Option<ActionState>,
) -> String {
    let event = SetTitleImageEvent {
        event: "setTitle".to_string(),
        context,
        payload: SetTitleImagePayload {
            image: None,
            title,
            target,
            state,
        },
    };
    serde_json::to_string(&event).unwrap()
}

pub fn register(event: String, uuid: String) -> String {
    let event = RegistrationEvent { event, uuid };
    serde_json::to_string(&event).unwrap()
}

pub fn show_ok(context: String) -> String {
    let event = ShowActionEvent {
        event: "showOk".to_string(),
        context,
    };
    serde_json::to_string(&event).unwrap()
}

pub fn show_alert(context: String) -> String {
    let event = ShowActionEvent {
        event: "showAlert".to_string(),
        context,
    };
    serde_json::to_string(&event).unwrap()
}

pub fn set_state(context: String, state: i32) -> String {
    let event = SetStateEvent {
        event: "setState".to_string(),
        context,
        state,
    };
    serde_json::to_string(&event).unwrap()
}

pub fn log_message(message: String) -> String {
    let event = LogMessageEvent {
        event: "logMessage".to_string(),
        payload: LogMessagePayload { message },
    };
    serde_json::to_string(&event).unwrap()
}

pub fn open_url(url: String) -> String {
    let event = OpenUrlEvent {
        event: "openUrl".to_string(),
        payload: OpenUrlPayload { url },
    };
    serde_json::to_string(&event).unwrap()
}

pub fn switch_to_profile(context: String, device: String, profile: String) -> String {
    let event = SwitchToProfileEvent {
        event: "switchToProfile".to_string(),
        context,
        device,
        payload: SwitchToProfilePayload { profile },
    };
    serde_json::to_string(&event).unwrap()
}

pub fn send_to_property_inspector(
    action: String,
    context: String,
    payload: HashMap<String, serde_json::Value>,
) -> String {
    let event = SendToPropertyInspectorEvent {
        event: "sendToPropertyInspector".to_string(),
        action,
        context,
        payload,
    };
    serde_json::to_string(&event).unwrap()
}

pub fn set_image(
    context: String,
    image: Option<String>,
    target: Option<StreamDeckTarget>,
    state: Option<ActionState>,
) -> String {
    let event = SetTitleImageEvent {
        event: "setImage".to_string(),
        context,
        payload: SetTitleImagePayload {
            title: None,
            image,
            target,
            state,
        },
    };
    serde_json::to_string(&event).unwrap()
}

pub fn get_settings_event(context: String) -> String {
    let event = GetSettingsEvent {
        event: "getSettings".to_string(),
        context,
    };
    serde_json::to_string(&event).unwrap()
}

pub fn set_settings<T: serde::ser::Serialize>(context: &String, payload: T) -> String {
    let event = SetSettingsEvent {
        event: "setSettings".to_string(),
        context: context.clone(),
        payload,
    };
    serde_json::to_string(&event).unwrap()
}

pub fn get_global_settings_event(context: String) -> String {
    let event = GetSettingsEvent {
        event: "getGlobalSettings".to_string(),
        context,
    };
    serde_json::to_string(&event).unwrap()
}

pub fn set_global_settings<T: serde::ser::Serialize>(context: String, payload: T) -> String {
    let event = SetSettingsEvent {
        event: "setGlobalSettings".to_string(),
        context,
        payload,
    };
    serde_json::to_string(&event).unwrap()
}
