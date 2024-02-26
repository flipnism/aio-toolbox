use tauri::api::file;
use tauri::AppHandle;

use crate::strct::{Config, ConfigEnum, MacroPadDataMulti};
use std::fs::{self, File};
use std::io::Error;
use std::io::Write;

pub fn get_setting_for(setting_key: ConfigEnum, app_handle: &AppHandle) -> String {
    match app_handle.path_resolver().app_data_dir() {
        Some(path) => {
            let userdir_path = path.join("users");

            fs::create_dir_all(userdir_path.to_string_lossy().to_string());
            let configfile = userdir_path.join("aio_config.json");
            let conf_file = match configfile.exists() {
                true => configfile.to_string_lossy().to_string(),
                false => {
                    let file_name = configfile.to_string_lossy().to_string();
                    let default_data = Config {
                        pm2_config_path: userdir_path.join("ecosystem.config.js").to_string_lossy().to_string(),
                        hotkey_config_path: "D:\\__CODE\\RUSTUP\\PROJECT\\console-server\\hotkeys.json"
                            .into(),
                        blender_asset_path: "I:\\_GOOGLE DRIVE\\GOOGLE DRIVE RK\\THUMBNAIL\\3DASSETS"
                            .into(),
                        blender_output_path: "D:\\_GIGAPIXEL".into(),
                        api_url: "http://localhost:3000".into(),
                        emblem_font: "I:\\\\_GOOGLE DRIVE\\\\GOOGLE DRIVE RK\\\\THUMBNAIL\\\\3DASSETS\\\\fonts\\\\Bebas Neue Pro Bold.otf".into(),
                        custom_script_path: "I:\\_GOOGLE DRIVE\\GOOGLE DRIVE RK\\THUMBNAIL\\_ROOT\\customscripts".into(),
                        web_socket_url: "ws://localhost:7898/Server".into(),
                    };
                    let serialized = serde_json::to_string_pretty(&default_data).unwrap();
                    let mut file = File::create(&file_name).unwrap();
                    file.write_all(serialized.as_bytes()).unwrap();
                    file_name
                }
            };

            let result = match fs::read_to_string(conf_file) {
                Ok(content) => match serde_json::from_str::<Config>(&content) {
                    Ok(config) => match setting_key {
                        ConfigEnum::Pm2ConfigPath => config.pm2_config_path,
                        ConfigEnum::HotkeyConfigPath => config.hotkey_config_path,
                        ConfigEnum::BlenderAssetPath => config.blender_asset_path,
                        ConfigEnum::BlenderOutputPath => config.blender_output_path,
                        ConfigEnum::ApiUrl => config.api_url,
                        ConfigEnum::EmblemFont => config.emblem_font,
                        ConfigEnum::CustomScriptPath => config.custom_script_path,
                        ConfigEnum::WSUrl => config.web_socket_url,
                    },
                    Err(_) => "".into(),
                },
                Err(_) => "".into(),
            };
            result
        }
        None => "".into(),
    }
}

pub fn read_hotkeys(
    file_name: String,
    app_handle: &AppHandle,
) -> Result<Vec<MacroPadDataMulti>, String> {
    match app_handle.path_resolver().app_data_dir() {
        Some(path) => {
            let filepath = path.join("users").join(file_name);
            if fs::metadata(&filepath).is_ok() {
                match File::open(&filepath.to_string_lossy().to_string()) {
                    Ok(read) => {
                        let data: Vec<MacroPadDataMulti> =
                            serde_json::from_reader(read).expect("error reading files");
                        Ok(data)
                    }
                    Err(err) => Err(format!("Error deserializing JSON: {}", err)),
                }
            } else {
                Err(format!("File not found"))
            }
        }
        None => Err(format!("dir not found")),
    }
}
