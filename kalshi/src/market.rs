use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize, Deserializer};
use std::collections::HashMap;

impl<'a> Kalshi {
    /// Retrieves a list of events from the Kalshi exchange based on specified criteria.
    ///
    /// This method fetches multiple events, allowing for filtering by status, series ticker,
    /// and pagination. The events represent prediction markets that users can trade on.
    ///
    /// # Arguments
    ///
    /// * `limit` - An optional integer to limit the number of events returned.
    /// * `cursor` - An optional string for pagination cursor.
    /// * `status` - An optional string to filter events by their status.
    /// * `series_ticker` - An optional string to filter events by series ticker.
    /// * `with_nested_markets` - An optional boolean to include nested markets in the response.
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
    ///     Some(10), None, Some("open".to_string()), None, Some(true)
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn get_events(
        &self,
        limit: Option<i64>, cursor: Option<String>,
        status: Option<String>, series_ticker: Option<String>,
        with_nested_markets: Option<bool>,
    ) -> Result<(Option<String>, Vec<Event>), KalshiError> {
        let url = format!("{}/events", self.base_url);
        let mut p = vec![];
        add_param!(p, "limit", limit);
        add_param!(p, "cursor", cursor);
        add_param!(p, "status", status);
        add_param!(p, "series_ticker", series_ticker);
        add_param!(p, "with_nested_markets", with_nested_markets);

        let res: EventListResponse = self.client
            .get(reqwest::Url::parse_with_params(&url, &p)?)
            .send().await?.json().await?;
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
    pub async fn get_event(&self, event_ticker: &str) -> Result<Event, KalshiError> {
        let url = format!("{}/events/{}", self.base_url, event_ticker);
        let res: SingleEventResponse = self.client.get(url).send().await?.json().await?;
        Ok(res.event)
    }

    /// Retrieves a list of markets from the Kalshi exchange based on specified criteria.
    ///
    /// This method fetches multiple markets, allowing for filtering by event ticker, series ticker,
    /// status, tickers, time range, and pagination. Markets represent the individual trading
    /// instruments within events.
    ///
    /// # Arguments
    ///
    /// * `limit` - An optional integer to limit the number of markets returned.
    /// * `cursor` - An optional string for pagination cursor.
    /// * `event_ticker` - An optional string to filter markets by event ticker.
    /// * `series_ticker` - An optional string to filter markets by series ticker.
    /// * `status` - An optional string to filter markets by their status.
    /// * `tickers` - An optional string to filter markets by specific tickers.
    /// * `min_close_ts` - An optional minimum timestamp for market close time.
    /// * `max_close_ts` - An optional maximum timestamp for market close time.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<Market>))`: A tuple containing an optional pagination cursor
    ///   and a vector of `Market` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let (cursor, markets) = kalshi_instance.get_markets(
    ///     Some(10), None, Some("SOME-EVENT".to_string()), None,
    ///     Some("open".to_string()), None, None, None
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn get_markets(
        &self,
        limit: Option<i64>, cursor: Option<String>,
        event_ticker: Option<String>, series_ticker: Option<String>,
        status: Option<String>, tickers: Option<String>,
        min_close_ts: Option<i64>, max_close_ts: Option<i64>,
    ) -> Result<(Option<String>, Vec<Market>), KalshiError> {
        let url = format!("{}/markets", self.base_url);
        let mut p = vec![];
        add_param!(p, "limit", limit);
        add_param!(p, "cursor", cursor);
        add_param!(p, "event_ticker", event_ticker);
        add_param!(p, "series_ticker", series_ticker);
        add_param!(p, "status", status);
        add_param!(p, "tickers", tickers);
        add_param!(p, "min_close_ts", min_close_ts);
        add_param!(p, "max_close_ts", max_close_ts);

        let res: MarketListResponse = self.client
            .get(reqwest::Url::parse_with_params(&url, &p)?)
            .send().await?.json().await?;
        Ok((res.cursor, res.markets))
    }

    /// Retrieves detailed information about a specific market from the Kalshi exchange.
    ///
    /// This method fetches data for a single market identified by its ticker.
    /// The market represents a specific trading instrument within an event.
    ///
    /// # Arguments
    ///
    /// * `ticker` - A string slice referencing the market's unique ticker identifier.
    ///
    /// # Returns
    ///
    /// - `Ok(Market)`: Detailed information about the specified market on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let ticker = "SOME-MARKET-2024";
    /// let market = kalshi_instance.get_market(ticker).await.unwrap();
    /// ```
    ///
    pub async fn get_market(&self, ticker: &str) -> Result<Market, KalshiError> {
        let url = format!("{}/markets/{}", self.base_url, ticker);
        let res: SingleMarketResponse = self.client.get(url).send().await?.json().await?;
        Ok(res.market)
    }

    /// Retrieves the orderbook for a specific market from the Kalshi exchange.
    ///
    /// This method fetches the current orderbook data for a market, showing the current
    /// bid and ask orders for both Yes and No sides of the market.
    ///
    /// # Arguments
    ///
    /// * `ticker` - A string slice referencing the market's unique ticker identifier.
    /// * `depth` - Optional depth parameter to limit the number of price levels returned.
    ///
    /// # Returns
    ///
    /// - `Ok(Orderbook)`: The current orderbook data for the specified market on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let ticker = "SOME-MARKET-2024";
    /// let orderbook = kalshi_instance.get_orderbook(ticker, Some(10)).await.unwrap();
    /// ```
    ///
    pub async fn get_orderbook(&self, ticker: &str, depth: Option<i32>) -> Result<Orderbook, KalshiError> {
        let mut url = format!("{}/markets/{}/orderbook", self.base_url, ticker);
        
        if let Some(d) = depth {
            url.push_str(&format!("?depth={}", d));
        }
        
        let res: OrderbookResponse = self.client.get(url).send().await?.json().await?;
        Ok(res.orderbook)
    }

    /// Retrieves the orderbook for a specific market from the Kalshi exchange (without depth limit).
    ///
    /// This is a convenience method that calls `get_orderbook(ticker, None)`.
    ///
    /// # Arguments
    ///
    /// * `ticker` - A string slice referencing the market's unique ticker identifier.
    ///
    /// # Returns
    ///
    /// - `Ok(Orderbook)`: The current orderbook data for the specified market on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let ticker = "SOME-MARKET-2024";
    /// let orderbook = kalshi_instance.get_orderbook_full(ticker).await.unwrap();
    /// ```
    ///
    pub async fn get_orderbook_full(&self, ticker: &str) -> Result<Orderbook, KalshiError> {
        self.get_orderbook(ticker, None).await
    }

    /// Retrieves candlestick data for a specific market from the Kalshi exchange.
    ///
    /// This method fetches historical price data in candlestick format for a market,
    /// allowing for analysis of price movements over time with various time intervals.
    ///
    /// # Arguments
    ///
    /// * `ticker` - A string slice referencing the market's unique ticker identifier.
    /// * `series_ticker` - A string slice referencing the series ticker.
    /// * `start_ts` - Optional timestamp for the start of the data range (restricts candlesticks to those ending on or after this timestamp).
    /// * `end_ts` - Optional timestamp for the end of the data range (restricts candlesticks to those ending on or before this timestamp).
    /// * `period_interval` - Optional integer specifying the length of each candlestick period in minutes (must be 1, 60, or 1440).
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Candle>)`: A vector of `Candle` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let candlesticks = kalshi_instance.get_market_candlesticks(
    ///     "SOME-MARKET-2024", "SOME-SERIES", 1640995200, 1641081600, 60
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn get_market_candlesticks(
        &self,
        ticker: &str,
        series_ticker: &str,
        start_ts: Option<i64>,
        end_ts: Option<i64>,
        period_interval: Option<i32>,
    ) -> Result<Vec<Candle>, KalshiError> {
        let url = format!("{}/series/{}/markets/{}/candlesticks",
                          self.base_url, series_ticker, ticker);
        let mut p = vec![];
        add_param!(p, "start_ts", start_ts);
        add_param!(p, "end_ts", end_ts);
        add_param!(p, "period_interval", period_interval);

        let res: CandlestickListResponse = self.client
            .get(reqwest::Url::parse_with_params(&url, &p)?)
            .send().await?.json().await?;
        Ok(res.candlesticks)
    }

    /// Retrieves a list of trades from the Kalshi exchange based on specified criteria.
    ///
    /// This method fetches multiple trades, allowing for filtering by ticker, time range,
    /// and pagination. Trades represent executed orders between buyers and sellers.
    ///
    /// # Arguments
    ///
    /// * `limit` - An optional integer to limit the number of trades returned.
    /// * `cursor` - An optional string for pagination cursor.
    /// * `ticker` - An optional string to filter trades by market ticker.
    /// * `min_ts` - An optional minimum timestamp for trade creation time.
    /// * `max_ts` - An optional maximum timestamp for trade creation time.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<Trade>))`: A tuple containing an optional pagination cursor
    ///   and a vector of `Trade` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let (cursor, trades) = kalshi_instance.get_trades(
    ///     Some(100), None, Some("SOME-MARKET-2024".to_string()),
    ///     Some(1640995200), Some(1641081600)
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn get_trades(
        &self,
        limit: Option<i64>, cursor: Option<String>,
        ticker: Option<String>, min_ts: Option<i64>, max_ts: Option<i64>,
    ) -> Result<(Option<String>, Vec<Trade>), KalshiError> {
        let url = format!("{}/markets/trades", self.base_url);
        let mut p = vec![];
        add_param!(p, "limit", limit);
        add_param!(p, "cursor", cursor);
        add_param!(p, "ticker", ticker);
        add_param!(p, "min_ts", min_ts);
        add_param!(p, "max_ts", max_ts);

        let res: TradeListResponse = self.client
            .get(reqwest::Url::parse_with_params(&url, &p)?)
            .send().await?.json().await?;
        Ok((res.cursor, res.trades))
    }

    /// Retrieves a list of series from the Kalshi exchange based on specified criteria.
    ///
    /// This method fetches multiple series, allowing for filtering by category, tags,
    /// and pagination. Series represent collections of related events and markets.
    ///
    /// # Arguments
    ///
    /// * `limit` - An optional integer to limit the number of series returned.
    /// * `cursor` - An optional string for pagination cursor.
    /// * `category` - An optional string to filter series by category.
    /// * `tags` - An optional string to filter series by tags.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<Series>))`: A tuple containing an optional pagination cursor
    ///   and a vector of `Series` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let (cursor, series) = kalshi_instance.get_series_list(
    ///     Some(20), None, Some("politics".to_string()), Some("election".to_string())
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn get_series_list(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
        category: Option<String>,
        tags: Option<String>,
    ) -> Result<(Option<String>, Vec<Series>), KalshiError> {
        // --- build query string ------------------------------------------------
        let mut p = Vec::new();
        add_param!(p, "limit",    limit);
        add_param!(p, "cursor",   cursor);
        add_param!(p, "category", category);
        add_param!(p, "tags",     tags);
    
        let path = if p.is_empty() {
            "/series".to_string()
        } else {
            format!("/series?{}", serde_urlencoded::to_string(&p)?)
        };
    
        // --- signed GET --------------------------------------------------------
        #[derive(Debug, serde::Deserialize)]
        struct SeriesListResponse {
            cursor: Option<String>,
            series: Option<Vec<Series>>,   // ← tolerate `null`
        }
    
        let res: SeriesListResponse = self.signed_get(&path).await?;
        Ok((res.cursor, res.series.unwrap_or_default()))
    }

    /// Retrieves detailed information about a specific series from the Kalshi exchange.
    ///
    /// This method fetches data for a single series identified by its series ticker.
    /// The series represents a collection of related events and markets.
    ///
    /// # Arguments
    ///
    /// * `series_ticker` - A string slice referencing the series' unique ticker identifier.
    ///
    /// # Returns
    ///
    /// - `Ok(Series)`: Detailed information about the specified series on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let series_ticker = "SOME-SERIES";
    /// let series = kalshi_instance.get_series(series_ticker).await.unwrap();
    /// ```
    ///
    pub async fn get_series(&self, series_ticker: &str) -> Result<Series, KalshiError> {
        let url = format!("{}/series/{}", self.base_url, series_ticker);
        let res: SingleSeriesResponse = self.client.get(url).send().await?.json().await?;
        Ok(res.series)
    }
}

