#[path = "common/mod.rs"]
mod common;
use common::setup_auth_test;

#[tokio::test]
async fn test_get_balance() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test getting balance
    let balance = kalshi.get_balance().await;
    assert!(balance.is_ok(), "Failed to get balance: {:?}", balance.err());
    
    let balance = balance.unwrap();
    assert!(balance >= 0, "Balance should be non-negative");
}

#[tokio::test]
async fn test_get_orders() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test getting orders
    let result = kalshi.get_orders(None, None, None, None, None, None, None).await;
    assert!(result.is_ok(), "Failed to get orders: {:?}", result.err());
    
    let (_cursor, _orders) = result.unwrap();
    // Orders might be empty, which is fine
}

#[tokio::test]
async fn test_get_fills() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test getting fills
    let result = kalshi.get_fills(None, None, None, None, None, None).await;
    assert!(result.is_ok(), "Failed to get fills: {:?}", result.err());
    
    let (_cursor, _fills) = result.unwrap();
    // Fills might be empty, which is fine
}

#[tokio::test]
async fn test_get_settlements() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test getting settlements
    let result = kalshi.get_settlements(None, None).await;
    assert!(result.is_ok(), "Failed to get settlements: {:?}", result.err());
    
    let (_cursor, _settlements) = result.unwrap();
    // Settlements might be empty, which is fine
}

#[tokio::test]
async fn test_get_positions() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test getting positions
    let result = kalshi.get_positions(None, None, None, None, None).await;
    assert!(result.is_ok(), "Failed to get positions: {:?}", result.err());
    
    let (_cursor, _event_positions, _market_positions) = result.unwrap();
    // Positions might be empty, which is fine
}
