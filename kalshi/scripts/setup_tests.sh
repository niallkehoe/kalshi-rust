#!/bin/bash

# Kalshi Test Setup Script
# This script helps you set up the test environment for the Kalshi Rust library

set -e

echo "🚀 Setting up Kalshi test environment..."

# Check if .env file exists
if [ -f ".env" ]; then
    echo "⚠️  .env file already exists. Backing up to .env.backup"
    cp .env .env.backup
fi

# Create .env file from example
if [ -f "env.example" ]; then
    cp env.example .env
    echo "✅ Created .env file from env.example"
else
    echo "❌ env.example not found. Creating basic .env file..."
    cat > .env << EOF
# Kalshi API Test Credentials
KALSHI_TEST_KEY_ID=your-key-id-here
KALSHI_TEST_PEM_PATH=/path/to/your/private-key.pem
KALSHI_TEST_ENV=demo
EOF
fi

echo ""
echo "📝 Please edit .env file with your actual credentials:"
echo "   1. Get your API credentials from https://demo.kalshi.com (recommended for testing)"
echo "   2. Update KALSHI_TEST_KEY_ID with your key ID"
echo "   3. Update KALSHI_TEST_PEM_PATH with the path to your private key file"
echo "   4. Keep KALSHI_TEST_ENV=demo for safe testing"
echo ""
echo "💡 Alternative: Use a custom env file location:"
echo "   export KALSHI_ENV_FILE=/path/to/your/kalshi-test.env"
echo "   cargo test"
echo ""
echo "🔒 Security reminder:"
echo "   - Never commit your .env file to version control"
echo "   - Use demo mode for testing to avoid real money trades"
echo "   - Keep your private key file secure"
echo ""
echo "🧪 To run tests:"
echo "   cargo test                    # Run all tests"
echo "   cargo test -- --nocapture     # Run with verbose output"
echo "   cargo test auth_tests         # Run specific test category"
echo ""
echo "✅ Setup complete! Edit .env and run 'cargo test' to get started." 