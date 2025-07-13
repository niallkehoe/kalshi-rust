#!/bin/bash

# Kalshi Test Runner with Custom Env File
# Usage: ./scripts/run_tests_with_env.sh /path/to/your/env/file

set -e

if [ $# -eq 0 ]; then
    echo "Usage: $0 /path/to/your/env/file [test_filter]"
    echo ""
    echo "Examples:"
    echo "  $0 ~/.kalshi/test.env                    # Run all tests"
    echo "  $0 ~/.kalshi/test.env auth_tests         # Run auth tests only"
    echo "  $0 ~/.kalshi/test.env -- --nocapture     # Run with verbose output"
    echo ""
    echo "The env file should contain:"
    echo "  KALSHI_TEST_KEY_ID=your-key-id"
    echo "  KALSHI_TEST_PEM_PATH=/path/to/private-key.pem"
    echo "  KALSHI_TEST_ENV=demo"
    exit 1
fi

ENV_FILE="$1"
shift  # Remove the first argument, leaving any test filters

# Check if env file exists
if [ ! -f "$ENV_FILE" ]; then
    echo "❌ Error: Env file not found: $ENV_FILE"
    exit 1
fi

echo "🚀 Running Kalshi tests with env file: $ENV_FILE"
echo ""

# Run tests with the custom env file
KALSHI_ENV_FILE="$ENV_FILE" cargo test "$@" 