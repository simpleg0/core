use std::str::FromStr;

use async_trait::async_trait;
use bitwarden::auth::login::AccessTokenLoginRequest;
use bitwarden::Client;
use bitwarden::secrets_manager::secrets::SecretGetRequest;
use serde_json::Value;
use uuid::Uuid;

use crate::error::Error;
use crate::error_kind::{SECRET_MANAGER_FAILURE, SERIALIZATION_FAILURE};
use crate::ok_or_return_error;
use crate::secrets::secrets_manager::SecretsManager;

pub struct BitwardenSecretsManager {
    client: Client,
}

impl BitwardenSecretsManager {
    pub async fn try_new(access_token: impl Into<String>) -> Result<Self, Error> {
        let mut client = Client::new(None);

        let login_request = AccessTokenLoginRequest {
            access_token: access_token.into(),
            state_file: None,
        };

        ok_or_return_error!(
            client.auth().login_access_token(&login_request).await,
            SECRET_MANAGER_FAILURE,
            "failed to authenticate to Bitwarden servers"
        );

        Ok(Self { client })
    }
}

#[async_trait]
impl SecretsManager for BitwardenSecretsManager {
    async fn get(&mut self, secret_id: String) -> Result<Value, Error> {
        let secret_uuid = ok_or_return_error!(
            Uuid::from_str(&secret_id),
            SECRET_MANAGER_FAILURE,
            "failed to read secret id as an uuid"
        );
        let secret_request = SecretGetRequest { id: secret_uuid };

        let secret = ok_or_return_error!(
            self.client.secrets().get(&secret_request).await,
            SECRET_MANAGER_FAILURE,
            "failed to get secret"
        )
        .value;

        let secret_value = ok_or_return_error!(
            serde_json::from_str::<Value>(&secret),
            SERIALIZATION_FAILURE,
            "failed to read secret as 'Value'"
        );

        Ok(secret_value)
    }
}
