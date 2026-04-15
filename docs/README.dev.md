# Yokogushi 開発環境セットアップ

ローカルでの開発手順をまとめます。

---

## 必要なもの

| ツール | バージョン | 用途 |
|---|---|---|
| Rust | 1.75+ (stable) | APIサーバー |
| Node.js | 20+ | フロントエンド |
| npm | 10+ | パッケージ管理 |

Rustのインストール:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## ディレクトリ構成

```
yokogushi/
├── Cargo.toml              # Rust workspace
├── crates/
│   ├── core/               # ドメイン型 (Skill, Certification)
│   ├── dict/               # スキル/資格マスタ辞書 + サジェスト関数
│   └── api/                # axum HTTPサーバー
└── web/                    # Next.js フロントエンド
```

---

## 起動方法

### 1. APIサーバー（Rust / axum）

ポート: **3001**

```bash
# 初回ビルド (依存解決に数分かかります)
cargo build

# 起動
cargo run -p yokogushi-api
```

ログレベルを変えたいとき:

```bash
RUST_LOG=debug cargo run -p yokogushi-api
```

動作確認:

```bash
curl 'http://localhost:3001/api/dict/skills?q=ja&limit=5'
curl 'http://localhost:3001/api/dict/certs?q=基本'
```

### 2. フロントエンド（Next.js）

ポート: **3000**

```bash
cd web
npm install      # 初回のみ
npm run dev
```

ブラウザで http://localhost:3000 を開く。

`next.config.mjs` の rewrite により `/api/*` は自動的に `http://localhost:3001` へプロキシされるため、CORS設定なしで同一オリジン扱いになります。

API向き先を変えたい場合:

```bash
API_BASE_URL=http://localhost:4000 npm run dev
```

---

## よく使うコマンド

| 目的 | コマンド |
|---|---|
| Rust 全体ビルド | `cargo build` |
| Rust 全体テスト | `cargo test` |
| 辞書crateのテストのみ | `cargo test -p yokogushi-dict` |
| Rust フォーマット | `cargo fmt` |
| Rust lint | `cargo clippy --all-targets` |
| フロント型チェック | `cd web && npx tsc --noEmit` |
| フロントビルド | `cd web && npm run build` |

---

## API エンドポイント（現状）

| メソッド | パス | 説明 |
|---|---|---|
| GET | `/api/dict/skills?q=<query>&limit=<n>` | スキルサジェスト |
| GET | `/api/dict/certs?q=<query>&limit=<n>` | 資格サジェスト |

`limit` は省略時 10。クエリは前方一致が最優先、部分一致がその次にランクされます。エイリアス（例: `k8s` → Kubernetes）にもマッチします。

---

## トラブルシュート

### `cargo run` が遅い
初回コンパイルは数分かかります。2回目以降はインクリメンタルビルドで数秒です。

### フロントから `/api/...` が 404
APIサーバー（3001）が起動していません。`cargo run -p yokogushi-api` を別ターミナルで実行してください。

### ポート競合
- `yokogushi-api`: `crates/api/src/main.rs` の `bind("0.0.0.0:3001")` を変更
- `web`: `web/package.json` の `dev` スクリプトで `-p` を変更
