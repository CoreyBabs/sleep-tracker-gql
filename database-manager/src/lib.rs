/// Exposes database manager, queries and mutations to the server

use db_manager::DBManager;

/// Module that manages the database connection, calls, models, and gql api
pub mod db_manager;

mod model;
pub use model::{QueryRoot, MutationRoot};

/// Initializes and returns a database manager to manage db calls.
pub async fn init_db() -> DBManager {
    DBManager::init("sqlite://debug.db").await.unwrap()
}

/// Initializes and returns a test database.
pub async fn _init_test_db() -> DBManager {
    db_manager::db_tests::create_test_db("test.db").await
}

/// Creates a mock database and tests all of the queries and mutations
async fn _test_db() {
    db_manager::db_tests::test_db_queries("test.db").await;

    // SQLX sometimes keeps a handle on the file for longer than expected,
    // so removing this until I get a better understanding of the issue
    //fs::remove_file("test.db").unwrap_or_else(|_| fs::remove_file("test.db").unwrap());
}
