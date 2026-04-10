# DevID — エンジニア向けアイデンティティ基盤

> **「一度登録すれば、どこでも使える。」**
> 転職サイト向けの、オープンソースなエンジニア特化IDaaS／プロファイルAPI基盤

---

## 解決する問題

エンジニアは転職サイトに登録するたびに、同じ情報を何度も入力し直している。  
転職サイトは会員登録・レジュメ機能をゼロから作り続けている。  
そのすべての努力にもかかわらず、**登録ユーザーの60〜70%がレジュメを未入力のまま放置している。**

```
現状:
エンジニア → 転職サイトA（レジュメ入力）
エンジニア → 転職サイトB（また入力）
エンジニア → 転職サイトC（また入力）
                ↑ 同じデータを3回入力、2サイトは古いまま

DevIDを使えば:
エンジニア → GitHubでログイン（一度だけ）
                    ↓
     サイトA ・ サイトB ・ サイトC
     すべてが同じ最新プロファイルを参照できる
```

---

## DevIDが提供するもの

DevIDは**ソフトウェアエンジニア特化のIDaaS（Identity as a Service）**です。  
転職サイトに対して、認証・プロファイルデータ・スキル検証をまとめて提供するAPIを一本提供します。  
転職サイト側はこれをゼロから構築する必要がなくなります。

### エンジニアにとって
- **入力ゼロのオンボーディング**: GitHubでログインするだけでプロファイルが自動生成される
- **一度更新すれば全サイトに反映**: スキルを変更すると、連携中のすべてのサービスに即座に同期
- **データは自分のもの**: どのサービスに何を公開するかを細かく制御できる

### 転職サイトにとって
- **すぐに使える認証**: OAuth 2.0 / OIDC連携を数時間で実装できる
- **登録初日から豊富なプロファイル**: 一文字も入力していないユーザーにも構造化されたスキルデータが存在する
- **Webhookによるリアルタイム通知**: 候補者のプロファイルが更新された瞬間に通知を受け取れる

---

## GitHubログインがレジュメ未入力問題を解決する理由

GitHubのOAuthログイン一回で取得できるデータ:

| データ | 取得元 | プロファイルでの意味 |
|---|---|---|
| 主要使用言語 | リポジトリ解析 | スキル（自動検出） |
| コントリビューション数 | GitHub API | 活動量 |
| 公開リポジトリ | GitHub API | ポートフォリオ |
| スター獲得数 | GitHub API | コミュニティからの評価 |
| 所属Organization | GitHub API | 職歴のヒント |
| ピン留めリポジトリ | GitHub API | 本人が推す代表作 |

**エンジニアが一文字も入力しなくても、プロファイルの80%が自動で埋まる。**

---

## アーキテクチャ

```
┌─────────────────────────────────────────────────┐
│                  DevID Platform                  │
│                                                  │
│  ┌─────────────┐   ┌──────────────────────────┐  │
│  │  認証レイヤー │   │   プロファイルAPI (Rust)  │  │
│  │  (OIDC/     │◄──│  - GitHubデータ取込       │  │
│  │   OAuth2)   │   │  - スキルスキーマ          │  │
│  └─────────────┘   │  - Webhookディスパッチャー │  │
│                    └──────────────────────────┘  │
│                              │                   │
│                   ┌──────────────────┐            │
│                   │   PostgreSQL     │            │
│                   │   + Redis        │            │
│                   └──────────────────┘            │
└─────────────────────────────────────────────────┘
          ▲                        ▲
     転職サイトA               転職サイトB
    （MAU課金）               （MAU課金）
```

**Rustで実装**。リアルタイムプロファイル同期と大量Webhook配信のパフォーマンス要件に対応するため。

---

## コアコンセプト

### 共通プロファイルスキーマ
エンジニアプロファイルのための、標準化されたオープンスキーマ。  
スキル・職歴・コントリビューションを、どの転職サイトでも扱えるフォーマットで表現する。

