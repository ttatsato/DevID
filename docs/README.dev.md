# Yokogushi 開発環境セットアップ

ローカルでの開発手順をまとめます。

---

## 必要なもの

| ツール | バージョン | 用途 |
|---|---|---|
| Rust | 1.75+ (stable) | APIサーバー |
| Node.js | 20+ | フロントエンド |
| npm | 10+ | パッケージ管理 |
| Docker | 20+ | PostgreSQL コンテナ |

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
│   ├── core/               # ドメイン型 (Skill, Certification, Employment...)
│   ├── dict/               # スキル/資格マスタ辞書 + サジェスト関数
│   └── api/                # axum HTTPサーバー
│       ├── src/
│       │   ├── main.rs
│       │   ├── state.rs        # AppState
│       │   ├── auth.rs         # OAuth + セッションハンドラ
│       │   ├── profile.rs      # プロフィールハンドラ
│       │   ├── portfolio.rs    # ポートフォリオハンドラ
│       │   └── repo/           # DB アクセス層（詳細は「開発方針」）
│       └── migrations/         # sqlx マイグレーション
├── web/                    # Next.js フロントエンド
└── docker-compose.yml      # PostgreSQL
```

---

## 開発方針

継続的に守りたいルール。破るときは必ず理由をコミットメッセージ/PRに残す。

### 1. DBアクセスは Repository 層に集約

**ルール**: ハンドラ（`*.rs`）では SQL を直接書かない。必ず `repo::<domain>::<fn>()` を経由する。

**なぜ**
- `WHERE user_id = $1` の書き忘れで他ユーザーのデータが出る事故を、**関数シグネチャで防ぐ**
- クエリ最適化・マイグレーションの影響範囲が localized される
- 将来テスト化するときに repo 単位でモック/結合テストを書きやすい

**ユーザースコープのクエリは `user_id` を必須引数にする**

```rust
// ✅ Good - user_id を書き忘れられない
repo::portfolio::get_by_user(pool, user_id).await?

// ❌ Bad - ハンドラで直接書かない
sqlx::query("SELECT ... FROM portfolios WHERE user_id = $1")
    .bind(user_id)
    .fetch_one(pool).await?
```

**複合操作（複数テーブル更新）は repo 関数内でトランザクションを閉じる**

呼び出し側はトランザクション境界を意識せず、repo 関数が atomic な単位として振る舞う。

### 2. `sqlx::query` / `query_as` は関数版（ランタイム検証）を使う

現状はコンパイル時検証の `query!` / `query_as!` マクロを**使っていない**。

**理由**: マクロ版は `DATABASE_URL` がコンパイル時に必須 or `.sqlx/` キャッシュの運用が必要で、個人開発フェーズではオーバーヘッドが大きい。

**将来の移行判断**
- コントリビューターが複数になったタイミング
- CI でビルド時 SQL 検証を入れたくなったタイミング

移行する際は `cargo sqlx prepare` → `.sqlx/` を commit、の運用で済む。

### 3. 認可は handler ＋ repo の二段で守る

- handler: `AuthUser` 抽出子で認証済みユーザーを取得
- repo: user スコープ関数で `user_id` を必ず WHERE に入れる

**RLS は現状導入しない**。理由は `Phase 3 以降のプラットフォーム連携で導入検討`（[#16](https://github.com/ttatsato/yokogushi/issues/16) 参照）。

### 4. マイグレーション

- ファイル名: `NNNN_description.sql`（ゼロ埋め4桁）
- **破壊的変更**（DROP/ALTER で既存データに影響）は開発フェーズのみ許容
- 本番運用開始後は additive-only（追加のみ）を原則とする

### 5. ドメインロジックは `yokogushi-core` に置く

- 純粋関数（I/Oなし）なのでテストが書きやすい
- WASM化して将来クライアント共有できる設計を保つ
- `api` crate からは依存して使う、逆はしない

### 6. エラー処理

- handler は型付きエラー（`ApiError` / `AuthError` / `ProfileError`）を返す
- `IntoResponse` で HTTP ステータスに変換、ログは `tracing::error!` で出す
- repo 層は `sqlx::Result` または `anyhow::Result`、handler 境界で変換

---

## 起動方法

### 1. PostgreSQL（Docker）

ポート: **5434**（ホスト側。コンテナ内は標準の5432）

```bash
docker compose up -d
docker compose ps                       # 状態確認
docker compose logs -f postgres         # ログ
docker compose down                     # 停止
docker compose down -v                  # データごと削除
```

接続情報

```
host: localhost
port: 5434
user: yokogushi
pass: yokogushi
db:   yokogushi
```

psql で入る:

```bash
docker exec -it yokogushi-postgres psql -U yokogushi -d yokogushi
```

### 2. APIサーバー（Rust / axum）

ポート: **3001**

`.env` を作成（任意。未設定でもデフォルト値で動作）

```bash
cp .env.example .env
```

起動

```bash
cargo build                  # 初回ビルド
cargo run -p yokogushi-api   # 起動 (起動時に sqlx migrate を自動実行)
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
| GET | `/api/auth/github/login` | GitHub OAuth 認可フローへリダイレクト |
| GET | `/api/auth/github/callback` | OAuth コールバック（自動処理） |
| POST | `/api/auth/logout` | ログアウト（セッション削除） |
| GET | `/api/me` | 現在のユーザー情報（未ログインは401） |
| GET | `/api/me/portfolio` | 自分のポートフォリオ取得 |
| POST | `/api/portfolios` | 自分のポートフォリオ保存（**要ログイン**） |
| GET | `/api/portfolios/:id` | ポートフォリオ取得（公開） |
| GET | `/api/portfolios/:id/skill-experience` | スキル経験集計 |

### GitHub OAuth Appの設定

1. https://github.com/settings/developers → **New OAuth App**
2. 以下を設定
   - **Homepage URL**: `http://localhost:3000`
   - **Authorization callback URL**: `http://localhost:3000/api/auth/github/callback`
3. 発行された Client ID / Client Secret を `.env` に転記

```bash
cp .env.example .env
# .env を編集して GITHUB_CLIENT_ID / GITHUB_CLIENT_SECRET を設定
```

コールバックURLは**Next.jsのリライト経由**（3000）にしているため、Cookieが同一オリジンで自然に流れます。

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
