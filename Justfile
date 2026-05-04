default:
    @just --list

# ── Dev ───────────────────────────────────────────────────────────────────────

# Start the Tauri dev server
dev:
    npm run tauri dev

# Start the frontend dev server only (no Tauri)
dev-web:
    npm run dev

# ── Build ─────────────────────────────────────────────────────────────────────

# Build the app for release
build:
    npm run tauri build

# ── Check & Lint ──────────────────────────────────────────────────────────────

# Run all checks (TypeScript + OpenAPI)
check: check-ts lint-openapi

# TypeScript / Svelte type check
check-ts:
    npm run check

# Lint the OpenAPI spec with Redocly
lint-openapi:
    npx redocly lint

# Preview the OpenAPI docs in a browser
preview-docs:
    npx redocly preview-docs docs/openapi.yaml
