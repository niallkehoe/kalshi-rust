//! collection.rs – wrappers for Kalshi Trade API → collection (multivariate)
use serde::Deserialize;
use serde_json::Value;
use crate::{Kalshi, kalshi_error::*};

impl Kalshi {
    /// Retrieves a list of multivariate event collections from the Kalshi exchange.
    ///
    /// This method fetches multiple multivariate event collections, allowing for
    /// pagination. Collections group related markets together for analysis.
    ///
    /// # Arguments
    ///
    /// * `limit` - An optional integer to limit the number of collections returned.
    /// * `cursor` - An optional string for pagination cursor.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<Collection>))`: A tuple containing an optional pagination cursor
    ///   and a vector of `Collection` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let (cursor, collections) = kalshi_instance.get_multivariate_event_collections(
    ///     Some(10), None
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn get_multivariate_event_collections(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<(Option<String>, Vec<Collection>), KalshiError> {
        let mut p = Vec::new();
        add_param!(p, "limit", limit);
        add_param!(p, "cursor", cursor);
        let path = if p.is_empty() {
            "/multivariate_event_collections".to_string()
        } else {
            format!("/multivariate_event_collections?{}", serde_urlencoded::to_string(&p)?)
        };
        let res: CollectionListResponse = self.signed_get(&path).await?;
        Ok((res.cursor, res.multivariate_event_collections))
    }

    /// Retrieves detailed information about a specific multivariate event collection.
    ///
    /// This method fetches data for a single multivariate event collection identified
    /// by its collection ticker. Collections group related markets together for analysis.
    ///
    /// # Arguments
    ///
    /// * `collection_ticker` - A string slice referencing the collection's unique ticker identifier.
    ///
    /// # Returns
    ///
    /// - `Ok(Collection)`: Detailed information about the specified collection on successful retrieval.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let collection_ticker = "SOME-COLLECTION";
    /// let collection = kalshi_instance.get_multivariate_event_collection(collection_ticker).await.unwrap();
    /// ```
    ///
    pub async fn get_multivariate_event_collection(
        &self,
        collection_ticker: &str,
    ) -> Result<Collection, KalshiError> {
        let path = format!("/multivariate_event_collections/{collection_ticker}");
        let res: SingleCollectionResponse = self.signed_get(&path).await?;
        Ok(res.multivariate_event_collection)
    }

    /// Retrieves the lookup history for a multivariate event collection.
    ///
    /// This method fetches the historical lookup data for a collection, allowing for
    /// pagination. Lookup history shows how markets within the collection have been
    /// queried over time.
    ///
    /// # Arguments
    ///
    /// * `collection_ticker` - A string slice referencing the collection's unique ticker identifier.
    /// * `limit` - An optional integer to limit the number of lookup entries returned.
    /// * `cursor` - An optional string for pagination cursor.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<LookupEntry>))`: A tuple containing an optional pagination cursor
    ///   and a vector of `LookupEntry` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let (cursor, lookups) = kalshi_instance.get_collection_lookup_history(
    ///     "SOME-COLLECTION", Some(50), None
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn get_collection_lookup_history(
        &self,
        collection_ticker: &str,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<(Option<String>, Vec<LookupEntry>), KalshiError> {
        let mut p = Vec::new();
        add_param!(p, "limit", limit);
        add_param!(p, "cursor", cursor);
        let query = if p.is_empty() {
            String::new()
        } else {
            format!("?{}", serde_urlencoded::to_string(&p)?)
        };
        let path = format!("/multivariate_event_collections/{collection_ticker}/lookup{query}");
        let res: LookupHistoryResponse = self.signed_get(&path).await?;
        Ok((res.cursor, res.lookups))
    }

    /// Creates a new market within a multivariate event collection.
    ///
    /// This method adds a new market to an existing collection. The market data
    /// is provided as a JSON value in the request body.
    ///
    /// # Arguments
    ///
    /// * `collection_ticker` - A string slice referencing the collection's unique ticker identifier.
    /// * `body` - A reference to a JSON value containing the market data to be added.
    ///
    /// # Returns
    ///
    /// - `Ok(Collection)`: The updated collection object after adding the market on successful creation.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// use serde_json::json;
    /// let market_data = json!({
    ///     "market_ticker": "NEW-MARKET-2024",
    ///     "title": "New Market Title"
    /// });
    /// let updated_collection = kalshi_instance.create_market_in_collection(
    ///     "SOME-COLLECTION", &market_data
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn create_market_in_collection(
        &self,
        collection_ticker: &str,
        body: &Value,
    ) -> Result<Collection, KalshiError> {
        let path = format!("/multivariate_event_collections/{collection_ticker}");
        let res: SingleCollectionResponse = self.signed_post(&path, body).await?;
        Ok(res.multivariate_event_collection)
    }

    /// Performs a batch lookup for market tickers within a collection.
    ///
    /// This method allows querying multiple market tickers within a collection
    /// in a single request, returning the updated collection with lookup results.
    ///
    /// # Arguments
    ///
    /// * `collection_ticker` - A string slice referencing the collection's unique ticker identifier.
    /// * `body` - A reference to a JSON value containing the market tickers to lookup
    ///   (e.g., `{"market_tickers": ["A", "B", "C"]}`).
    ///
    /// # Returns
    ///
    /// - `Ok(Collection)`: The updated collection object with lookup results on successful operation.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// use serde_json::json;
    /// let tickers = json!({
    ///     "market_tickers": ["MARKET-A", "MARKET-B", "MARKET-C"]
    /// });
    /// let result = kalshi_instance.lookup_tickers_for_market(
    ///     "SOME-COLLECTION", &tickers
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn lookup_tickers_for_market(
        &self,
        collection_ticker: &str,
        body: &Value,
    ) -> Result<Collection, KalshiError> {
        let path = format!("/multivariate_event_collections/{collection_ticker}/tickers");
        let res: SingleCollectionResponse = self.signed_put(&path, Some(body)).await?;
        Ok(res.multivariate_event_collection)
    }
}

// -------- public models --------

/// Represents a multivariate event collection on the Kalshi exchange.
///
/// Collections group related markets together for analysis and management.
/// The schema is currently untyped as it's not publicly documented.
pub type Collection = Value;

/// Represents a lookup entry in a collection's history.
///
/// Lookup entries track how markets within a collection have been queried.
/// The schema is currently untyped as it's not publicly documented.
pub type LookupEntry = Value;

// -------- response wrappers --------

#[derive(Debug, Deserialize)]
struct CollectionListResponse {
    cursor: Option<String>,
    multivariate_event_collections: Vec<Collection>,
}

#[derive(Debug, Deserialize)]
struct SingleCollectionResponse {
    multivariate_event_collection: Collection,
}

#[derive(Debug, Deserialize)]
struct LookupHistoryResponse {
    cursor: Option<String>,
    lookups: Vec<LookupEntry>,
}
