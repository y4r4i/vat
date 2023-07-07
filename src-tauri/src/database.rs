use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement};
use url::Url;

fn create_uri(parsed_url: Url) -> String {
    return if parsed_url.scheme() != "sqlite" {
        if parsed_url.port().is_none() {
            format!(
                "{}://{}:{}@{}",
                parsed_url.scheme(),
                parsed_url.username(),
                parsed_url.password().unwrap(),
                parsed_url.host().unwrap()
            )
        } else {
            format!(
                "{}://{}:{}@{}:{}",
                parsed_url.scheme(),
                parsed_url.username(),
                parsed_url.password().unwrap(),
                parsed_url.host().unwrap(),
                parsed_url.port().unwrap()
            )
        }
    } else {
        format!("{}:{}", parsed_url.scheme(), parsed_url.path())
    };
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
