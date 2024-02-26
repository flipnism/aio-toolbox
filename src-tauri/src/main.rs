// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod client;
mod macropad;
mod settings;
mod strct;
mod utils;
mod websocket;
use arboard::Clipboard;
use client::{
    add_thumbnail, convert_psd_to_png, create_emblem, delete_thumbnail, send_image_to_whatsapp,
    write_thumbnail,
};
use settings::{get_setting_for, read_hotkeys};

use crate::websocket::Message as Msg;
use global_hotkey::GlobalHotKeyEvent;
use macropad::MacroPadEvent;
use std::collections::HashMap;
use std::fmt::format;
use std::io::{BufRead, BufReader};
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use strct::{CommandArgs, JsonMessage, LoggingPayload, MacroPadDataMulti};
use websocket::{
    Event::Connect, Event::Disconnect, Event::Message, Message as MessageText, Responder,
};
use window_shadows::set_shadow;

use lazy_static::lazy_static;
use tauri::Manager;
use tauri::{AppHandle, Url};

lazy_static! {
    pub static ref CLIENTS: Mutex<HashMap<u64, Responder>> = Mutex::new(HashMap::new());
}
static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();
pub static PORT: u16 = 6969;

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

fn handle_message(message: JsonMessage) {
    for client in CLIENTS.lock().unwrap().values() {
        println!("{:?}", client);
    }
}
#[tokio::main]
async fn main() {
    let mut mpad = MacroPadEvent::new().await.expect("msg");
    let _hotkeys_manager = mpad.register();
    let global_hotkey_channel = GlobalHotKeyEvent::receiver();
    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();
            let _ = window.set_resizable(true);
            set_shadow(&window, true).expect("Unsupported platform!");

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

                            handle_message(message);
                        }
                    }
                }
            });

            tauri::async_runtime::spawn(async move {
                loop {
                    if let Ok(event) = global_hotkey_channel.try_recv() {
                        // println!("{:?}", &event);
                        let _ = &mpad.find_key(&event, &APP_HANDLE.get().unwrap()).await;
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![run_command, get_hotkey_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
