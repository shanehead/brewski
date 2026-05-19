set dotenv-load

default:
    @just --list

# ── Dev ───────────────────────────────────────────────────────────────────────

# Configure Git to use the repo-managed hooks in .githooks/
install-hooks:
    git config core.hooksPath .githooks

# Start the Tauri dev server
dev:
    bun run tauri dev

# Start the Tauri iOS dev server
dev-ios device=env_var_or_default('IOS_SIMULATOR', 'iPhone 17'):
    xcrun simctl boot "{{device}}" 2>/dev/null || true
    $BUN run tauri ios dev "{{device}}"

# Start the Tauri Android dev server
dev-android:
    bun run tauri android dev 2>&1 

# Start the frontend dev server only (no Tauri)
dev-web:
    bun run dev

# ── Build ─────────────────────────────────────────────────────────────────────

# Build the app for release
build:
    bun run tauri build --no-bundle

# ── Check & Lint ──────────────────────────────────────────────────────────────

# Run all checks (TypeScript + OpenAPI)
check: check-ts lint-openapi

# TypeScript / Svelte type check
check-ts:
    bun run check

# Lint the OpenAPI spec with Redocly
lint-openapi:
    bunx redocly lint

# Preview the OpenAPI docs in a browser
preview-docs:
    bunx redocly build-docs docs/openapi/openapi.yaml --output docs/openapi.html && open docs/openapi.html

# Regenerate TypeScript and Rust types from the OpenAPI spec
gen: gen-ts gen-rust

gen-ts:
    bunx redocly bundle docs/openapi/openapi.yaml -o /tmp/brewski-bundled.yaml
    bunx openapi-typescript /tmp/brewski-bundled.yaml -o src/lib/api.gen.ts

gen-rust:
    bunx redocly bundle docs/openapi/openapi.yaml -o /tmp/brewski-bundled.yaml
    bun scripts/extract-schemas.mjs /tmp/brewski-bundled.yaml /tmp/brewski-schemas.json
    cargo typify /tmp/brewski-schemas.json -o src-tauri/src/models.gen.rs
    cargo fmt --manifest-path src-tauri/Cargo.toml

# ── Test & Coverage ───────────────────────────────────────────────────────────

# Run all tests (Rust + frontend)
test: test-rust test-frontend

# Run Rust tests
test-rust:
    cd src-tauri && cargo test

# Run frontend unit tests
test-frontend:
    bun run test

# Show Rust coverage report (excludes Tauri IPC glue and generated entity code)
coverage:
    cd src-tauri && cargo llvm-cov \
        --ignore-filename-regex '(commands/|entities/|migration/|bin/|main\.rs|lib\.rs)' \
        --fail-under-lines 95

# ── Database ──────────────────────────────────────────────────────────────────

# Apply SeaORM migrations to a local dev database
migrate:
    cargo run --manifest-path src-tauri/Cargo.toml --bin migrate -- sqlite://./dev.db?mode=rwc

seed-styles:
    bun scripts/seed-styles.mjs
    just migrate

seed-equipment:
    bun scripts/seed-equipment.mjs
    just migrate

# Regenerate SeaORM entities from the dev database (runs migrate first)
gen-entities: migrate
    sea-orm-cli generate entity \
      --database-url sqlite://./dev.db \
      --output-dir src-tauri/src/entities \
      --with-serde both \
      --serde-skip-hidden-column
    bash scripts/fix-entities.sh src-tauri/src/entities
