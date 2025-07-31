# 1inch Fusion+ Cross-Chain Architecture (正しい理解版)

## Overview

1inch Fusion+ をEVMとNEAR間で実現するための正しいアーキテクチャ。

## Core Components

### 1. Order Management Layer (インテントベース)

#### EVM Side - Limit Order Protocol
- Purpose: オフチェイン署名とオンチェイン実行の分離
- Components:
  - OrderMixin: 注文の実行
  - OrderLib: 注文の検証とエンコーディング
  - DutchAuctionCalculator: 価格計算
- Source: `references/cross-chain-swap/lib/limit-order-protocol/`

#### NEAR Side - インテント Order Protocol (要実装)
```rust
pub struct IntentOrder {
    pub maker: AccountId,
    pub taker_asset: String,      // チェインIDを含む
    pub maker_asset: String,      // チェインIDを含む
    pub taking_amount: U128,
    pub making_amount: U128,
    pub auction_start_time: u64,
    pub auction_duration: u64,
    pub deadline: u64,
    pub nonce: U128,
}
```

### 2. Escrow Layer (資金管理)

#### EVM Side - 1inch Official Contracts
- EscrowFactory: エスクローのデプロイ
- EscrowSrc: ソースチェインでの資金ロック
- EscrowDst: デスティネーションチェインでの資金保持
- Source: `references/cross-chain-swap/contracts/`

#### NEAR Side - HTLC Implementation
- 既存実装を活用: `contracts/near-htlc/`
- 追加要件:
  - 1inch互換のインタフェース
  - Limit Order Protocolとの統合
  - Dutch Auctionサポート

### 3. Resolver Network (実行レイヤー)

#### Professional Resolvers
- 役割:
  - Dutch Auctionへの参加
  - 注文の実行
  - 流動性の提供
- 要件:
  - 両チェインでの資金保有
  - KYC/AML準拠
  - 高速実行能力

#### Resolver Service Implementation
```typescript
interface ResolverService {
  // Dutch Auctionの監視
  watchAuctions(): AsyncIterator<Auction>

  // 入札計算
  calculateBid(auction: Auction): Bid

  // 注文実行
  executeOrder(order: Order): Promise<ExecutionResult>

  // クロスチェーン調整
  coordinateCrossChain(execution: Execution): Promise<void>
}
```

### 4. Relayer Infrastructure (調整レイヤー)

#### Event Monitoring
```typescript
// EVM Events
interface EVMEventWatcher {
  onOrderFilled(callback: (event: OrderFilledEvent) => void): void
  onEscrowCreated(callback: (event: EscrowCreatedEvent) => void): void
  onSecretRevealed(callback: (event: SecretRevealedEvent) => void): void
}

// NEAR Events
interface NEAREventWatcher {
  onIntentCreated(callback: (event: IntentCreatedEvent) => void): void
  onHTLCCreated(callback: (event: HTLCCreatedEvent) => void): void
  onClaimed(callback: (event: ClaimedEvent) => void): void
}
```

#### Cross-Chain Coordination
- シークレット管理
- タイムアウト調整
- 失敗時のリカバリー

## Implementation Phases

### Phase 1: Foundation (必須)
1. EVM Contracts Deployment
   - Limit Order Protocolをテストネットにデプロイ
   - 公式Escrowコントラクトをデプロイ

2. NEARインテントSystem
   - インテントOrder構造の定義
   - 署名と検証システム
   - HTLCとの統合

### Phase 2: Resolver Implementation
1. Basic Resolver
   - シンプルな固定価格実行
   - 手動トリガー

2. Advanced Resolver
   - Dutch Auction参加
   - 自動実行
   - 利益最適化

### Phase 3: Production Features
1. Partial Fills
   - Merkle tree実装
   - 分割実行

2. Multi-chain Support
   - Cosmos追加
   - Stellar追加

## Critical Success Factors

### 1. Limit Order Protocol の正しい理解と実装
- EVMとNEAR両方で必要
- インテントベースが本質

### 2. Dutch Auction メカニズム
- 価格発見の仕組み
- リゾルバー競争

### 3. 双方向性
- EVM → NEAR
- NEAR → EVM
- 両方が動作

## Testing Strategy

### Unit Tests
- Order creation and validation
- Auction calculations
- HTLC operations

### Integration Tests
- Full swap flow (both directions)
- Timeout scenarios
- Partial fill scenarios

### Testnet Demo
- Base Sepolia ↔ NEAR Testnet
- Live execution with real assets
- Error recovery demonstration

## Security Considerations

### Order Security
- Replay attack prevention (nonce)
- Signature validation
- Deadline enforcement

### Fund Security
- Atomic execution
- Timeout protection
- No single point of failure

### Cross-chain Security
- Hash verification
- Time synchronization
- Reorg protection
