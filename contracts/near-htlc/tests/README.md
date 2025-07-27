# NEAR HTLC Security Tests

This directory contains comprehensive security tests for the NEAR HTLC implementation, addressing all vulnerabilities identified in the code review.

## Test Structure

```
tests/
├── README.md                 # This file
├── integration_tests.rs      # Rust integration tests using near-workspaces
├── orchestrator.test.ts      # TypeScript tests for the orchestrator
└── test_token.rs            # Mock NEP-141 token for testing
```

## Quick Start

```bash
# Run all security tests
npm run test:security

# Run Rust tests only
cargo test

# Run TypeScript tests only
npm test

# Run with coverage
cargo tarpaulin --out Html
npm run test:coverage
```

## Test Categories

### 1. **Unit Tests** (in `src/fusion_htlc.rs`)
- Binary data hash verification
- Timestamp precision and overflow protection
- Authorization and access control
- Input validation
- State transition tests

### 2. **Integration Tests** (`integration_tests.rs`)
- Full HTLC flow testing
- Cross-contract interactions
- Gas limit testing
- Reentrancy protection
- Token transfer scenarios

### 3. **TypeScript Tests** (`orchestrator.test.ts`)
- Proper NEAR API usage (fixing base_encode issue)
- Binary data handling
- Dynamic gas calculation
- Error recovery mechanisms
- Cross-chain consistency

## Key Security Tests

| Vulnerability | Test Coverage |
|--------------|---------------|
| Binary Data Handling | ✅ Hash verification with actual binary data |
| Timestamp Overflow | ✅ Nanosecond conversion and overflow detection |
| Fixed Gas Limits | ✅ Dynamic gas calculation tests |
| Reentrancy | ✅ Batch operation safety tests |
| API Errors | ✅ Correct bs58 encoding implementation |
| Cross-contract Failures | ✅ Callback and promise failure handling |

## Running Specific Security Tests

```bash
# Test binary data handling
cargo test test_hash_verification_with_binary_data -- --nocapture

# Test timestamp overflow protection
cargo test test_timestamp_overflow_protection -- --nocapture

# Test reentrancy protection
cargo test test_batch_cancel_reentrancy_protection -- --nocapture

# Test TypeScript fixes
npm test -- --testNamePattern="binary data"
```

## Test Dependencies

### Rust
- `near-sdk` with unit-testing features
- `near-workspaces` for integration tests
- `tokio` for async runtime
- `sha2`, `bs58`, `hex` for cryptographic operations

### TypeScript
- `jest` and `ts-jest` for testing
- `@types/jest`, `@types/node` for TypeScript support
- `ethers`, `near-api-js` for blockchain interactions
- `bs58` for proper base58 encoding

## Adding New Tests

1. Identify the security concern
2. Add unit test in `src/fusion_htlc.rs` for isolated testing
3. Add integration test in `integration_tests.rs` for full scenario
4. Add TypeScript test if it involves client-side logic
5. Update documentation

## Continuous Testing

These tests should be run:
- On every commit (CI/CD)
- Before deployments
- After NEAR SDK updates
- During security audits

## Test Results

All tests are designed to catch the specific vulnerabilities identified:
- ✅ Binary data is handled correctly
- ✅ Timestamps don't overflow
- ✅ Gas limits are calculated dynamically
- ✅ Reentrancy is prevented
- ✅ API calls use correct functions
- ✅ Failed transfers are handled properly

For detailed test documentation, see `/docs/SECURITY_TESTS.md`