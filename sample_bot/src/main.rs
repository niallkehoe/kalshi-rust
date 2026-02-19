use dotenv::dotenv;
use kalshi::Kalshi;
use std::env;

extern crate kalshi;

enum APIType {
    Live,
    Demo,
}

fn retreive_credentials(setting: APIType) -> Result<(String, String), std::io::Error> {
    let mut password: String = "dummy".to_string();
    let mut username: String = "dummy".to_string();
    match setting {
        APIType::Live => {
            if let Ok(key) = env::var("LIVE_PASSWORD") {
                println!("got password");
                password = key;
            }
            if let Ok(user) = env::var("LIVE_USER_NAME") {
                println!("got user");
                username = user;
            }
        }

        APIType::Demo => {
            if let Ok(key) = env::var("DEMO_PASSWORD") {
                println!("got password");
                password = key;
            }
            if let Ok(user) = env::var("DEMO_USER_NAME") {
                println!("got user");
                username = user;
            }
        }
    }
    Ok((username, password))
}
#[tokio::main]
async fn main() {
    dotenv().ok();

    let key_id = env::var("DEMO_API_KEY").unwrap_or_else(|_| "dummy".to_string());
    let pem_path = env::var("DEMO_PEM_PATH").unwrap_or_else(|_| "dummy.pem".to_string());

    let mut kalshi_instance = Kalshi::new(kalshi::TradingEnvironment::DemoMode, &key_id, &pem_path).await.unwrap();

    let new_york_ticker = "HIGHNY-23NOV13-T51".to_string();

    let nytemp_market_data = kalshi_instance.get_market(&new_york_ticker).await.unwrap();
    
    let nytemp_market_orderbook = kalshi_instance.get_orderbook(&new_york_ticker, Some(10)).await.unwrap();


      let bought_order = kalshi_instance
        .create_order(
            kalshi::Action::Buy,
            None,
            None,
            kalshi::Side::Yes,
            new_york_ticker,
            kalshi::OrderType::Limit,
            None,
            None,
            None,
            None,
            None,
            Some("0.05".to_string()),
            None,
            Some("1.00".to_string()),
        )
        .await
        .unwrap();

    let ny_order_id = bought_order.order_id.clone();
    
    let cancelled_order = kalshi_instance.cancel_order(&ny_order_id).await.unwrap();
    println!("{:?}", cancelled_order);

    
}
