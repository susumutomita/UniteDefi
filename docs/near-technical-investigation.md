# NEAR Protocol 技術調査レポート

## 🎯 重要な前提：NEARではSolidityは動作しません

NEARは独自のランタイムを持つブロックチェインで、Ethereum Virtual Machine (EVM) とは異なるアーキテクチャです。

### NEAR vs Ethereum の根本的な違い

| 項目 | Ethereum | NEAR |
|------|----------|------|
| 仮想マシン | EVM | NEAR Runtime (WebAssembly) |
| スマートコントラクト言語 | Solidity | Rust, AssemblyScript |
| アカウントモデル | アドレスベース | 人間が読めるアカウントID |
| ガス代支払い | ユーザーが支払う | メタトランザクション可能 |
| ストレージコスト | ガス代に含まれる | 別途ストレージステーキングが必要 |

## 🔧 NEARでスマートコントラクトを書く方法

### 1. 開発言語の選択

NEARでは2つの言語が選択可能。

#### Rust（推奨）
- 高性能
- 型安全性
- より多くの機能にアクセス可能
- プロダクション環境向け

```rust
use near_sdk::{near_bindgen, env};

#[near_bindgen]
pub struct Contract {
    value: String,
}

#[near_bindgen]
impl Contract {
    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}
```

#### AssemblyScript
- TypeScriptに似た構文
- 学習曲線が緩やか
- プロトタイピング向け

### 2. NEARの基本概念

#### アカウント
- 例：`alice.near`、`contract.alice.near`
- サブアカウントの作成が可能
- アカウント自体がコントラクトをホストできる

#### ストレージステーキング
- データ保存にはNEARトークンのロックが必要
- 1バイトあたり約0.00001 NEAR

#### メソッドの種類
- viewメソッド: 状態を読むだけ（無料）
- changeメソッド: 状態を変更（ガス代必要）

## 🔐 HTLCエスクロー実装に必要な要素

### 1. タイムロック機能
```rust
pub struct Timelock {
    pub deployment_time: u64,      // ナノ秒単位のUNIXタイムスタンプ
    pub finality_lock: u64,        // ファイナリティロック期間
    pub resolver_unlock: u64,      // リゾルバー専用期間
    pub anybody_unlock: u64,       // 誰でも実行可能期間
    pub resolver_cancel: u64,      // リゾルバーキャンセル期間
}
```

### 2. ハッシュロック機能
```rust
use sha2::{Sha256, Digest};

fn verify_secret(secret: &str, hash: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(secret.as_bytes());
    let result = hasher.finalize();
    let computed_hash = hex::encode(result);
    computed_hash == hash
}
```

### 3. クロスコントラクトコール
NEARでは`Promise`を使用して他のコントラクトを呼び出す。
```rust
use near_sdk::Promise;

// NEARトークンの転送
Promise::new(recipient).transfer(amount);

// NEP-141トークンの転送
Promise::new(token_contract)
    .function_call(
        "ft_transfer",
        json!({
            "receiver_id": recipient,
            "amount": amount.to_string(),
        }).to_string().as_bytes(),
        1, // 1 yoctoNEAR
        5_000_000_000_000, // 5 TGas
    );
```

### 4. セーフティデポジット
```rust
pub struct Escrow {
    pub safety_deposit: Balance,  // リゾルバーが預けるデポジット
    pub safety_deposit_beneficiary: Option<AccountId>, // 獲得者
}
```

## 📋 1inch Fusion+ NEAR実装のための要件

### 必須機能
1. エスクロー作成
   - リゾルバーがNEARトークンをロック
   - ハッシュロックとタイムロックの設定
   - セーフティデポジットの受け取り

2. シークレット検証と引き出し
   - 正しいシークレットの検証
   - タイムロック期間のチェック
   - 受取人への送金

3. タイムアウト処理
   - 各期間での権限チェック
   - キャンセル機能
   - 返金処理

### NEARの制約事項
1. 非同期実行
   - クロスコントラクトコールは非同期
   - コールバックパターンの使用が必要

2. ガスリミット
   - 1トランザクションあたり最大300 TGas
   - 複雑な処理は分割が必要

3. ストレージ管理
   - データ保存にはデポジットが必要
   - 不要になったデータは削除してデポジットを回収

## 🛠 開発環境セットアップ

### 1. 必要なツール
```bash
# Rustのインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# NEAR CLIのインストール
npm install -g near-cli

# WebAssemblyターゲットの追加
rustup target add wasm32-unknown-unknown
```

### 2. プロジェクト構造
```
contracts/near-htlc/
├── Cargo.toml
├── src/
│   ├── lib.rs          # メインコントラクト
│   ├── escrow.rs       # エスクロー構造体
│   ├── timelock.rs     # タイムロック管理
│   └── tests/          # ユニットテスト
├── build.sh            # ビルドスクリプト
└── deploy.sh           # デプロイスクリプト
```

### 3. テストネットアカウント作成
```bash
# テストネットアカウント作成
near create-account htlc.testnet --useFaucet

# コントラクトデプロイ
near deploy --accountId htlc.testnet --wasmFile target/wasm32-unknown-unknown/release/near_htlc.wasm
```

## 📚 参考リソース

### 公式ドキュメント
- [NEAR Documentation](https://docs.near.org/)
- [NEAR SDK-RS](https://docs.rs/near-sdk/latest/near_sdk/)
- [NEAR Examples](https://github.com/near-examples)

### HTLCの参考実装
- [Atomic Swaps on NEAR](https://github.com/near/near-atomic-swap)
- [Rainbow Bridge](https://github.com/aurora-is-near/rainbow-bridge) - NEARとEthereumのブリッジ

### 開発ツール
- [NEAR Explorer](https://explorer.testnet.near.org/) - トランザクション確認
- [NEAR Wallet](https://wallet.testnet.near.org/) - テストネットウォレット
- [near-api-js](https://github.com/near/near-api-js) - JavaScript SDK

## 🎯 次のステップ

1. Rust環境のセットアップ
   - Rustとnear-cliのインストール
   - テストネットアカウントの作成

2. HTLCコントラクトの設計
   - 1inch Fusion+の仕様に合わせた実装
   - タイムロック期間の調整

3. テストとデバッグ
   - ユニットテストの作成
   - テストネットでの動作確認

4. CLIツールの実装
   - クロスチェインスワップのオーケストレーション
   - Ethereumとの連携

## ⚠️ 重要な注意点

1. NEARはEVMチェインではない
   - Solidityコードは動作しない
   - 独自のスマートコントラクト開発が必要

2. 非同期処理の理解が必須
   - Promiseベースの設計
   - コールバックパターン

3. ストレージコストの考慮
   - データ保存にはNEARのロックが必要
   - データ構造の設計が重要

この調査に基づいて、NEAR側のHTLCエスクローコントラクトを実装していきます。
