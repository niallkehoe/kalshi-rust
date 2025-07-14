use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Serialize};

impl Kalshi {
    /// Retrieves the current status of the Kalshi exchange and trading engine.
    ///
    /// This method checks whether both the exchange platform and the trading engine
    /// are currently operational and accepting orders.
    ///
    /// # Returns
    ///
    /// - `Ok(ExchangeStatus)`: The current status of the exchange and trading engine on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let status = kalshi_instance.get_exchange_status().await.unwrap();
    /// if status.trading_active && status.exchange_active {
    ///     println!("Exchange is live and accepting trades");
    /// }
    /// ```
    ///
    pub async fn get_exchange_status(&self) -> Result<ExchangeStatus, KalshiError> {
        let url = format!("{}/exchange/status", self.base_url);
        Ok(self.client.get(&url).send().await?.json().await?)
    }

    /// Retrieves the exchange schedule including trading hours and maintenance windows.
    ///
    /// This method fetches the standard trading hours for each day of the week
    /// and any scheduled maintenance windows when the exchange may be unavailable.
    ///
    /// # Returns
    ///
    /// - `Ok(ExchangeSchedule)`: The exchange schedule including trading hours and maintenance windows on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let schedule = kalshi_instance.get_exchange_schedule().await.unwrap();
    /// println!("Standard hours: {:?}", schedule.standard_hours);
    /// println!("Maintenance windows: {:?}", schedule.maintenance_windows);
    /// ```
    ///
    pub async fn get_exchange_schedule(&self) -> Result<ExchangeSchedule, KalshiError> {
        let url = format!("{}/exchange/schedule", self.base_url);
        let res: ExchangeScheduleResponse = self.client.get(&url).send().await?.json().await?;
        Ok(res.schedule)
    }

    /// Retrieves exchange announcements including active and historical messages.
    ///
    /// This method fetches all exchange-wide announcements, both active and expired,
    /// allowing users to stay informed about important updates, maintenance schedules,
    /// and other exchange-related information.
    ///
    /// # Arguments
    ///
    /// * `limit` - An optional integer to limit the number of announcements returned.
    /// * `cursor` - An optional string for pagination cursor.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<ExchangeAnnouncement>))`: A tuple containing an optional pagination cursor
    ///   and a vector of `ExchangeAnnouncement` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let (cursor, announcements) = kalshi_instance.get_exchange_announcements(
    ///     Some(10), None
    /// ).await.unwrap();
    /// for announcement in announcements {
    ///     println!("{}: {}", announcement.ts, announcement.message);
    /// }
    /// ```
    ///
    pub async fn get_exchange_announcements(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<(Option<String>, Vec<ExchangeAnnouncement>), KalshiError> {
        let url = format!("{}/exchange/announcements", self.base_url);

        let mut params = vec![];
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);

        let final_url = reqwest::Url::parse_with_params(&url, &params)?;
        let res: ExchangeAnnouncementsResponse = self.client.get(final_url).send().await?.json().await?;
        Ok((res.cursor, res.announcements))
    }

    /// Retrieves the timestamp indicating when portfolio data was last refreshed.
    ///
    /// This method provides the timestamp of the last update to portfolio-related
    /// endpoints, allowing users to determine the freshness of their account data.
    ///
    /// # Returns
    ///
    /// - `Ok(UserDataTimestamp)`: The timestamp of the last portfolio data refresh on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let timestamp = kalshi_instance.get_user_data_timestamp().await.unwrap();
    /// println!("Portfolio data last updated: {}", timestamp.last_validated_ts);
    /// ```
    ///
    pub async fn get_user_data_timestamp(&self) -> Result<UserDataTimestamp, KalshiError> {
        let url = format!("{}/exchange/user_data_timestamp", self.base_url);
        Ok(self.client.get(&url).send().await?.json().await?)
    }

    /// Checks if the exchange is active with exponential backoff retry logic.
    ///
    /// This method attempts to verify that the exchange is operational and trading is active.
    /// If the exchange is not active, it will retry with exponential backoff delays.
    /// After all attempts are exhausted, the program will exit with a non-zero status code.
    ///
    /// # Arguments
    ///
    /// * `max_attempts` - Maximum number of attempts to check exchange status (default: 5)
    /// * `base_delay_secs` - Base delay in seconds for exponential backoff (default: 30.0)
    /// * `max_delay_secs` - Maximum delay in seconds to cap exponential growth (default: 300.0)
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the exchange becomes active within the retry attempts
    /// - `Err(KalshiError)`: If there's an unrecoverable error during the process
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// // This will exit the program if exchange doesn't become active after retries
    /// kalshi_instance.check_exchange_active_with_backoff().await.unwrap();
    /// 
    /// // Custom configuration
    /// kalshi_instance.check_exchange_active_with_backoff(3, 60.0, 600.0).await.unwrap();
    /// ```
    ///
    pub async fn check_exchange_active_with_backoff(
        &self,
        max_attempts: u32,
        base_delay_secs: f64,
        max_delay_secs: f64
    ) -> Result<(), KalshiError> {
        use std::process;
        use tokio::time::{sleep, Duration};
        
        for attempt in 1..=max_attempts {
            match self.get_exchange_status().await {
                Ok(status) => {
                    if status.trading_active && status.exchange_active {
                        println!("Exchange is active (attempt {}/{}", attempt, max_attempts);
                        return Ok(());
                    }
                    
                    if attempt < max_attempts {
                        let delay_secs = (base_delay_secs * (2.0_f64.powi((attempt - 1) as i32)))
                            .min(max_delay_secs);
                        println!(
                            "Exchange not active (attempt {}/{}). Waiting {:.1} seconds before retry...",
                            attempt, max_attempts, delay_secs
                        );
                        sleep(Duration::from_secs_f64(delay_secs)).await;
                    } else {
                        println!("Exchange not active after {} attempts", max_attempts);
                    }
                }
                Err(e) => {
                    if attempt < max_attempts {
                        let delay_secs = (base_delay_secs * (2.0_f64.powi((attempt - 1) as i32)))
                            .min(max_delay_secs);
                        println!(
                            "Error checking exchange status (attempt {}/{}): {}. Waiting {:.1} seconds before retry...",
                            attempt, max_attempts, e, delay_secs
                        );
                        sleep(Duration::from_secs_f64(delay_secs)).await;
                    } else {
                        println!("Failed to check exchange status after {} attempts: {}", max_attempts, e);
                    }
                }
            }
        }
        
        println!("Exiting as exchange is not active after all retry attempts");
        process::exit(1);
    }

