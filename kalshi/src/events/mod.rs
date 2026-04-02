use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};

pub use crate::generated::types::{EventData as Event, MarketCandlestick};

impl Kalshi {
    /// Retrieves a list of events from the Kalshi exchange based on specified criteria.
    ///
    /// This method fetches multiple events, allowing for filtering by status, series ticker,
    /// and pagination. The events represent prediction markets that users can trade on.
    ///
    /// # Arguments
    ///
    /// * `limit` - An optional integer to limit the number of events returned.
    /// * `cursor` - An optional string for pagination cursor.
    /// * `status` - An optional string to filter events by their status (e.g., "open", "closed", "settled").
    /// * `series_ticker` - An optional string to filter events by series ticker.
    /// * `with_nested_markets` - An optional boolean to include nested markets in the response.
    /// * `with_milestones` - An optional boolean to include related milestones.
    /// * `min_close_ts` - An optional Unix timestamp to filter events with at least one market closing after this time.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<Event>))`: A tuple containing an optional pagination cursor
    ///   and a vector of `Event` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let (cursor, events) = kalshi_instance.get_events(
    ///     Some(10), None, Some("open".to_string()), None, Some(true), Some(false), None
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn get_events(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
        status: Option<String>,
        series_ticker: Option<String>,
        with_nested_markets: Option<bool>,
        with_milestones: Option<bool>,
        min_close_ts: Option<i64>,
    ) -> Result<(Option<String>, Vec<Event>), KalshiError> {
        let path = "/events";
        let mut params = vec![];
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "status", status);
        add_param!(params, "series_ticker", series_ticker);
        add_param!(params, "with_nested_markets", with_nested_markets);
        add_param!(params, "with_milestones", with_milestones);
        add_param!(params, "min_close_ts", min_close_ts);

        let url = format!("{}{}", self.base_url, path);
        let final_url = reqwest::Url::parse_with_params(&url, &params)?;
        let res: EventListResponse = self.client.get(final_url).send().await?.json().await?;
        Ok((res.cursor, res.events))
    }

    /// Retrieves detailed information about a specific event from the Kalshi exchange.
    ///
    /// This method fetches data for a single event identified by its event ticker.
    /// The event represents a prediction market with associated markets that users can trade on.
    ///
    /// # Arguments
    ///
    /// * `event_ticker` - A string slice referencing the event's unique ticker identifier.
    ///
    /// # Returns
    ///
    /// - `Ok(Event)`: Detailed information about the specified event on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let event_ticker = "SOME-EVENT-2024";
    /// let event = kalshi_instance.get_event(event_ticker).await.unwrap();
    /// ```
    ///
    /// Retrieves multivariate (combo) events, optionally filtered by series or collection.
    pub async fn get_multivariate_events(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
        series_ticker: Option<String>,
        collection_ticker: Option<String>,
        with_nested_markets: Option<bool>,
    ) -> Result<(Option<String>, Vec<Event>), KalshiError> {
        let mut params: Vec<(&str, String)> = Vec::new();
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "series_ticker", series_ticker);
        add_param!(params, "collection_ticker", collection_ticker);
        add_param!(params, "with_nested_markets", with_nested_markets);

        let path = if params.is_empty() {
            "/events/multivariate".to_string()
        } else {
            let qs = params.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("&");
            format!("/events/multivariate?{}", qs)
        };

        let res: MultivariateEventListResponse = self.signed_get(&path).await?;
        Ok((res.cursor, res.events))
    }

    pub async fn get_event(&self, event_ticker: &str) -> Result<Event, KalshiError> {
        let path = format!("/events/{}", event_ticker);
        self.signed_get(&path).await
    }

    /// Retrieves candlestick data aggregated across all markets in an event.
    ///
    /// This method provides event-level candlestick data by aggregating trading data
    /// from all markets within the specified event.
    ///
    /// # Arguments
    ///
    /// * `series_ticker` - The series ticker containing the event.
    /// * `event_ticker` - The event ticker to get candlestick data for.
    /// * `start_ts` - Optional start timestamp (Unix timestamp).
    /// * `end_ts` - Optional end timestamp (Unix timestamp).
    /// * `period_interval` - Optional candlestick interval (e.g., "1m", "5m", "1h", "1d").
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Candlestick>)`: A vector of candlestick data on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let candlesticks = kalshi_instance.get_event_candlesticks(
    ///     "SERIES", "EVENT-TICKER", None, None, Some("1h".to_string())
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn get_event_candlesticks(
        &self,
        series_ticker: &str,
        event_ticker: &str,
        start_ts: Option<i64>,
        end_ts: Option<i64>,
        period_interval: Option<String>,
    ) -> Result<Vec<MarketCandlestick>, KalshiError> {
        let path = format!("/series/{}/events/{}/candlesticks", series_ticker, event_ticker);
        let mut params = vec![];
        add_param!(params, "start_ts", start_ts);
        add_param!(params, "end_ts", end_ts);
        add_param!(params, "period_interval", period_interval);

        let url = format!("{}{}", self.base_url, path);
        let final_url = reqwest::Url::parse_with_params(&url, &params)?;
        let res: CandlestickResponse = self.client.get(final_url).send().await?.json().await?;
        Ok(res.candlesticks)
    }

    /// Retrieves metadata for a specific event.
    ///
    /// This method provides additional metadata and information about an event
    /// that may not be included in the standard event details.
    ///
    /// # Arguments
    ///
    /// * `event_ticker` - The event ticker to get metadata for.
    ///
    /// # Returns
    ///
    /// - `Ok(EventMetadata)`: The event metadata on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let metadata = kalshi_instance.get_event_metadata("EVENT-TICKER").await.unwrap();
    /// ```
    ///
    pub async fn get_event_metadata(&self, event_ticker: &str) -> Result<EventMetadata, KalshiError> {
        let path = format!("/events/{}/metadata", event_ticker);
        self.signed_get(&path).await
    }

    /// Retrieves forecast percentile history for a specific event.
    ///
    /// This method provides historical percentile data for event forecasts,
    /// useful for analyzing prediction accuracy and trends over time.
    ///
    /// # Arguments
    ///
    /// * `event_ticker` - The event ticker to get forecast history for.
    ///
    /// # Returns
    ///
    /// - `Ok(ForecastPercentileHistory)`: The forecast percentile history data on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let history = kalshi_instance.get_event_forecast_percentile_history("EVENT-TICKER").await.unwrap();
    /// ```
    ///
    pub async fn get_event_forecast_percentile_history(
        &self,
        event_ticker: &str,
    ) -> Result<ForecastPercentileHistory, KalshiError> {
        let path = format!("/events/{}/forecast_percentile_history", event_ticker);
        self.signed_get(&path).await
    }
}

// -------- Response wrappers --------

#[derive(Debug, Deserialize)]
struct EventListResponse {
    cursor: Option<String>,
    events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
struct MultivariateEventListResponse {
    cursor: Option<String>,
    events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
struct SingleEventResponse {
    event: Event,
}

#[derive(Debug, Deserialize)]
struct CandlestickResponse {
    candlesticks: Vec<MarketCandlestick>,
}

// -------- Public models --------

/// Represents additional metadata for an event.
#[derive(Debug, Deserialize, Serialize)]
pub struct EventMetadata {
    #[serde(flatten)]
    pub fields: std::collections::HashMap<String, serde_json::Value>,
}

/// Represents forecast percentile history for an event.
#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastPercentileHistory {
    pub history: Vec<ForecastDataPoint>,
}

/// Represents a single forecast data point.
#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastDataPoint {
    pub ts: String,
    pub percentiles: std::collections::HashMap<String, f64>,
}

