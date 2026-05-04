default:
    @just --list

# ── Dev ───────────────────────────────────────────────────────────────────────

# Start the Tauri dev server
dev:
    bun run tauri dev

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
    bunx redocly build-docs docs/openapi.yaml --output docs/openapi.html && open docs/openapi.html
