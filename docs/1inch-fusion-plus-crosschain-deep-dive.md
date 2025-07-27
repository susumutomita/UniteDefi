# 1inch Fusion+ クロスチェイン実装 完全解説

## 🌐 概要

Fusion+は、1inch Fusionのインテントベースシステムを異なるブロックチェイン間に拡張したプロトコルです。従来のアトミックスワップの問題点を解決し、ユーザーフレンドリーなクロスチェイン取引を実現します。

## 🔗 基本アーキテクチャ

### 2つのエスクロー契約

```
┌─────────────────────┐         ┌─────────────────────┐
│   Chain A (EVM)     │         │   Chain B (NEAR)    │
├─────────────────────┤         ├─────────────────────┤
│   EscrowSrc         │         │   EscrowDst         │
│  (ユーザー資金)     │         │  (リゾルバー資金)    │
└─────────────────────┘         └─────────────────────┘
```

1. EscrowSrc（ソースチェイン）
   - ユーザーのトークンをロック
   - シークレット提示で資金をリゾルバーへ

2. EscrowDst（デスティネーションチェイン）
   - リゾルバーのトークンをロック
   - 同じシークレットで資金をユーザーへ

## ⏰ タイムロックメカニズム

### タイムライン詳細

![Timelocks](../references/cross-chain-swap/timelocks.png)

### 重要な期間

#### 1. Dutch Auction期間
- 目的: 最適価格の発見
- 動作: 価格が時間とともに変化し、リゾルバーに有利な条件へ移行
- 終了後: 価格が固定される

#### 2. Finality Lock（A1, B1）
- 目的: チェインのリオーガナイゼーション対策
- 期間: 通常5-10分
- 制限: この期間中は誰も資金を動かせない

#### 3. Resolver Unlock（A2, B2）
- Chain A: リゾルバーのみが引き出し可能
- Chain B: リゾルバーのみが引き出し可能
- 条件: 正しいシークレットの提示が必要

#### 4. Anybody Can Unlock（A3, B3）
- 目的: リゾルバーが失敗した場合の救済
- Chain A: 誰でも引き出し可能（ユーザーへ）
- Chain B: 誰でも引き出し可能（ユーザーへ）
- インセンティブ: セーフティデポジットを獲得

#### 5. Resolver Cancellation（A4, B4）
- 目的: 失敗時のリゾルバー資金回収
- Chain A: リゾルバーがキャンセル（ユーザーへ返金）
- Chain B: リゾルバーがキャンセル（自分へ返金）

#### 6. Public Cancellation（A5）
- 目的: 最終的な資金回収手段
- 対象: Chain Aのみ
- 動作: 誰でもキャンセル可能（ユーザーへ返金）

## 🔄 実行フロー詳細

### Phase 1: アナウンス

```
時刻: E1以前
```

1. ユーザーが注文に署名
   ```typescript
   {
     fromChain: "Ethereum",
     toChain: "NEAR",
     fromToken: "ETH",
     toToken: "NEAR",
     amount: "1 ETH",
     minReturn: "100 NEAR",
     secretHash: "0x1234..." // シークレットのハッシュ
   }
   ```

2. 1inchリレーヤーが全リゾルバーに配信
   - Dutch Auctionが開始
   - リゾルバーが利益計算開始

### Phase 2: エスクロー作成

```
時刻: E1 → E2
```

1. E1: ソースチェインエスクロー作成
   ```solidity
   // Chain A (Ethereum)
   EscrowSrc deployed with:
   - maker: ユーザーアドレス
   - taker: リゾルバーアドレス
   - token: ETH
   - amount: 1 ETH
   - hashlock: 0x1234...
   - timelock: 各期間の設定
   - safetyDeposit: 0.01 ETH
   ```

2. E2: デスティネーションチェインエスクロー作成
   ```rust
   // Chain B (NEAR)
   EscrowDst deployed with:
   - maker: リゾルバーアドレス
   - taker: ユーザーアドレス
   - token: NEAR
   - amount: 100 NEAR
   - hashlock: 0x1234... // 同じハッシュ
   - timelock: 各期間の設定
   - safetyDeposit: 1 NEAR
   ```

