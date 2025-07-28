# Quick Start Guide - クロスチェインスワップ実装

## 30分で理解する実装の全体像

### 🎯 やりたいこと
Monad（EVM）⇔ NEAR間でトークンをスワップ。

### 🏗️ 必要なコンポーネント

```
[User] → [Monad HTLC] → [Relayer] → [NEAR HTLC] → [User]
           (未実装)      (未実装)      (実装済み✅)
```

## 📝 今すぐやるべき3つのこと

### 1. 1inchのサンプルを動かす（15分）
```bash
# クローン
git clone https://github.com/1inch/cross-chain-resolver-example
cd cross-chain-resolver-example

# インストール & テスト
npm install
npx hardhat test

# コントラクトを読む
code contracts/CrossChainResolver.sol
```

### 2. 最小限のリレイヤーを作る（30分）
```typescript
// relayer/index.ts
import { ethers } from 'ethers';
import { connect as nearConnect } from 'near-api-js';

// EVMイベントを監視
const evmProvider = new ethers.providers.JsonRpcProvider(EVM_RPC);
const evmContract = new ethers.Contract(RESOLVER_ADDRESS, ABI, evmProvider);

evmContract.on("EscrowCreated", async (escrowId, secretHash) => {
    console.log("EVM Escrow Created:", escrowId);
    // TODO: NEAR側に通知
});

// NEARの変更を監視
const near = await nearConnect(nearConfig);
// TODO: NEARイベント監視
```

### 3. ローカルでテスト（15分）
```bash
# Terminal 1: Hardhat（EVM）
npx hardhat node

# Terminal 2: デプロイ
npx hardhat run scripts/deploy.js --network localhost

# Terminal 3: NEARコントラクト
near dev-deploy target/wasm32-unknown-unknown/release/near_htlc.wasm

# Terminal 4: リレイヤー
node relayer/index.js
```

## 🔑 重要な概念

### HTLCの流れ
1. Alice（Monad）: トークンをロック + ハッシュ設定
2. Bob（NEAR）: 同じハッシュでHTLC作成
3. Alice: NEARでシークレット公開してクレーム
4. Bob: 公開されたシークレットMonadでクレーム

### なぜリレイヤーが必要
- EVMとNEARは直接通信できない
- リレイヤーが両チェインのイベントを監視
- 必要な情報を相手チェインに中継

## 🚨 よくある問題と解決策

### Q: 1inchのコントラクトをそのまま使える
A: 基本構造は使えるが、Monad対応の確認が必要。

### Q: リレイヤーはどこで動かす
A: 開発時はローカル、本番はクラウド（Heroku/AWS等）。

### Q: テストネットのトークンはどうする
A: Faucetから取得orテスト用トークンをデプロイ。

## 📊 実装優先順位

1. 必須（Day 1）
   - [ ] EVMコントラクト（1inchベース）
   - [ ] 最小限のリレイヤー

2. 重要（Day 2）
   - [ ] エラーハンドリング
   - [ ] E2Eテスト

3. あれば良い（Day 3）
   - [ ] UI
   - [ ] 複数トークン対応

## 🔗 すぐ使えるリンク

- [1inch Resolver Example](https://github.com/1inch/cross-chain-resolver-example)
- [NEAR TestNet Faucet](https://near-faucet.io/)
- [Monad Discord](https://discord.gg/monad)
- [実装済みNEAR HTLC](../contracts/near-htlc/)
