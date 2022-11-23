use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};
use sqlx::sqlite::SqlitePoolOptions;

pub struct DBManager {
    pub connection_pool: SqlitePool
}

impl DBManager {
    pub async fn init(db_path: &str) -> Result<DBManager, sqlx::Error>{
        let db_doesnt_exist = !Sqlite::database_exists(&db_path).await.unwrap_or(false);
        if db_doesnt_exist {
            let db_create_result = Sqlite::create_database(&db_path).await;
            match db_create_result {
                Ok(_) => println!("DB Created!"),
                Err(e) => {
                    println!("Unable to create DB! {}", e);
                    return Err(e);
                }
            }
        }
        else {
            println!("DB Exists!");
        }

        let connection_pool = SqlitePoolOptions::new()
            .max_connections(4)
            .connect(db_path).await?;

        let dbm = DBManager { connection_pool };

        if db_doesnt_exist {
            match db_migrations::initalize_db(&dbm.connection_pool).await {
                Ok(_) => println!("DB Initalized!"),
                Err(e) => {
                    println!("Unable to initalize DB! {}", e);
                    return Err(e);
                }
            }
        }

        Ok(dbm)
    }
}

mod db_migrations;