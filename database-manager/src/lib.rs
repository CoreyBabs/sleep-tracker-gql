use db_manager::DBManager;
pub mod db_manager;

mod model;
pub use model::QueryRoot;


pub async fn init_db() -> DBManager {
    let dbm = DBManager::init("sqlite://debug.db").await.unwrap();
    dbm
}

pub async fn _init_test_db() -> DBManager {
    let dbm = db_manager::db_tests::create_test_db("test.db").await;
    dbm
}


async fn _test_db() {
    db_manager::db_tests::test_db_queries("test.db").await;

    // SQLX sometimes keeps a handle on the file for longer than expected,
    // so removing this until I get a better understanding of the issue
    //fs::remove_file("test.db").unwrap_or_else(|_| fs::remove_file("test.db").unwrap());
}
