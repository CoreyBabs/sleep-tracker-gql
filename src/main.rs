use db_manager::DBManager;

pub mod db_manager;

#[tokio::main]
async fn main() {
    let dbm = DBManager::init("sqlite://debug.db").await.unwrap();
    println!("db opened with {} connections.", dbm.connection_pool.size());
}
