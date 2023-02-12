use sqlx::{SqlitePool, sqlite};

pub async fn initalize_db(pool: &SqlitePool) -> Result<sqlite::SqliteQueryResult, sqlx::Error> {
    let mut query = String::from("PRAGMA foreign_keys = ON;");

    let create_sleep_table = 
    "CREATE TABLE IF NOT EXISTS sleep
        (
            id         INTEGER  NOT NULL PRIMARY KEY AUTOINCREMENT,
            night      TEXT NOT NULL,
            amount     REAL NOT NULL,
            quality    INTEGER NOT NULL,
            created_on TEXT NOT NULL DEFAULT (datetime('now','localtime')),
            updated_on TEXT NOT NULL DEFAULT (datetime('now','localtime'))
        );";

    let create_tag_table = 
    "CREATE TABLE IF NOT EXISTS tag
        (
            id         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            name       TEXT NOT NULL,
            color      INTEGER NOT NULL,
            created_on TEXT NOT NULL DEFAULT (datetime('now','localtime')),
            updated_on TEXT NOT NULL DEFAULT (datetime('now','localtime'))
        );";

    let create_sleep_tag_table =
    "CREATE TABLE IF NOT EXISTS sleep_tags 
        (
            id         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            sleep_id   INTEGER NOT NULL,
            tag_id     INTEGER NOT NULL,
            FOREIGN KEY (sleep_id)
            REFERENCES sleep (id) 
                ON UPDATE CASCADE
                ON DELETE CASCADE,
            FOREIGN KEY (tag_id)
            REFERENCES tag (id) 
                ON UPDATE CASCADE
                ON DELETE CASCADE
        );";

        let create_comment_table =
        "CREATE TABLE IF NOT EXISTS comment 
            (
                id         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                sleep_id   INTEGER NOT NULL,
                comment    TEXT NOT NULL,
                FOREIGN KEY (sleep_id)
                REFERENCES sleep (id) 
                    ON UPDATE CASCADE
                    ON DELETE CASCADE
            );";

    let set_user_version = "PRAGMA user_version = 1;";
              
    query.push_str(create_sleep_table);
    query.push_str(create_tag_table);
    query.push_str(create_sleep_tag_table);
    query.push_str(create_comment_table);
    query.push_str(set_user_version);

    sqlx::query(query.as_str()).execute(pool).await
}