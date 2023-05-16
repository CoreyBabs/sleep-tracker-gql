use sqlx::{SqlitePool};

/// Representation of the sleep_tag table. Maps the many to many relationships between sleeps and tags
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DBSleepTags {
    /// Primary key
    pub id: i64,

    /// Fk to the sleep table
    pub sleep_id: i64,

    /// Fk to the tag table
    pub tag_id: i64,
}

impl DBSleepTags {
    pub async fn insert(pool: &SqlitePool, sleep_id: i64, tag_id: i64) -> Result<i64, sqlx::Error>  {
        let mut conn = pool.acquire().await?;

        let result = sqlx::query!(
            r#"
            INSERT INTO sleep_tags ( sleep_id, tag_id )
            VALUES ( ?1, ?2 )
                "#,
            sleep_id,
            tag_id,
        )
        .execute(&mut conn)
        .await;

        match result {
            Ok(r) => Ok(r.last_insert_rowid()),
            Err(e) => Err(e),
        }
    }

    pub async fn delete(pool: &SqlitePool, sleep_id: i64, tag_id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM sleep_tags
            WHERE sleep_id = ?1 AND tag_id = ?2
                "#,
                sleep_id,
                tag_id
        )
        .execute(pool)
        .await;

        match result {
            Ok(r) => Ok(r.rows_affected() > 0),
            Err(e) => Err(e)
        }
    }

    pub async fn select_by_sleep_id(pool: &SqlitePool, sleep_id: i64) -> Result<Vec<DBSleepTags>, sqlx::Error>  {
        sqlx::query_as!(DBSleepTags,
            r#"
            SELECT id, sleep_id, tag_id
            FROM sleep_tags
            WHERE sleep_id = ?1
            ORDER BY id
                "#,
                sleep_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn select_by_tag_id(pool: &SqlitePool, tag_id: i64) -> Result<Vec<DBSleepTags>, sqlx::Error>  {
        sqlx::query_as!(DBSleepTags,
            r#"
            SELECT id, sleep_id, tag_id
            FROM sleep_tags
            WHERE tag_id = ?1
            ORDER BY id
                "#,
                tag_id
        )
        .fetch_all(pool)
        .await
    }
}