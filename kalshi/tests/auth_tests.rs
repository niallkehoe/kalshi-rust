#[path = "common/mod.rs"]
mod common;
use common::{require_auth, setup_auth_test};
use kalshi::{Kalshi, TradingEnvironment};

#[tokio::test]
async fn test_authentication_creation() {
    let kalshi = setup_auth_test().await.unwrap();
    // Test that we can create a Kalshi instance
    let balance = kalshi.get_balance().await.unwrap();
    assert!(balance >= 0, "Should be able to get balance");
}

#[tokio::test]
async fn test_authentication_with_invalid_credentials() {
    // Test with invalid credentials - should fail gracefully
    let result = Kalshi::new(
        TradingEnvironment::DemoMode,
        "invalid-key-id",
        "/nonexistent/path/to/key.pem"
    ).await;
    
    assert!(result.is_err(), "Should fail with invalid credentials");
}

#[tokio::test]
async fn test_environment_detection() {
    let auth = require_auth();
    
    // Test that the environment is correctly detected
    match auth.environment {
        TradingEnvironment::DemoMode => {
            assert_eq!(std::env::var("KALSHI_TEST_ENV").unwrap_or_else(|_| "demo".to_string()), "demo");
        }
        TradingEnvironment::ProdMode => {
            assert_eq!(std::env::var("KALSHI_TEST_ENV").unwrap(), "prod");
        }
    }
}

#[tokio::test]
async fn test_logout_functionality() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test logout (should not error)
    let result = kalshi.logout().await;
    assert!(result.is_ok(), "Logout should succeed: {:?}", result.err());
} 