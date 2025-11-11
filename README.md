# Helloworld - Cryptocurrency Price Tracker

このプロジェクトは、CoinGecko APIを使用して複数の暗号資産の価格をUSD建てで取得するRustプログラムです。

## 概要

このプログラムは以下の暗号資産の現在のUSD価格を取得して表示します：

- Bitcoin (BTC)
- Ethereum (ETH)
- Dogecoin (DOGE)
- Binance Coin (BNB)
- Cardano (ADA)
- Solana (SOL)
- Polkadot (DOT)
- Shiba Inu (SHIB)
- Litecoin (LTC)

## 機能

- **非同期処理**: Tokioを使用した非同期実行
- **HTTP通信**: reqwestライブラリでCoinGecko APIにアクセス
- **JSON解析**: serdeを使用したJSON解析
- **エラーハンドリング**: 適切なエラーハンドリング

## 技術スタック

- **言語**: Rust
- **ビルドシステム**: Cargo
- **主要な依存ライブラリ**:
  - `tokio`: 非同期ランタイム
  - `reqwest`: HTTPクライアント
  - `serde`: シリアライゼーション/デシリアライゼーション

## 使用方法

### ビルド

```bash
cargo build --release
```

### 実行

```bash
cargo run
```

## API

このプログラムはCoinGecko APIの以下のエンドポイントを使用しています：

```
https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum,dogecoin,binancecoin,cardano,solana,polkadot,shiba-inu,litecoin&vs_currencies=usd
```

## ライセンス

MIT License
