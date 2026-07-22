# Architecture — starter-fullstack-rust

## Opinionated choices

### 1. Axum over Actix

Axum aligns with the tokio + tower ecosystem and has become the de-facto standard for new Rust HTTP services. Its extractor-based API is easy for beginners while remaining flexible.

### 2. sqlx over Diesel or SeaORM

- Raw SQL kept close to the code (no query DSL to learn).
- Compile-time query checking (via `sqlx::query!`) when a live DB is available; falls back to `SQLX_OFFLINE=true` for CI.
- Async-native, no blocking pool.

### 3. SvelteKit 5 with runes

- Reactive primitives are explicit (`$state`, `$derived`, `$effect`) — easier to reason about than legacy stores.
- Zero-config TypeScript.
- Server- and client-side rendering in the same file, with the option to add server actions later.

### 4. Docker Compose as the single entrypoint

- One `docker compose up` covers db + backend + frontend + adminer.
- Same commands work on Linux, macOS, and Windows (via WSL2).

### 5. i18n via a runes-backed class

- No i18n library dependency.
- Language state is a single `$state`-holding class; components read `lang.t.<key>`.
- Fine for starter-scale apps; swap in `svelte-i18n` if you outgrow it.

## What's out of scope for the starter

- Authentication (add your own auth crate, e.g. `axum-login`, JWT, or an external IdP).
- Observability beyond `tracing` (no Prometheus/OTel here — see `starter-devops`).
- Multi-tenancy / RLS.
- Rate limiting / anti-abuse.

## Suggested first extensions

- Add auth: `axum-login` with a `users` table + session cookie.
- Add a background job queue: `apalis` or `sqlx-based-outbox` for lightweight cases.
- Add OpenAPI: `utoipa` + `swagger-ui`.
