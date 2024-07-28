use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};
use std::env;
fn main() {
    let db_url: String =
        env::var("DATABASE_URL").expect("Environment variable 'DATABASE_URL' not found");
    let db_name: String =
        env::var("DATABASE_NAME").expect("Environment variable 'DATABASE_NAME' not found");
    if let Err(err) = block_on(run(&db_url, &db_name)) {
        println!("Database error: {}", err);
    }
}

async fn run(db_url: &str, db_name: &str) -> Result<(), DbErr> {
    let db = Database::connect(db_url).await?;

    let _db = &match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(db.get_database_backend(), format!("CREATE DATABASE IF NOT EXISTS {}", db_name))).await?;
            let url = format!("{}/{}",db_url, db_name);

            Database::connect(url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(db.get_database_backend(), format!("DROP DATABASE IF EXISTS \"{}\" ", db_name))).await?;
            db.execute(Statement::from_string(db.get_database_backend(), format!("CREATE DATABASE \"{}\" ", db_name))).await?;
            let url = format!("{}/{}",db_url, db_name);

            Database::connect(url).await?
        }
        DbBackend::Sqlite => {
            db
        }
    };
    Ok(())
}
