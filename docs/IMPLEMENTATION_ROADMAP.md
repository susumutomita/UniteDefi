# 実装ロードマップ - ETHGlobal Unite向け

## 🎯 目標
EVMチェイン（Monad）とNEARチェイン間でのアトミックスワップを実現。

## 📊 現在の進捗

### ✅ Phase 1: NEAR側実装（完了）
- [x] FusionHTLCコントラクト実装
- [x] セキュリティ機能追加
- [x] Storage Limits実装
- [x] テストスイート作成

### 🚧 Phase 2: EVM側実装（進行中）
- [ ] 1inch cross-chain-resolver-exampleの調査
- [ ] EVM用HTLCコントラクト実装
- [ ] Monadチェイン対応確認

### 📋 Phase 3: ブリッジ実装（未着手）
- [ ] リレイヤーサービス設計
- [ ] イベント監視システム
- [ ] メッセージ中継ロジック

### 🎨 Phase 4: フロントエンド（未着手）
- [ ] UI/UXデザイン
- [ ] ウォレット接続（MetaMask + NEAR Wallet）
- [ ] スワップフロー実装

## 🔧 具体的な実装タスク

### 今すぐやるべきこと

#### 1. EVM側コントラクト（優先度: 高）
```bash
# 1inchのサンプルをクローン
git clone https://github.com/1inch/cross-chain-resolver-example
cd cross-chain-resolver-example

# 依存関係をインストール
npm install

# コントラクトを理解
# contracts/CrossChainResolver.sol を参照
```

#### 2. リレイヤーサービス基本構造（優先度: 高）
```typescript
// relayer/src/config.ts
export const config = {
  evm: {
    rpc: process.env.EVM_RPC_URL,
    resolver: process.env.EVM_RESOLVER_ADDRESS,
  },
  near: {
    network: process.env.NEAR_NETWORK,
    contract: process.env.NEAR_CONTRACT_ID,
  }
};

// relayer/src/watchers/evmWatcher.ts
export class EVMWatcher {
  async watchForEscrowCreation() {
    // Escrow作成イベントを監視
  }

  async watchForSecretReveal() {
    // シークレット公開を監視
  }
}

// relayer/src/watchers/nearWatcher.ts
export class NEARWatcher {
  async watchForClaim() {
    // NEAR側のクレームを監視
  }
}
```

### テスト戦略

#### ローカル開発環境
```bash
# Terminal 1: Hardhat Node（EVM）
npx hardhat node --fork https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY

# Terminal 2: NEAR Sandbox
npm run sandbox

# Terminal 3: Relayer
npm run relayer:dev
```

#### テストネット環境
1. **Monad Testnet** + **NEAR Testnet**
2. **Sepolia** + **NEAR Testnet**（バックアップ案）

## 📝 実装チェックリスト

### 必須機能
- [ ] EVM→NEARスワップ
- [ ] NEAR→EVMスワップ
- [ ] タイムアウト処理
- [ ] エラーリカバリ

### セキュリティ
- [ ] ハッシュ検証
- [ ] タイムロック検証
- [ ] リエントランシー保護
- [ ] ガスリミット設定

### UI/UX
- [ ] ウォレット接続
- [ ] スワップ状態表示
- [ ] エラーハンドリング
- [ ] トランザクション履歴

## 🚀 デプロイ手順

### 1. コントラクトデプロイ
```bash
# EVM側
npx hardhat run scripts/deploy.js --network monad-testnet

# NEAR側
near deploy --wasmFile target/wasm32-unknown-unknown/release/near_htlc.wasm --accountId YOUR_ACCOUNT.testnet
```

### 2. リレイヤー起動
```bash
# 環境変数設定
cp .env.example .env
# .envを編集

# リレイヤー起動
npm run relayer:start
```

### 3. フロントエンド起動
```bash
cd frontend
npm install
npm run dev
```

## 📚 参考になるコード例

### 1inch Resolver の重要な部分
```solidity
function initiateSwap(
    address token,
    uint256 amount,
    bytes32 secretHash,
    address recipient,
    uint256 timelock
) external {
    // トークンをロック
    IERC20(token).transferFrom(msg.sender, address(this), amount);

    // エスクロー作成
    escrows[escrowId] = Escrow({
        sender: msg.sender,
        recipient: recipient,
        token: token,
        amount: amount,
        secretHash: secretHash,
        timelock: timelock,
        claimed: false
    });

    emit EscrowCreated(escrowId, msg.sender, recipient, amount);
}
```

### NEARとの統合ポイント
```typescript
// リレイヤーでのイベント処理
evmContract.on("EscrowCreated", async (escrowId, sender, recipient, amount) => {
    // NEAR側にメッセージを送信
    await nearContract.notify_escrow_created({
        escrow_id: escrowId.toString(),
        sender: sender,
        recipient: nearRecipient,
        amount: amount.toString()
    });
});
```

## 🎯 ハッカソン向け最小実装

1. **Day 1**: EVM側コントラクトとリレイヤー基礎
2. **Day 2**: NEAR統合とE2Eテスト
3. **Day 3**: UI実装とデモ準備

## 困ったときは

- 1inchのDiscordチャンネルで質問
- NEARのDiscordチャンネルで質問
- ETHGlobal Uniteのメンターに相談
