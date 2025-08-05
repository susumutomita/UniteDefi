# Requirements Document

## Introduction

This feature aims to implement a production-ready, real-world atomic swap system between Ethereum (Base Sepolia) and NEAR Protocol. The system will enable users to perform trustless, bidirectional token swaps using Hash Time-Locked Contracts (HTLC) and the 1inch Limit Order Protocol, with actual blockchain transactions and real asset transfers.

## Requirements

### Requirement 1

**User Story:** As a user, I want to swap ETH tokens to NEAR tokens atomically, so that I can transfer value between chains without counterparty risk.

#### Acceptance Criteria

1. WHEN a user initiates an ETH→NEAR swap THEN the system SHALL create a limit order on Ethereum using the 1inch Limit Order Protocol
2. WHEN the limit order is created THEN the system SHALL generate a cryptographic secret and create an HTLC on NEAR with the secret hash
3. WHEN the Ethereum order is filled by a resolver THEN the system SHALL automatically detect the fulfillment and reveal the secret
4. WHEN the secret is revealed THEN the system SHALL claim the NEAR tokens from the HTLC using the secret
5. IF the order is not filled within the timeout period THEN the system SHALL allow refund of the locked NEAR tokens

### Requirement 2

**User Story:** As a user, I want to swap NEAR tokens to ETH tokens atomically, so that I can transfer value from NEAR to Ethereum without counterparty risk.

#### Acceptance Criteria

1. WHEN a user initiates a NEAR→ETH swap THEN the system SHALL create an HTLC on NEAR locking the NEAR tokens
2. WHEN the NEAR HTLC is created THEN the system SHALL create a corresponding limit order on Ethereum that references the NEAR HTLC
3. WHEN a resolver fills the Ethereum order THEN the system SHALL create an HTLC on Ethereum with the same secret hash
4. WHEN the Ethereum HTLC is created THEN the system SHALL claim the Ethereum tokens using the secret
5. IF the swap fails at any step THEN the system SHALL allow refund of locked tokens after timeout

### Requirement 3

**User Story:** As a user, I want to monitor my swap progress in real-time, so that I can track the status and take action if needed.

#### Acceptance Criteria

1. WHEN a swap is initiated THEN the system SHALL provide a unique swap ID for tracking
2. WHEN any blockchain transaction occurs THEN the system SHALL display the transaction hash and explorer link
3. WHEN the swap status changes THEN the system SHALL update the user with the current step and estimated completion time
4. WHEN monitoring is enabled THEN the system SHALL automatically check for order fulfillment every 30 seconds
5. IF an error occurs THEN the system SHALL provide clear error messages and recovery instructions

### Requirement 4

**User Story:** As a user, I want to configure swap parameters, so that I can control slippage, timeout, and other trading preferences.

#### Acceptance Criteria

1. WHEN creating a swap THEN the system SHALL accept slippage tolerance between 0.1% and 50%
2. WHEN creating a swap THEN the system SHALL accept timeout values between 30 minutes and 7 days
3. WHEN creating a swap THEN the system SHALL validate token addresses and amounts before execution
4. WHEN creating a swap THEN the system SHALL calculate and display estimated fees and execution time
5. IF invalid parameters are provided THEN the system SHALL reject the swap with descriptive error messages

### Requirement 5

**User Story:** As a developer, I want to execute batch swaps from configuration files, so that I can automate multiple swaps efficiently.

#### Acceptance Criteria

1. WHEN a batch configuration is provided THEN the system SHALL validate all swap configurations before execution
2. WHEN executing batch swaps THEN the system SHALL process each swap independently with proper error isolation
3. WHEN a batch swap fails THEN the system SHALL continue processing remaining swaps and report individual results
4. WHEN dry-run mode is enabled THEN the system SHALL validate and display swap plans without executing transactions
5. IF the configuration file is invalid THEN the system SHALL provide detailed validation errors

### Requirement 6

**User Story:** As a user, I want automatic claiming of funds, so that I don't need to manually monitor and claim completed swaps.

#### Acceptance Criteria

1. WHEN auto-claim is enabled THEN the system SHALL monitor both chains for swap completion events
2. WHEN an order is filled on the source chain THEN the system SHALL automatically claim funds from the target chain HTLC
3. WHEN claiming funds THEN the system SHALL use the revealed secret from the order fulfillment
4. WHEN auto-claim succeeds THEN the system SHALL provide confirmation with transaction details
5. IF auto-claim fails THEN the system SHALL provide manual claiming instructions with the secret

### Requirement 7

**User Story:** As a user, I want secure secret management, so that my HTLC secrets are protected throughout the swap process.

#### Acceptance Criteria

1. WHEN a swap is initiated THEN the system SHALL generate cryptographically secure 32-byte secrets
2. WHEN storing secrets THEN the system SHALL never log or expose secrets in plain text output
3. WHEN a secret is revealed on-chain THEN the system SHALL extract it from transaction data for claiming
4. WHEN a swap completes THEN the system SHALL securely dispose of secret data
5. IF secret extraction fails THEN the system SHALL provide manual recovery options

### Requirement 8

**User Story:** As a user, I want real blockchain integration, so that my swaps involve actual asset transfers on live networks.

#### Acceptance Criteria

1. WHEN creating orders THEN the system SHALL submit signed transactions to Base Sepolia testnet
2. WHEN creating HTLCs THEN the system SHALL interact with deployed contracts on NEAR testnet
3. WHEN transactions are submitted THEN the system SHALL provide verifiable transaction hashes and explorer links
4. WHEN monitoring swaps THEN the system SHALL query real blockchain state from RPC endpoints
5. IF network connectivity fails THEN the system SHALL provide retry mechanisms and offline recovery options