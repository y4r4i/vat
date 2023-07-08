use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement};
use url::Url;

fn create_uri(parsed_url: Url) -> String {
    let auth = if !parsed_url.username().is_empty() && parsed_url.password().is_some() {
        format!(
            "{}:{}@",
            parsed_url.username(),
            parsed_url.password().unwrap()
        )
    } else {
        ""
    };
    let host = parsed_url.host().unwrap().to_string();
    let port = parsed_url.port();
    let mut uri = parsed_url.scheme();
    uri += if uri == "sqlite" { ":" } else { "://" };
    uri += auth;
    uri += host;
    if port.is_some() {
        uri += port;
    };
    if parsed_url.query().is_some() {
        uri += "?" + parsed_url.query().unwrap();
    }
    uri.to_string()
}

pub async fn connection(db_uri: String) -> Result<DatabaseConnection, DbErr> {
    Ok(Database::connect(db_uri).await?)
}

pub async fn create_db(db_uri: String) -> Result<(), DbErr> {
    let parsed_uri = Url::parse(&db_uri).unwrap();
    let uri = create_uri(parsed_uri.clone());
    let name = parsed_uri.path()[1..].to_string();
    let db = connection(create_uri(parsed_uri.clone())).await?;
    match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", name),
            ))
            .await?;
            let url = format!("{}/{}", uri, name);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", name),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", name),
            ))
            .await?;
            let url = format!("{}/{}", uri, name);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };
    Ok(())
}
