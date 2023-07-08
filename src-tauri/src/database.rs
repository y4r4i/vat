use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement};
use url::Url;

pub async fn connection(db_uri: String) -> Result<DatabaseConnection, DbErr> {
    Ok(Database::connect(db_uri).await?)
}

pub async fn create_db(db_uri: String) -> Result<(), DbErr> {
    let mut parsed_uri = Url::parse(&db_uri).unwrap();
    let name = parsed_uri.path()[1..].to_string();
    if parsed_uri.scheme() != "sqlite" {
        parsed_uri.set_path("");
    }
    println!("{}", parsed_uri.as_str());
    let db = connection(parsed_uri.to_string()).await?;
    match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", name),
            ))
            .await?;
            Database::connect(db_uri).await?
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
            Database::connect(db_uri).await?
        }
        DbBackend::Sqlite => db,
    };
    Ok(())
}
