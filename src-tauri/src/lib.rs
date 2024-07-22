use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;
use reqwest::{header::AUTHORIZATION, Client};
use serde::{ Deserialize, Serialize };
use log::info;


const URL_GET_TOKEN: &str = "https://seld-lock.ru/auth/token/login/";
const URL_GET_DEVICES: &str = "https://seld-lock.ru/api/v1.0/devices/";

lazy_static! {
    static ref RESPONSE_TOKEN: Mutex<String> = Mutex::new("key".to_string());
    static ref CLIENT: Client = reqwest::Client::new();
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Token {
    auth_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Devices {
    id: i32,
    device_name: String,
    serial_num: String,
    status: String,
    settings: String,
    admin: String,
    sync: String,
    user: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Keys {
    id: i32,
    key: i64,
    used: String,
    time_start: String,
    time_end: String,
    selection: String,
    device: i32,
}


#[tauri::command]
async fn get_token(login: String, password: String) -> String {
    let mut map = HashMap::new();
    map.insert("password", password);
    map.insert("username", login);

    info!("{:?}", &map);

    let response_get_token = CLIENT.post(URL_GET_TOKEN)
    .header("Content-Type", "application/json")
    .json(&map)
    .send()
    .await
    .unwrap();

    if response_get_token.status() == 400 {
        return response_get_token.status().to_string();
    }

    info!("{:?}", response_get_token);

    let response_auth_token = response_get_token.json::<Token>().await.unwrap();

    info!("{}", response_auth_token.auth_token);

    *RESPONSE_TOKEN.lock().unwrap() = response_auth_token.auth_token;

    "".to_string()
}


#[tauri::command]
async fn get_devices() -> Vec<Devices> {
    let response_get_devices = CLIENT.get(URL_GET_DEVICES)
    .header(AUTHORIZATION, format!("Token {}", RESPONSE_TOKEN.lock().unwrap().clone()))
    .send()
    .await
    .unwrap();

    info!("OK");

    response_get_devices.json::<Vec<Devices>>().await.unwrap()
}


#[tauri::command]
async fn get_keys(serial_num: String) -> Vec<Keys> {
    let url_get_keys = format!("https://seld-lock.ru/api/v1.0/devices/{}/keys/", serial_num);
    info!("{}", &url_get_keys);

    let response_get_devices = CLIENT.get(url_get_keys)
    .header(AUTHORIZATION, format!("Token {}", RESPONSE_TOKEN.lock().unwrap().clone()))
    .send()
    .await
    .unwrap();

    info!("OK");

    response_get_devices.json::<Vec<Keys>>().await.unwrap()
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_token, get_devices, get_keys])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
