# Deployment Guide

## 準備

### 1. 環境変数の設定

`contracts/ethereum/`ディレクトリで以下を実行します。

```bash
cd contracts/ethereum/

# .envファイルを作成
cat > .env <<EOF
# Sepolia Configuration
SEPOLIA_RPC_URL=https://eth-sepolia.g.alchemy.com/v2/YOUR_API_KEY
SEPOLIA_SENDER_ADDRESS=0xYourWalletAddress
SEPOLIA_PRIVATE_KEY=0xYourPrivateKey

# Base Sepolia Configuration
BASE_SEPOLIA_RPC_URL=https://sepolia.base.org
BASE_SENDER_ADDRESS=0xYourWalletAddress
BASE_PRIVATE_KEY=0xYourPrivateKey
BASESCAN_API_KEY=YourBasescanAPIKey

# Monad Testnet Configuration (when available)
MONAD_RPC_URL=https://monad-testnet-rpc.url
MONAD_SENDER_ADDRESS=0xYourWalletAddress
MONAD_PRIVATE_KEY=0xYourPrivateKey

# Etherscan API for verification
ETHERSCAN_API_KEY=YourEtherscanAPIKey
EOF
```

### 2. Foundryのインストール

```bash
# Foundryのインストール（未インストールの場合）
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

## デプロイ手順

### EVM側（EscrowFactory）

#### Sepoliaへのデプロイ

```bash
cd contracts/ethereum/
make deploy-sepolia
```

#### Base Sepoliaへのデプロイ

```bash
cd contracts/ethereum/
make deploy-base-sepolia
```

#### Monadへのデプロイ（テストネット公開後）

```bash
cd contracts/ethereum/
make deploy-monad
```

### NEAR側（FusionHTLC）

#### NEAR Testnetへのデプロイ

```bash
# NEARコントラクトのビルド
cargo build -p near-htlc --target wasm32-unknown-unknown --release

# デプロイ
near deploy \
  --wasmFile target/wasm32-unknown-unknown/release/near_htlc.wasm \
  --accountId your-contract.testnet
```

## デプロイ後の設定

### 1. アドレスの記録

デプロイ後、以下のファイルにアドレスを記録します。

```bash
# contracts/ethereum/README.mdを更新
# - Sepolia EscrowFactory: 0x...
# - Base Sepolia EscrowFactory: 0x...
# - NEAR FusionHTLC: your-contract.testnet
```

### 2. fusion-coreへの設定

```rust
// fusion-core/src/config.rsを更新
pub const SEPOLIA_ESCROW_FACTORY: &str = "0x...";
pub const BASE_ESCROW_FACTORY: &str = "0x...";
pub const NEAR_FUSION_HTLC: &str = "your-contract.testnet";
```

### 3. 検証

#### EVMコントラクトの検証

```bash
# Sepoliaの場合
make verify CONTRACT_ADDRESS=0x... CHAIN_ID=11155111

# Base Sepoliaの場合
make verify CONTRACT_ADDRESS=0x... CHAIN_ID=84532
```

#### NEARコントラクトの検証

```bash
# 状態を確認
near view your-contract.testnet get_all_escrows '{}'
```

## トラブルシューティング

### ガス不足エラー

```bash
# EVMの場合: ガスリミットを増やす
--gas-limit 3000000

# NEARの場合: ガスを増やす
--gas 300000000000000
```

### デプロイ失敗

1. ネットワーク接続を確認
2. 残高を確認（テストトークンが必要）
3. RPCエンドポイントが正しいか確認

### 検証エラー

1. Etherscan APIキーが正しいか確認
2. コントラクトのソースコードが公開されているか確認
3. 正しいコンパイラバージョンを使用しているか確認

## Faucets

### Sepolia ETH
- https://sepoliafaucet.com/
- https://www.alchemy.com/faucets/ethereum-sepolia

### Base Sepolia ETH
- https://www.coinbase.com/faucets/base-ethereum-goerli-faucet

### NEAR Testnet
- https://near-faucet.io/

## 次のステップ

デプロイ完了後は次の設定をします。

1. リレイヤーサービスの設定
2. E2Eテストの実行
3. フロントエンドの接続
