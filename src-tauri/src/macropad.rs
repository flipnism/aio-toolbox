use crate::{
    handle_message,
    strct::{
        DataMacro, JsonMessage, MPadPayload, MacroPadData, MacroPadDataMulti, ModPayload,
        RegisteredKey,
    },
    APP_HANDLE, CLIENTS, PORT,
};

use crate::websocket::Message as MessageText;
use enigo::{
    agent::{Agent, Token},
    Button, Enigo, Key, Settings,
};
use futures_util::StreamExt;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use serde::ser;
use serde_json::Value;
use std::{
    collections::HashMap,
    fmt::format,
    fs::{self, File},
    result::Result,
};
use std::{sync::Arc, time::Instant};
use tauri::{App, AppHandle, Manager};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct MacroPadEvent {
    pub registered_keys: Vec<RegisteredKey>,
    delay: u128,
    last_press: Instant,
    mod_hold: bool,
    mod_pressed: bool,
    macropad_config: Vec<MacroPadDataMulti>,
    layer_index: u32,
    delay_time: u128,
    mod_key: Code,
}

impl MacroPadEvent {
    pub async fn new() -> Result<Self, std::io::Error> {
        let stime = Instant::now();

        Ok(Self {
            registered_keys: Vec::new(),
            delay: 0,
            last_press: stime,
            mod_hold: false,
            mod_pressed: false,
            macropad_config: Vec::new(),
            layer_index: 0,
            delay_time: 300,
            mod_key: Code::End,
        })
    }

    pub fn read_config_file(&mut self, app: &App) {
        match app.path_resolver().app_data_dir() {
            Some(path) => {
                let config_path = path.join("users").join("macropad.json");
                let setting_path = path.join("users").join("global_config_file.json");
                if fs::metadata(&config_path).is_ok() {
                    let config_file =
                        File::open(&config_path.to_string_lossy().to_string()).unwrap();
                    self.macropad_config = serde_json::from_reader(config_file).expect("cant read");
                } else {
                    println!("File not found")
                }
                if fs::metadata(&setting_path).is_ok() {
                    let setting_file =
                        File::open(&setting_path.to_string_lossy().to_string()).unwrap();
                    let m: HashMap<String, Value> =
                        serde_json::from_reader(setting_file).expect("Error reading fille");
                    for (key, value) in m {
                        if key == "keypress_delay" {
                            self.delay_time = serde_json::from_value(value).expect("error parsing");
                        }
                    }
                }
            }
            None => {
                println!("Error")
            }
        }
    }

    pub fn register(&mut self) -> GlobalHotKeyManager {
        let hotkeys_manager = GlobalHotKeyManager::new().unwrap();
        let hotkey_rows = vec![
            Code::Numpad1, //Code::F13,
            Code::Numpad2, //Code::F14,
            Code::Numpad3, //Code::F15,
            Code::Numpad4, //Code::F16,
            Code::Numpad5, //Code::F17,
            Code::Numpad6, //Code::F18,
            Code::Numpad7, //Code::F19,
            Code::Numpad8, //Code::F20,
            Code::Numpad9, //Code::F21,
            Code::Numpad0, //Code::F22,
            Code::Insert,  //Code::F23,
            Code::End,     //Code::F24,
        ];
        for hotkey in hotkey_rows {
            let hkey = HotKey::new(Some(Modifiers::empty()), hotkey);
            let curkey: RegisteredKey = RegisteredKey {
                id: hkey.id(),
                key: hotkey.to_string(),
                mod_key: hotkey == self.mod_key,
            };
            self.registered_keys.push(curkey);
            hotkeys_manager.register(hkey).unwrap();
        }

        hotkeys_manager
    }

    pub async fn find_key(&mut self, event: &GlobalHotKeyEvent, handle: &AppHandle) {
        if let Some(modkey) = self.registered_keys.iter().find(|&s| s.mod_key) {
            if event.id == modkey.id {
                self.mod_pressed = event.state == HotKeyState::Pressed;

                if self.mod_pressed {
                    self.last_press = Instant::now();
                } else {
                    let stime = Instant::now();
                    self.delay = (stime - self.last_press).as_millis();
                    self.mod_hold = self.delay > self.delay_time;
                }
                self.on_mod_pressed(&handle);
                self.delay = self.delay.min(self.delay_time);
            }
        }
        if event.state == HotKeyState::Released {
            self.executekey(&event).await;
        }
    }

