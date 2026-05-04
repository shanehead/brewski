# Brewski

Homebrewing recipe manager — Tauri 2 desktop app with a SvelteKit frontend and SQLite backend.

## Tech stack

- **Frontend:** SvelteKit + Svelte 5 + TypeScript + TailwindCSS 4
- **Backend:** Rust (Tauri 2 command handlers), SQLite via sqlx
- **IPC:** Tauri `invoke()` — see `src/lib/api.ts` for all typed wrappers

## Package manager

Use **bun** — not npm or npx.

```bash
bun install        # install deps
bun run dev        # frontend only
bunx some-cli      # run a package binary
```

## Common commands (via Justfile)

```bash
just dev           # Tauri dev server (frontend + backend)
just dev-web       # frontend only
just build         # release build
just check         # TypeScript + OpenAPI lint
just lint-openapi  # Redocly lint of docs/openapi.yaml
just preview-docs  # render API docs in browser
```

## API spec

The Tauri IPC interface is documented as an OpenAPI 3.1 spec at `docs/openapi.yaml`.
Lint it with `just lint-openapi` before committing changes to it.
