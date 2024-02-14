// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use serde::Serialize;


#[derive(Serialize)]
struct Car {
    name: String,
    number: String,
    owner: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_car() -> Car {
    Car {
        name: String::from("Mercedes"),
        number: String::from("5022A7"),
        owner: String::from("Горбунов"),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_car])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}