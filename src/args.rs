use std::env;

use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, Clone)]
#[repr(u16)]
pub enum DeviceType {
    StreamDeck = 0,
    StreamDeckMini = 1,
    StreamDeckXL = 2,
    StreamDeckMobile = 3,
    StreamDeckPedal = 5,
    CorsairVoyager = 6,
    StreamDeckPlus = 7,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StreamDeckArgs {
    pub port: i32,
    pub plugin_uuid: String,
    pub register_event: String,
    pub info: Option<Info>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationInfo {
    pub font: Option<String>,
    pub language: Option<String>,
    pub platform: Option<String>,
    pub platform_version: Option<String>,
    pub version: Option<String>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ColorsInfo {
    pub button_mouse_over_background_color: Option<String>,
    pub button_pressed_background_color: Option<String>,
    pub button_pressed_border_color: Option<String>,
    pub button_pressed_text_color: Option<String>,
    pub highlight_color: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct PluginInfo {
    pub uuid: Option<String>,
    pub version: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct DeviceSize {
    pub columns: i32,
    pub rows: i32,
}

#[derive(Deserialize, Clone)]
pub struct DeviceInfo {
    pub id: Option<String>,
    pub name: Option<String>,
    pub size: Option<DeviceSize>,
    #[serde(rename = "type")]
    pub device_type: i32,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub application: ApplicationInfo,
    pub device_pixel_ratio: Option<i32>,
    pub devices: Option<Vec<DeviceInfo>>,
    pub plugin: PluginInfo,
}

pub fn parse_args() -> StreamDeckArgs {
    let mut port: i32 = 0;
    let mut plugin_uuid: String = "".to_string();
    let mut register_event: String = "".to_string();
    let mut info: Option<Info> = None;
    let args: Vec<String> = env::args().collect();
    args.iter().enumerate().for_each(|(i, p)| match p.as_str() {
        "-port" => port = args[i + 1].parse().unwrap(),
        "-pluginUUID" => plugin_uuid = args[i + 1].to_string(),
        "-registerEvent" => register_event = args[i + 1].to_string(),
        "-info" => {
            let parsed = serde_json::from_str(&args[i + 1].to_string());
            if parsed.is_ok() {
                info = parsed.unwrap();
            }
        }
        &_ => {}
    });
    return StreamDeckArgs {
        port,
        plugin_uuid,
        register_event,
        info,
    };
}
