# Design Document

## Overview

The Real ETH-NEAR Swap system implements a production-ready atomic swap bridge between Ethereum (Base Sepolia) and NEAR Protocol. The system leverages Hash Time-Locked Contracts (HTLC) and the 1inch Limit Order Protocol to enable trustless, bidirectional token swaps with real blockchain transactions.

The architecture follows a hybrid approach combining limit orders for price discovery and HTLCs for atomic execution, ensuring both parties receive their desired assets or neither party loses funds.

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           UniteSwap System                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐      │
│  │  Base Sepolia   │     │   Fusion CLI    │     │  NEAR Protocol  │      │
│  │     (EVM)       │◄────┤   Rust Core     ├────►│   (Non-EVM)     │      │
│  │                 │     │                 │     │                 │      │
│  │ • 1inch LOP     │     │ • HTLC Logic    │     │ • HTLC Contract │      │
│  │ • Escrow        │     │ • Secret Mgmt   │     │ • htlc-v2.      │      │
│  │ • ERC20 Tokens  │     │ • Monitoring    │     │   testnet       │      │
│  │ • Event Monitor │     │ • Price Oracle  │     │ • Event Monitor │      │
│  └─────────────────┘     └─────────────────┘     └─────────────────┘      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Component Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              CLI Layer                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│ SwapHandler │ OrderHandler │ HTLCHandler │ MonitorHandler │ BatchHandler    │
├─────────────────────────────────────────────────────────────────────────────┤
│                             Core Layer                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│ SwapOrchestrator │ SecretManager │ PriceOracle │ EventMonitor │ StateManager │
├─────────────────────────────────────────────────────────────────────────────┤
│                           Chain Connectors                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│        EthereumConnector        │           NEARConnector                   │
│  • 1inch LOP Integration        │  • HTLC Contract Interface                │
│  • EIP-712 Signing              │  • NEAR RPC Integration                   │
│  • Transaction Submission       │  • Account Management                     │
│  • Event Monitoring             │  • Gas Estimation                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                          Smart Contracts                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│    1inch Limit Order Protocol   │         NEAR HTLC Contract               │
│  • Order Creation & Execution   │  • Escrow Management                      │
│  • HTLC Integration              │  • Secret Verification                   │
│  • Cross-chain Metadata         │  • Timeout Handling                      │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Components and Interfaces

### 1. Swap Orchestrator

The central component that coordinates the entire swap process.

```rust
pub struct SwapOrchestrator {
    ethereum_connector: EthereumConnector,
    near_connector: NEARConnector,
    secret_manager: SecretManager,
    price_oracle: PriceOracle,
    event_monitor: EventMonitor,
}

impl SwapOrchestrator {
    pub async fn execute_eth_to_near_swap(&self, params: SwapParams) -> Result<SwapResult>;
    pub async fn execute_near_to_eth_swap(&self, params: SwapParams) -> Result<SwapResult>;
    pub async fn monitor_swap_progress(&self, swap_id: &str) -> Result<SwapStatus>;
    pub async fn claim_funds_automatically(&self, swap_id: &str) -> Result<ClaimResult>;
}
```

### 2. Chain Connectors

#### Ethereum Connector
Handles all Ethereum blockchain interactions including 1inch Limit Order Protocol integration.

```rust
pub struct EthereumConnector {
    provider: Arc<Provider<Http>>,
    limit_order_protocol: Address,
    chain_id: u64,
}

impl EthereumConnector {
    pub async fn create_limit_order(&self, order: LimitOrder) -> Result<OrderResult>;
    pub async fn submit_signed_order(&self, signed_order: SignedOrder) -> Result<TxHash>;
    pub async fn monitor_order_execution(&self, order_hash: &str) -> Result<ExecutionStatus>;
    pub async fn extract_secret_from_tx(&self, tx_hash: &str) -> Result<Secret>;
}
```

#### NEAR Connector
Manages NEAR Protocol interactions and HTLC contract operations.

