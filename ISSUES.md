# Issues and Improvements

このファイルは、プロジェクトの課題と改善タスクを追跡します。

## アクティブなIssue

### 🔒 Security: PR #10フィードバックに基づくセキュリティ改善
- **Issue**: [Security Improvements](.github/ISSUE_TEMPLATE/security-improvements.md)
- **詳細**: [PR10-security-improvements](docs/issues/PR10-security-improvements.md)
- **優先度**: 高
- **ステータス**: 未着手

主な改善点：
1. 入力検証の強化
2. 暗号操作のセキュリティ向上
3. より堅牢な時刻処理

### 🚀 Feature: CLIインターフェースの実装
- **優先度**: 高
- **ステータス**: 未着手

必要な機能：
- `create-htlc`: HTLCの作成
- `claim`: シークレットを使用したクレーム
- `refund`: タイムアウト後のリファンド

### 🌐 Integration: NEARチェーンとの統合
- **優先度**: 高
- **ステータス**: 未着手

タスク：
- NEARスマートコントラクトの作成
- Rust SDKを使用した統合
- テストネットでのデプロイ

## 完了したタスク ✅

- HTLCコア機能の実装
- 基本的なテストスイート
- CI/CDパイプラインの設定
- 動作デモの作成

## 今後の計画

1. **短期（〜2日）**: セキュリティ改善の実装
2. **中期（〜5日）**: CLI完成とNEAR統合
3. **長期（〜8日）**: Cosmos、Stellar統合とハッカソン提出準備

## 貢献方法

1. Issueを選択
2. ブランチを作成: `git checkout -b feature/issue-name`
3. 実装とテストを追加
4. PRを作成してレビューを依頼

## 関連リンク

- [プロジェクトREADME](README.md)
- [実装戦略](docs/implementation-strategy.md)
- [技術ガイド](docs/Fusion-Plus技術ガイド.md)