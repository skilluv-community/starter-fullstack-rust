# starter-fullstack-rust

> A Skilluv starter template — Rust + Axum backend, SvelteKit frontend, PostgreSQL.

[![CI](https://github.com/skilluv-community/starter-fullstack-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/skilluv-community/starter-fullstack-rust/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Skilluv](https://img.shields.io/badge/skilluv-community-emerald)](https://skilluv.io)

## English

### What this is

A production-ready starting point for a fullstack app built with the Skilluv signature stack:

- **Backend**: Rust (Axum, sqlx, PostgreSQL 18)
- **Frontend**: SvelteKit 5 (runes) + Tailwind
- **Orchestration**: Docker Compose (postgres + backend + frontend + adminer)
- **Tests**: `cargo test` + Vitest + Playwright
- **CI**: GitHub Actions

Clone, `cp .env.example .env`, `docker compose up`. That's it.

### Quickstart

```bash
git clone git@github.com:skilluv-community/starter-fullstack-rust.git
cd starter-fullstack-rust
cp .env.example .env
docker compose up --build
```

- Frontend: <http://localhost:5173>
- Backend: <http://localhost:3001/health>
- Adminer (DB): <http://localhost:8080>

### Structure

```
backend/     Axum server + sqlx migrations + integration tests
frontend/    SvelteKit app (Svelte 5 runes, Tailwind)
docs/        Bilingual docs (fr, en)
.github/     CI + dependabot + PR template
```

### What's inside

- `GET /health` — liveness probe
- `GET /api/hello?name=Ada` — dummy greeting endpoint
- `GET/POST/DELETE /api/notes` — CRUD demo backed by PostgreSQL
- SvelteKit pages `/` (hello demo) and `/notes` (CRUD demo)
- FR/EN i18n via a runes-backed store

### Docs

- [`docs/en/getting-started.md`](./docs/en/getting-started.md)
- [`docs/en/architecture.md`](./docs/en/architecture.md)

---

## Français

### C'est quoi

Un point de départ prêt-à-l'emploi pour une app fullstack basée sur la stack signature Skilluv :

- **Backend** : Rust (Axum, sqlx, PostgreSQL 18)
- **Frontend** : SvelteKit 5 (runes) + Tailwind
- **Orchestration** : Docker Compose (postgres + backend + frontend + adminer)
- **Tests** : `cargo test` + Vitest + Playwright
- **CI** : GitHub Actions

Clone, `cp .env.example .env`, `docker compose up`. C'est tout.

### Démarrage rapide

```bash
git clone git@github.com:skilluv-community/starter-fullstack-rust.git
cd starter-fullstack-rust
cp .env.example .env
docker compose up --build
```

- Frontend : <http://localhost:5173>
- Backend : <http://localhost:3001/health>
- Adminer (DB) : <http://localhost:8080>

### Structure

```
backend/     Serveur Axum + migrations sqlx + tests d'intégration
frontend/    App SvelteKit (Svelte 5 runes, Tailwind)
docs/        Documentation bilingue (fr, en)
.github/     CI + dependabot + template PR
```

### Ce qu'il y a dedans

- `GET /health` — liveness probe
- `GET /api/hello?name=Ada` — endpoint de salutation
- `GET/POST/DELETE /api/notes` — démo CRUD sur PostgreSQL
- Pages SvelteKit `/` (démo hello) et `/notes` (démo CRUD)
- i18n FR/EN via un store runes

### Docs

- [`docs/fr/getting-started.md`](./docs/fr/getting-started.md)
- [`docs/fr/architecture.md`](./docs/fr/architecture.md)

---

## License

MIT — see [LICENSE](./LICENSE).

## Related

- [Skilluv](https://skilluv.io) — the platform this template is designed for
- [Skilluv Community Charter](https://github.com/skilluv-community/community-charter)
- [Other starters](https://github.com/orgs/skilluv-community/repositories?q=starter)