```json
{
  "id": "uuid",
  "github_handle": "...",
  "skills": [
    { "name": "Rust", "level": "advanced", "years": 3, "source": "github_detected" },
    { "name": "PostgreSQL", "level": "intermediate", "source": "self_reported" }
  ],
  "public_contributions": 847,
  "top_repositories": [...],
  "updated_at": "2026-04-10T00:00:00Z"
}
```

### Webhook
プロファイルが変更されると、連携中の全プラットフォームにリアルタイム通知を送信:

```json
{
  "event": "profile.updated",
  "user_id": "...",
  "updated_fields": ["skills", "top_repositories"],
  "timestamp": 1712707200
}
```

---

## リポジトリ構成

```
devid/
├── crates/
│   ├── core/          # ドメインロジック、プロファイルスキーマ定義
│   ├── api/           # HTTPサーバー (axum)
│   ├── webhook/       # Webhookディスパッチャー
│   └── schema/        # 共通スキルスキーマ（オープン標準）
├── integrations/
│   ├── github/        # GitHub OAuth + データ取込
│   ├── zenn/          # Zenn記事集約（予定）
│   └── qiita/         # Qiita集約（予定）
├── docs/
│   ├── rfcs/          # 設計判断をRFCとして公開
│   └── api/           # OpenAPI仕様
└── docker-compose.yml # 5分でローカル起動
```

---

## ビジネスモデル

DevIDは **Open Core** モデルを採用:

| | OSS（セルフホスト） | クラウド（有料） |
|---|---|---|
| 認証・プロファイルAPIコア | ✅ | ✅ |
| GitHub連携 | ✅ | ✅ |
| Webhook配信 | ✅ | ✅ |
| マルチテナント管理 | ❌ | ✅ |
| SLA / 稼働率保証 | ❌ | ✅ |
| 監査ログ | ❌ | ✅ |
| 高度な分析機能 | ❌ | ✅ |
| サポート | コミュニティ | 専任 |

**クラウド版の課金軸はMAU（月間アクティブユーザー数）** — 転職サイト側の得る価値と連動する。

---

## 競合との比較

| | DevID | LinkedIn | Okta/Auth0 | 一般ATS |
|---|---|---|---|---|
| エンジニア特化スキーマ | ✅ | ❌ | ❌ | ❌ |
| オープンソース | ✅ | ❌ | ❌ | △ |
| GitHub自動プロファイル生成 | ✅ | ❌ | ❌ | ❌ |
| 第三者転職サイト向けAPI | ✅ | 制限あり | ❌ | ❌ |
| セルフホスト可能 | ✅ | ❌ | △ | △ |

---

## ロードマップ

### Phase 1 — 基盤構築（現在）
- [ ] GitHub OAuth連携
- [ ] GitHubデータからの自動プロファイル生成
- [ ] 基本プロファイルCRUD API
- [ ] Docker Composeでのローカル起動環境

### Phase 2 — プラットフォーム化
- [ ] OIDCサーバー（転職サイトがDevIDをIdPとして利用）
- [ ] 配信保証付きWebhookディスパッチャー
- [ ] OpenAPI仕様 + 転職サイト向けTypeScript SDK
- [ ] スキルスキーマRFC（コミュニティ公開・議論）

### Phase 3 — エコシステム拡大
- [ ] Zenn / Qiita / Stack Overflow連携
- [ ] クラウドホスティング版リリース
- [ ] 転職サイト向け分析ダッシュボード

---

## コントリビューション

特に以下の領域で世界中の知見を求めています:

- **スキルスキーマ設計** — グローバルに通用するスキル定義はどうあるべきか？（[RFC議論 →](./docs/rfcs/)）
- **各国のプラットフォーム連携** — あなたの地域で重要なサービスは何か？
- **セキュリティレビュー** — 認証レイヤーは多くの目で確認する必要がある

詳細は [CONTRIBUTING.md](./CONTRIBUTING.md) を参照してください。

---

## RFCプロセス

重要な設計判断はすべてオープンに行います。  
実装前に `docs/rfcs/` にRFCとして提案し、コミュニティで議論します。

