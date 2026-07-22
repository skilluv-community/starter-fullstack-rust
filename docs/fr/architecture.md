# Architecture — starter-fullstack-rust

## Choix opinionated

### 1. Axum plutôt qu'Actix

Axum est aligné avec l'écosystème tokio + tower et est devenu le standard de fait pour les nouveaux services HTTP Rust. Son API à base d'extractors est simple pour débuter tout en restant flexible.

### 2. sqlx plutôt que Diesel ou SeaORM

- Le SQL brut reste dans le code (pas de DSL à apprendre).
- Vérification des requêtes à la compilation (via `sqlx::query!`) quand une DB live est dispo ; fallback `SQLX_OFFLINE=true` en CI.
- Async natif, pas de pool bloquant.

### 3. SvelteKit 5 avec runes

- Primitives réactives explicites (`$state`, `$derived`, `$effect`) — plus faciles à raisonner que les stores legacy.
- TypeScript zéro-config.
- SSR et CSR dans le même fichier, avec possibilité d'ajouter des server actions.

### 4. Docker Compose comme point d'entrée unique

- Un seul `docker compose up` couvre db + backend + frontend + adminer.
- Mêmes commandes sur Linux, macOS, Windows (via WSL2).

### 5. i18n via une classe runes

- Zéro dépendance i18n.
- L'état langue est un singleton `$state` ; les composants lisent `lang.t.<key>`.
- Suffisant à l'échelle starter ; migrer vers `svelte-i18n` si nécessaire.

## Hors scope du starter

- Authentification (ajouter `axum-login`, JWT, ou un IdP externe).
- Observabilité au-delà de `tracing` (voir `starter-devops`).
- Multi-tenancy / RLS.
- Rate limiting / anti-abus.

## Extensions suggérées

- Auth : `axum-login` avec table `users` + cookie session.
- Job queue : `apalis` ou une outbox sqlx pour les cas légers.
- OpenAPI : `utoipa` + `swagger-ui`.
