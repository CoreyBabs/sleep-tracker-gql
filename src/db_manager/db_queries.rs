use sqlx::{SqlitePool};
use super::db_types::{DBSleep, DBTag, DBSleepTags};

impl DBSleep {
    pub async fn select_all(pool: &SqlitePool) -> Result<Vec<DBSleep>, sqlx::Error>  {
        sqlx::query_as!(DBSleep,
            r#"
            SELECT id, night, amount, quality
            FROM sleep
            ORDER BY id
                "#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn select_one(pool: &SqlitePool, id: i64) -> Result<DBSleep, sqlx::Error>  {
        sqlx::query_as!(DBSleep,
            r#"
            SELECT id, night, amount, quality
            FROM sleep
            WHERE id = ?1
            ORDER BY id
                "#,
                id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn insert(pool: &SqlitePool, night: &str, amount: f64, quality: i64) -> Result<i64, sqlx::Error>  {
        let mut conn = pool.acquire().await?;

        let result = sqlx::query!(
            r#"
            INSERT INTO sleep ( night, amount, quality )
            VALUES ( ?1, ?2, ?3 )
                "#,
            night,
            amount,
            quality,
        )
        .execute(&mut conn)
        .await;
        
        match result {
            Ok(r) => Ok(r.last_insert_rowid()),
            Err(e) => Err(e),
        }
    }

    pub async fn update_amount(pool: &SqlitePool, id: i64, amount: f64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE sleep
            SET amount = ?1
            WHERE id = ?2
                "#,
                amount,
                id
        )
        .execute(pool)
        .await;

        match result {
            Ok(r) => Ok(r.rows_affected() > 0),
            Err(e) => Err(e)
        }
    }

    pub async fn update_quality(pool: &SqlitePool, id: i64, quality: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE sleep
            SET quality = ?1
            WHERE id = ?2
                "#,
                quality,
                id
            )
        .execute(pool)
        .await;

        match result {
            Ok(r) => Ok(r.rows_affected() > 0),
            Err(e) => Err(e)
        }
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM sleep
            WHERE id = ?1
                "#,
                id
        )
        .execute(pool)
        .await;

        match result {
            Ok(r) => Ok(r.rows_affected() > 0),
            Err(e) => Err(e)
        }
    }
}

impl DBTag {

    pub async fn select_all(pool: &SqlitePool) -> Result<Vec<DBTag>, sqlx::Error>  {
        sqlx::query_as!(DBTag,
            r#"
            SELECT id, name, color
            FROM tag
            ORDER BY id
                "#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn select_one(pool: &SqlitePool, id: i64) -> Result<DBTag, sqlx::Error>  {
        sqlx::query_as!(DBTag,
            r#"
            SELECT id, name, color
            FROM tag
            WHERE id = ?1
            ORDER BY id
                "#,
                id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn insert(pool: &SqlitePool, name: &str, color: i64) -> Result<i64, sqlx::Error>  {
        let mut conn = pool.acquire().await?;

        let result = sqlx::query!(
            r#"
            INSERT INTO tag ( name, color )
            VALUES ( ?1, ?2 )
                "#,
            name,
            color,
        )
        .execute(&mut conn)
        .await;

        match result {
            Ok(r) => Ok(r.last_insert_rowid()),
            Err(e) => Err(e),
        }
    }

    pub async fn update_name(pool: &SqlitePool, id: i64, name: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE tag
            SET name = ?1
            WHERE id = ?2
                "#,
                name,
                id
        )
        .execute(pool)
        .await;

        match result {
            Ok(r) => Ok(r.rows_affected() > 0),
            Err(e) => Err(e)
        }
    }

    pub async fn update_color(pool: &SqlitePool, id: i64, color: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE tag
            SET color = ?1
            WHERE id = ?2
                "#,
                color,
                id
            )
        .execute(pool)
        .await;

        match result {
            Ok(r) => Ok(r.rows_affected() > 0),
            Err(e) => Err(e)
        }
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM tag
            WHERE id = ?1
                "#,
                id
        )
        .execute(pool)
        .await;

        match result {
            Ok(r) => Ok(r.rows_affected() > 0),
            Err(e) => Err(e)
        }
    }
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