use std::env;

use simpleg_core::secrets::bitwarden_secrets_manager::BitwardenSecretsManager;
use simpleg_core::secrets::secrets_manager::SecretsManager;

#[cfg(test)]
#[tokio::test]
pub async fn test_get_valid_secret() {
    const TEST_SECRET_ID: &str = "b6499052-d21a-4951-a511-b18a010134ec";
    const EXPECTED_SECRET: &str = "le_secret:)";
    let access_token = env::var("SECRETS_MANAGER_ACCESS_TOKEN")
        .expect("expected 'SECRETS_MANAGER_ACCESS_TOKEN' to be set");

    let mut manager = BitwardenSecretsManager::try_new(access_token)
        .await
        .expect("expected correct 'BitwardenSecretsManager' initialization");

    let secret = manager
        .get(TEST_SECRET_ID.to_string())
        .await
        .expect("expected successful secret retrieval");

    assert_eq!(
        EXPECTED_SECRET,
        secret.as_str().expect("expected secret to be a string")
    )
}
