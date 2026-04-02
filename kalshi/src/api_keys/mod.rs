use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};

pub use crate::generated::types::{
    ApiKey, CreateApiKeyResponse, GetAccountApiLimitsResponse as AccountApiLimits,
    GetApiKeysResponse,
};

impl Kalshi {
    /// Retrieves all API keys for the authenticated user.
    ///
    /// This method lists all API keys associated with your account,
    /// including their metadata but not the secret values.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<ApiKey>)`: A vector of API key information on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let keys = kalshi_instance.get_api_keys().await.unwrap();
    /// ```
    ///
    pub async fn get_api_keys(&self) -> Result<Vec<ApiKey>, KalshiError> {
        let path = "/api_keys";
        let res: GetApiKeysResponse = self.signed_get(path).await?;
        Ok(res.api_keys)
    }

    /// Creates a new API key.
    ///
    /// This method generates a new API key for programmatic access.
    /// The secret will only be shown once during creation.
    ///
    /// # Arguments
    ///
    /// * `label` - A descriptive label for the API key.
    ///
    /// # Returns
    ///
    /// - `Ok(ApiKeyCreated)`: The created API key with its secret on successful creation.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let new_key = kalshi_instance.create_api_key("My Trading Bot").await.unwrap();
    /// println!("Save this secret: {}", new_key.secret);
    /// ```
    ///
    pub async fn create_api_key(&self, name: &str) -> Result<CreateApiKeyResponse, KalshiError> {
        let path = "/api_keys";
        let body = CreateApiKeyRequest { name: name.to_string() };
        self.signed_post(path, &body).await
    }

    /// Generates a new secret for an existing API key.
    ///
    /// This method rotates the secret for an existing API key.
    /// The old secret will be invalidated and a new one generated.
    ///
    /// # Arguments
    ///
    /// * `key_id` - The UUID of the API key to regenerate.
    ///
    /// # Returns
    ///
    /// - `Ok(ApiKeySecret)`: The new API key secret on successful generation.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let new_secret = kalshi_instance.generate_api_key("key-uuid").await.unwrap();
    /// ```
    ///
    pub async fn generate_api_key(&self, key_id: &str) -> Result<serde_json::Value, KalshiError> {
        let path = format!("/api_keys/{}/generate", key_id);
        self.signed_post(&path, &()).await
    }

    /// Deletes an API key.
    ///
    /// This method permanently deletes an API key. The key will immediately
    /// stop working and cannot be recovered.
    ///
    /// # Arguments
    ///
    /// * `key_id` - The UUID of the API key to delete.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: Success confirmation.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// kalshi_instance.delete_api_key("key-uuid").await.unwrap();
    /// ```
    ///
    pub async fn delete_api_key(&self, key_id: &str) -> Result<(), KalshiError> {
        let path = format!("/api_keys/{}", key_id);
        let _res: DeleteApiKeyResponse = self.signed_delete(&path).await?;
        Ok(())
    }

    /// Retrieves the API rate-limit tier for the authenticated user.
    pub async fn get_account_api_limits(&self) -> Result<AccountApiLimits, KalshiError> {
        self.signed_get("/account/api_limits").await
    }
}

// -------- Request bodies --------

#[derive(Debug, Serialize)]
struct CreateApiKeyRequest {
    name: String,
}

// -------- Response wrappers --------

#[derive(Debug, Deserialize)]
struct DeleteApiKeyResponse {}

