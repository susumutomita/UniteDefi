# Cross-Chain Swap Implementation Guide

## 概要
このドキュメントは、EVMチェイン（Ethereum/Monad等）とNEARチェイン間でのクロスチェインスワップを実装するためのガイドです。

## 現在の実装状況

### ✅ 完了済み
- NEAR側HTLCコントラクト (`contracts/near-htlc/`)
  - FusionHTLC実装完了
  - セキュリティ機能実装済み
  - Storage Limits（DoS対策）実装済み

### ❌ 未実装
- EVM側リゾルバーコントラクト
- リレイヤーサービス
- フロントエンド

## アーキテクチャ

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│  EVM Chain  │     │   Relayer    │     │ NEAR Chain  │
│             │     │   Service    │     │             │
│  Resolver   │◄────┤              ├────►│ FusionHTLC  │
│  Contract   │     │ - Watch Both │     │  Contract   │
│             │     │ - Relay Msgs │     │             │
└─────────────┘     └──────────────┘     └─────────────┘
```

## 実装手順

### 1. EVM側リゾルバーコントラクトの実装

参考: [1inch/cross-chain-resolver-example](https://github.com/1inch/cross-chain-resolver-example)

```solidity
// contracts/evm-resolver/FusionResolver.sol
contract FusionResolver {
    // HTLCの作成
    function createEscrow(...) external payable {
        // トークンをロック
        // ハッシュロックを設定
        // タイムロックを設定
    }

    // シークレットでクレーム
    function claim(bytes32 escrowId, bytes memory secret) external {
        // シークレットを検証
        // トークンを転送
    }

    // タイムアウト後のキャンセル
    function cancel(bytes32 escrowId) external {
        // タイムアウトを確認
        // トークンを返金
    }
}
```

### 2. リレイヤーサービスの実装

```typescript
// relayer/src/index.ts
class CrossChainRelayer {
    // 両チェーンを監視
    watchEVM() {
        // EVMイベントを監視
        // NEAR側にメッセージを中継
    }

    watchNEAR() {
        // NEARイベントを監視
        // EVM側にメッセージを中継
    }

    // シークレットの公開を検知して中継
    relaySecret(secret: string, fromChain: string, toChain: string) {
        // クロスチェーンでシークレットを共有
    }
}
```

### 3. テストネットでのデプロイ

#### オプション1: フォークメインネット（推奨）
```bash
# Hardhatでローカルフォーク
npx hardhat node --fork https://eth-mainnet.alchemyapi.io/v2/YOUR_KEY

# NEARテストネット
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/near_htlc.wasm
```

#### オプション2: 実際のテストネット
- EVM: Sepolia, Goerli, またはMonad Testnet
- NEAR: NEAR Testnet

### 4. 統合テスト

```typescript
// tests/e2e/crossChainSwap.test.ts
describe("Cross-Chain Swap", () => {
    it("EVMからNEARへのスワップ", async () => {
        // 1. EVM側でエスクローを作成
        // 2. NEAR側でクレーム待機
        // 3. シークレット公開
        // 4. 両チェーンで完了確認
    });
});
```

## 必要な環境変数

```env
# .env.example
# EVM
EVM_RPC_URL=
EVM_PRIVATE_KEY=
EVM_RESOLVER_ADDRESS=

# NEAR
NEAR_NETWORK=testnet
NEAR_ACCOUNT_ID=
NEAR_PRIVATE_KEY=
NEAR_CONTRACT_ID=

# Relayer
RELAYER_PORT=3000
```

## セキュリティ考慮事項

1. ハッシュロック: SHA256を使用
2. タイムロック: タイムアウト期間の設定
3. リエントランシー保護: 状態変更前のチェック
4. ガスリミット: ガス設定

## 参考リソース

- [1inch Cross-Chain Resolver Example](https://github.com/1inch/cross-chain-resolver-example)
- [1inch Fusion+ Documentation](https://docs.1inch.io/docs/fusion-swap/introduction)
- [NEAR SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [ETHGlobal Unite Hackathon Resources](https://ethglobal.com/events/unite)

## 次のステップ

1. [ ] EVM側リゾルバーコントラクトの実装
2. [ ] リレイヤーサービスの実装
3. [ ] テストネットへのデプロイ
4. [ ] E2Eテストの作成
5. [ ] フロントエンドの実装

## トラブルシューティング

### WASM Deserialization Error
- `near-sdk`のバージョン互換性を確認
- WASMファイルのビルドオプションを確認

### ガス不足エラー
- `GAS_FOR_FT_TRANSFER`の値を調整
- バッチ処理時のガス計算を見直し

## コントリビューション

PRを作成する前に以下を確認してください。

- [ ] 全てのテストが通過
- [ ] セキュリティレビュー完了
- [ ] ドキュメント更新
