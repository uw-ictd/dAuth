use std::time::Duration;

use dauth_tests::{TestDauthContext, TestDirectoryContext};

const NUM_USERS: usize = 10;

#[tokio::test]
async fn test_simple_user_registration() {
    let test_context = TestDauthContext::new("test-network-id", "127.0.0.1")
        .await
        .unwrap();

    let dir_context = TestDirectoryContext::new("127.0.0.1").await.unwrap();

    let user_ids = test_context.add_users(NUM_USERS).await.unwrap();
    tokio::time::sleep(Duration::from_secs_f32(0.5)).await;

    test_context.check_users_exists(&user_ids).await.unwrap();
    dir_context.check_users_exists(&user_ids).await.unwrap();
}

#[tokio::test]
#[should_panic]
async fn test_failed_user_registration() {
    let test_context = TestDauthContext::new("test-network-id", "127.0.0.1")
        .await
        .unwrap();

    let _dir_context = TestDirectoryContext::new("127.0.0.1").await;

    let user_ids = vec!["unregistered-user".to_string()];
    tokio::time::sleep(Duration::from_secs_f32(0.5)).await;

    test_context.check_users_exists(&user_ids).await.unwrap(); // Should fail
}
