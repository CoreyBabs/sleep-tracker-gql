use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};
use sqlx::sqlite::SqlitePoolOptions;
use db_types::*;

#[derive(Debug, Clone)]
pub struct DBManager {
    connection_pool: SqlitePool,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DbmSleep {
    pub sleep: DBSleep,
    pub tags: Option<Vec<DBTag>>
}

impl DBManager {
    pub async fn init(db_path: &str) -> Result<DBManager, sqlx::Error>{
        let db_doesnt_exist = !Sqlite::database_exists(db_path).await.unwrap_or(false);
        if db_doesnt_exist {
            let db_create_result = Sqlite::create_database(db_path).await;
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
        println!("db opened with {} connections.", dbm.connection_pool.size());

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

    pub async fn close_connection(&self) {
        self.connection_pool.close().await;
    }

    pub async fn insert_sleep(&self, night: &str, amount: f64, quality: i64) -> i64 {
        DBSleep::insert(&self.connection_pool, night, amount, quality).await.unwrap()
    }

    pub async fn get_sleep(&self, id: i64, include_tags: bool) -> Option<DbmSleep>  {
        let result = DBSleep::select_one(&self.connection_pool, id).await;
        
        let db_sleep = 
        match result {
            Ok(s) => s,
            Err(_) => return None
        };

        let mut sleep = DbmSleep { sleep: db_sleep, tags: None };

        if include_tags {
            let sleep_tags = DBSleepTags::select_by_sleep_id(&self.connection_pool, id).await;

            let tag_ids =
            match sleep_tags {
                Ok(st) => st.iter().map(|x| x.tag_id).collect(),
                Err(_) => return None
            };

            let tags = self.get_multiple_tags(tag_ids).await;
            sleep.tags = tags;
        }

        Some(sleep)
    }

    pub async fn get_all_sleeps(&self) -> Option<Vec<DbmSleep>>  {
        let result = DBSleep::select_all(&self.connection_pool).await;
        
        match result {
            Ok(s) => Some(s.into_iter().map(|x| DbmSleep { sleep: x, tags: None }).collect()),
            Err(_) => None
        }
    }

    // Note: Ideally a WHERE id IN clause would be used for this, however, that is not directly supported by
    // sqlx v0.6 so select all tags and filter them manually
    // TODO: Add tag support
    pub async fn get_mulitple_sleeps(&self, ids: Vec<i64>) -> Option<Vec<DbmSleep>> {
        let result = DBSleep::select_all(&self.connection_pool).await;

        match result {
            Ok(sleeps) => Some(sleeps.into_iter()
            .filter(|s| ids.contains(&s.id))
            .map(|d| DbmSleep { sleep: d, tags: None}).collect()),
            Err(_) => None
        }
    }

    pub async fn get_sleeps_by_tag(&self, tag_id: i64) ->  Option<Vec<DbmSleep>> {
        let sleep_tags = DBSleepTags::select_by_tag_id(&self.connection_pool, tag_id).await;

        let sleep_ids =
        match sleep_tags {
            Ok(st) => st.iter().map(|x| x.sleep_id).collect(),
            Err(_) => return None
        };

        self.get_mulitple_sleeps(sleep_ids).await
    }

    pub async fn update_sleep_amount(&self, id: i64, amount: f64) -> bool {
        DBSleep::update_amount(&self.connection_pool, id, amount).await
            .unwrap_or(false)
    }

    pub async fn update_sleep_quality(&self, id: i64, quality: i64) -> bool {
        DBSleep::update_quality(&self.connection_pool, id, quality).await
            .unwrap_or(false)
    }

    pub async fn delete_sleep(&self, id: i64) -> bool {
        DBSleep::delete(&self.connection_pool, id).await
            .unwrap_or(false)
    }

    pub async fn insert_tag(&self, name: &str, color: i64) -> i64 {
        DBTag::insert(&self.connection_pool, name, color).await
            .unwrap_or(-1)
    }

    pub async fn get_tag(&self, id: i64) -> Option<DBTag> {
        let result = DBTag::select_one(&self.connection_pool, id).await;
        
        match result {
            Ok(t) => Some(t),
            Err(_) => {
                None
            }
        } 
    }

    pub async fn get_all_tags(&self) -> Option<Vec<DBTag>> {
        let result = DBTag::select_all(&self.connection_pool).await;
        
        match result {
            Ok(t) => Some(t),
            Err(_) => {
                None
            }
        } 
    }

    pub async fn get_tags_by_sleep(&self, sleep_id: i64) ->  Option<Vec<DBTag>> {
        let sleep_tags = DBSleepTags::select_by_sleep_id(&self.connection_pool, sleep_id).await;

        let tag_ids =
        match sleep_tags {
            Ok(st) => st.iter().map(|x| x.tag_id).collect(),
            Err(_) => return None
        };

       self.get_multiple_tags(tag_ids).await
    }

    // Note: Ideally a WHERE id IN clause would be used for this, however, that is not directly supported by
    // sqlx v0.6 so select all tags and filter them manually
    pub async fn get_multiple_tags(&self, ids: Vec<i64>) -> Option<Vec<DBTag>> {
        let result = DBTag::select_all(&self.connection_pool).await;

        match result {
            Ok(tags) => {
                Some(tags.into_iter().filter(|t| ids.contains(&t.id)).collect())
            },
            Err(_) => {
                None
            }
        }
    }

    pub async fn update_tag_name(&self, id: i64, name: &str) -> bool {
        DBTag::update_name(&self.connection_pool, id, name).await
            .unwrap_or(false)
    }

    pub async fn update_tag_color(&self, id: i64, color: i64) -> bool {
        DBTag::update_color(&self.connection_pool, id, color).await
            .unwrap_or(false)
    }

    pub async fn delete_tag(&self, id: i64) -> bool {
        DBTag::delete(&self.connection_pool, id).await
            .unwrap_or(false)
    }

    pub async fn add_tags_to_sleep(&self, sleep_id: i64, tag_ids: Vec<i64>) -> bool {
        let mut result = true;
        for tag_id in tag_ids {
            match DBSleepTags::insert(&self.connection_pool, sleep_id, tag_id).await {
                Ok(_) => result = true,
                Err(_) => {
                    result = false;
                    break;
                }
            }  
        }

        result
    }

    pub async fn remove_tag_from_sleep(&self, sleep_id: i64, tag_id: i64) -> bool {
        DBSleepTags::delete(&self.connection_pool, sleep_id, tag_id).await
            .unwrap_or(false)
    }

    pub async fn insert_comment(&self, sleep_id: i64, comment: &str) -> i64 {
        DBComment::insert(&self.connection_pool, sleep_id, comment).await
            .unwrap_or(-1)
    }

    pub async fn get_comment(&self, comment_id: i64) -> Option<DBComment> {
        let result = DBComment::select_by_id(&self.connection_pool, comment_id).await;
        match result {
            Ok(c) => Some(c),
            Err(_) => {
                None
            }
        }
    }

    pub async fn get_comments_by_sleep(&self, sleep_id: i64) -> Option<Vec<DBComment>> {
        let comments = DBComment::select_by_sleep_id(&self.connection_pool, sleep_id).await;

        match comments {
            Ok(coms) => {
                Some(coms)
            },
            Err(_e) => {
                None
            }
        }
    }

    pub async fn update_comment(&self, comment_id: i64, comment: &str) -> bool {
        DBComment::update_comment(&self.connection_pool, comment_id, comment).await
            .unwrap_or(false)
    }

    pub async fn delete_comment(&self, id: i64) -> bool {
        DBComment::delete(&self.connection_pool, id).await
            .unwrap_or(false)
    }
}

mod db_migrations;
mod db_types;
pub mod db_tests;