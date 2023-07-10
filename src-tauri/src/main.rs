// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error;

use futures::executor::block_on;
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;
use tauri::Manager;

use crate::command::db_initialize;
use crate::config::AppConfig;
use crate::database::create_db;
use crate::migrator::Migrator;

mod annotation_entities;
mod annotation_migrator;
mod command;
mod config;
mod database;
mod entities;
mod migrator;

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = confy::load_path::<AppConfig>("config.toml")?;
    confy::store_path("config.toml", config.clone())?;
    block_on(create_db(config.clone().db_uri))?;
    let db = block_on(Database::connect(config.clone().db_uri))?;
    block_on(Migrator::up(&db, None))?;
    block_on(db.close())?;
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![db_initialize])
        .manage(config)
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
