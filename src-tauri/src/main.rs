// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::config::AppConfig;
use crate::database::create_db;
use futures::executor::block_on;
use std::error;

mod config;
mod database;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = confy::load_path::<AppConfig>("config.toml")?;
    confy::store_path("config.toml", config.clone())?;
    block_on(create_db(config.clone().db_uri))?;
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