/// When the API gives `"field": null` treat it as an empty Vec.
fn null_to_empty_vec<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<Vec<T>>::deserialize(d)?;
    Ok(opt.unwrap_or_default())
}

// -------- public models --------

/// Represents an event on the Kalshi exchange.
///
/// An event is a prediction market that contains multiple markets for trading.
/// Events can have various statuses and may include nested markets.
#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    pub event_ticker: String,
    pub series_ticker: String,
    pub title: String,
    pub sub_title: String,
    pub mutually_exclusive: bool,
    pub category: String,
    pub strike_date: Option<String>,
    pub strike_period: Option<String>,
    pub markets: Option<Vec<Market>>,
}

/// Represents a market on the Kalshi exchange.
///
/// A market is a specific trading instrument within an event, representing
/// a binary outcome that users can trade on (Yes/No).
#[derive(Debug, Deserialize, Serialize)]
pub struct Market {
    pub ticker: String,
    pub event_ticker: String,
    pub market_type: String,
    pub title: String,
    pub subtitle: String,
    pub yes_sub_title: String,
    pub no_sub_title: String,
    pub open_time: String,
    pub close_time: String,
    pub expected_expiration_time: Option<String>,
    pub expiration_time: Option<String>,
    pub latest_expiration_time: String,
    pub settlement_timer_seconds: i64,
    pub status: String,
    pub response_price_units: String,
    pub notional_value: i64,
    pub tick_size: i64,
    pub yes_bid: i64,
    pub yes_ask: i64,
    pub no_bid: i64,
    pub no_ask: i64,
    pub last_price: i64,
    pub previous_yes_bid: i64,
    pub previous_yes_ask: i64,
    pub previous_price: i64,
    pub volume: i64,
    pub volume_24h: i64,
    pub liquidity: i64,
    pub open_interest: i64,
    pub result: SettlementResult,
    pub cap_strike: Option<f64>,
    pub can_close_early: bool,
    pub expiration_value: String,
    pub category: String,
    pub risk_limit_cents: i64,
    pub strike_type: Option<String>,
    pub floor_strike: Option<f64>,
    pub rules_primary: String,
    pub rules_secondary: String,
    pub settlement_value: Option<String>,
    pub functional_strike: Option<String>,
}

