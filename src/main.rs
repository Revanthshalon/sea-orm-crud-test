use futures::executor::block_on;
use sea_orm::*;
use std::env;

mod entities;
use entities::{prelude::*, *};

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

    let db = &match db.get_database_backend() {
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
    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set("Happy Bakery".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let _res = Bakery::insert(happy_bakery).exec(db).await?;

    let la_boulangerie = bakery::ActiveModel {
        name: ActiveValue::Set("La Boulangerie".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let bakery_res = Bakery::insert(la_boulangerie).exec(db).await?;
    for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
        let chef = chef::ActiveModel {
            name: ActiveValue::Set(chef_name.to_owned()),
            bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
            ..Default::default()
        };
        Chef::insert(chef).exec(db).await?;
    }
    // println!("{:?}", bakery_res.last_insert_id);
    Ok(())
}
