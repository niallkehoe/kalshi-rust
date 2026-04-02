use super::Kalshi;
use crate::kalshi_error::*;
// All public types are re-exported from the OpenAPI-generated module.
pub use crate::generated::types::{
    GetMarketOrderbookResponse, GetMarketOrderbooksResponse, GetMarketResponse, GetMarketsResponse,
    GetSeriesListResponse, GetSeriesResponse, GetTradesResponse, Market, MarketCandlestick,
    MarketCandlesticksResponse, MarketMarketType, MarketOrderbookFp, MarketResult, MarketStatus,
    MultivariateEventCollection, OrderbookCountFp, PriceLevelDollarsCountFp, Series,
    SettlementSource, Trade, TradeTakerSide,
};

/// Alias for [`OrderbookCountFp`] — the dollar-denominated orderbook returned by the API.
pub type Orderbook = OrderbookCountFp;

impl<'a> Kalshi {
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

        let res: GetMarketsResponse = self.client
            .get(reqwest::Url::parse_with_params(&url, &p)?)
            .send().await?.json().await?;
        let cursor = if res.cursor.is_empty() { None } else { Some(res.cursor) };
        Ok((cursor, res.markets))
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
        let res: GetMarketResponse = self.client.get(url).send().await?.json().await?;
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
    /// - `Ok(OrderbookCountFp)`: The current orderbook data for the specified market on successful retrieval.
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
    pub async fn get_orderbook(&self, ticker: &str, depth: Option<i32>) -> Result<OrderbookCountFp, KalshiError> {
        let mut url = format!("{}/markets/{}/orderbook", self.base_url, ticker);
        if let Some(d) = depth {
            url.push_str(&format!("?depth={}", d));
        }
        let res: GetMarketOrderbookResponse = self.client.get(&url).send().await?.json().await?;
        Ok(res.orderbook_fp)
    }

    /// Retrieves the orderbook for a specific market (without depth limit).
    pub async fn get_orderbook_full(&self, ticker: &str) -> Result<OrderbookCountFp, KalshiError> {
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
    ) -> Result<Vec<MarketCandlestick>, KalshiError> {
        let url = format!("{}/series/{}/markets/{}/candlesticks",
                          self.base_url, series_ticker, ticker);
        let mut p = vec![];
        add_param!(p, "start_ts", start_ts);
        add_param!(p, "end_ts", end_ts);
        add_param!(p, "period_interval", period_interval);

        let res: MarketCandlesticksResponse = self.client
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

        let res: GetTradesResponse = self.client
            .get(reqwest::Url::parse_with_params(&url, &p)?)
            .send().await?.json().await?;
        let cursor = if res.cursor.is_empty() { None } else { Some(res.cursor) };
        Ok((cursor, res.trades))
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
    ) -> Result<Vec<Series>, KalshiError> {
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

        // The API returns `null` for array fields inside Series (e.g. tags, settlement_sources,
        // additional_prohibitions). Patch nulls to [] before deserializing the generated type.
        let mut raw: serde_json::Value = self.signed_get(&path).await?;
        if let Some(arr) = raw.get_mut("series").and_then(|v| v.as_array_mut()) {
            for series in arr.iter_mut() {
                if let Some(obj) = series.as_object_mut() {
                    for key in ["additional_prohibitions", "settlement_sources", "tags"] {
                        if obj.get(key).map_or(false, |v| v.is_null()) {
                            obj.insert(key.to_string(), serde_json::Value::Array(vec![]));
                        }
                    }
                }
            }
        }
        #[derive(serde::Deserialize)]
        struct SeriesListResponse {
            #[serde(default)]
            series: Option<Vec<Series>>,
        }
        let res: SeriesListResponse = serde_json::from_value(raw)
            .map_err(|e| KalshiError::UserInputError(format!("series deserialization: {}", e)))?;
        Ok(res.series.unwrap_or_default())
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
        let res: GetSeriesResponse = self.client.get(url).send().await?.json().await?;
        Ok(res.series)
    }

    /// Retrieves orderbooks for multiple markets in a single request.
    ///
    /// Returns a list of `(ticker, orderbook)` pairs. The orderbook format is the
    /// full-precision dollar format (`orderbook_fp`) from the API.
    ///
    /// # Arguments
    /// * `tickers` - Slice of market tickers (1–100).
    pub async fn get_market_orderbooks(
        &self,
        tickers: &[&str],
    ) -> Result<Vec<MarketOrderbookFp>, KalshiError> {
        let qs = tickers.iter().map(|t| format!("tickers={}", t)).collect::<Vec<_>>().join("&");
        let path = format!("/markets/orderbooks?{}", qs);
        let res: GetMarketOrderbooksResponse = self.signed_get(&path).await?;
        Ok(res.orderbooks)
    }

    /// Retrieves candlesticks for multiple markets in a single batch request.
    ///
    /// Returns the raw JSON response since the batch format contains per-ticker
    /// candlestick arrays with complex types.
    ///
    /// # Arguments
    /// * `tickers` - Comma-separated market tickers.
    /// * `start_ts` - Start Unix timestamp.
    /// * `end_ts` - End Unix timestamp.
    /// * `period_interval` - Interval in minutes: 1, 60, or 1440.
    pub async fn batch_get_market_candlesticks(
        &self,
        tickers: &str,
        start_ts: i64,
        end_ts: i64,
        period_interval: i32,
    ) -> Result<serde_json::Value, KalshiError> {
        let path = format!(
            "/markets/candlesticks?tickers={}&start_ts={}&end_ts={}&period_interval={}",
            tickers, start_ts, end_ts, period_interval
        );
        self.signed_get(&path).await
    }
}