    /// Convenience method to check exchange status with default backoff settings.
    ///
    /// This method calls `check_exchange_active_with_backoff` with default parameters:
    /// - max_attempts: 5
    /// - base_delay_secs: 30.0
    /// - max_delay_secs: 300.0
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the exchange becomes active within the retry attempts
    /// - `Err(KalshiError)`: If there's an unrecoverable error during the process
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// kalshi_instance.check_exchange_active().await.unwrap();
    /// ```
    ///
    pub async fn check_exchange_active(&self) -> Result<(), KalshiError> {
        self.check_exchange_active_with_backoff(5, 30.0, 300.0).await
    }

    
}

// -------- public models --------

/// Represents the operational status of the Kalshi exchange.
///
/// This struct provides simple boolean flags indicating whether the exchange
/// platform and trading engine are currently active and operational.
#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeStatus {
    /// Indicates whether the trading engine is currently active and accepting orders.
    pub trading_active: bool,
    /// Indicates whether the exchange platform is currently operational.
    pub exchange_active: bool,
}

/// Represents the trading schedule and maintenance windows for the Kalshi exchange.
///
/// This struct contains the standard trading hours for each day of the week
/// and any scheduled maintenance windows when the exchange may be unavailable.
#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeSchedule {
    /// The standard trading hours for each day of the week.
    pub standard_hours: Vec<StandardHours>,
    /// Scheduled maintenance windows when the exchange may be unavailable.
    pub maintenance_windows: Vec<MaintenanceWindow>,
}

/// Represents an exchange-wide announcement from Kalshi.
///
/// Announcements provide important information about exchange updates,
/// maintenance schedules, new features, or other relevant information
/// that users need to be aware of.
#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeAnnouncement {
    /// The announcement message content.
    pub message: String,
    /// The timestamp when the announcement was created (seconds since epoch, ISO string, or RFC3339).
    pub ts: String,
    /// The current status of the announcement (e.g., "active" or "expired").
    pub status: String,
}

/// Represents the timestamp of the last portfolio data refresh.
///
/// This struct provides information about when user portfolio data
/// was last updated, allowing users to determine data freshness.
#[derive(Debug, Deserialize, Serialize)]
pub struct UserDataTimestamp {
    /// The timestamp of the last portfolio data validation/refresh.
    pub last_validated_ts: String,
}

/// Represents a scheduled maintenance window for the Kalshi exchange.
///
/// Maintenance windows indicate periods when the exchange may be unavailable
/// for trading or other operations due to scheduled maintenance.
#[derive(Debug, Deserialize, Serialize)]
pub struct MaintenanceWindow {
    /// The start datetime of the maintenance window.
    pub start_datetime: String,
    /// The end datetime of the maintenance window.
    pub end_datetime: String,
}

/// Represents the trading schedule for a specific day.
///
/// This struct defines the opening and closing times for trading
/// on a particular day of the week.
#[derive(Debug, Deserialize, Serialize)]
pub struct DaySchedule {
    /// The time when trading opens for this day.
    pub open_time: String,
    /// The time when trading closes for this day.
    pub close_time: String,
}

/// Represents the standard trading hours for the Kalshi exchange.
///
/// This struct defines the trading schedule for each day of the week,
/// including multiple time slots per day if applicable.
#[derive(Debug, Deserialize, Serialize)]
pub struct StandardHours {
    /// The start time for the trading period.
    pub start_time: String,
    /// The end time for the trading period.
    pub end_time: String,
    /// Trading schedule for Monday.
    #[serde(default)] pub monday: Vec<DaySchedule>,
    /// Trading schedule for Tuesday.
    #[serde(default)] pub tuesday: Vec<DaySchedule>,
    /// Trading schedule for Wednesday.
    #[serde(default)] pub wednesday: Vec<DaySchedule>,
    /// Trading schedule for Thursday.
    #[serde(default)] pub thursday: Vec<DaySchedule>,
    /// Trading schedule for Friday.
    #[serde(default)] pub friday: Vec<DaySchedule>,
    /// Trading schedule for Saturday.
    #[serde(default)] pub saturday: Vec<DaySchedule>,
    /// Trading schedule for Sunday.
    #[serde(default)] pub sunday: Vec<DaySchedule>,
}

// -------- response wrappers --------

#[derive(Debug, Deserialize)]
struct ExchangeScheduleResponse {
    schedule: ExchangeSchedule,
}

#[derive(Debug, Deserialize)]
struct ExchangeAnnouncementsResponse {
    cursor: Option<String>,
    announcements: Vec<ExchangeAnnouncement>,
}
