use std::time::Duration;

use dauth_tests::{TestDauthContext, TestDirectoryContext};

const NUM_USERS: usize = 10;

#[tokio::test]
async fn test_simple_user_registration() {
    let test_context =
        TestDauthContext::new("test-network-id".to_string(), "127.0.0.1".to_string()).await;
    
    let dir_context = TestDirectoryContext::new().await;

    let user_ids = test_context.add_users(NUM_USERS).await;
    tokio::time::sleep(Duration::from_secs_f32(0.5)).await;

    test_context.check_users_exists(&user_ids).await;
    dir_context.check_users_exists(&user_ids).await;
}

#[tokio::test]
#[should_panic]
async fn test_failed_user_registration() {
    let test_context =
        TestDauthContext::new("test-network-id".to_string(), "127.0.0.1".to_string()).await;
    
    let _dir_context = TestDirectoryContext::new().await;

    let user_ids = vec!["unregistered-user".to_string()];
    tokio::time::sleep(Duration::from_secs_f32(0.5)).await;

    test_context.check_users_exists(&user_ids).await;  // Should fail
}

