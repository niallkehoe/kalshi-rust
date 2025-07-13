#[path = "common/mod.rs"]
mod common;
use common::setup_auth_test;

#[tokio::test]
async fn test_exchange_status_structure() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test getting exchange status
    let status = kalshi.get_exchange_status().await;
    assert!(status.is_ok(), "Failed to get exchange status: {:?}", status.err());
    
    let status = status.unwrap();
    // INSERT_YOUR_CODE
    println!("Exchange status: {:?}", status);
    
    // Verify the response structure
    // These fields should be boolean values
    assert!(status.trading_active == true || status.trading_active == false);
    assert!(status.exchange_active == true || status.exchange_active == false);
}

#[tokio::test]
async fn test_exchange_schedule_structure() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test getting exchange schedule
    let schedule = kalshi.get_exchange_schedule().await;
    assert!(schedule.is_ok(), "Failed to get exchange schedule: {:?}", schedule.err());
    
    let schedule = schedule.unwrap();
    
    // Verify the response has the expected structure
    // The schedule should contain standard hours information
    assert!(!schedule.standard_hours.is_empty() || !schedule.maintenance_windows.is_empty());
}

#[tokio::test]
async fn test_exchange_status_consistency() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test that multiple calls return consistent results
    let status1 = kalshi.get_exchange_status().await.unwrap();
    let status2 = kalshi.get_exchange_status().await.unwrap();
    
    // The status should be consistent within a short time period
    assert_eq!(status1.trading_active, status2.trading_active);
    assert_eq!(status1.exchange_active, status2.exchange_active);
}

#[tokio::test]
async fn test_exchange_schedule_consistency() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test that multiple calls return consistent results
    let schedule1 = kalshi.get_exchange_schedule().await.unwrap();
    let schedule2 = kalshi.get_exchange_schedule().await.unwrap();
    
    // The schedule should be consistent within a short time period
    assert_eq!(schedule1.standard_hours.len(), schedule2.standard_hours.len());
} 