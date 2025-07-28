# 実装方針（Implementation Strategy）

## 概要

このドキュメントは、UniteSwapの段階的な実装方針を記載します。
各フェーズで動作確認を行いながら、着実に機能を積み上げていきます。

## 実装原則

1. Test-Driven Development (TDD): 常にテストを先に書く
2. 段階的な実装: 小さな動作する単位で開発を進める
3. 動作確認の重視: 各段階で実際に動かして確認する
4. ごまかしのない実装: モックやスタブは明確に分離し、本実装は動作するものを作る

## フェーズ別実装計画

### フェーズ1: HTLCコア機能（現在）

目標: HTLCの最も基本的な機能を実装し、動作を確認する。

1. シークレット生成とハッシュ計算
   - 32バイトのランダムシークレット生成
   - SHA256によるハッシュ計算
   - テストで動作確認

2. 基本的なHTLC構造体
   - HTLCの状態管理（Pending, Claimed, Refunded）
   - タイムロック機能
   - シークレットによる解除機能

3. テストの作成
   - シークレット生成のテスト
   - ハッシュ検証のテスト
   - タイムアウトのテスト

動作確認方法。
```bash
cd fusion-core
cargo test -- --nocapture
```

### フェーズ2: CLIの基本構造

目標: コマンドラインから基本的な操作ができるようにする。

1. CLIフレームワーク
   - clap v4を使用したコマンドライン引数処理
   - サブコマンド構造（init, create, claim, refund）
   - ヘルプとバージョン表示

2. 設定管理
   - 設定ファイルの読み込み（TOML形式）
   - 環境変数によるオーバーライド
   - デフォルト値の設定

3. 基本コマンドの実装
   - `fusion-cli secret generate` - シークレット生成
   - `fusion-cli secret hash <secret>` - ハッシュ計算
   - `fusion-cli htlc create` - HTLC作成（ローカルシミュレーション）

動作確認方法。
```bash
cd fusion-cli
cargo build
./target/debug/fusion-cli --help
./target/debug/fusion-cli secret generate
```

### フェーズ3: 単一チェインでのHTLC動作確認

目標: 実際のブロックチェイン接続なしで、HTLCの全体フローを確認する。

1. モックチェインの実装
   - インメモリのチェイン状態管理
   - ブロック番号とタイムスタンプのシミュレーション
   - イベント発行の仕組み

2. HTLCライフサイクル
   - Create: HTLCの作成とロック
   - Claim: シークレット提供による資金の請求
   - Refund: タイムアウト後の返金

3. 統合テスト
   - エンドツーエンドのフローテスト
   - エラーケースのテスト
   - 並行処理のテスト

動作確認方法。
```bash
# 統合テストの実行
cargo test --workspace

# CLIでのデモ
./target/debug/fusion-cli demo single-chain
```

### フェーズ4: Ethereum統合

目標: 実際のEthereumテストネットでHTLCを動作させる。

1. Ethereum接続
   - ethers-rsを使用したRPC接続
   - ウォレット管理
   - ガス推定と手数料処理

2. スマートコントラクト
   - HTLCコントラクトのデプロイ
   - 1inch Fusion+との互換性確認
   - イベントの監視

3. テストネット動作確認
   - Sepoliaテストネットでの動作
   - 実際のトランザクション送信
   - エクスプローラーでの確認

### フェーズ5: NEAR統合

目標: NEARプロトコルでHTLCを実装し、Ethereumとのクロスチェインスワップを実現。

1. NEAR接続
   - near-sdk-rsを使用
   - アカウント管理
   - ストレージステーキング

2. NEARスマートコントラクト
   - AssemblyScriptまたはRustでのコントラクト実装
   - クロスコントラクト呼び出し
   - 非同期処理の扱い

3. クロスチェインフロー
   - Ethereum → NEARスワップ
   - NEAR → Ethereumスワップ
   - リレイヤーの実装

### フェーズ6以降

- Cosmos統合
- Stellar統合
- リレイヤーサービスの本格実装
- UI/UXの改善
- パフォーマンス最適化

## 各フェーズの完了基準

1. すべてのテストが通る
2. ドキュメントが更新される
3. READMEに動作確認手順が記載される
4. 実際に動作することを確認する
5. コードレビューを実施する

## 現在の状態

- プロジェクト構造の作成
- フェーズ1: HTLCコア機能の実装中
- フェーズ2以降: 未着手

## 参考資料

- [1inch Fusion+ Documentation](https://docs.1inch.io/)
- [HTLC Explained](https://en.bitcoin.it/wiki/Hash_Time_Locked_Contracts)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
