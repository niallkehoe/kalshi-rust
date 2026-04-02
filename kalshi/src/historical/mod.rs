use super::Kalshi;
use crate::kalshi_error::*;
use crate::market::{Market, Trade};
use crate::portfolio::{Fill, Order};
use serde::Deserialize;

pub use crate::generated::types::GetHistoricalCutoffResponse as HistoricalCutoff;

impl Kalshi {
    /// Returns the cutoff timestamps separating live data from historical archives.
    pub async fn get_historical_cutoff(&self) -> Result<HistoricalCutoff, KalshiError> {
        self.signed_get("/historical/cutoff").await
    }

    /// Retrieves historical candlestick data for an archived market.
    ///
    /// `period_interval` must be 1, 60, or 1440 (minutes).
    pub async fn get_historical_candlesticks(
        &self,
        ticker: &str,
        start_ts: i64,
        end_ts: i64,
        period_interval: i32,
    ) -> Result<serde_json::Value, KalshiError> {
        let path = format!(
            "/historical/markets/{}/candlesticks?start_ts={}&end_ts={}&period_interval={}",
            ticker, start_ts, end_ts, period_interval
        );
        self.signed_get(&path).await
    }

    /// Retrieves historical fills (trades you participated in) from before the cutoff.
    pub async fn get_historical_fills(
        &self,
        ticker: Option<String>,
        max_ts: Option<i64>,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<(Option<String>, Vec<Fill>), KalshiError> {
        let mut params: Vec<(&str, String)> = Vec::new();
        add_param!(params, "ticker", ticker);
        add_param!(params, "max_ts", max_ts);
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);

        let path = build_path("/historical/fills", &params);
        let res: FillsHistoricalResponse = self.signed_get(&path).await?;
        Ok((empty_str_to_none(res.cursor), res.fills))
    }

    /// Retrieves historical orders (cancelled or executed) from before the cutoff.
    pub async fn get_historical_orders(
        &self,
        ticker: Option<String>,
        max_ts: Option<i64>,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<(Option<String>, Vec<Order>), KalshiError> {
        let mut params: Vec<(&str, String)> = Vec::new();
        add_param!(params, "ticker", ticker);
        add_param!(params, "max_ts", max_ts);
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);

        let path = build_path("/historical/orders", &params);
        let res: OrdersHistoricalResponse = self.signed_get(&path).await?;
        Ok((empty_str_to_none(res.cursor), res.orders))
    }

    /// Retrieves historical trades for all markets from before the cutoff.
    pub async fn get_historical_trades(
        &self,
        ticker: Option<String>,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<(Option<String>, Vec<Trade>), KalshiError> {
        let mut params: Vec<(&str, String)> = Vec::new();
        add_param!(params, "ticker", ticker);
        add_param!(params, "min_ts", min_ts);
        add_param!(params, "max_ts", max_ts);
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);

        let path = build_path("/historical/trades", &params);
        let res: TradesHistoricalResponse = self.signed_get(&path).await?;
        Ok((empty_str_to_none(res.cursor), res.trades))
    }

    /// Retrieves markets that have been archived to the historical database.
    pub async fn get_historical_markets(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
        tickers: Option<String>,
        event_ticker: Option<String>,
    ) -> Result<(Option<String>, Vec<Market>), KalshiError> {
        let mut params: Vec<(&str, String)> = Vec::new();
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "tickers", tickers);
        add_param!(params, "event_ticker", event_ticker);

        let path = build_path("/historical/markets", &params);
        let res: MarketsHistoricalResponse = self.signed_get(&path).await?;
        Ok((empty_str_to_none(res.cursor), res.markets))
    }

    /// Retrieves a single market from the historical database by its ticker.
    pub async fn get_historical_market(&self, ticker: &str) -> Result<Market, KalshiError> {
        let path = format!("/historical/markets/{}", ticker);
        let res: SingleMarketHistoricalResponse = self.signed_get(&path).await?;
        Ok(res.market)
    }
}

// -------- Helpers --------

fn build_path(base: &str, params: &[(&str, String)]) -> String {
    if params.is_empty() {
        base.to_string()
    } else {
        let qs = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        format!("{}?{}", base, qs)
    }
}

fn empty_str_to_none(s: String) -> Option<String> {
    if s.is_empty() { None } else { Some(s) }
}

// -------- Response wrappers --------

#[derive(Debug, Deserialize)]
struct FillsHistoricalResponse {
    cursor: String,
    fills: Vec<Fill>,
}

#[derive(Debug, Deserialize)]
struct OrdersHistoricalResponse {
    cursor: String,
    orders: Vec<Order>,
}

#[derive(Debug, Deserialize)]
struct TradesHistoricalResponse {
    cursor: String,
    trades: Vec<Trade>,
}

#[derive(Debug, Deserialize)]
struct MarketsHistoricalResponse {
    #[serde(default)]
    cursor: String,
    markets: Vec<Market>,
}

#[derive(Debug, Deserialize)]
struct SingleMarketHistoricalResponse {
    market: Market,
}

