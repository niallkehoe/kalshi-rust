//! An HTTPS and Websocket wrapper that allows users to write trading bots for the [Kalshi events trading platform](https://kalshi.com).
//!
//! kalshi-rust is asynchronous, performant, and succint. Dash past verbose and annoying HTTPS requests
//! and use this wrapper to quickly write blazingly fast trading bots in Rust!
//!
//! As of version 0.9.0, HTTPS features are fully complete but websocket support and advanced API access features are not complete.
//! If you'd like to keep up on kalshi-rust's development, report bugs, or view a sample trading script,
//! feel free to visit the [github](https://github.com/dpeachpeach/kalshi-rust)!
//! A star would also be greatly appreciated, I'm a student developer writing this for free and any recognition is incredibly helpful!
//!
//! ## The Kalshi Struct
//!
//! The [Kalshi](Kalshi) struct is the central component of this crate.
//! All authentication, order routing, market requests, and position snapshots are handled through the struct and its methods.
//!
//! For more details, see [Kalshi](Kalshi).
//!
//! For a quick tutorial / beginners guide, jump [here](#quick-start-guide).
//!
//! ### Initializing the Kalshi struct in demo mode.
//! ```
//! use kalshi::Kalshi;
//! use kalshi::TradingEnvironment;
//!
//! let kalshi_instance = Kalshi::new(TradingEnvironment::DemoMode, "your-key-id", "path/to/private.pem").await?;
//! ```
//!
//! ## Quick Start Guide
//!
//! First, list the Kalshi struct as a dependency in your crate.
//!
//! ```toml
//! kalshi = { version = "0.9"}
//! ```
//!
//! Initialize the Kalshi Struct with key-based authentication:
//! - **IMPORTANT**:  The authentication is handled automatically when creating a new instance.
//! - Store your key ID and private key file securely, an implementation of extracting these from local environmental variables
//! is available [here](https://github.com/dpeachpeach/kalshi-rust/blob/main/sample_bot/src/main.rs#L12)
//! ```
//! use kalshi::Kalshi;
//! use kalshi::TradingEnvironment;
//!
//! let key_id = "your-key-id";
//! let pem_path = "path/to/private.pem";
//!
//! let kalshi_instance = Kalshi::new(TradingEnvironment::DemoMode, key_id, pem_path).await?;
//! ```
//!
//! After logging in, you can call any method present in the crate without issue.
//! Here is a script that buys a 'yes' contract on November 13th's New York temperature
//! market.
//!
//! ```
//! let new_york_ticker = "HIGHNY-23NOV13-T51".to_string();
//!
//! let bought_order = kalshi_instance
//!     .create_order(
//!     kalshi::Action::Buy,
//!     None,
//!     1,
//!     kalshi::Side::Yes,
//!     new_york_ticker,
//!     kalshi::OrderType::Limit,
//!     None,
//!     None,
//!     None,
//!     None,
//!     Some(5)).await.unwrap();
//! ```
//!
//! Refer to the rest of the documentation for details on all other methods!
//! 
//! ## Returned Values
//!
//! Whenever a user makes a method call using the kalshi struct, data is typically returned
//! in structs that encapsulate the json fields returned by the server. All data
//! in the structs is owned so a user can access the attributes without issue.
//!
//! ### Examples:
//!
//! #### Obtaining the Exchange's current status
//! Returns a struct that represents whether trading or the exchange are currently active.
//! ```
//! use kalshi::Kalshi;
//! use kalshi::TradingEnvironment;
//! let kalshi_instance = Kalshi::new(TradingEnvironment::DemoMode, "your-key-id", "path/to/private.pem").await?;
//!
//! kalshi_instance.get_exchange_status().await.unwrap();
//! ```
//!
//! #### Obtaining 5 miscellaneous market events
//! Returns a vector of 'event' structs and a cursor.
//! ```
//! use kalshi::Kalshi;
//! use kalshi::TradingEnvironment;
//! let kalshi_instance = Kalshi::new(TradingEnvironment::DemoMode, "your-key-id", "path/to/private.pem").await?;
//!
//! kalshi_instance.get_multiple_events(Some(5), None, None, None, None).await.unwrap();
//! ```
//! #### Checking the User's balance
//! Returns an i64 representing the user's balance in cents.
//! ```
//! use kalshi::Kalshi;
//! use kalshi::TradingEnvironment;
//! let kalshi_instance = Kalshi::new(TradingEnvironment::DemoMode, "your-key-id", "path/to/private.pem").await?;
//!
//! kalshi_instance.get_balance();
//! ```
//!

