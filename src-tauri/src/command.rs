use crate::annotation_migrator::AnnotationMigrator;
use crate::config::AppConfig;
use crate::database::create_db;
use crate::entities::destination;
use crate::entities::prelude::*;
use sea_orm::{ActiveValue, Database, EntityTrait};
use sea_orm_migration::MigratorTrait;
use tauri::State;
use url::Url;
use uuid::Uuid;

#[tauri::command]
pub async fn db_initialize(
    config: State<'_, AppConfig>,
    db_uri: String,
    name: String,
) -> Result<i8, String> {
    let parsed_uri = Url::parse(&db_uri.clone());
    if parsed_uri.clone().is_err() {
        return Err(String::from("URIが間違っています。"));
    }
    if parsed_uri.clone().unwrap().scheme() != "sqlite" {
        parsed_uri.clone().unwrap().set_path("");
    }
    let conn = Database::connect(parsed_uri.unwrap().to_string()).await;
    if conn.is_err() {
        return Err(String::from("データベースに接続できませんでした。"));
    }
    let create_status = create_db(db_uri.clone()).await;
    if create_status.is_err() {
        return Err(String::from("データベースの作成に失敗しました。"));
    }
    let db = Database::connect(db_uri.clone()).await;
    if db.is_err() {
        return Err(String::from("作成したデータベースの接続に失敗しました。"));
    }
    let db = db.unwrap();
    let migration_status = AnnotationMigrator::up(&db, None).await;
    if migration_status.is_err() {
        return Err(String::from("マイグレーションに失敗しました。"));
    }
    let close_status = db.close().await;
    if close_status.is_err() {
        return Err(String::from("切断に失敗しました。1"));
    }
    let uri = &config.db_uri.as_str();
    let db = Database::connect(uri.to_string()).await;
    if db.is_err() {
        return Err(String::from("データベースの接続に失敗しました。"));
    }
    let db = db.unwrap();
    let destination = destination::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4().to_string()),
        name: ActiveValue::Set(name),
        uri: ActiveValue::Set(db_uri),
    };
    let result = Destination::insert(destination).exec(&db).await;
    let close_status = db.close().await;
    if close_status.is_err() {
        return Err(String::from("切断に失敗しました。2"));
    }
    if result.is_err() {
        return Err(String::from("すでに登録済みです。"));
    }
    return Ok(0);
}
