use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};

impl Kalshi {
    /// Asynchronously retrieves the current status of the exchange.
    ///
    /// This function makes an HTTP GET request to the Kalshi exchange status endpoint
    /// and returns the current status of the exchange, including whether trading
    /// and the exchange itself are active.
    ///
    /// # Returns
    /// - `Ok(ExchangeStatus)`: ExchangeStatus object on successful retrieval.
    /// - `Err(KalshiError)`: Error in case of a failure in the HTTP request or response parsing.
    /// ```
    /// kalshi_instance.get_exchange_status().await.unwrap();
    /// ```
    pub async fn get_exchange_status(&self) -> Result<ExchangeStatus, KalshiError> {
        let exchange_status_url: &str = &format!("{}/exchange/status", self.base_url.to_string());
        
        let result: ExchangeStatus = self
            .client
            .get(exchange_status_url)
            .send()
            .await?
            .json()
            .await?;

        return Ok(result);
    }

    /// Asynchronously retrieves the exchange's trading schedule.
    ///
    /// Sends a GET request to the Kalshi exchange schedule endpoint to obtain
    /// detailed schedule information, including standard trading hours and
    /// maintenance windows.
    ///
    /// # Returns
    /// - `Ok(ExchangeScheduleStandard)`: ExchangeScheduleStandard object on success.
    /// - `Err(KalshiError)`: Error in case of a failure in the HTTP request or response parsing.
    /// ```
    /// kalshi_instance.get_exchange_schedule().await.unwrap();
    /// ```
    pub async fn get_exchange_schedule(&self) -> Result<ExchangeScheduleStandard, KalshiError> {
        let exchange_schedule_url: &str =
            &format!("{}/exchange/schedule", self.base_url.to_string());

        let result: ExchangeScheduleResponse = self
            .client
            .get(exchange_schedule_url)
            .send()
            .await?
            .json()
            .await?;
        return Ok(result.schedule);
    }
}

/// Represents the standard trading hours and maintenance windows of the exchange.
#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeScheduleStandard {
    pub standard_hours: Vec<StandardHours>,
    pub maintenance_windows: Vec<MaintenanceWindow>,
}

/// Internal struct used for deserializing the response from the exchange schedule endpoint.
#[derive(Debug, Deserialize, Serialize)]
struct ExchangeScheduleResponse {
    schedule: ExchangeScheduleStandard,
}

/// Represents the status of the exchange, including trading and exchange activity.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeStatus {
    pub trading_active: bool,
    pub exchange_active: bool,
}

/// Represents a maintenance window with start and end times.
#[derive(Debug, Deserialize, Serialize)]
pub struct MaintenanceWindow {
    pub start_datetime: String,
    pub end_datetime: String,
}

/// Contains the daily schedule for each day of the week.
#[derive(Debug, Deserialize, Serialize)]
pub struct StandardHours {
    pub start_time: String,
    pub end_time: String,
    pub monday: Vec<DaySchedule>,
    pub tuesday: Vec<DaySchedule>,
    pub wednesday: Vec<DaySchedule>,
    pub thursday: Vec<DaySchedule>,
    pub friday: Vec<DaySchedule>,
    pub saturday: Vec<DaySchedule>,
    pub sunday: Vec<DaySchedule>,
}

/// Represents the opening and closing times of the exchange for a single day.
#[derive(Debug, Deserialize, Serialize)]
pub struct DaySchedule {
    pub open_time: String,
    pub close_time: String,
}
