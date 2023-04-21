use std::time::Duration;

use dauth_service::data::config::UserInfoConfig;

#[tokio::test]
async fn test_simple_user_registration() {
    let user_id = "test-user".to_string();
    let test_context = dauth_tests::TestContext::build_test_context(
        "test-network-id".to_string(),
        "127.0.0.1".to_string(),
    )
    .await;

    let context = test_context.context.clone();
    tokio::spawn(async move { dauth_tests::run_test_env(context) });
    tokio::time::sleep(Duration::from_secs_f32(0.1)).await;

    let context = test_context.context.clone();

    dauth_service::management::add_user(
        context.clone(),
        &UserInfoConfig {
            user_id: user_id.clone(),
            k: dauth_tests::TEST_K.to_string(),
            opc: dauth_tests::TEST_OPC.to_string(),
            sqn_max: 32,
            backups: Vec::new(),
        },
    )
    .await
    .unwrap();

    let mut transaction = context.local_context.database_pool.begin().await.unwrap();
    assert_eq!(dauth_service::database::user_infos::get(&mut transaction, &user_id, 0).await.unwrap().id, user_id);
}
