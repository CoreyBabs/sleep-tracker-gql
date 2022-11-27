use std::fs;
use db_manager::DBManager;

pub mod db_manager;

#[tokio::main]
async fn main() {
    let test = true;
    if test {
        test_inserts("test.db", false).await
    }
    else {
        let dbm = DBManager::init("sqlite://debug.db").await.unwrap();
        ()
    }
}

async fn test_inserts(db_path: &str, delete_when_done: bool) {
    if std::path::Path::new(db_path).exists() {
        println!("Deleting test db and starting fresh");
        fs::remove_file(db_path).unwrap();
    }
    let mut dbm = DBManager::init("sqlite://test.db").await.unwrap();

    // Sleep valid inserts
    assert_eq!(dbm.insert_sleep("2022-11-25", 7.5, 1).await, 1);
    assert_eq!(dbm.insert_sleep("2022-11-24", 6.0, 2).await, 2);
    assert_eq!(dbm.insert_sleep("2022-11-26", 8.0, 3).await, 3);

    // tag valid inserts
    assert_eq!(dbm.insert_tag("test name", 3713678).await, 1);
    assert_eq!(dbm.insert_tag("screen", 9590460).await, 2);

    // sleep_tag valid inserts
    assert_eq!(dbm.add_tag_to_sleep(2, vec![2]).await, true);
    assert_eq!(dbm.add_tag_to_sleep(1, vec![1,2]).await, true);


    if delete_when_done {
        fs::remove_file(db_path).unwrap();
    }
}

