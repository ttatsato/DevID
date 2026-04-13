# DevID — Identity Platform for Software Engineers

**English** | [日本語](docs/README.ja.md)

> **"Register once, use everywhere."**
> An open-core IDaaS and profile API platform built for software engineers and job platforms.

---

## The Problem

Engineers re-enter the same information every time they sign up for a new job platform.  
Job platforms rebuild registration and resume features from scratch.  
Despite all that effort, **60–70% of registered users never complete their profiles.**

```
Today:
Engineer → Job Platform A (fill out resume)
Engineer → Job Platform B (fill it out again)
Engineer → Job Platform C (fill it out again)
              ↑ Same data entered 3 times, 2 sites go stale

With DevID:
Engineer → Sign in with GitHub (once)
                    ↓
     Platform A · Platform B · Platform C
     All reference the same up-to-date profile
```

---

## What DevID Provides

DevID is an **IDaaS (Identity as a Service) purpose-built for software engineers**.  
It offers job platforms a single API for authentication, profile data, and skill verification — so they don't have to build these from scratch.

### For Engineers
- **Unified portfolio**: Sign in with GitHub and your profile is automatically generated
- **Own your data**: Fine-grained control over what each service can see
- **Update once, sync everywhere** *(Phase 2)*: Changes propagate to all connected services automatically

### For Job Platforms
- **Rich profiles from day one**: Structured skill data exists even for users who haven't typed a single character
- **Drop-in authentication** *(Phase 2)*: Integrate DevID as an IdP via OIDC in hours, not weeks
- **Real-time webhooks** *(Phase 2)*: Get notified the moment a candidate updates their profile

---

## Architecture

```
┌─────────────────────────────────────────────────┐
│                  DevID Platform                  │
│                                                  │
│  ┌─────────────┐   ┌──────────────────────────┐  │
│  │  Auth Layer  │   │   Profile API (Rust)     │  │
│  │  (OIDC/     │◄──│  - GitHub data ingestion  │  │
│  │   OAuth2)   │   │  - Skill schema           │  │
│  └─────────────┘   │  - Webhook dispatcher     │  │
│                    └──────────────────────────┘  │
│                              │                   │
│                   ┌──────────────────┐            │
│                   │   PostgreSQL     │            │
│                   │   + Redis        │            │
│                   └──────────────────┘            │
└─────────────────────────────────────────────────┘
          ▲                        ▲
     Platform A               Platform B
    (MAU-based billing)     (MAU-based billing)
```

**Built with Rust** for the performance demands of real-time profile sync and high-volume webhook delivery.

## Roadmap

### Phase 1 — Foundation (current)
- [ ] GitHub OAuth integration
- [ ] Auto-generate profiles from GitHub data
- [ ] Basic profile CRUD API
- [ ] Local dev environment via Docker Compose

### Phase 2 — Platform
- [ ] OIDC server (job platforms use DevID as an IdP)
- [ ] Webhook dispatcher with delivery guarantees
- [ ] OpenAPI spec + TypeScript SDK for job platforms
- [ ] Skill schema RFC (open community discussion)

### Phase 3 — Ecosystem
- [ ] Zenn / Qiita / Stack Overflow integrations
- [ ] Managed cloud hosting
- [ ] Analytics dashboard for job platforms

---

## Contributing

We're looking for global expertise in these areas:

- **Skill schema design** — What should a universal skill taxonomy look like? ([Join the RFC discussion →](./docs/rfcs/))
- **Regional platform integrations** — What services matter in your region?
- **Security review** — The auth layer needs many eyes

See [CONTRIBUTING.md](./CONTRIBUTING.md) for details.

---

## RFC Process

All major design decisions are made in the open.  
Proposals go to `docs/rfcs/` for community discussion before implementation.

Open RFCs:
- `RFC-001`: Skill Schema v1 Design
- `RFC-002`: Webhook Delivery Guarantees (at-least-once vs exactly-once)

---

## License

Core (`crates/`) — Apache 2.0  
Cloud-only features (`cloud/`) — Proprietary

---

## Why Rust?

- **Webhook throughput**: Parallel delivery to hundreds of platforms demands async performance
- **Type safety**: Profile schema consistency enforced at compile time, not runtime
- **Long-term reliability**: An auth platform needs memory safety without GC pauses
