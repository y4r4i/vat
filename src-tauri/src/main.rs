// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error;

use futures::executor::block_on;
use sea_orm_migration::MigratorTrait;

use crate::config::AppConfig;
use crate::database::{connection, create_db};
use crate::migrator::Migrator;

mod annotation_migrator;
mod config;
mod database;
mod migrator;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = confy::load_path::<AppConfig>("config.toml")?;
    confy::store_path("config.toml", config.clone())?;
    block_on(create_db(config.clone().db_uri))?;
    let db = block_on(connection(config.clone().db_uri))?;
    block_on(Migrator::up(&db, None))?;
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