/// Represents a series on the Kalshi exchange.
///
/// A series is a collection of related events and markets, typically
/// organized around a common theme or category.
#[derive(Debug, Deserialize, Serialize)]
pub struct Series {
    #[serde(default)]
    pub ticker: Option<String>,
    #[serde(default)]
    pub frequency: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(
        default,
        deserialize_with = "null_to_empty_vec"
    )]
    pub tags: Vec<String>,
    #[serde(
        default,
        deserialize_with = "null_to_empty_vec"
    )]
    pub settlement_sources: Vec<SettlementSource>,
    #[serde(default)]
    pub contract_url: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Represents a multivariate event collection on the Kalshi exchange.
///
/// A multivariate event collection contains multiple related markets
/// that are analyzed together as a group.
#[derive(Debug, Deserialize, Serialize)]
pub struct MultivariateEventCollection {
    pub collection_ticker: String,
    pub title: String,
    pub description: String,
    pub category: String,
    #[serde(
        default,
        deserialize_with = "null_to_empty_vec"
    )]
    pub tags: Vec<String>,
    #[serde(
        default,
        deserialize_with = "null_to_empty_vec"
    )]
    pub markets: Vec<Market>,
    pub created_time: String,
    pub updated_time: String,
}

/// Represents a candlestick data point for market analysis.
///
/// Candlesticks provide historical price data including open, high, low, and close
/// prices for both Yes and No sides of a market over a specific time period.
#[derive(Debug, Deserialize, Serialize)]
pub struct Candle {
    pub start_ts: i64,
    pub end_ts: i64,
    pub yes_open: i32,
    pub yes_high: i32,
    pub yes_low: i32,
    pub yes_close: i32,
    pub no_open: i32,
    pub no_high: i32,
    pub no_low: i32,
    pub no_close: i32,
    pub volume: i64,
    pub open_interest: i64,
}

