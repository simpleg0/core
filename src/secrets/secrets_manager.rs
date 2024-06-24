use async_trait::async_trait;
use serde_json::Value;

use crate::error::Error;

#[async_trait]
pub trait SecretsManager {
    async fn get(&mut self, secret_id: String) -> Result<Value, Error>;
}