### Phase 3: シークレット公開と引き出し

```
時刻: E3
```

1. E3: リレーヤーがシークレットを公開
   - 両方のエスクローが正しく作成されたことを確認
   - 全リゾルバーにシークレットを配信

2. 引き出し実行
   ```solidity
   // Chain B (NEAR) - ユーザーが受け取る
   escrowDst.publicWithdraw(secret, immutables)
   → 100 NEARがユーザーへ

   // Chain A (Ethereum) - リゾルバーが受け取る
   escrowSrc.withdraw(secret, immutables)
   → 1 ETHがリゾルバーへ
   ```

### Phase 4: タイムアウト処理

```
時刻: E4 → E5
```

もし何か問題が発生した場合。

1. E4: リゾルバータイムアウト
   - リゾルバーが自分の資金を回収可能
   - Chain A: ユーザーへ返金
   - Chain B: リゾルバーへ返金

2. E5: ユーザータイムアウト
   - 最終的な保護メカニズム
   - 誰でもキャンセルを実行可能
   - セーフティデポジットがインセンティブ

## 💰 セーフティデポジット

### 目的
- リゾルバーの誠実な行動を促す
- パブリック期間での実行インセンティブ
- ガス代の補償

### 仕組み
```
リゾルバーがデポジット
    ↓
成功時：実行者が獲得
失敗時：キャンセル実行者が獲得
```

## 🔐 セキュリティ機能

### 1. アトミック性の保証
- 両方成功or両方失敗
- 部分的な実行は不可能
- HTLCによる数学的保証

### 2. タイムアウト保護
- 各段階に明確な期限
- 資金が永遠にロックされることはない
- 段階的な権限移譲

### 3. MEV対策
- Dutch Auctionによる公平な価格発見
- Finality Lockでリオーグ対策
- 優先実行期間の設定

## 📊 通常のブリッジとの比較

| 項目 | 従来のブリッジ | Fusion+ |
|------|--------------|---------|
| 信頼モデル | 中央集権的バリデーター | トラストレス（HTLC） |
| 実行時間 | 10-30分 | 2-5分 |
| 手数料 | 固定（高い） | 競争による最適化 |
| 失敗時 | サポートに連絡 | 自動返金 |
| 流動性 | プール依存 | リゾルバーが提供 |

## 🎯 NEARへの実装要件

### 1. HTLCコントラクト
```rust
pub struct Escrow {
    pub maker: AccountId,
    pub taker: AccountId,
    pub amount: Balance,
    pub token_id: Option<AccountId>,
    pub secret_hash: Hash,
    pub timelocks: Timelocks,
    pub safety_deposit: Balance,
    pub state: EscrowState,
}
```

### 2. タイムロック実装
```rust
pub struct Timelocks {
    pub deployment_time: Timestamp,
    pub finality_lock: Duration,
    pub resolver_unlock: Duration,
    pub public_unlock: Duration,
    pub cancellation: Duration,
}
```

### 3. 必要な関数
- `create_escrow()`: エスクロー作成
- `withdraw()`: シークレットでの引き出し
- `public_withdraw()`: パブリック期間の引き出し
- `cancel()`: キャンセル
- `public_cancel()`: パブリックキャンセル

## 🚀 実装の課題と解決策

### 課題1: チェイン間の時間同期
- 問題: 異なるブロック生成時間
- 解決: UNIXタイムスタンプベースの同期

### 課題2: ガス代の違い
- 問題: EVMとNEARでガス代が大きく異なる
- 解決: セーフティデポジットで調整

### 課題3: トランザクションの最終性
- 問題: チェインによって最終性の時間が異なる
- 解決: Finality Lockを適切に設定

## 📝 まとめ

Fusion+は、巧妙なタイムロックメカニズムとHTLCを組み合わせることで、安全かつ高速（2-5分）で低コストなクロスチェインスワップを実現します。各期間の役割を理解し、適切に実装することが成功の鍵となります。

NEARへの実装では、これらの概念をRustで表現し、NEARの特性（低ガス代、高速実行）を活かしながら、1inch Fusion+の完全な体験を再現することが目標です。