/// Represents the orderbook for a market on the Kalshi exchange.
///
/// The orderbook contains current bid and ask orders for both Yes and No sides
/// of a market, showing the current market depth and liquidity.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Orderbook {
    pub yes: Option<Vec<Vec<i32>>>,
    pub no: Option<Vec<Vec<i32>>>,
}

/// Represents a market snapshot at a specific point in time.
///
/// A snapshot provides a summary of market activity including current prices,
/// volume, and open interest at a specific timestamp.
#[derive(Debug, Deserialize, Serialize)]
pub struct Snapshot {
    pub yes_price: i32,
    pub yes_bid: i32,
    pub yes_ask: i32,
    pub no_bid: i32,
    pub no_ask: i32,
    pub volume: i32,
    pub open_interest: i32,
    pub ts: i64,
}

/// Represents a trade executed on the Kalshi exchange.
///
/// A trade represents a completed transaction between a buyer and seller,
/// including the price, quantity, and timing of the execution.
#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
    pub trade_id: String,
    pub taker_side: String,
    pub ticker: String,
    pub count: i32,
    pub yes_price: i32,
    pub no_price: i32,
    pub created_time: String,
}

/// Represents the possible settlement results for a market.
///
/// Markets can settle in various ways depending on the outcome of the event
/// and the specific market rules.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SettlementResult {
    Yes,
    No,
    #[serde(rename = "")]
    Void,
    #[serde(rename = "all_no")]
    AllNo,
    #[serde(rename = "all_yes")]
    AllYes,
}

