// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod client;
mod settings;
mod strct;
mod utils;
mod websocket;
use crate::strct::ConfigEnum;
use crate::websocket::Message as Msg;
use arboard::Clipboard;
use client::{add_thumbnail, create_emblem, delete_thumbnail, send_image_to_whatsapp};
use global_hotkey::GlobalHotKeyEvent;
use lazy_static::lazy_static;
use macropad::config::{
    read_action_lists, read_app_config, read_macropad_config, read_script_lists, write_app_config,
};
use macropad::model::MacropadData;
use macropad::{Event, MacropPad};
use reqwest::{self, Url};
use serde_json::{json, Value};
use settings::{get_setting_for, read_hotkeys};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Mutex, OnceLock};
use std::thread::sleep;
use std::time::Duration;
use strct::{CommandArgs, JsonMessage, LoggingPayload, MacroPadDataMulti};
use tauri::async_runtime::block_on;
use tauri::AppHandle;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::{CustomMenuItem, Manager};
use tauri::{SystemTrayMenu, SystemTrayMenuItem};
use websocket::{
    Event::Connect, Event::Disconnect, Event::Message, Message as MessageText, Responder,
};
use window_shadows::set_shadow;
lazy_static! {
    pub static ref CLIENTS: Mutex<HashMap<u64, Responder>> = Mutex::new(HashMap::new());
}
static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

pub static PORT: u16 = 7898;
//pub static PORT: u16 = 6969;

use enigo::{Enigo, Mouse, Settings};
#[tauri::command]
fn mouse_location() -> (i32, i32) {
    let enigo = Enigo::new(&Settings::default()).unwrap();
    match Enigo::location(&enigo) {
        Ok(x) => x,
        Err(_) => (0, 0),
    }
}

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
fn update_config(app: AppHandle) {
    let config = read_app_config(&app.path_resolver().app_data_dir().unwrap()).unwrap();
    match app.get_window("main") {
        Some(window) => {
            let _ = window.set_ignore_cursor_events(config.click_throught);
            let _ = window.set_always_on_top(config.always_on_top);
            let _ = app.emit_all("update_config", true);
        }
        None => {}
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
    let _ = handle_message(message).await;
}
#[tauri::command]
fn action_lists() -> Vec<String> {
    let appdir = APP_HANDLE
        .get()
        .unwrap()
        .path_resolver()
        .app_data_dir()
        .unwrap();
    read_action_lists(&appdir)
}
#[tauri::command]
fn script_lists() -> Vec<String> {
    let appdir = APP_HANDLE
        .get()
        .unwrap()
        .path_resolver()
        .app_data_dir()
        .unwrap();
    read_script_lists(&appdir)
}
#[tauri::command]
fn filter_keys(key: &str) -> Vec<MacropadData> {
    let appdir = APP_HANDLE
        .get()
        .unwrap()
        .path_resolver()
        .app_data_dir()
        .unwrap();
    match read_macropad_config(&appdir) {
        Ok(alldata) => alldata
            .into_iter()
            .filter(|data| data.key_1 == Some(key.to_string()))
            .filter(|data| data.key_2 != Some("".into()))
            .collect(),
        Err(_) => Vec::new(),
    }
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

                    //println!("Image downloaded successfully to: {:?}", &file_path);
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
            .disable_file_drop_handler()
            .inner_size(1000.0, 800.0)
            .maximizable(false)
            .build()
            .unwrap();
            window.on_window_event(|event| match event {
                tauri::WindowEvent::Resized(..) => {
                    sleep(Duration::from_millis(1));
                }
                _ => {}
            });
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
    let tray_menu = SystemTrayMenu::new()
        .add_item(settings)
        .add_item(restart)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

#[tokio::main]
async fn main() {
    let system_tray = setting_up_tray();
    let mut mpad = MacropPad::new().expect("msg");
    let _hotkeys_manager = &mpad.register();
    let global_hotkey_channel = GlobalHotKeyEvent::receiver();
    tauri::Builder::default()
        .setup(move |app| {
            match app.get_window("main") {
                Some(window) => {
                    let _ = window.set_skip_taskbar(true);

                    window.on_window_event(|event| match event {
                        tauri::WindowEvent::Resized(..) => {
                            sleep(Duration::from_millis(1));
                        }
                        tauri::WindowEvent::Moved(..) => {
                            sleep(Duration::from_millis(1));
                        }
                        _ => {}
                    });
                }
                None => {}
            }
            let ws = websocket::launch(PORT).expect("failed to launch websocket");
            APP_HANDLE.set(app.app_handle().clone()).unwrap();
            update_config(app.app_handle());
            let _ = &mpad
                .update_macropad_configs(app.app_handle().path_resolver().app_data_dir().unwrap());

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
                let _ = &mpad.event_emitter.on("event", |v: Event| match v {
                    Event::PayloadEvent(c, pp) => {
                        let _ = APP_HANDLE.get().unwrap().emit_all(c.as_str(), pp);
                    }
                    Event::ModEvent(c, pp) => {
                        let _ = APP_HANDLE.get().unwrap().emit_all(c.as_str(), pp);
                    }
                    Event::ActionEvent(ae) => {
                        let mut script_model = JsonMessage::default();
                        script_model.fromserver = true;
                        script_model.data_type = ae.action_type;
                        script_model.data = ae.data;

                        let _ = block_on(async {
                            handle_message(script_model).await;
                        });
                    }
                    Event::CustomScriptEvent(cse) => {
                        let script_name = cse.data.replace(".json", "");

                        let mut script_model = JsonMessage::default();
                        script_model.fromserver = true;
                        script_model.data_type = "customscript".into();
                        script_model.data = script_name;

                        let _ = block_on(async {
                            handle_message(script_model).await;
                        });
                    }
                    Event::AppFunctionEvent(func) => {
                        let _ = APP_HANDLE.get().unwrap().emit_all("func_event", func);
                    }
                });

                loop {
                    if let Ok(event) = global_hotkey_channel.try_recv() {
                        let _ = &mpad.process_key(&event);
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
                "clicktru" => match app.get_window("main") {
                    Some(window) => {
                        let appdir = APP_HANDLE
                            .get()
                            .unwrap()
                            .path_resolver()
                            .app_data_dir()
                            .unwrap();
                        let mut config = read_app_config(&appdir).unwrap();
                        config.click_throught = !config.click_throught;
                        let _ = write_app_config(&appdir, config.clone());
                        let item_handle = app.tray_handle().get_item(&id);
                        let _ = item_handle
                            .set_title(format!("Click throught : {}", &config.click_throught))
                            .unwrap();
                        let _ = window.set_ignore_cursor_events(config.click_throught);
                    }
                    None => println!("not found window"),
                },

                "hotkeypreview" => {
                    //launch_preview_window(app);
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
                            println!("error error");
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
            send_socket_message,
            update_config,
            action_lists,
            filter_keys,
            script_lists,
            mouse_location
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
