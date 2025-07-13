use chrono::Utc;
use openssl::{
    hash::MessageDigest,
    rsa::Padding,
    sign::{RsaPssSaltlen, Signer},
};
use reqwest::header::{HeaderMap, HeaderValue};
use base64::Engine;

use crate::kalshi_error::KalshiError;
use crate::Kalshi; // struct defined in lib.rs

impl Kalshi {

    /// “Logout” for the key-based scheme – just delete the key material.
    pub async fn logout(&self) -> Result<(), KalshiError> {
        // TODO: implement logout
        // Nothing to tell the server.  Caller can simply drop the client.
        Ok(())
    }

    // -----------------------------------------------------------------------
    //  Helpers used by the other modules (signing + generic request)
    // -----------------------------------------------------------------------

    pub(crate) async fn signed_get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, KalshiError> {
        self.signed_request::<(), T>("GET", path, None).await
    }

    pub(crate) async fn signed_post<B: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, KalshiError> {
        self.signed_request("POST", path, Some(body)).await
    }

    pub(crate) async fn signed_delete<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, KalshiError> {
        self.signed_request::<(), T>("DELETE", path, None).await
    }

    pub(crate) async fn signed_put<B: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: Option<&B>,
    ) -> Result<T, KalshiError> {
        self.signed_request("PUT", path, body).await
    }

    async fn signed_request<B: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        path: &str,
        body: Option<&B>,
    ) -> Result<T, KalshiError> {
        let key_id = &self.key_id;
        let pkey = &self.private_key;

        let ts_ms = Utc::now().timestamp_millis();
        
        // Remove query parameters from path (like Python code does)
        let path_without_query = path.split('?').next().unwrap_or(path);
        let message = format!("{ts_ms}{method}/trade-api/v2{path_without_query}");

        // --- RSA-PSS / SHA-256 signature -----------------------------------
        let mut signer = Signer::new(MessageDigest::sha256(), pkey)?;
        signer.set_rsa_padding(Padding::PKCS1_PSS)?;
        signer.set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH)?;
        signer.update(message.as_bytes())?;
        let sig_raw = signer.sign_to_vec()?;
        let sig_b64 = base64::engine::general_purpose::STANDARD.encode(sig_raw);

        // --- build request --------------------------------------------------
        let url = format!("{}{}", self.base_url, path);
        let mut headers = HeaderMap::with_capacity(3);
        headers.insert("KALSHI-ACCESS-KEY", HeaderValue::from_str(key_id)?);
        headers.insert("KALSHI-ACCESS-TIMESTAMP", HeaderValue::from(ts_ms));
        headers.insert("KALSHI-ACCESS-SIGNATURE", HeaderValue::from_str(&sig_b64)?);

        let builder = match method {
            "GET" => self.client.get(&url),
            "POST" => self.client.post(&url),
            "PUT" => self.client.put(&url),
            "DELETE" => self.client.delete(&url),
            other => self.client.request(other.parse()?, &url),
        }
        .headers(headers);

        let resp = if let Some(b) = body {
            builder.json(b).send().await?
        } else {
            builder.send().await?
        }
        .error_for_status()?;

        Ok(resp.json::<T>().await?)
    }

}
