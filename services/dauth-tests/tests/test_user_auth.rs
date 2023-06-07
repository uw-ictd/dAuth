use std::time::Duration;

use dauth_service::data::config::UserInfoConfig;
use dauth_tests::{TestCore, TestDauth, TestDirectory, TEST_K, TEST_OPC};

const NUM_USERS: usize = 10;

#[tokio::test]
async fn test_home_auth_successful() {
    let id = "test-network-id";
    let dauth = TestDauth::new(id, "127.0.0.1", "127.0.0.1").await.unwrap();
    let dir = TestDirectory::new("127.0.0.1").await.unwrap();
    tokio::time::sleep(Duration::from_secs_f32(1.0)).await;

    let test_core = TestCore::new("127.0.0.1").await.unwrap();

    let mut user_infos = Vec::new();
    let mut user_ids = Vec::new();
    for num in 0..NUM_USERS {
        let user_id = format!("user-{}-{}", id, num);
        user_ids.push(user_id.to_owned());
        user_infos.push(UserInfoConfig {
            user_id,
            k: TEST_K.to_string(),
            opc: TEST_OPC.to_string(),
            sqn_max: 32,
            backups: Vec::new(),
        });
    }

    dauth.add_users(&user_infos).await.unwrap();
    tokio::time::sleep(Duration::from_secs_f32(0.5)).await;

    dauth.check_users_exists(&user_ids, 0).await.unwrap();
    dir.check_users_exists(&user_ids).await.unwrap();

    for user_id in user_ids {
        test_core.auth_user(&user_id).await.unwrap();
    }

    dauth.stop();
    dir.stop();
}

#[tokio::test]
async fn test_home_auth_failure() {
    let id = "test-network-id";
    let dauth = TestDauth::new(id, "127.0.0.2", "127.0.0.2").await.unwrap();
    let dir = TestDirectory::new("127.0.0.2").await.unwrap();
    tokio::time::sleep(Duration::from_secs_f32(2.0)).await;

    let test_core = TestCore::new("127.0.0.2").await.unwrap();
    assert!(test_core.auth_user("nonexistent-user").await.is_err());

    dauth.stop();
    dir.stop();
}
