use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};
use sqlx::sqlite::SqlitePoolOptions;
use db_types::*;

pub struct DBManager {
    connection_pool: SqlitePool,
    last_error: String
}

pub struct Sleep {
    sleep: DBSleep,
    tags: Option<Vec<DBTag>>
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

        let dbm = DBManager { connection_pool, last_error: String::from("")};
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

    pub fn insert_sleep(&mut self, night: String, amount: f64, quality: i64) -> i64 {
        DBSleep::insert(&self.connection_pool, night, amount, quality)
            .unwrap_or_else(|e|
            {
                self.last_error = e.to_string();
                -1
            })
        }

    pub fn get_sleep(&mut self, id: i64, include_tags: bool) -> Option<Sleep>  {
        let result = DBSleep::select_one(&self.connection_pool, id);
        
        let db_sleep: DBSleep;  
        match result {
            Ok(s) => db_sleep = s,
            Err(e) => { 
                self.last_error = e.to_string();
                return None
            }
        }

        let mut sleep = Sleep { sleep: db_sleep, tags: None };

        if include_tags {
            let sleep_tags = DBSleepTags::select_by_sleep_id(&self.connection_pool, id);

            let tag_ids: Vec<i64>;
            match sleep_tags {
                Ok(st) => tag_ids = st.iter().map(|x| x.tag_id).collect(),
                Err(e) => {
                    self.last_error = e.to_string();
                    return None
                }
            }

            let tags = self.get_multiple_tags(tag_ids);
            sleep.tags = tags;
        }

        Some(sleep)
    }

    pub fn get_all_sleeps(&mut self) -> Option<Vec<Sleep>>  {
        let result = DBSleep::select_all(&self.connection_pool);
        
        match result {
            Ok(s) => Some(s.into_iter().map(|x| Sleep { sleep: x, tags: None }).collect()),
            Err(e) => { 
                self.last_error = e.to_string();
                return None
            }
        }
    }

    // Note: Ideally a WHERE id IN clause would be used for this, however, that is not directly supported by
    // sqlx v0.6 so select all tags and filter them manually
    // TODO: Add tag support
    pub fn get_mulitple_sleeps(&mut self, ids: Vec<i64>) -> Option<Vec<Sleep>> {
        let result = DBSleep::select_all(&self.connection_pool);

        match result {
            Ok(sleeps) => Some(sleeps.into_iter()
            .take_while(|s| ids.contains(&s.id))
            .map(|d| Sleep { sleep: d, tags: None}).collect()),
            Err(e) => {
                self.last_error = e.to_string();
                None
            }
        }
    }

    pub fn get_sleeps_by_tag(&mut self, tag_id: i64) ->  Option<Vec<Sleep>> {
        let sleep_tags = DBSleepTags::select_by_tag_id(&self.connection_pool, tag_id);

        let sleep_ids: Vec<i64>;
        match sleep_tags {
            Ok(st) => sleep_ids = st.iter().map(|x| x.sleep_id).collect(),
            Err(e) => {
                self.last_error = e.to_string();
                return None
            }
        }

        self.get_mulitple_sleeps(sleep_ids)
    }

    pub fn update_sleep_amount(&mut self, id: i64, amount: f64) -> bool {
        DBSleep::update_amount(&self.connection_pool, id, amount)
            .unwrap_or_else(|e| {
                self.last_error = e.to_string();
                false
            })
    }

    pub fn update_sleep_quality(&mut self, id: i64, quality: i64) -> bool {
        DBSleep::update_quality(&self.connection_pool, id, quality)
            .unwrap_or_else(|e| {
                self.last_error = e.to_string();
                false
            })
    }

    pub fn delete_sleep(&mut self, id: i64) -> bool {
        DBSleep::delete(&self.connection_pool, id)
            .unwrap_or_else(|e| {
                self.last_error = e.to_string();
                false
            })
    }

    pub fn insert_tag(&mut self, name: &str, color: i64) -> i64 {
        DBTag::insert(&self.connection_pool, name, color)
            .unwrap_or_else(|e|
            {
                self.last_error = e.to_string();
                -1
            })
    }

    pub fn get_tag(&mut self, id: i64) -> Option<DBTag> {
        let result = DBTag::select_one(&self.connection_pool, id);
        
        match result {
            Ok(t) => Some(t),
            Err(e) => {
                self.last_error = e.to_string();
                None
            }
        } 
    }

    pub fn get_all_tags(&mut self) -> Option<Vec<DBTag>> {
        let result = DBTag::select_all(&self.connection_pool);
        
        match result {
            Ok(t) => Some(t),
            Err(e) => {
                self.last_error = e.to_string();
                None
            }
        } 
    }

    // Note: Ideally a WHERE id IN clause would be used for this, however, that is not directly supported by
    // sqlx v0.6 so select all tags and filter them manually
    pub fn get_multiple_tags(&mut self, ids: Vec<i64>) -> Option<Vec<DBTag>> {
        let result = DBTag::select_all(&self.connection_pool);

        match result {
            Ok(tags) => Some(tags.into_iter().take_while(|t| ids.contains(&t.id)).collect()),
            Err(e) => {
                self.last_error = e.to_string();
                None
            }
        }
    }

    pub fn update_tag_name(&mut self, id: i64, name: &str) -> bool {
        DBTag::update_name(&self.connection_pool, id, name)
            .unwrap_or_else(|e| {
                self.last_error = e.to_string();
                false
            })
    }

    pub fn update_tag_color(&mut self, id: i64, color: i64) -> bool {
        DBTag::update_color(&self.connection_pool, id, color)
            .unwrap_or_else(|e| {
                self.last_error = e.to_string();
                false
            })
    }

    pub fn delete_tag(&mut self, id: i64) -> bool {
        DBTag::delete(&self.connection_pool, id)
            .unwrap_or_else(|e| {
                self.last_error = e.to_string();
                false
            })
    }

    pub fn add_tag_to_sleep(&mut self, sleep_id: i64, tag_ids: Vec<i64>) -> bool {
        let mut result = true;
        for tag_id in tag_ids {
            match DBSleepTags::insert(&self.connection_pool, sleep_id, tag_id) {
                Ok(_) => result = true,
                Err(e) => {
                    self.last_error = e.to_string();
                    result = false;
                    break;
                }
            }  
        }

        result
    }

    pub fn remove_tag_from_sleep(&mut self, sleep_id: i64, tag_id: i64) -> bool {
        DBSleepTags::delete(&self.connection_pool, sleep_id, tag_id)
            .unwrap_or_else(|e| {
                self.last_error = e.to_string();
                false
            })
    }
}

mod db_migrations;
mod db_types;
mod db_queries;