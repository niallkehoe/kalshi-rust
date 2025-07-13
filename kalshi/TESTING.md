# Testing Guide for Kalshi Rust Library

This document explains how to set up and run tests for the Kalshi Rust library, including handling authentication for tests that require API access.

## Overview

The test suite is designed to work with both authenticated and non-authenticated scenarios:

- **Unit tests**: Test individual functions without API calls
- **Integration tests**: Test API functionality with real credentials
- **Mock tests**: Test error handling and edge cases

## Test Structure

```
kalshi/tests/
├── mod.rs              # Main test module
├── common/
│   └── mod.rs          # Common test utilities and auth helpers
├── auth_tests.rs       # Authentication tests
├── market_tests.rs     # Market data tests
├── portfolio_tests.rs  # Trading and portfolio tests
└── exchange_tests.rs   # Exchange status tests
```

## Setting Up Authentication for Tests

### 1. Environment Variables

You can set up your test credentials in two ways:

#### Option A: Local .env file
Create a `.env` file in the `kalshi/` directory with your test credentials:

```bash
# Test credentials (use demo mode for safety)
KALSHI_DEMO_API_KEY=your-demo-api-key
KALSHI_DEMO_PEM_PATH=/path/to/your/demo-private-key.pem
KALSHI_TEST_ENV=demo  # or "prod" for production testing
```

#### Option B: Custom env file location
Set the `KALSHI_ENV_FILE` environment variable to point to your env file:

```bash
# Set the path to your env file
export KALSHI_ENV_FILE=/path/to/your/kalshi-test.env

# Or run tests with the env file path
KALSHI_ENV_FILE=/path/to/your/kalshi-test.env cargo test
```

Your env file should contain:
```bash
KALSHI_DEMO_API_KEY=your-demo-api-key
KALSHI_DEMO_PEM_PATH=/path/to/your/demo-private-key.pem
KALSHI_TEST_ENV=demo
```

### 2. Getting Test Credentials

1. **Demo Mode (Recommended for Testing)**:
   - Go to [Kalshi Demo](https://demo.kalshi.com)
   - Create an account
   - Generate API credentials
   - Download your private key file

2. **Production Mode (Use with Caution)**:
   - Go to [Kalshi Production](https://kalshi.com)
   - Create an account
   - Generate API credentials
   - Download your private key file

### 3. Security Best Practices

- **Never commit credentials to version control**
- **Use demo mode for most testing**
- **Keep production credentials secure**
- **Rotate credentials regularly**

## Running Tests

### All Tests
```bash
cd kalshi
cargo test
```

### Specific Test Categories
```bash
# Authentication tests only
cargo test auth_tests

# Market data tests only
cargo test market_tests

# Portfolio/trading tests only
cargo test portfolio_tests

# Exchange status tests only
cargo test exchange_tests
```

### Tests Without Authentication
```bash
# Run tests that don't require auth
cargo test -- --skip auth_tests --skip market_tests --skip portfolio_tests --skip exchange_tests
```

### Verbose Output
```bash
cargo test -- --nocapture
```

## Test Categories

### 1. Authentication Tests (`auth_tests.rs`)
- Tests authentication creation and validation
- Tests invalid credential handling
- Tests environment detection
- Tests logout functionality

### 2. Market Tests (`market_tests.rs`)
- Tests market data retrieval
- Tests event and series data
- Tests orderbook and trade data
- Tests market history

### 3. Portfolio Tests (`portfolio_tests.rs`)
- Tests balance and position queries
- Tests order creation and management
- Tests batch operations
- **Safety**: Only runs trading operations in demo mode

### 4. Exchange Tests (`exchange_tests.rs`)
- Tests exchange status queries
- Tests trading schedule
- Tests response consistency

## Test Utilities

### Common Test Module (`common/mod.rs`)

The common module provides utilities for all tests:

```rust
use crate::common::{require_auth, setup_auth_test};

#[tokio::test]
async fn my_test() {
    let kalshi = setup_auth_test().await.unwrap();  // Automatically handles auth and creates Kalshi instance
    // ... test code using kalshi
}
```

### Test Authentication Helper

```rust
pub struct TestAuth {
    pub key_id: String,
    pub pem_path: String,
    pub environment: TradingEnvironment,
}

impl TestAuth {
    pub fn from_env() -> Option<Self> { /* ... */ }
    pub async fn create_kalshi(&self) -> Result<Kalshi, KalshiError> { /* ... */ }
}
```

## Continuous Integration

### GitHub Actions Example

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run unit tests
        run: |
          cd kalshi
          cargo test --lib
      
      - name: Run integration tests (if credentials available)
        run: |
          cd kalshi
          cargo test --test '*'
        env:
          KALSHI_DEMO_API_KEY: ${{ secrets.KALSHI_DEMO_API_KEY }}
          KALSHI_DEMO_PEM_PATH: ${{ secrets.KALSHI_DEMO_PEM_PATH }}
          KALSHI_TEST_ENV: demo
        continue-on-error: true  # Don't fail if no credentials
```

## Troubleshooting

### Common Issues

1. **"Skipping test: KALSHI_DEMO_API_KEY and KALSHI_DEMO_PEM_PATH environment variables not set"**
   - Solution: Set up the environment variables as described above

2. **"Failed to read private key file"**
   - Solution: Check the path to your private key file
   - Ensure the file has correct permissions

3. **"Authentication failed"**
   - Solution: Verify your key ID and private key are correct
   - Ensure you're using the right environment (demo vs prod)

4. **"Order creation failed"**
   - Solution: Ensure you're using demo mode for trading tests
   - Check that the market is active and accepting orders

### Debug Mode

For debugging test issues, run with verbose output:

```bash
RUST_LOG=debug cargo test -- --nocapture
```

## Best Practices

1. **Always use demo mode for testing** unless specifically testing production features
2. **Clean up orders** created during tests
3. **Use realistic test data** but avoid high-value trades
4. **Test error conditions** as well as success cases
5. **Keep tests independent** - don't rely on state from other tests
6. **Use timeouts** for API calls to avoid hanging tests

## Contributing

When adding new tests:

1. Follow the existing test structure
2. Use the `setup_auth_test!()` macro for authenticated tests
3. Add appropriate error handling
4. Document any new test utilities
5. Update this guide if needed

## Security Notes

- Test credentials should have minimal permissions
- Use separate credentials for testing vs production
- Never log or expose credentials in test output
- Regularly rotate test credentials
- Consider using test-specific API keys with limited scope 