use std::time::Duration;

use dauth_service::data::config::{UserInfoConfig, BackupConfig};
use dauth_tests::{TestDauth, TestDirectory, TEST_K, TEST_OPC};

const NUM_USERS: usize = 10;
const NUM_BACKUPS: usize = 3;

#[tokio::test]
async fn test_simple_user_registration() {
    let id = "test-network-id";
    let test_context = TestDauth::new(id, "127.0.0.1", "127.0.0.1").await.unwrap();

    let dir_context = TestDirectory::new("127.0.0.1").await.unwrap();

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

    test_context.add_users(&user_infos).await.unwrap();
    tokio::time::sleep(Duration::from_secs_f32(0.5)).await;

    test_context.check_users_exists(&user_ids, 0).await.unwrap();
    dir_context.check_users_exists(&user_ids).await.unwrap();
}

#[tokio::test]
#[should_panic]
async fn test_failed_user_registration() {
    let test_context = TestDauth::new("test-network-id", "127.0.0.2", "127.0.0.2")
        .await
        .unwrap();

    let _dir_context = TestDirectory::new("127.0.0.2").await;

    let user_ids = vec!["unregistered-user".to_string()];
    tokio::time::sleep(Duration::from_secs_f32(0.5)).await;

    test_context.check_users_exists(&user_ids, 0).await.unwrap(); // Should fail
}

#[tokio::test]
async fn test_user_with_backups() {
    let id = "test-home-1";
    let test_context = TestDauth::new(id, "127.0.0.3", "127.0.0.3").await.unwrap();

    let mut backups = Vec::new();
    for backup_num in 0..NUM_BACKUPS {
        let backup_id = format!("test-backup-{}", backup_num);
        backups.push(TestDauth::new(&backup_id, &format!("127.0.0.{}", 4+backup_num), "127.0.0.3").await.unwrap());
    }

    let dir_context = TestDirectory::new("127.0.0.3").await.unwrap();

    let mut user_infos = Vec::new();
    let mut user_ids = Vec::new();
    for num in 0..NUM_USERS {
        let user_id = format!("user-{}-{}", id, num);

        let mut backup_configs = Vec::new();
        for backup_num in 0..NUM_BACKUPS {
            let backup_id = format!("test-backup-{}", backup_num);
            backup_configs.push(BackupConfig {
                backup_id: backup_id.to_owned(),
                sqn_slice: 1 + backup_num as i64,
                sqn_max: 33 + backup_num as i64,
            });
        }

        user_ids.push(user_id.to_owned());
        user_infos.push(UserInfoConfig {
            user_id,
            k: TEST_K.to_string(),
            opc: TEST_OPC.to_string(),
            sqn_max: 32,
            backups: backup_configs,
        });
    }

    test_context.add_users(&user_infos).await.unwrap();
    // TODO: Time value is unstable, need to find a better way
    tokio::time::sleep(Duration::from_secs_f32(5.0)).await;

    test_context.check_users_exists(&user_ids, 0).await.unwrap();
    dir_context.check_users_exists(&user_ids).await.unwrap();

    for backup in backups {
        backup.check_backup_user_exists(&user_ids).await.unwrap();
    }

}
