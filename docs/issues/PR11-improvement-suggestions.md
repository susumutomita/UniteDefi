# PR #11 改善提案

このドキュメントは、PR #11（bugfix/pr10）のレビューフィードバックに基づく改善タスクを管理します。

## 実装済みの改善 ✅

### 1. セキュリティ改善
- ✅ HTLC作成時の入力検証（空文字、金額、ハッシュ長チェック）
- ✅ 定数時間比較によるタイミング攻撃への耐性
- ✅ 詳細なエラーメッセージ（InvalidInput）

### 2. テストカバレッジ
- ✅ 異常系テスト（空の送信者/受信者、ゼロ金額、不正なハッシュ長）
- ✅ 二重クレーム、状態遷移テスト
- ✅ タイムアウト境界値テスト

### 3. 開発環境改善
- ✅ YAMLリント導入
- ✅ Makefileターゲット拡充
- ✅ before_commitターゲットによる品質チェック自動化

## 今後の改善タスク 📋

### 優先度: 高

#### 1. 型安全性の向上
- [ ] `Vec<u8>`から`[u8; 32]`固定長配列への移行
  - secret_hashの実行時チェックをコンパイル時保証に
  - 型エイリアス（`type SecretHash = [u8; 32]`）の導入

#### 2. YAMLリントのエラー無視問題
- [x] `lint:yaml`コマンドの`|| true`を削除
  - CIでリントエラーが隠蔽されないように修正

### 優先度: 中

#### 3. 時刻処理の本番対応
- [ ] `Instant::now()`から`SystemTime`への移行
- [ ] ブロックチェーンのブロック高ベースのタイムアウト管理の検討
  ```rust
  pub struct Htlc {
      // ...
      created_at: SystemTime,
      timeout_block: u64,  // ブロック高でのタイムアウト
  }
  ```

#### 4. テストの安定性向上
- [ ] `thread::sleep`を使用したテストの改善
  - モックタイマーの導入
  - テスト用の時刻制御機能

### 優先度: 低

#### 5. 並行性テストの追加
- [ ] `Arc<Mutex<Htlc>>`を使用した並行アクセステスト
- [ ] 競合状態・レースコンディションの検証
  ```rust
  #[tokio::test]
  async fn test_concurrent_claims() {
      // 複数スレッドから同時にクレーム
  }
  ```

#### 6. ドキュメントの充実
- [ ] HTLCの使用例をREADMEに追加
- [ ] APIドキュメント（rustdoc）の充実
- [ ] セキュリティ考慮事項のドキュメント化

## 実装例

### 型安全性の向上例
```rust
// 型エイリアスの定義
pub type SecretHash = [u8; 32];
pub type Secret = [u8; 32];

// HTLCの修正
pub struct Htlc {
    // ...
    secret_hash: SecretHash,
    // ...
}

// 関数シグネチャの修正
pub fn hash_secret(secret: &Secret) -> SecretHash {
    let mut hasher = Sha256::new();
    hasher.update(secret);
    hasher.finalize().into()
}
```

### SystemTime使用例
```rust
use std::time::SystemTime;

impl Htlc {
    pub fn is_timed_out(&self) -> bool {
        match SystemTime::now().duration_since(self.created_at) {
            Ok(elapsed) => elapsed > self.timeout,
            Err(_) => true, // 時刻が過去の場合もタイムアウトとする
        }
    }
}
```

## 関連リンク
- [PR #11](https://github.com/susumutomita/UniteDefi/pull/11)
- [PR #10 セキュリティ改善](PR10-security-improvements.md)