```rust
pub struct NEARConnector {
    rpc_client: NearRpcClient,
    contract_id: AccountId,
    signer: InMemorySigner,
}

impl NEARConnector {
    pub async fn create_htlc(&self, params: HTLCParams) -> Result<HTLCResult>;
    pub async fn claim_htlc(&self, escrow_id: &str, secret: &Secret) -> Result<ClaimResult>;
    pub async fn refund_htlc(&self, escrow_id: &str) -> Result<RefundResult>;
    pub async fn monitor_htlc_events(&self, escrow_id: &str) -> Result<HTLCStatus>;
}
```

### 3. Secret Management

Secure handling of HTLC secrets throughout the swap lifecycle.

```rust
pub struct SecretManager {
    secrets: HashMap<String, SecretData>,
}

pub struct SecretData {
    secret: Secret,
    secret_hash: SecretHash,
    swap_id: String,
    created_at: SystemTime,
    revealed_at: Option<SystemTime>,
}

impl SecretManager {
    pub fn generate_secret(&mut self, swap_id: &str) -> Result<SecretHash>;
    pub fn get_secret(&self, swap_id: &str) -> Result<&Secret>;
    pub fn mark_secret_revealed(&mut self, swap_id: &str, revealed_at: SystemTime);
    pub fn cleanup_expired_secrets(&mut self);
}
```

### 4. Event Monitoring System

Real-time monitoring of blockchain events across both chains.

```rust
pub struct EventMonitor {
    ethereum_monitor: EthereumEventMonitor,
    near_monitor: NEAREventMonitor,
}

pub enum SwapEvent {
    OrderCreated { order_hash: String, chain: String },
    HTLCCreated { htlc_id: String, chain: String },
    OrderFilled { order_hash: String, secret: Option<Secret> },
    HTLCClaimed { htlc_id: String, claimer: String },
    HTLCRefunded { htlc_id: String, refunder: String },
    SwapCompleted { swap_id: String },
    SwapFailed { swap_id: String, reason: String },
}

impl EventMonitor {
    pub async fn start_monitoring(&self, swap_id: &str) -> Result<EventStream>;
    pub async fn wait_for_event(&self, event_type: SwapEventType) -> Result<SwapEvent>;
    pub async fn stop_monitoring(&self, swap_id: &str);
}
```

### 5. Price Oracle Integration

Price discovery and conversion between different tokens.

```rust
pub struct PriceOracle {
    price_feeds: HashMap<String, PriceFeed>,
}

impl PriceOracle {
    pub async fn get_price(&self, from_token: &str, to_token: &str) -> Result<Price>;
    pub async fn calculate_output_amount(&self, input: TokenAmount, output_token: &str) -> Result<TokenAmount>;
    pub async fn apply_slippage(&self, amount: TokenAmount, slippage_bps: u16) -> Result<TokenAmount>;
}
```

## Data Models