現在オープンなRFC:
- `RFC-001`: スキルスキーマ v1 設計
- `RFC-002`: Webhook配信保証（at-least-once vs exactly-once）

---

## ライセンス

コア機能 (`crates/`) — Apache 2.0  
クラウド専用機能 (`cloud/`) — プロプライエタリ

---

## なぜRustか

- **Webhookのスループット**: 数百の転職サイトへの並列配信には非同期処理の性能が必要
- **型安全性**: プロファイルスキーマの整合性をランタイムではなくコンパイル時に保証できる
- **長期的な信頼性**: 認証基盤にはGCポーズのないメモリ安全性が求められる

---

*DevIDはGitHub・LinkedIn・いかなる転職サービスとも無関係です。*  
*すべてをオープンに構築します。フィードバック歓迎。*


# DevID — Engineer Identity Platform

> **"Register once. Apply everywhere."**
> The open-source identity and profile infrastructure for engineer job platforms.

---

## The Problem

Engineers waste hours re-entering the same information across every job site.  
Job platforms waste engineering resources rebuilding the same registration and resume systems from scratch.  
And despite all this effort, **60–70% of registered users never complete their profile** — leaving both sides of the market broken.

```
Today:
Engineer → Site A (fill resume)
Engineer → Site B (fill resume again)
Engineer → Site C (fill resume again)
                ↑ same data, entered 3 times, outdated on 2 of them

With DevID:
Engineer → GitHub Login (once)
              ↓
    Site A · Site B · Site C
    all get the same live profile
```

---

## What DevID Does

DevID is an **Identity-as-a-Service (IDaaS) platform built specifically for software engineers**.  
It gives job platforms a single API to handle authentication, profile data, and skill verification — without building any of it themselves.

### For Engineers
- **Zero-input onboarding**: Log in with GitHub and your profile is built automatically
- **One update, everywhere**: Change your skills once and every connected platform sees it instantly
- **You own your data**: Granular control over what each platform can access

### For Job Platforms
- **Drop-in auth**: OAuth 2.0 / OIDC integration in hours, not weeks
- **Rich profiles on day one**: Access structured skill data even for users who never typed a word
- **Webhook notifications**: Get notified the moment a candidate's profile changes

---

## Why GitHub Login Solves the Resume Problem

A GitHub OAuth login alone yields:

| Data Point | Source | Value |
|---|---|---|
| Primary languages | Repository analysis | Skills (auto-detected) |
| Contribution graph | GitHub API | Activity level |
| Public repositories | GitHub API | Portfolio |
| Stars received | GitHub API | Community recognition |
| Organizations | GitHub API | Work history hints |
| Pinned repos | GitHub API | Self-curated highlights |

**80% of an engineer's profile can be generated before they type a single character.**

---

## Architecture

```
┌─────────────────────────────────────────────────┐
│                   DevID Platform                 │
│                                                  │
│  ┌─────────────┐   ┌──────────────────────────┐  │
│  │  Auth Layer │   │    Profile API (Rust)    │  │
│  │  (OIDC/     │◄──│  - GitHub data ingestion │  │
│  │   OAuth2)   │   │  - Skill schema          │  │
│  └─────────────┘   │  - Webhook dispatcher    │  │
│                    └──────────────────────────┘  │
│                              │                   │
│                   ┌──────────────────┐            │
│                   │   PostgreSQL     │            │
│                   │   + Redis        │            │
│                   └──────────────────┘            │
└─────────────────────────────────────────────────┘
          ▲                        ▲
   Job Platform A           Job Platform B
   (pays per MAU)           (pays per MAU)
```

**Built with Rust** for the performance demands of real-time profile sync and high-throughput webhook delivery.

---

## Core Concepts

### Shared Profile Schema
A standardized, open schema for engineer profiles. Skills, work history, and contributions expressed in a format every platform can consume.

```json
{
  "id": "uuid",
  "github_handle": "...",
  "skills": [
    { "name": "Rust", "level": "advanced", "years": 3, "source": "github_detected" },
    { "name": "PostgreSQL", "level": "intermediate", "source": "self_reported" }
  ],
  "public_contributions": 847,
  "top_repositories": [...],
  "updated_at": "2026-04-10T00:00:00Z"
}
```

