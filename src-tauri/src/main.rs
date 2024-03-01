// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod client;
mod macropad;
mod settings;
mod strct;
mod utils;
mod websocket;
use crate::strct::ConfigEnum;
use crate::websocket::Message as Msg;
use arboard::Clipboard;
use client::{
    add_thumbnail, convert_psd_to_png, create_emblem, delete_thumbnail, send_image_to_whatsapp,
    write_thumbnail,
};
use global_hotkey::GlobalHotKeyEvent;
use macropad::MacroPadEvent;
use reqwest::{self, Url};
use serde_json::{json, Value};
use settings::{get_setting_for, read_hotkeys};
use std::collections::HashMap;
use std::fmt::{format, Alignment};
use std::io::{BufRead, BufReader};
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use strct::{CommandArgs, JsonMessage, LoggingPayload, MacroPadDataMulti};
use websocket::{
    Event::Connect, Event::Disconnect, Event::Message, Message as MessageText, Responder,
};
use window_shadows::set_shadow;

use lazy_static::lazy_static;
use std::fs::{self, File};
use tauri::AppHandle;
use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
lazy_static! {
    pub static ref CLIENTS: Mutex<HashMap<u64, Responder>> = Mutex::new(HashMap::new());
}
static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();
pub static PORT: u16 = 7898; //;

#[tauri::command]
fn get_hotkey_config(
    filename: String,
    app_handle: AppHandle,
) -> Result<Vec<MacroPadDataMulti>, String> {
    match read_hotkeys(filename, &app_handle) {
        Ok(result) => Ok(result),
        Err(err) => Err(format!("failed {}", err)),
    }
}

#[tauri::command]
async fn execute_script(script_name: String) {
    let script_model = JsonMessage {
        fromserver: true,
        data_type: "customscript".into(),
        data: script_name,
        channel: Default::default(),
        image64: Default::default(),
        textdata: Default::default(),
    };

    handle_message(script_model).await;
}

#[tauri::command]
async fn ignore_cursor_events(app_handle: AppHandle, ignore: bool) -> Result<(), tauri::Error> {
    app_handle
        .get_window("main")
        .unwrap()
        .set_ignore_cursor_events(ignore)
}

#[tauri::command]
fn run_command(app_handle: AppHandle, argument: CommandArgs) -> Result<(), String> {
    let cmd = Command::new("pwsh")
        .args(argument.args)
        .stdout(Stdio::piped())
        .creation_flags(0x08000000)
        .spawn()
        .expect("error doing things");
    if let Some(stdout) = cmd.stdout {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let _ = app_handle.emit_all(
                "logger",
                &LoggingPayload {
                    message: line.unwrap(),
                },
            );
        }
    };
    Ok(())
}

#[tauri::command]
fn list_customscripts(app: AppHandle) -> Value {
    let paths = fs::read_dir(get_setting_for(ConfigEnum::CustomScriptPath, &app)).unwrap();
    let mut key_value_pairs = Vec::new();
    for path in paths {
        if let Ok(entry) = path {
            if let Some(extension) = entry.path().extension() {
                if extension == "json" {
                    if let Ok(file_content) = fs::read_to_string(entry.path()) {
                        if let Ok(json_data) = serde_json::from_str::<Value>(&file_content) {
                            if let Some(file_name) = entry.file_name().to_str() {
                                key_value_pairs.push((file_name.to_string(), json_data));
                            }
                        }
                    }
                }
            }
        }
    }
    json!(key_value_pairs)
}
#[tauri::command]
async fn send_socket_message(message: JsonMessage) {
    println!("message : {:?}", &message);
    let _ = handle_message(message).await;
}

async fn handle_message(msg: JsonMessage) {
    let app_handle = APP_HANDLE.get().unwrap();
    let mut message = msg;
    message.fromserver = true;
    let tamper_message = match message.data_type.as_str() {
        "clipboard" => {
            let mut clipboard = Clipboard::new().unwrap();
            clipboard.set_text(&message.data).unwrap();
            message
        }
        "filepath" => {
            send_image_to_whatsapp(&message.data, &message.channel, app_handle).await;
            message
        }
        "createthumb" => {
            add_thumbnail(message.data.as_str(), app_handle).await;
            message.fromserver = true;
            message
        }
        "deletethumb" => {
            delete_thumbnail(&message.data).await;
            message
        }
        "createemblem" => {
            let data = message.data.replace("\'", "\\\'").replace("\"", "\\\"");
            let new_emblem = create_emblem(data.as_str(), app_handle).await;
            message.data = new_emblem;
            message
        }
        "url" => {
            let image_url = &message.data.to_string();

            let url = Url::parse(&image_url).expect("Invalid URL");

            if let Some(filename) = url.path_segments().and_then(|segments| segments.last()) {
                let client = reqwest::Client::new();
                let response = client.get(image_url).send().await.unwrap();
                if response.status().is_success() {
                    // Open a file for writing with the extracted filename
                    let file_path = Path::new("D:\\_GIGAPIXEL").join(filename);
                    let mut file = File::create(&file_path).unwrap();
                    message.data = file_path.to_string_lossy().to_string();
                    // Copy the response body (image content) to the file
                    std::io::copy(&mut response.bytes().await.unwrap().as_ref(), &mut file)
                        .unwrap();

                    println!("Image downloaded successfully to: {:?}", &file_path);
                }
            } else {
                println!("Failed to retrieve filename from URL");
            }
            message
        }

        _ => message,
    };
    let _ = app_handle.emit_all("socket_message", &tamper_message);
    for client in CLIENTS.lock().unwrap().values() {
        match serde_json::to_string::<JsonMessage>(&tamper_message) {
            Ok(msg) => {
                client.send(MessageText::Text(msg.clone()));
            }
            Err(err) => {
                println!("error sending to client :{}", err)
            }
        }
    }
}

