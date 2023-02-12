use sqlx::{SqlitePool};

#[derive(Debug)]
pub struct DBComment {
    pub id: i64,
    pub sleep_id: i64,
    pub comment: String,
}

impl DBComment {
    pub async fn insert(pool: &SqlitePool, sleep_id: i64, comment: &str) -> Result<i64, sqlx::Error> {
        let mut conn = pool.acquire().await?;

        let result = sqlx::query!(
            r#"
            INSERT INTO comment ( sleep_id, comment )
            VALUES ( ?1, ?2 )
                "#,
            sleep_id,
            comment,
        )
        .execute(&mut conn)
        .await;

        match result {
            Ok(r) => Ok(r.last_insert_rowid()),
            Err(e) => Err(e),
        }
    }

    pub async fn select_by_id(pool: &SqlitePool, id: i64) -> Result<DBComment, sqlx::Error> {
        sqlx::query_as!(DBComment,
            r#"
            SELECT id, sleep_id, comment
            FROM comment
            WHERE id = ?1
            ORDER BY id
                "#,
                id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn select_by_sleep_id(pool: &SqlitePool, sleep_id: i64) -> Result<Vec<DBComment>, sqlx::Error> {
        sqlx::query_as!(DBComment,
            r#"
            SELECT id, sleep_id, comment
            FROM comment
            WHERE sleep_id = ?1
            ORDER BY id
                "#,
                sleep_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn update_comment(pool: &SqlitePool, id: i64, comment: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE comment
            SET comment = ?1
            WHERE id = ?2
                "#,
                comment,
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
            DELETE FROM comment
            WHERE id = ?1
                "#,
                id,
        )
        .execute(pool)
        .await;

        match result {
            Ok(r) => Ok(r.rows_affected() > 0),
            Err(e) => Err(e)
        }
    }
}