#[macro_use]
mod utils;
mod auth;
mod collection;
mod exchange;
mod kalshi_error;
mod market;
mod milestone;
mod portfolio;

// pub use auth::*;  // Unused import
pub use collection::*;
pub use exchange::*;
pub use kalshi_error::*;
pub use market::*;
pub use milestone::*;
pub use portfolio::*;

// imports
use reqwest;
use openssl::pkey::{PKey, Private};
use std::fs;
use std::path::Path;

/// The Kalshi struct is the core of the kalshi-crate. It acts as the interface
/// between the user and the market, abstracting away the meat of requests
/// by encapsulating authentication information and the client itself.
///
/// ## Creating a new `Kalshi` instance for demo mode:
///
/// ```
/// use kalshi::Kalshi;
/// use kalshi::TradingEnvironment;
///
/// let kalshi_instance = Kalshi::new(TradingEnvironment::DemoMode, "your-key-id", "path/to/private.pem").await?;
/// ```
///
///
#[derive(Debug, Clone)]

pub struct Kalshi {
    /// - `base_url`: The base URL for the API, determined by the trading environment.
    base_url: String,
    /// - `key_id`: Key ID for key-based authentication
    key_id: String,
    /// - `private_key`: Private key for key-based authentication
    private_key: PKey<Private>,
    /// - `client`: The HTTP client used for making requests to the marketplace.
    client: reqwest::Client,
}

impl Kalshi {
    /// Creates a new instance of Kalshi with the specified trading environment and authenticates immediately.
    /// This environment determines the base URL used for API requests.
    ///
    /// # Arguments
    ///
    /// * `trading_env` - The trading environment to be used (ProdMode: Trading with real money. DemoMode: Paper Trading).
    /// * `key_id` - The UUID shown next to the key in your Kalshi UI
    /// * `pem_path` - Path to the private key file you downloaded
    ///
    /// # Example
    ///
    /// ## Creating a Demo instance with authentication.
    /// ```
    /// use kalshi::{Kalshi, TradingEnvironment};
    /// let kalshi = Kalshi::new(TradingEnvironment::DemoMode, "your-key-id", "path/to/private.pem").await?;
    /// ```
    ///
    /// ## Creating a Live Trading instance with authentication (Warning, you're using real money!)
    /// ```
    /// use kalshi::{Kalshi, TradingEnvironment};
    /// let kalshi = Kalshi::new(TradingEnvironment::ProdMode, "your-key-id", "path/to/private.pem").await?;
    /// ```
    ///
    pub async fn new(trading_env: TradingEnvironment, key_id: &str, pem_path: &str) -> Result<Self, crate::kalshi_error::KalshiError> {
        println!("Loading private key from: {}", pem_path);
        
        // Load the private key first
        let pem = match fs::read(Path::new(pem_path)) {
            Ok(pem) => {
                println!("Successfully read private key file");
                pem
            },
            Err(e) => {
                eprintln!("Failed to read private key file: {:?}", e);
                return Err(e.into());
            }
        };
        
        let private_key = match PKey::private_key_from_pem(&pem) {
            Ok(key) => {
                println!("Successfully parsed private key");
                key
            },
            Err(e) => {
                eprintln!("Failed to parse private key: {:?}", e);
                return Err(e.into());
            }
        };
        
        let base_url = utils::build_base_url(trading_env).to_string();
        let kalshi = Self {
            base_url,
            key_id: key_id.to_string(),
            private_key,
            client: reqwest::Client::new(),
        };
        
        // Verify authentication by hitting the exchange status endpoint
        println!("Verifying authentication with exchange status endpoint...");
        match kalshi.get_exchange_status().await {
            Ok(status) => {
                println!("Authentication successful! Exchange status: {:?}", status);
                Ok(kalshi)
            },
            Err(e) => {
                eprintln!("Authentication failed: {:?}", e);
                eprintln!("Please check your API key and private key file");
                Err(e)
            }
        }
    }
}

// GENERAL ENUMS
// -----------------------------------------------

/// Defines the trading environment for the Kalshi exchange.
///
/// This enum is used to specify whether the interaction with the Kalshi API should be in a demo (simulated) environment
/// or in the live market with real financial transactions.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TradingEnvironment {
    /// The demo mode represents a simulated environment where trades do not involve real money.
    /// This mode is typically used for testing and practice purposes.
    DemoMode,

    /// The live market mode is the real trading environment where all transactions involve actual financial stakes.
    /// Use this mode for actual trading activities with real money.
    ProdMode,
}
