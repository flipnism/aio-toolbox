use std::time::Instant;

use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

/**
{
   "fromserver":false,
   "type":"createemblem",
   "data":"this is some data u want!"
}
*/
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct JsonMessage {
    pub fromserver: bool,
    #[serde(rename = "type")]
    pub data_type: String,
    pub data: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub channel: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub image64: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub textdata: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct MessagePayload {
    pub message: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyData {
    pub hotkey_desc: String,
    pub id: u32,
    pub shift: bool,
    pub alt: bool,
    pub ctrl: bool,
    pub win: bool,
    pub keys: String,
    pub name: String,
    pub data: JsonMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HKData {
    pub fromserver: bool,
    #[serde(rename = "type")]
    pub data_type: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thumbnail {
    pub name: String,
    pub thumbfile: String,
}

#[derive(Deserialize, Debug)]
pub struct KeyMode {
    pub mode: i8,
}

#[derive(Serialize, Clone, Debug)]
pub struct KeypadPayload {
    pub message: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct LoggingPayload {
    pub message: String,
}
#[derive(Deserialize)]
pub struct CommandArgs {
    pub args: Vec<String>,
}

#[derive(EnumString)]
pub enum ConfigEnum {
    Pm2ConfigPath,
    HotkeyConfigPath,
    BlenderAssetPath,
    BlenderOutputPath,
    ApiUrl,
    WSUrl,
    EmblemFont,
    CustomScriptPath,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub pm2_config_path: String,
    pub hotkey_config_path: String,
    pub blender_asset_path: String,
    pub blender_output_path: String,
    pub api_url: String,
    pub web_socket_url: String,
    pub emblem_font: String,
    pub custom_script_path: String,
}

#[derive(Debug, Clone)]
pub struct RegisteredKey {
    pub id: u32,
    pub key: String,
    pub mod_key: bool,
}

#[derive(Serialize, Debug)]
struct MacropadPayload {
    key: String,
    is_pressed: bool,
    is_mod: bool,
}
#[derive(Serialize)]
pub struct ModPayload<'a> {
    pub layer_index: &'a u32,
}
#[derive(Serialize)]
pub struct ModKey {
    pub is_pressed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MPadPayload {
    pub key: String,
    pub is_pressed: bool,
    pub is_mod: bool,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct DataMacro {
    pub key: String,
    pub down: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MacroPadDataMulti {
    pub id: u32,
    pub data: Vec<Vec<MacroPadData>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MacroPadData {
    pub key: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub data_mode: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_macro: Option<Vec<DataMacro>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<JsonMessage>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub macro_title: String,
}
