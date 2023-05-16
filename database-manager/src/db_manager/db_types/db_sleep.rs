use sqlx::{SqlitePool};

/// Representation of the sleep table
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DBSleep {
    /// Primary key
    pub id: i64,

    /// date in yyyy-mm-dd format, must be unique
    pub night: String,

    /// amount of sleep
    pub amount: f64,

    /// quality of sleep, scale is open ended
    pub quality: i64,
}

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

    pub async fn select_by_month(pool: &SqlitePool, month: u8, year: u16) -> Result<Vec<DBSleep>, sqlx::Error>  {
        // format month to match the expected yyyy-mm-dd format
        let  month = format!("{:02}", month);
        let year = year.to_string();
        let mut date = [year, month].join("-");
        date.push('%');

        sqlx::query_as!(DBSleep,
            r#"
            SELECT id, night, amount, quality
            FROM sleep
            WHERE night LIKE ?1
            ORDER BY id
                "#,
                date
        )
        .fetch_all(pool)
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