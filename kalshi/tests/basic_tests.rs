use kalshi::TradingEnvironment;

#[test]
fn test_trading_environment_equality() {
    let demo1 = TradingEnvironment::DemoMode;
    let demo2 = TradingEnvironment::DemoMode;
    let prod = TradingEnvironment::ProdMode;
    
    assert_eq!(demo1, demo2);
    assert_ne!(demo1, prod);
    assert_ne!(demo2, prod);
}

#[test]
fn test_trading_environment_debug() {
    let demo = TradingEnvironment::DemoMode;
    let prod = TradingEnvironment::ProdMode;
    
    // Test that Debug trait works
    let demo_str = format!("{:?}", demo);
    let prod_str = format!("{:?}", prod);
    
    assert_eq!(demo_str, "DemoMode");
    assert_eq!(prod_str, "ProdMode");
} 