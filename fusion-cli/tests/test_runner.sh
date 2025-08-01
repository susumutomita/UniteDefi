#!/bin/bash

# UniteDefi CLI Test Runner
# Comprehensive test suite for all CLI commands

set -e

echo "========================================"
echo "UniteDefi CLI Test Runner"
echo "========================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to run a test suite
run_test_suite() {
    local test_name=$1
    local test_file=$2
    
    echo -e "${YELLOW}Running $test_name...${NC}"
    
    if cargo test --test $test_file -- --nocapture 2>&1; then
        echo -e "${GREEN}✓ $test_name passed${NC}"
        return 0
    else
        echo -e "${RED}✗ $test_name failed${NC}"
        return 1
    fi
}

# Build the project first
echo "Building project..."
cargo build --bin fusion-cli

echo ""
echo "Running test suites..."
echo ""

# Track test results
total_tests=0
passed_tests=0
failed_tests=0

# Run each test suite
test_suites=(
    "CLI Basic Tests:cli_tests"
    "Order CLI Tests:order_cli_tests"
    "Order Management Tests:order_management_tests"
    "Relay Order Tests:relay_order_tests"
    "Comprehensive CLI Tests:comprehensive_cli_tests"
    "Data Integrity Tests:data_integrity_tests"
    "Error Handling Tests:error_handling_tests"
)

for suite in "${test_suites[@]}"; do
    IFS=':' read -r name file <<< "$suite"
    total_tests=$((total_tests + 1))
    
    if run_test_suite "$name" "$file"; then
        passed_tests=$((passed_tests + 1))
    else
        failed_tests=$((failed_tests + 1))
    fi
    echo ""
done

# Run unit tests in src files
echo -e "${YELLOW}Running unit tests...${NC}"
if cargo test --lib -- --nocapture 2>&1; then
    echo -e "${GREEN}✓ Unit tests passed${NC}"
    passed_tests=$((passed_tests + 1))
else
    echo -e "${RED}✗ Unit tests failed${NC}"
    failed_tests=$((failed_tests + 1))
fi
total_tests=$((total_tests + 1))

echo ""
echo "========================================"
echo "Test Summary"
echo "========================================"
echo "Total test suites: $total_tests"
echo -e "Passed: ${GREEN}$passed_tests${NC}"
echo -e "Failed: ${RED}$failed_tests${NC}"
echo ""

if [ $failed_tests -eq 0 ]; then
    echo -e "${GREEN}All tests passed! ✨${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed. Please check the output above.${NC}"
    exit 1
fi