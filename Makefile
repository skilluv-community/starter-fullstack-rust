.PHONY: dev test lint build clean fmt help

help:
	@echo "Targets:"
	@echo "  dev    — start the whole stack (postgres + backend + frontend)"
	@echo "  test   — run backend + frontend tests"
	@echo "  lint   — run clippy + eslint + svelte-check"
	@echo "  build  — production build for both backend and frontend"
	@echo "  fmt    — auto-format code"
	@echo "  clean  — remove build artifacts"

dev:
	docker compose up --build

test:
	cd backend && cargo test --all-features
	cd frontend && npm run test:unit -- --run

lint:
	cd backend && cargo clippy --all-targets -- -D warnings
	cd frontend && npm run lint && npm run check

build:
	cd backend && cargo build --release
	cd frontend && npm run build

fmt:
	cd backend && cargo fmt
	cd frontend && npm run format

clean:
	cd backend && cargo clean
	cd frontend && rm -rf node_modules .svelte-kit build .vite dist
