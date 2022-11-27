use std::fs;
use db_manager::DBManager;
use db_manager::db_types;

pub mod db_manager;

#[tokio::main]
async fn main() {
    let test = true;
    if test {
        test_inserts("test.db").await; 
        
    }
    else {
        let dbm = DBManager::init("sqlite://debug.db").await.unwrap();
        ()
    }

    if test {
        fs::remove_file("test.db").unwrap();
    }
}

async fn test_inserts(db_path: &str) {
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

    test_sleep_selects(&mut dbm).await;
    test_tag_selects(&mut dbm).await;
    test_updates(&mut dbm).await;
    test_deletes(&mut dbm).await;

    dbm.close_connection().await;
}

async fn test_sleep_selects(dbm: &mut DBManager) {
    let sleep_test = dbm.get_sleep(2, false).await.expect("Sleep test failed");
    let expected_sleep = db_types::DBSleep { id: 2, night: String::from("2022-11-24"), amount: 6.0, quality: 2 };
    assert_eq!(sleep_test.sleep.id, expected_sleep.id);
    assert_eq!(sleep_test.sleep.night, expected_sleep.night);
    assert_eq!(sleep_test.sleep.amount, expected_sleep.amount);
    assert_eq!(sleep_test.sleep.quality, expected_sleep.quality);
    assert!(sleep_test.tags.is_none());

    let sleep_with_tags_test = dbm.get_sleep(1, true).await.expect("Sleep test failed");
    let expected_sleep = db_types::DBSleep { id: 1, night: String::from("2022-11-25"), amount: 7.5, quality: 1 };
    assert_eq!(sleep_with_tags_test.sleep.id, expected_sleep.id);
    assert_eq!(sleep_with_tags_test.sleep.night, expected_sleep.night);
    assert_eq!(sleep_with_tags_test.sleep.amount, expected_sleep.amount);
    assert_eq!(sleep_with_tags_test.sleep.quality, expected_sleep.quality);
    assert!(sleep_with_tags_test.tags.is_some());
    assert_eq!(sleep_with_tags_test.tags.unwrap().len(), 2);

    let none_test = dbm.get_sleep(100, false).await;
    assert!(none_test.is_none());

    let all_sleeps = dbm.get_all_sleeps().await.expect("All sleep test failed");
    assert_eq!(all_sleeps.len(), 3);

    let some_sleeps = dbm.get_sleeps_by_tag(2).await.expect("Some sleep test failed");
    assert_eq!(some_sleeps.len(), 2);

    let sleep_by_tag_no_tag = dbm.get_sleeps_by_tag(100).await.expect("sleep no tags test failed");
    assert_eq!(sleep_by_tag_no_tag.len(), 0);
}

// Note: I dont have a lot of asserts here but visually verify the data instead with println!
async fn test_tag_selects(dbm: &mut DBManager) {
    let tag_test = dbm.get_tag(2).await.expect("tag test failed");
    let expected_tag = db_types::DBTag {id: 2, name: String::from("screen"), color: 9590460 };
    assert_eq!(tag_test.id, expected_tag.id);
    assert_eq!(tag_test.name, expected_tag.name);
    assert_eq!(tag_test.color, expected_tag.color);

    let none_test = dbm.get_tag(100).await;
    assert!(none_test.is_none());

    let all_tags = dbm.get_all_tags().await.expect("All tag test failed");
    assert_eq!(all_tags.len(), 2);

    let sleep_with_tags_test = dbm.get_sleep(2, true).await.expect("sleep with single tag test failed");
    assert!(sleep_with_tags_test.tags.is_some());
    let tags = sleep_with_tags_test.tags.unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].id, expected_tag.id);
    assert_eq!(tags[0].name, expected_tag.name);
    assert_eq!(tags[0].color, expected_tag.color);
}

async fn test_updates(dbm: &mut DBManager) {
    assert_eq!(dbm.get_sleep(1, false).await.unwrap().sleep.amount, 7.5);
    assert_eq!(dbm.update_sleep_amount(1, 7.0).await, true);
    assert_eq!(dbm.get_sleep(1, false).await.unwrap().sleep.amount, 7.0);

    assert_eq!(dbm.get_sleep(3, false).await.unwrap().sleep.quality, 3);
    assert_eq!(dbm.update_sleep_quality(3, 1).await, true);
    assert_eq!(dbm.get_sleep(3, false).await.unwrap().sleep.quality, 1);

    assert_eq!(dbm.get_tag(1).await.unwrap().name, "test name");
    assert_eq!(dbm.update_tag_name(1, "update test").await, true);
    assert_eq!(dbm.get_tag(1).await.unwrap().name, "update test");

    assert_eq!(dbm.get_tag(2).await.unwrap().color, 9590460);
    assert_eq!(dbm.update_tag_color(2, 65535).await, true);
    assert_eq!(dbm.get_tag(2).await.unwrap().color, 65535);
}

async fn test_deletes(dbm: &mut DBManager) {
    // remove tag from sleep first because cascade with auto delete rows in sleep_tag
    assert_eq!(dbm.get_sleep(1, true).await.unwrap().tags.unwrap().len(), 2);
    assert_eq!(dbm.remove_tag_from_sleep(1, 1).await, true);
    assert_eq!(dbm.get_sleep(1, true).await.unwrap().tags.unwrap().len(), 1);

    assert_eq!(dbm.get_all_sleeps().await.unwrap().len(), 3);
    assert_eq!(dbm.delete_sleep(1).await, true);
    assert_eq!(dbm.get_all_sleeps().await.unwrap().len(), 2);

    assert_eq!(dbm.get_all_tags().await.unwrap().len(), 2);
    assert_eq!(dbm.delete_tag(1).await, true);
    assert_eq!(dbm.get_all_tags().await.unwrap().len(), 1);
}

