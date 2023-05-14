/// Module that manages the database connection, queries and mutations.

use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};
use sqlx::sqlite::SqlitePoolOptions;
use db_types::*;

/// Struct to manage the connection pool to the sqlite database
/// Also provides an interface to interact with the db with queries and mutations
#[derive(Debug, Clone)]
pub struct DBManager {
    /// sqlx connection pool to a sqlite database
    connection_pool: SqlitePool,
}

/// An intermediate representation of a sleep struct
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DbmSleep {
    /// Sleep from the database
    pub sleep: DBSleep,
    /// Optional vector of database tags associated to the sleep
    pub tags: Option<Vec<DBTag>>
}

impl DBManager {
    /// Returns a result containing either a DBManager or sqlx error
    /// Checks if the provided database exists, and if it doesn't
    /// it will create a new one and migrate to the current schema.
    /// 
    /// # Arguments
    ///
    /// * `db_path` - A string slice that holds the path to the sqlite db file
    ///
    pub async fn init(db_path: &str) -> Result<DBManager, sqlx::Error> {
        // Checks if db exists, and creates one if it doesn't
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

        // Migrate db to current schema if it did not exist
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

    /// Closes all of the connections in the connection pool. Maybe unneccessary
    /// sqlx might close connections on drop, but I have not confirmed that yet
    pub async fn close_connection(&self) {
        self.connection_pool.close().await;
    }

    /// Adds a night, amount and quality to the sleep table in the database.
    /// Returns the pk of the newly added row.
    /// 
    /// # Arguments
    /// 
    /// * `night` - A string slice representing the date. Should be in yyyy-mm-dd format
    /// * `amount` - A float representing how much time was spent sleeping during the night
    /// * `quality` - An int representing the quality of sleep on a night
    /// 
    /// # Examples
    /// 
    /// let pk = insert_sleep("2023-05-13", 7.5, 5).await;
    pub async fn insert_sleep(&self, night: &str, amount: f64, quality: i64) -> i64 {
        DBSleep::insert(&self.connection_pool, night, amount, quality).await.unwrap()
    }

    /// Gets a sleep from the database with the given id.
    /// Optionally includes all tags associated to the sleep.
    /// Returns Some [DbmSleep](DbmSleep) if the sleep was queried successfully, otherwise None
    /// 
    /// # Arguments
    /// 
    /// * `id` - the pk of the sleep to query
    /// * `include_tags` - boolean dictating if associated tags should also be queried
    /// 
    /// # Examples
    /// 
    /// let sleep = get_sleep(1, false).await;
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

    /// Queries all sleeps in the database
    /// Returns all of the [sleeps](DbmSleep) or None if there was an error.
    pub async fn get_all_sleeps(&self) -> Option<Vec<DbmSleep>>  {
        let result = DBSleep::select_all(&self.connection_pool).await;
        
        match result {
            Ok(s) => Some(s.into_iter().map(|x| DbmSleep { sleep: x, tags: None }).collect()),
            Err(_) => None
        }
    }

    /// Get sleeps with the given ids and returns an Optional Vector of [DbmSleeps](DbmSleep)
    /// Note: Ideally a WHERE id IN clause would be used for this, however, that is not directly supported by
    /// sqlx v0.6 so select all tags and filter them manually
    /// TODO: Add tag support
    /// 
    /// # Arguments
    /// 
    /// * `ids` - vector of sleep ids to query
    /// 
    pub async fn get_multiple_sleeps(&self, ids: Vec<i64>) -> Option<Vec<DbmSleep>> {
        let result = DBSleep::select_all(&self.connection_pool).await;

        match result {
            Ok(sleeps) => Some(sleeps.into_iter()
            .filter(|s| ids.contains(&s.id))
            .map(|d| DbmSleep { sleep: d, tags: None}).collect()),
            Err(_) => None
        }
    }

    /// Queries all sleeps in the database that have an association with the given tag
    /// Returns vector of the [sleeps](DbmSleep) that have the tag, or None if there was an error
    /// 
    /// # Arguments
    /// 
    /// * `tag_id` - the id of the tag associated with the sleeps
    /// 
    pub async fn get_sleeps_by_tag(&self, tag_id: i64) ->  Option<Vec<DbmSleep>> {
        let sleep_tags = DBSleepTags::select_by_tag_id(&self.connection_pool, tag_id).await;

        let sleep_ids =
        match sleep_tags {
            Ok(st) => st.iter().map(|x| x.sleep_id).collect(),
            Err(_) => return None
        };

        self.get_multiple_sleeps(sleep_ids).await
    }

    /// Queries the sleeps within a given month
    /// Returns vector of [sleeps](DbmSleep) within the month or None if there is an error
    /// 
    /// # Arguments
    /// 
    /// * `month` - month value (1-12) to get the sleeps from
    /// * `year` - 4 digit year to determine which year to use for the month
    /// 
    /// # Examples
    /// 
    /// let sleeps = get_sleeps_by_month(5, 2023).await;
    /// 
    pub async fn get_sleeps_by_month(&self, month: u8, year: u16) -> Option<Vec<DbmSleep>> {
        let result = DBSleep::select_by_month(&self.connection_pool, month, year).await;

        match result {
            Ok(s) => Some(s.into_iter().map(|x| DbmSleep { sleep: x, tags: None }).collect()),
            Err(_) => None
        }
    }

    /// Updates the amount value of the sleep in the database
    /// Returns true if the update was successful, otherwise false
    /// 
    /// # Arguments
    /// 
    /// * `id` - the id of the sleep to update
    /// * `amount` - the new amount value to update to
    /// 
    pub async fn update_sleep_amount(&self, id: i64, amount: f64) -> bool {
        DBSleep::update_amount(&self.connection_pool, id, amount).await
            .unwrap_or(false)
    }

    /// Updates the quality value of the sleep in the database
    /// Returns true if the update was successful, otherwise false
    /// 
    /// # Arguments
    /// 
    /// * `id` - the id of the sleep to update
    /// * `quality` - the new quality value to update to
    /// 
    pub async fn update_sleep_quality(&self, id: i64, quality: i64) -> bool {
        DBSleep::update_quality(&self.connection_pool, id, quality).await
            .unwrap_or(false)
    }

    /// Deletes the sleep from the database
    /// Returns true if the deletion was successful, otherwise false
    /// 
    /// # Arguments
    /// 
    /// * `id` - the id of the sleep to delete
    /// 
    pub async fn delete_sleep(&self, id: i64) -> bool {
        DBSleep::delete(&self.connection_pool, id).await
            .unwrap_or(false)
    }

    /// Adds a name and color to the tag table in the database.
    /// Returns the pk of the newly added row.
    /// 
    /// # Arguments
    /// 
    /// * `name` - A string slice representing the name of the tag. Tag names are unique.
    /// * `color` - A decimal represntation of the rgb color value
    /// 
    /// # Examples
    /// 
    /// let tag_id = insert_tag("tag name", 9590460).await;
    /// 
    pub async fn insert_tag(&self, name: &str, color: i64) -> i64 {
        DBTag::insert(&self.connection_pool, name, color).await
            .unwrap_or(-1)
    }

    /// Gets a tag from the database with the given id.
    /// Returns the tag if it was queried successfully, otherwise None
    /// 
    /// # Arguments
    /// 
    /// * `id` - the pk of the tag to query
    /// 
    pub async fn get_tag(&self, id: i64) -> Option<DBTag> {
        let result = DBTag::select_one(&self.connection_pool, id).await;
        
        match result {
            Ok(t) => Some(t),
            Err(_) => {
                None
            }
        } 
    }

    /// Queries all tags in the database
    /// Returns all of the tags or None if there was an error.
    pub async fn get_all_tags(&self) -> Option<Vec<DBTag>> {
        let result = DBTag::select_all(&self.connection_pool).await;
        
        match result {
            Ok(t) => Some(t),
            Err(_) => {
                None
            }
        } 
    }

    /// Queries all tags in the database that have an association with the given sleep
    /// Returns vector of the tags that are related to the sleep, or None if there was an error
    /// 
    /// # Arguments
    /// 
    /// * `sleep_id` - the id of the sleep associated with the tags
    /// 
    pub async fn get_tags_by_sleep(&self, sleep_id: i64) ->  Option<Vec<DBTag>> {
        let sleep_tags = DBSleepTags::select_by_sleep_id(&self.connection_pool, sleep_id).await;

        let tag_ids =
        match sleep_tags {
            Ok(st) => st.iter().map(|x| x.tag_id).collect(),
            Err(_) => return None
        };

       self.get_multiple_tags(tag_ids).await
    }

    /// Gets multiple tags based on the given ids.
    /// Returns the tags that match the ids, or None if there was an error
    /// Note: Ideally a WHERE id IN clause would be used for this, however, that is not directly supported by
    /// sqlx v0.6 so select all tags and filter them manually
    /// 
    /// # Arguments
    /// 
    /// * `ids` - list of the tag ids to query
    /// 
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

    /// Updates the name value of the tag in the database
    /// Returns true if the update was successful, otherwise false
    /// 
    /// # Arguments
    /// 
    /// * `id` - the id of the tag to update
    /// * `name` - the new name value to update to
    /// 
    pub async fn update_tag_name(&self, id: i64, name: &str) -> bool {
        DBTag::update_name(&self.connection_pool, id, name).await
            .unwrap_or(false)
    }

    /// Updates the color value of the tag in the database
    /// Returns true if the update was successful, otherwise false
    /// 
    /// # Arguments
    /// 
    /// * `id` - the id of the tag to update
    /// * `color` - the new decimal represntation of the rgb color value value to update to.
    ///
    /// # Examples
    /// 
    /// let success = update_tag_color(2, 65535).await;
    /// 
    pub async fn update_tag_color(&self, id: i64, color: i64) -> bool {
        DBTag::update_color(&self.connection_pool, id, color).await
            .unwrap_or(false)
    }

    /// Deletes the tag from the database
    /// Returns true if the deletion was successful, otherwise false
    /// 
    /// # Arguments
    /// 
    /// * `id` - the id of the tag to delete
    ///
    pub async fn delete_tag(&self, id: i64) -> bool {
        DBTag::delete(&self.connection_pool, id).await
            .unwrap_or(false)
    }

    /// Adds an association between a list of tags and a sleep
    /// returns true if all of the relationships are created successfully, otherwise false
    /// 
    /// # Arguments
    /// 
    /// * `sleep_id` - the id of the sleep to add the tags to
    /// * `tag_ids` - the ids of the tags to add to the sleep
    /// 
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

    /// Removes a relationship between a tag and a sleep
    /// returns true if the relationship was deleted successfully, otherwise false
    /// 
    /// # Arguments
    /// 
    /// * `sleep_id` - id of the sleep to remove the relationship from
    /// * `tag_id` - id of tag to remove the relationship from
    /// 
    pub async fn remove_tag_from_sleep(&self, sleep_id: i64, tag_id: i64) -> bool {
        DBSleepTags::delete(&self.connection_pool, sleep_id, tag_id).await
            .unwrap_or(false)
    }

    /// Adds a comment to the comment table in the database and relates it to a sleep.
    /// Returns the pk of the newly added row.
    /// 
    /// # Arguments
    /// 
    /// * `sleep_id` - sleep to relate the comment to
    /// * `comment` - text comment to add
    /// 
    pub async fn insert_comment(&self, sleep_id: i64, comment: &str) -> i64 {
        DBComment::insert(&self.connection_pool, sleep_id, comment).await
            .unwrap_or(-1)
    }

    /// Gets a comment from the database with the given id.
    /// Returns the comment if it was queried successfully, otherwise None
    /// 
    /// # Arguments
    /// 
    /// * `comment_id` - the pk of the comment to query
    ///
    pub async fn get_comment(&self, comment_id: i64) -> Option<DBComment> {
        let result = DBComment::select_by_id(&self.connection_pool, comment_id).await;
        match result {
            Ok(c) => Some(c),
            Err(_) => {
                None
            }
        }
    }

    /// Get all comments associated to a sleep
    /// Returns the comments related to the sleep, or None if there is an error
    /// 
    /// # Arguments
    /// 
    /// * `sleep-id` - The id of the sleep to get the comments from
    /// 
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

    /// Updates the text of a comment in the database
    /// Returns true if the update is successful, otherwise false
    /// 
    /// # Arguments
    /// 
    /// * `comment_id` - the id of the comment to edit
    /// * `comment` - the new text value to update the comment to
    /// 
    pub async fn update_comment(&self, comment_id: i64, comment: &str) -> bool {
        DBComment::update_comment(&self.connection_pool, comment_id, comment).await
            .unwrap_or(false)
    }

    /// Deletes the comment from the database
    /// Returns true if the deletion was successful, otherwise false
    /// 
    /// # Arguments
    /// 
    /// * `id` - the id of the comment to delete
    ///
    pub async fn delete_comment(&self, id: i64) -> bool {
        DBComment::delete(&self.connection_pool, id).await
            .unwrap_or(false)
    }
}

mod db_migrations;
mod db_types;

/// Module for creating a mock db and testing the CRUD functionality of it
pub mod db_tests;