/// Represents the possible statuses of a market.
///
/// Markets can be in various states throughout their lifecycle from creation to settlement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarketStatus {
    Open,
    Closed,
    Settled,
}

/// Represents a settlement source for a series.
///
/// Settlement sources provide the data or methodology used to determine
/// the final outcome of markets in a series.
#[derive(Debug, Deserialize, Serialize)]
pub struct SettlementSource {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

// -------- response wrappers --------

#[derive(Debug, Deserialize)]
struct EventListResponse {
    cursor: Option<String>,
    events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
struct MarketListResponse {
    cursor: Option<String>,
    markets: Vec<Market>,
}

#[derive(Debug, Deserialize)]
struct SeriesListResponse {
    cursor: Option<String>,
    #[serde(default)]
    series: Vec<Series>,
}

#[derive(Debug, Deserialize)]
struct TradeListResponse {
    cursor: Option<String>,
    trades: Vec<Trade>,
}

#[derive(Debug, Deserialize)]
struct CandlestickListResponse {
    cursor: Option<String>,
    candlesticks: Vec<Candle>,
}

#[derive(Debug, Deserialize)]
struct SingleEventResponse {
    event: Event,
    markets: Option<Vec<Market>>,
}

#[derive(Debug, Deserialize)]
struct SingleMarketResponse {
    market: Market,
}

#[derive(Debug, Deserialize)]
struct SingleSeriesResponse {
    series: Series,
}

#[derive(Debug, Deserialize)]
struct OrderbookResponse {
    orderbook: Orderbook,
}