### Webhooks
Every connected platform receives real-time notifications when a profile changes:

```json
{
  "event": "profile.updated",
  "user_id": "...",
  "updated_fields": ["skills", "top_repositories"],
  "timestamp": 1712707200
}
```

---

## Repository Structure

```
devid/
├── crates/
│   ├── core/          # Domain logic, profile schema definitions
│   ├── api/           # HTTP API server (axum)
│   ├── webhook/       # Webhook dispatcher
│   └── schema/        # Shared skill schema (open standard)
├── integrations/
│   ├── github/        # GitHub OAuth + data ingestion
│   ├── zenn/          # Zenn article aggregation (planned)
│   └── qiita/         # Qiita aggregation (planned)
├── docs/
│   ├── rfcs/          # Design decisions as open RFCs
│   └── api/           # OpenAPI spec
└── docker-compose.yml # Run locally in 5 minutes
```

---

## Business Model

DevID follows the **Open Core** model:

| | OSS (Self-host) | Cloud (Paid) |
|---|---|---|
| Core auth & profile API | ✅ | ✅ |
| GitHub integration | ✅ | ✅ |
| Webhook delivery | ✅ | ✅ |
| Multi-tenant management | ❌ | ✅ |
| SLA / uptime guarantee | ❌ | ✅ |
| Audit logs | ❌ | ✅ |
| Advanced analytics | ❌ | ✅ |
| Support | Community | Dedicated |

**Cloud pricing is per MAU (Monthly Active Users)** — aligned with the value delivered to job platforms.

---

## Competitive Landscape

| | DevID | LinkedIn | Okta/Auth0 | Generic job ATS |
|---|---|---|---|---|
| Engineer-specific schema | ✅ | ❌ | ❌ | ❌ |
| Open source | ✅ | ❌ | ❌ | △ |
| GitHub auto-profile | ✅ | ❌ | ❌ | ❌ |
| API for 3rd-party job sites | ✅ | Limited | ❌ | ❌ |
| Self-hostable | ✅ | ❌ | △ | △ |

---

## Roadmap

### Phase 1 — Foundation (Now)
- [ ] GitHub OAuth integration
- [ ] Auto-profile generation from GitHub data
- [ ] Basic profile CRUD API
- [ ] Docker Compose local setup

### Phase 2 — Platform
- [ ] OIDC server (job platforms can use DevID as IdP)
- [ ] Webhook dispatcher with delivery guarantees
- [ ] OpenAPI spec + TypeScript SDK for job platforms
- [ ] Skill schema RFC (open for community input)

### Phase 3 — Ecosystem
- [ ] Zenn / Qiita / Stack Overflow integrations
- [ ] Cloud-hosted offering
- [ ] Analytics dashboard for job platforms

---

## Contributing

We especially need input on:

- **Skill schema design** — How should skills be structured to work globally? ([RFC discussion →](./docs/rfcs/))
- **Additional integrations** — What platforms matter in your region?
- **Security review** — The auth layer needs eyes

See [CONTRIBUTING.md](./CONTRIBUTING.md) to get started.

---

## RFC Process

Major design decisions are made openly.  
Before implementation, significant changes are proposed as RFCs in `docs/rfcs/`.

Current open RFCs:
- `RFC-001`: Skill Schema v1 Design
- `RFC-002`: Webhook delivery guarantees (at-least-once vs exactly-once)

---

## License

Core (`crates/`) — Apache 2.0  
Cloud features (`cloud/`) — Proprietary

---

## Why Rust?

- **Webhook throughput**: Dispatching to hundreds of job platforms concurrently demands async performance
- **Type safety**: The profile schema is enforced at compile time, not just at runtime
- **Long-term reliability**: Memory safety without GC pauses matters for an identity-critical service

---

*DevID is not affiliated with GitHub, LinkedIn, or any job platform.*  
*Built in public. Feedback welcome.*
