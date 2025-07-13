#[path = "common/mod.rs"]
mod common;
use common::setup_auth_test;

#[tokio::test]
async fn test_get_exchange_status() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test getting exchange status
    let status = kalshi.get_exchange_status().await;
    assert!(status.is_ok(), "Failed to get exchange status: {:?}", status.err());
}

#[tokio::test]
async fn test_get_exchange_schedule() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test getting exchange schedule
    let schedule = kalshi.get_exchange_schedule().await;
    assert!(schedule.is_ok(), "Failed to get exchange schedule: {:?}", schedule.err());
}

#[tokio::test]
async fn test_get_events() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test getting events with limit
    let result = kalshi.get_events(Some(5), None, None, None, None).await;
    assert!(result.is_ok(), "Failed to get events: {:?}", result.err());
    
    let (_cursor, events) = result.unwrap();
    assert!(events.len() <= 5, "Should return at most 5 events");
}

#[tokio::test]
async fn test_get_series_list() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test getting series list
    let result = kalshi.get_series_list(None, None, None, None).await;
    match result {
        Ok((cursor, series)) => {
            println!("Series list test successful - cursor: {:?}, series count: {}", cursor, series.len());
            // Even if no series are returned, that's still a valid response
            assert!(true, "Successfully got series list");
        }
        Err(e) => {
            println!("Series list error: {:?}", e);
            assert!(false, "Failed to get series list: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_markets() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test getting markets
    let result = kalshi.get_markets(None, None, None, None, None, None, None, None).await;
    assert!(result.is_ok(), "Failed to get markets: {:?}", result.err());
    
    let (_cursor, markets) = result.unwrap();
    assert!(!markets.is_empty(), "Should return at least one market");
}

#[tokio::test]
async fn test_get_trades() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test getting trades
    let result = kalshi.get_trades(None, None, None, None, None).await;
    assert!(result.is_ok(), "Failed to get trades: {:?}", result.err());
}
