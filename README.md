# Fusion+ Universal Rust Gateway

1inch Fusion+プロトコルを拡張し、EVMと非EVMチェーン間でトラストレスなアトミックスワップを実現するRustベースのCLIツールです。

## 🎯 プロジェクト概要

ETHGlobal Uniteハッカソン向けに開発中のプロジェクトで、以下のチェーンをサポート予定：
- Ethereum (EVM)
- NEAR
- Cosmos
- Stellar

## 📦 現在の実装状況

### フェーズ1: HTLCコア機能 ✅

- [x] シークレット生成（32バイトのランダム値）
- [x] SHA256ハッシュ計算
- [x] 単体テストの実装
- [x] 動作確認用サンプル

### 今後の実装予定

- [ ] HTLC構造体と状態管理
- [ ] タイムロック機能
- [ ] CLIインターフェース
- [ ] 各チェーンとの統合

## 🚀 動作確認方法

### 必要な環境

- Rust 1.70以上
- Cargo

### インストール

```bash
# リポジトリのクローン
git clone https://github.com/UniteDefi/fusion-plus.git
cd fusion-plus

# 依存関係のインストール
cargo build
```

### テストの実行

```bash
# すべてのテストを実行
cargo test

# 詳細な出力付きでテストを実行
cargo test -- --nocapture
```

### サンプルプログラムの実行

HTLCのシークレット生成とハッシュ計算のデモ：

```bash
cargo run --example secret_demo
```

実行結果の例：
```
=== HTLC Secret Demo ===

生成されたシークレット（16進数）:
0b6831d4bda7948139ab5d77a613435cf5b489318f6b981a6917c563188844a1

シークレットのSHA256ハッシュ:
c655df2fb567efd3d6a6b3ab3ad7f8cad7a9449579be58ac744bb89db4b2401f

再計算したハッシュ（同じはず）:
c655df2fb567efd3d6a6b3ab3ad7f8cad7a9449579be58ac744bb89db4b2401f

別のシークレットとハッシュ:
シークレット: 29cfb8cb8fd6bf861111055e2f98119742fc4c2f0be58ad581738a8f8a4d700a
ハッシュ: 23658dcc3d91fc25dac2728fc8bcb3ef9a97572c221e54b95aded1fd30ce55b9
```

## 📂 プロジェクト構造

```
UniteDefi/
├── Cargo.toml              # ワークスペース設定
├── fusion-core/            # コアライブラリ
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   └── htlc.rs        # HTLC実装
│   ├── tests/
│   │   └── htlc_tests.rs  # 単体テスト
│   └── examples/
│       └── secret_demo.rs  # デモプログラム
└── docs/
    └── implementation-strategy.md  # 実装方針
```

## 🔧 開発方法

### TDD（テスト駆動開発）の流れ

1. **テストを書く（Red）**
   ```rust
   // tests/htlc_tests.rs にテストを追加
   #[test]
   fn test_new_feature() {
       // テストコード
   }
   ```

2. **テストを実行して失敗を確認**
   ```bash
   cargo test
   ```

3. **最小限の実装を追加（Green）**
   ```rust
   // src/htlc.rs に実装を追加
   pub fn new_feature() {
       // 実装コード
   }
   ```

4. **テストが通ることを確認**
   ```bash
   cargo test
   ```

5. **リファクタリング（Refactor）**

## 📝 ドキュメント

- [実装方針](docs/implementation-strategy.md) - 段階的な実装計画
- [技術ガイド](docs/Fusion-Plus技術ガイド.md) - 1inch Fusion+の技術詳細

## 🤝 コントリビューション

現在、ETHGlobal Uniteハッカソン向けに開発中です。

## 📄 ライセンス

MIT License