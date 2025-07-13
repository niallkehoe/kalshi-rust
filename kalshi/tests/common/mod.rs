use kalshi::{Kalshi, TradingEnvironment, KalshiError};
use std::env;
use std::sync::Once;

static INIT: Once = Once::new();
static SKIP_MESSAGE_SHOWN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Initialize test environment - loads environment variables
pub fn init_test_env() {
    INIT.call_once(|| {
        // Try to load from custom env file path first
        if let Ok(env_path) = std::env::var("KALSHI_ENV_FILE") {
            if let Err(e) = dotenv::from_path(&env_path) {
                eprintln!("Warning: Failed to load env file from {}: {:?}", env_path, e);
            }
        } else {
            // Fall back to default .env in current directory
            dotenv::dotenv().ok();
        }
    });
}

/// Test authentication configuration
pub struct TestAuth {
    pub key_id: String,
    pub pem_path: String,
    pub environment: TradingEnvironment,
}

impl TestAuth {
    /// Create test auth from environment variables
    pub fn from_env() -> Option<Self> {
        let key_id = env::var("KALSHI_DEMO_API_KEY").ok()?;
        let pem_path = env::var("KALSHI_DEMO_PEM_PATH").ok()?;
        let environment = match env::var("KALSHI_TEST_ENV").unwrap_or_else(|_| "demo".to_string()).as_str() {
            "prod" => TradingEnvironment::ProdMode,
            "demo" | _ => TradingEnvironment::DemoMode,
        };
        
        Some(TestAuth {
            key_id,
            pem_path,
            environment,
        })
    }
    
    /// Create a Kalshi instance for testing
    pub async fn create_kalshi(&self) -> Result<Kalshi, KalshiError> {
        Kalshi::new(self.environment, &self.key_id, &self.pem_path).await
    }
}

/// Require authentication for test - will panic if not available
pub fn require_auth() -> TestAuth {
    init_test_env();
    match TestAuth::from_env() {
        Some(auth) => auth,
        None => {
            eprintln!("❌ Test requires authentication: KALSHI_DEMO_API_KEY and KALSHI_DEMO_PEM_PATH environment variables not set");
            eprintln!("   Set KALSHI_ENV_FILE=/path/to/env or create .env file to run authenticated tests");
            panic!("Authentication required but not available");
        }
    }
}

/// Skip test if authentication is not available (legacy function for backward compatibility)
pub fn skip_if_no_auth() -> Option<TestAuth> {
    init_test_env();
    TestAuth::from_env()
}

/// Show skip message only once (legacy function for backward compatibility)
pub fn show_skip_message_once() {
    if !SKIP_MESSAGE_SHOWN.load(std::sync::atomic::Ordering::Relaxed) {
        eprintln!("⚠️  Skipping authenticated tests: KALSHI_DEMO_API_KEY and KALSHI_DEMO_PEM_PATH environment variables not set");
        eprintln!("   Set KALSHI_ENV_FILE=/path/to/env or create .env file to run all tests");
        SKIP_MESSAGE_SHOWN.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}



/// Function to create an authenticated test setup
/// Usage: 
/// ```
/// #[tokio::test]
/// async fn my_test() {
///     let kalshi = setup_auth_test().await.unwrap();
///     // ... test code using kalshi
/// }
/// ```
pub async fn setup_auth_test() -> Result<Kalshi, KalshiError> {
    let auth = require_auth();
    auth.create_kalshi().await
}

/// Test utilities for common operations
pub mod utils {
    use super::*;
    
    /// Get a test market ticker that's likely to exist
    pub fn get_test_market_ticker() -> String {
        // Use a market that's likely to exist for testing
        "TEST-MARKET-2024".to_string()
    }
    
    /// Create a minimal test order that's unlikely to execute
    pub async fn create_test_order(kalshi: &Kalshi) -> Result<kalshi::Order, KalshiError> {
        // Create an order with very low probability of execution for testing
        kalshi
            .create_order(
                kalshi::Action::Buy,
                None,
                1,
                kalshi::Side::Yes,
                get_test_market_ticker(),
                kalshi::OrderType::Limit,
                None,
                None,
                None,
                None,
                Some(1), // Very low price
            )
            .await
    }
} 