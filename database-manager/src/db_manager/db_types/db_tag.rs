use sqlx::{SqlitePool};

#[derive(Debug)]
pub struct DBTag {
    pub id: i64,
    pub name: String,
    pub color: i64,
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