### Core Swap Data Structures

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SwapParams {
    pub from_chain: String,
    pub to_chain: String,
    pub from_token: TokenInfo,
    pub to_token: TokenInfo,
    pub amount: TokenAmount,
    pub from_address: String,
    pub to_address: String,
    pub slippage_bps: u16,
    pub timeout_seconds: u64,
    pub auto_claim: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub symbol: String,
    pub address: Option<String>,
    pub decimals: u8,
    pub chain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAmount {
    pub amount: U256,
    pub token: TokenInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapResult {
    pub swap_id: String,
    pub status: SwapStatus,
    pub secret_hash: String,
    pub transactions: Vec<TransactionInfo>,
    pub htlc_ids: HashMap<String, String>, // chain -> htlc_id
    pub order_hashes: HashMap<String, String>, // chain -> order_hash
    pub estimated_completion: SystemTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SwapStatus {
    Initiated,
    OrderCreated,
    HTLCCreated,
    OrderFilled,
    Claiming,
    Completed,
    Failed { reason: String },
    TimedOut,
}
```

### HTLC Data Structures

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct HTLCParams {
    pub recipient: String,
    pub amount: TokenAmount,
    pub secret_hash: SecretHash,
    pub timeout_seconds: u64,
    pub chain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HTLCResult {
    pub htlc_id: String,
    pub tx_hash: String,
    pub block_number: Option<u64>,
    pub gas_used: Option<u64>,
    pub explorer_url: String,
}
```

### Order Data Structures

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct LimitOrder {
    pub maker_asset: Address,
    pub taker_asset: Address,
    pub maker: Address,
    pub making_amount: U256,
    pub taking_amount: U256,
    pub salt: U256,
    pub receiver: Address,
    pub allowed_sender: Address,
    pub interactions: Bytes,
    pub htlc_data: HTLCMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HTLCMetadata {
    pub secret_hash: SecretHash,
    pub timeout_seconds: u64,
    pub recipient_chain: String,
    pub recipient_address: String,
}
```

## Error Handling

### Error Types

```rust
#[derive(Error, Debug)]
pub enum SwapError {
    #[error("Invalid swap parameters: {0}")]
    InvalidParameters(String),
    
    #[error("Chain connection error: {0}")]
    ChainConnectionError(String),
    
    #[error("Order creation failed: {0}")]
    OrderCreationError(String),
    
    #[error("HTLC creation failed: {0}")]
    HTLCCreationError(String),
    
    #[error("Secret management error: {0}")]
    SecretError(String),
    
    #[error("Monitoring error: {0}")]
    MonitoringError(String),
    
    #[error("Timeout exceeded")]
    TimeoutError,
    
    #[error("Insufficient funds")]
    InsufficientFunds,
    
    #[error("Price oracle error: {0}")]
    PriceOracleError(String),
}
```

### Error Recovery Strategies

1. **Automatic Retry**: For transient network errors
2. **Manual Recovery**: For user intervention required
3. **Timeout Handling**: Automatic refunds after timeout
4. **State Persistence**: Recovery from partial failures

## Testing Strategy

### Unit Testing

1. **Secret Management Tests**
   - Secret generation and validation
   - Hash computation verification
   - Secure storage and retrieval

2. **HTLC Logic Tests**
   - Contract creation and validation
   - Claim and refund scenarios
   - Timeout handling

3. **Order Creation Tests**
   - EIP-712 signature validation
   - Order parameter encoding
   - Cross-chain metadata handling

### Integration Testing

1. **End-to-End Swap Tests**
   - Complete ETH→NEAR swap flow
   - Complete NEAR→ETH swap flow
   - Error scenarios and recovery

2. **Chain Interaction Tests**
   - Ethereum contract interactions
   - NEAR contract interactions
   - Event monitoring accuracy

3. **Concurrent Swap Tests**
   - Multiple simultaneous swaps
   - Resource contention handling
   - State isolation verification

### Testnet Testing

1. **Live Network Testing**
   - Base Sepolia integration
   - NEAR testnet integration
   - Real asset transfers

2. **Performance Testing**
   - Transaction confirmation times
   - Gas usage optimization
   - Monitoring efficiency

3. **Security Testing**
   - Secret exposure prevention
   - Replay attack resistance
   - Timeout enforcement

## Security Considerations

### Secret Security

1. **Generation**: Cryptographically secure random number generation
2. **Storage**: In-memory only, never persisted to disk
3. **Transmission**: Secure channels only
4. **Disposal**: Immediate cleanup after use

### Transaction Security

1. **Signature Validation**: EIP-712 compliant signing
2. **Replay Protection**: Nonce-based order uniqueness
3. **Amount Validation**: Overflow protection and bounds checking
4. **Address Validation**: Checksum verification

### Cross-Chain Security

1. **Atomic Guarantees**: Both chains succeed or both fail
2. **Timeout Synchronization**: Consistent timeout handling
3. **Secret Revelation**: Secure extraction from transaction data
4. **State Consistency**: Proper state machine transitions

### Operational Security

1. **Private Key Management**: Environment variable based
2. **RPC Endpoint Security**: HTTPS only connections
3. **Error Information**: No sensitive data in error messages
4. **Logging**: Structured logging without secrets