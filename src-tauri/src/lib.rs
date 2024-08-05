use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;
use reqwest::{ header::{AUTHORIZATION, CONTENT_TYPE}, Client };
use serde::{ Deserialize, Serialize };
use reqwest;


lazy_static! {
    static ref RESPONSE_TOKEN: Mutex<String> = Mutex::new("auth_token".to_string());
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Keys {
    id: Option<i32>,
    key: i64,
    used: String,
    time_end: String,
    selection: String,
    device: i32,
}


#[tauri::command]
async fn get_token(login: String, password: String) -> String {
    let mut map = HashMap::new();
    map.insert("password", password);
    map.insert("username", login);

    let response_get_token = CLIENT.post("https://seld-lock.ru/auth/token/login/")
    .header(CONTENT_TYPE, "application/json")
    .json(&map)
    .send()
    .await
    .unwrap();

    if response_get_token.status() == 400 {
        return response_get_token.status().to_string();
    }

    let response_auth_token = response_get_token.json::<Token>().await.unwrap();

    *RESPONSE_TOKEN.lock().unwrap() = response_auth_token.auth_token;

    "".to_string()
}


#[tauri::command]
async fn get_devices() -> Vec<Devices> {
    let response_get_devices = CLIENT.get("https://seld-lock.ru/api/v1.0/devices/")
    .header(AUTHORIZATION, format!("Token {}", RESPONSE_TOKEN.lock().unwrap().clone()))
    .send()
    .await
    .unwrap();

    response_get_devices.json::<Vec<Devices>>().await.unwrap()
}


#[tauri::command]
async fn get_keys(serial_num: String) -> Vec<Keys> {
    let url_get_keys = format!("https://seld-lock.ru/api/v1.0/devices/{}/keys/", serial_num);

    let response_get_devices = CLIENT.get(url_get_keys)
    .header(AUTHORIZATION, format!("Token {}", RESPONSE_TOKEN.lock().unwrap().clone()))
    .send()
    .await
    .unwrap();

    response_get_devices.json::<Vec<Keys>>().await.unwrap()
}


#[tauri::command]
async fn update_device(serial_num: String, change_param: String, value: String) {
    let mut map = HashMap::new();

    match change_param.as_str() {
        "device_name" => map.insert("device_name", value),
        "status" => map.insert("status", value),
        "admin" => map.insert("admin", value),
        _ => None
    };

    let url_update_device = format!("https://seld-lock.ru/api/v1.0/devices/{}/", serial_num);

    let _request_update_device = CLIENT.patch(url_update_device)
    .header(AUTHORIZATION, format!("Token {}", RESPONSE_TOKEN.lock().unwrap().clone()))
    .header(CONTENT_TYPE, "application/json")
    .json(&map)
    .send()
    .await
    .unwrap();
}


#[tauri::command]
async fn create_key(form: Keys, serial_num: String) {
    let url_create_keys = format!("https://seld-lock.ru/api/v1.0/devices/{}/keys/", serial_num);

    let _response_create_devices = CLIENT.post(url_create_keys)
    .header(AUTHORIZATION, format!("Token {}", RESPONSE_TOKEN.lock().unwrap().clone()))
    .header(CONTENT_TYPE, "application/json")
    .json(&form)
    .send()
    .await
    .unwrap();
}


#[tauri::command]
async fn delete_key(pk: i32, serial_num: String) {
    let url_delete_keys = format!("https://seld-lock.ru/api/v1.0/devices/{}/keys/{}/", serial_num, pk);

    let _response_delete_devices = CLIENT.delete(url_delete_keys)
    .header(AUTHORIZATION, format!("Token {}", RESPONSE_TOKEN.lock().unwrap().clone()))
    .send()
    .await
    .unwrap();
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_token, get_devices, get_keys, update_device, create_key, delete_key])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
