# 実装修正計画

## 現在の実装の問題点

1. 独自Escrowコントラクトの作成
   - 1inchの公式実装を使わず独自実装してしまった
   - Limit Order Protocolとの統合がない
   - ハッカソン要件を満たしていない

2. 正しい1inch Fusion+フロー
   ```
   1. ユーザーがオフチェーンで注文に署名
   2. リゾルバーがLimit Order Protocolで注文を実行
   3. EscrowSrcが自動的にデプロイされる
   4. リゾルバーがEscrowDstをデプロイ
   5. クロスチェーンでの資金移動
   ```

## 修正すべき点

### Phase 1: 1inch公式コントラクトの使用

1. Limit Order Protocolのデプロイ
   ```bash
   # references/cross-chain-swap/lib/limit-order-protocol/をベースに
   # Base SepoliaとSepoliaにデプロイ
   ```

2. 1inch公式EscrowFactoryの使用
   ```bash
   # references/cross-chain-swap/contracts/をベースに
   # 独自実装は破棄
   ```

### Phase 2: 正しい統合

1. Order作成とSignature
   - 1inch SDKを使用してオーダーを作成
   - EIP-712準拠の署名

2. Resolverの実装
   - Limit Order Protocolを呼び出してEscrowSrcを作成
   - NEARでHTLCを作成
   - シークレット管理

### Phase 3: デモの準備

1. フロントエンド
   - 1inch SDKを使用した注文作成UI
   - ウォレット接続（MetaMask + NEAR Wallet）

2. リレイヤー/リゾルバー
   - イベント監視
   - クロスチェーン実行

## 実装スケジュール

1. Day 1: 1inch公式コントラクトのテストネットデプロイ
2. Day 2: リゾルバーとリレイヤーの実装
3. Day 3: フロントエンドとデモ準備

## 結論

現在の独自実装は破棄し、1inch公式実装をベースに作り直す必要があります。
これにより、ハッカソン要件を正しく満たすことができます。