pub fn launch_settings_window(app: &tauri::AppHandle) {
    match app.get_window("settings") {
        Some(window) => {
            let _ = window.unminimize();
            let _ = window.set_focus();
        }
        None => {
            let window = tauri::WindowBuilder::new(
                app,
                "settings",
                tauri::WindowUrl::App("settings.html".into()),
            )
            .title("Settings!")
            .decorations(false)
            .inner_size(600.0, 400.0)
            .maximizable(false)
            .skip_taskbar(true)
            .build()
            .unwrap();

            set_shadow(&window, true).unwrap();
        }
    }
}

pub fn launch_preview_window(app: &tauri::AppHandle) {
    if app.get_window("preview").is_none() {
        tauri::WindowBuilder::new(app, "preview", tauri::WindowUrl::App("preview.html".into()))
            .title("Preview!")
            .decorations(false)
            .inner_size(10.0, 10.0)
            .maximizable(false)
            .resizable(false)
            .skip_taskbar(true)
            .build()
            .unwrap();
    }
}

fn setting_up_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let settings = CustomMenuItem::new("settings".to_string(), "Settings");
    let restart = CustomMenuItem::new("restart".to_string(), "Restart");
    let hotkeypreview = CustomMenuItem::new("hotkeypreview".to_string(), "Hotkey Preview!");
    let tray_menu = SystemTrayMenu::new()
        .add_item(settings)
        .add_item(hotkeypreview)
        .add_item(restart)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

#[tokio::main]
async fn main() {
    let system_tray = setting_up_tray();
    let mut mpad = MacroPadEvent::new().await.expect("msg");
    let _hotkeys_manager = mpad.register();
    let global_hotkey_channel = GlobalHotKeyEvent::receiver();
    tauri::Builder::default()
        .setup(move |app| {
            let ws = websocket::launch(PORT).expect("failed to launch websocket");
            let _ = &mpad.read_config_file(&app);
            APP_HANDLE.set(app.app_handle().clone()).unwrap();
            tauri::async_runtime::spawn(async move {
                loop {
                    match ws.poll_event() {
                        Connect(id, responder) => {
                            CLIENTS.lock().unwrap().insert(id, responder);
                        }
                        Disconnect(id) => {
                            CLIENTS.lock().unwrap().remove(&id);
                        }
                        Message(_id, msg) => {
                            let default_message = JsonMessage {
                                fromserver: true,
                                data_type: "".to_string(),
                                data: "Something went wrong".to_string(),
                                textdata: "".to_string(),
                                channel: "".to_string(),
                                image64: "".to_string(),
                            };
                            let message = match msg {
                                Msg::Text(message) => {
                                    match serde_json::from_str::<JsonMessage>(&message) {
                                        Ok(msg) => msg,
                                        Err(_) => default_message,
                                    }
                                }
                                _ => default_message,
                            };

                            handle_message(message).await;
                        }
                    }
                }
            });
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Ok(event) = global_hotkey_channel.try_recv() {
                        let _ = &mpad.find_key(&event, &APP_HANDLE.get().unwrap()).await;
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
            });

            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {}
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "settings" => {
                    launch_settings_window(app);
                }

                "hotkeypreview" => {
                    launch_preview_window(app);
                }
                "restart" => tauri::api::process::restart(&app.env()),
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    match window.is_visible() {
                        Ok(ok) => {
                            if ok {
                                window.hide().unwrap();
                            } else {
                                window.show().unwrap();
                            }
                        }
                        Err(_) => {
                            println!("error");
                        }
                    }
                }
                _ => {}
            },
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {}
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {}
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            run_command,
            get_hotkey_config,
            list_customscripts,
            execute_script,
            ignore_cursor_events,
            send_socket_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
