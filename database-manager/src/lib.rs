use db_manager::DBManager;

pub mod db_manager;

pub async fn init_db() -> DBManager {
    let dbm = DBManager::init("sqlite://debug.db").await.unwrap();
    dbm
}

async fn _test_db() {
    db_manager::db_tests::test_db_queries("test.db").await;

    // SQLX sometimes keeps a handle on the file for longer than expected,
    // so removing this until I get a better understanding of the issue
    //fs::remove_file("test.db").unwrap_or_else(|_| fs::remove_file("test.db").unwrap());
}
