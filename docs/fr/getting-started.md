# Démarrage — starter-fullstack-rust

## Prérequis

- Docker + Docker Compose 2.24+
- (Optionnel, pour dev local sans Docker) Rust 1.80+, Node 24+, PostgreSQL 18

## Premier lancement

```bash
git clone git@github.com:skilluv-community/starter-fullstack-rust.git
cd starter-fullstack-rust
cp .env.example .env
docker compose up --build
```

Le premier build prend ~3-5 minutes (compilation Rust + npm install). Les lancements suivants sont en cache.

Une fois lancé :

- Frontend : <http://localhost:5173>
- Health backend : <http://localhost:3001/health>
- Adminer (UI DB) : <http://localhost:8080> (serveur `postgres`, user `skilluv`, mot de passe `skilluv`, base `skilluv`)

## Modifier le code

- **Backend** (`backend/`) : éditer `backend/src/`. En dev, redémarrer le conteneur (`docker compose restart backend`) ou rebuild.
- **Frontend** (`frontend/`) : HMR Vite actif, les changements se rechargent automatiquement.
- **Schéma DB** : ajouter une migration `backend/migrations/000X_*.sql`. `sqlx::migrate!` l'applique au démarrage.

## Lancer les tests

```bash
make test
```

Les tests unitaires (backend + frontend) tournent sans service externe. Pour les tests d'intégration backend qui touchent la DB :

```bash
docker compose up -d postgres backend
cd backend && cargo test -- --ignored
```

## Déploiement

Ce starter cible un déploiement self-hosted via [Coolify](https://coolify.io/). Pointer Coolify sur le repo et sa recette Docker Compose buildera les deux services.

Pour la prod, au minimum changer :

- `POSTGRES_PASSWORD` par un vrai secret
- `PUBLIC_API_BASE` vers l'URL publique du backend
- Restreindre le CORS dans `backend/src/main.rs`
