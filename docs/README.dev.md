# Yokogushi 開発環境

## 必要なもの

Rust 1.75+ / Node.js 20+ / Docker

初回のみ

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
cp .env.example .env
# .env に GITHUB_CLIENT_ID / GITHUB_CLIENT_SECRET を設定
```

## 起動

```bash
docker compose up -d          # Postgres (5434)
cargo run -p yokogushi-api    # API (3001)
cd web && npm run dev         # Frontend (3000)
```

http://localhost:3000 を開く。

## 開発ルール

- **SQLはマクロ版** (`sqlx::query!` / `query_as!`) を使う。SQL追加・変更時は必ず `cargo sqlx prepare --workspace` を実行して `.sqlx/` を commit
- **DB操作は `repo` 層に集約**。ハンドラから直接 `sqlx::query!` を呼ばない
- **repo 関数命名**: `Option<T>` → `find_`、`Vec<T>` → `list_`、`bool` → `exists_`、書き込みは動詞 (`upsert_for_user`)。ユーザースコープは `_by_user` / `_for_user` サフィックスで `user_id` を必須引数に
- **ドメインロジックは `yokogushi-core`**（I/O なしの純粋関数）
- **マイグレーション**: `sqlx migrate add <name>`

## コマンド

```bash
cargo build              # DATABASE_URL があればオンライン検証
SQLX_OFFLINE=true cargo build   # DB なしでビルド
cargo test
cargo fmt
cargo clippy --all-targets
cd web && npx tsc --noEmit
```

Postgres に入る

```bash
docker exec -it yokogushi-postgres psql -U yokogushi -d yokogushi
```

## GitHub OAuth App 設定

https://github.com/settings/developers → New OAuth App

- Homepage URL: `http://localhost:3000`
- Authorization callback URL: `http://localhost:3000/api/auth/github/callback`

Client ID / Client Secret を `.env` に転記。
