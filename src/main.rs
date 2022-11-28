use db_manager::DBManager;

pub mod db_manager;

#[tokio::main]
async fn main() {
    let test = true;
    if test {
        db_manager::db_tests::test_db_queries("test.db").await; 
        
    }
    else {
        let dbm = DBManager::init("sqlite://debug.db").await.unwrap();
        ()
    }

    // SQLX sometimes keeps a handle on the file for longer than expected,
    // so removing this until I get a better understanding of the issue
    // if test {
    //     fs::remove_file("test.db").unwrap_or_else(|_| fs::remove_file("test.db").unwrap());
    // }
}
