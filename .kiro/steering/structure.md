# Project Structure

## Root Directory Organization

```
UniteDefi/
├── .kiro/                    # Kiro spec-driven development files
│   ├── steering/            # Project steering documents (this directory)
│   └── specs/               # Feature specifications
├── fusion-core/             # Core HTLC logic library
│   ├── src/                # Core trait definitions and shared logic
│   ├── tests/              # Unit and integration tests
│   └── Cargo.toml          # Package manifest
├── src/                     # Main CLI application (planned)
├── contracts/               # Smart contracts for each chain (planned)
│   ├── ethereum/           # Solidity HTLC contracts
│   ├── near/              # NEAR HTLC contracts
│   ├── cosmos/            # CosmWasm contracts
│   └── stellar/           # Soroban contracts
├── docs/                    # Project documentation
├── .claude/                 # Claude Code configuration
│   ├── agents/             # AI agent configurations
│   └── commands/           # Custom slash commands
├── .github/                 # GitHub configuration
│   └── workflows/          # CI/CD workflows
├── Cargo.toml              # Workspace manifest
├── Cargo.lock              # Dependency lock file
├── package.json            # Node.js dependencies (for textlint)
├── pnpm-lock.yaml          # pnpm lock file
├── Makefile                # Build automation
├── README.md               # Project overview
├── CLAUDE.md               # Claude Code instructions
└── LICENSE                 # MIT license
```

## Subdirectory Structures

### fusion-core/ - Core Library
```
fusion-core/
├── src/
│   ├── lib.rs              # Library entry point, exports public API
│   ├── htlc.rs             # HTLC trait definition and core types
│   ├── error.rs            # Custom error types (planned)
│   ├── secret.rs           # Secret generation and validation (planned)
│   └── utils.rs            # Utility functions (planned)
└── tests/
    ├── htlc_tests.rs       # HTLC implementation tests
    └── integration/        # Integration test scenarios (planned)
```

### src/ - CLI Application (Planned Structure)
```
src/
├── main.rs                 # CLI entry point
├── cli/
│   ├── mod.rs             # CLI module organization
│   ├── commands/          # Command implementations
│   │   ├── init.rs        # Initialize configuration
│   │   ├── swap.rs        # Swap commands (create, status, complete)
│   │   └── monitor.rs     # Monitoring commands
│   └── config.rs          # Configuration management
├── chains/                 # Chain-specific implementations
│   ├── mod.rs             # Chain registry
│   ├── ethereum/          # Ethereum integration
│   ├── near/              # NEAR integration
│   ├── cosmos/            # Cosmos integration
│   └── stellar/           # Stellar integration
└── relayer/               # Relayer service (planned)
    ├── mod.rs             # Relayer module
    └── monitor.rs         # Event monitoring
```

### contracts/ - Smart Contracts (Planned Structure)
```
contracts/
├── ethereum/
│   ├── HTLC.sol           # Solidity HTLC contract
│   └── test/              # Contract tests
├── near/
│   ├── htlc/
│   │   ├── Cargo.toml     # NEAR contract manifest
│   │   └── src/lib.rs     # NEAR HTLC implementation
│   └── tests/             # NEAR contract tests
├── cosmos/
│   ├── htlc/
│   │   ├── Cargo.toml     # CosmWasm manifest
│   │   └── src/           # CosmWasm HTLC
│   └── tests/             # Cosmos tests
└── stellar/
    ├── htlc/
    │   ├── Cargo.toml     # Soroban manifest
    │   └── src/           # Stellar HTLC
    └── tests/             # Stellar tests
```

## Code Organization Patterns

### Module Organization
- **Trait-Based Design**: Core functionality defined as traits in `fusion-core`
- **Chain Abstraction**: Each blockchain has its own module implementing core traits
- **Separation of Concerns**: CLI, core logic, and chain integrations are separate
- **Test Proximity**: Tests live close to the code they test

### Async Patterns
```rust
// All blockchain operations are async
#[async_trait]
pub trait HTLCContract {
    async fn create_lock(...) -> Result<String>;
    async fn claim_with_secret(...) -> Result<TransactionHash>;
}
```

### Error Handling
```rust
// Custom error types for each module
pub enum HTLCError {
    ChainError(String),
    InvalidSecret,
    Timeout,
    // ...
}

// Result type alias
pub type Result<T> = std::result::Result<T, HTLCError>;
```

## File Naming Conventions

### Rust Files
- **Module files**: snake_case (e.g., `htlc_contract.rs`)
- **Test files**: `<module>_tests.rs` or in `tests/` directory
- **Binary targets**: snake_case (e.g., `fusion_cli`)

### Documentation
- **Markdown files**: Use hyphens for multi-word files (e.g., `Fusion-Plus-Technical-Guide.md`)
- **Japanese docs**: Can use Japanese characters (e.g., `優勝アイデア.md`)

### Configuration
- **TOML files**: `Cargo.toml` (standard Rust convention)
- **JSON files**: camelCase keys (e.g., `package.json`)
- **Environment files**: `.env` (uppercase variables with underscores)

## Import Organization

### Standard Order
1. Standard library imports
2. External crate imports
3. Internal crate imports
4. Module declarations

```rust
// Example in lib.rs
use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::{HTLCError, Result};
use crate::secret::SecretHash;

pub mod error;
pub mod htlc;
pub mod secret;
```

## Key Architectural Principles

### 1. Modularity
- Each blockchain implementation is independent
- Core logic is chain-agnostic
- Easy to add new chains by implementing traits

### 2. Type Safety
- Use Rust's type system for compile-time guarantees
- Newtype pattern for domain types (e.g., `SecretHash`, `Timeout`)
- Builder pattern for complex configurations

### 3. Testing Strategy
- Unit tests for each module
- Integration tests for cross-chain scenarios
- Property-based testing for cryptographic functions
- Test-driven development (TDD) approach

### 4. Documentation
- Doc comments on all public APIs
- Examples in documentation
- README files in major directories
- Architecture decision records (ADRs) for major choices

### 5. Security First
- No hardcoded secrets
- Input validation at boundaries
- Fail-safe defaults
- Audit trail for critical operations