    fn find_by_key<'a>(
        &self,
        data: &'a MacroPadDataMulti,
        key_to_find: &str,
    ) -> Option<&'a MacroPadData> {
        for row in &data.data {
            for mc in row {
                if mc.key == key_to_find {
                    return Some(mc);
                }
            }
        }
        None
    }
    fn processing_macro(&self, macro_data: &Vec<DataMacro>) {
        let mut enigo_keys = Vec::new();
        for entry in macro_data {
            match entry.key.as_str() {
                "Control" => enigo_keys.push(Token::Key(
                    Key::Control,
                    if entry.down {
                        enigo::Direction::Press
                    } else {
                        enigo::Direction::Release
                    },
                )),
                "Shift" => enigo_keys.push(Token::Key(
                    Key::Shift,
                    if entry.down {
                        enigo::Direction::Press
                    } else {
                        enigo::Direction::Release
                    },
                )),
                "Alt" => enigo_keys.push(Token::Key(
                    Key::Alt,
                    if entry.down {
                        enigo::Direction::Press
                    } else {
                        enigo::Direction::Release
                    },
                )),
                "Meta" => enigo_keys.push(Token::Key(
                    Key::Meta,
                    if entry.down {
                        enigo::Direction::Press
                    } else {
                        enigo::Direction::Release
                    },
                )),
                _ => enigo_keys.push(Token::Key(
                    Key::Unicode(entry.key.to_lowercase().chars().next().unwrap()),
                    if entry.down {
                        enigo::Direction::Press
                    } else {
                        enigo::Direction::Release
                    },
                )),
            };
        }
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        for token in enigo_keys {
            enigo.execute(&token).unwrap();
        }
    }

    async fn processing_payload(&self, data: &MacroPadData) {
        if !data.data_mode.is_empty() {
            match data.data_mode.as_str() {
                "action" => {
                    if let Some(message) = &data.data {
                        let _ = APP_HANDLE
                            .get()
                            .unwrap()
                            .emit_all("socket_message", &message);
                        for client in CLIENTS.lock().unwrap().values() {
                            match serde_json::to_string::<JsonMessage>(message) {
                                Ok(msg) => {
                                    client.send(MessageText::Text(msg.clone()));
                                }
                                Err(err) => {
                                    println!("error sending to client :{}", err)
                                }
                            }
                        }

                        // for (&_uid, tx) in GLOBAL_USER.get().unwrap().read().await.iter() {
                        //     let newmessage = serde_json::to_string(_jsonmessage).unwrap();

                        //     if let Err(_disconnected) = tx.send(Message::text(newmessage.clone())) {
                        //         println!("failed parsing message: {}", _disconnected);
                        //     }
                        // }
                        // let global_map = GLOBAL_MAP.lock().unwrap();
                        // for client in global_map.values() {
                        //     client[]
                        //         .send(Message::Text(serde_json::to_string(&jsonmessage).unwrap()));
                        // }
                    }
                }
                "macro" => match &data.data_macro {
                    Some(data) => self.processing_macro(&data),
                    None => {}
                },
                _ => {}
            }
        }
    }

    async fn send_payload(&self, payload: &MPadPayload) {
        if let Some(macropad_data_layer) = self
            .macropad_config
            .iter()
            .find(|&id| id.id == self.layer_index)
        {
            if let Some(payload_to_send) = self.find_by_key(macropad_data_layer, &payload.key) {
                self.processing_payload(payload_to_send).await;
            };
        }
    }

    fn on_mod_pressed(&mut self, handle: &AppHandle) {
        if !self.mod_pressed && self.mod_hold {
            self.mod_hold = false;

            self.layer_index = (self.layer_index + 2) % 3;

            let _ = &handle.emit_all(
                "mod-event",
                &ModPayload {
                    layer_index: &self.layer_index,
                },
            );
        }
        if self.mod_pressed && !self.mod_hold {
            self.layer_index = (self.layer_index + 1) % 3;
            let _ = &handle.emit_all(
                "mod-event",
                &ModPayload {
                    layer_index: &self.layer_index,
                },
            );
        }
        if self.mod_pressed && self.mod_hold {
            //println!("press on hold");
        }
    }
    async fn executekey(&self, event: &GlobalHotKeyEvent) {
        if let Some(key) = self
            .registered_keys
            .iter()
            .find(|r| r.id == event.id && !r.mod_key)
        {
            let payload = MPadPayload {
                key: key.key.to_string(),
                is_pressed: event.state == HotKeyState::Pressed,
                is_mod: self.mod_pressed,
            };
            self.send_payload(&payload).await;

            //let _ = handle.emit_all("macropad-pressed", &payload);
        }
    }
}
