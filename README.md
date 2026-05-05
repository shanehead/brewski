<p align="center">
  <img src="logo.png" alt="Brewski" width="320"/>
</p>

# Brewski

A homebrewing recipe manager — native desktop app for crafting, organizing, and refining beer recipes.

Built with Tauri 2, SvelteKit, and Rust. Runs on macOS, Windows, and Linux.

## Architecture

<p align="center">
  <img src="docs/architecture.svg" alt="Architecture diagram" width="580"/>
</p>

The frontend is a SvelteKit app running inside Tauri's WebView. All backend access goes through Tauri's IPC bridge via `tauri.invoke()` — typed wrappers live in [`src/lib/api.ts`](src/lib/api.ts). The Rust side handles commands, delegates to a repository layer, and persists everything to an embedded SQLite database via SeaORM and sqlx.

## Tech Stack

| Layer | Technology |
|---|---|
| Frontend | SvelteKit · Svelte 5 · TypeScript · TailwindCSS 4 |
| Backend | Rust · Tauri 2 |
| IPC | `tauri.invoke()` |
| ORM | SeaORM · sqlx |
| Database | SQLite (embedded, single file) |

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/)
- [Bun](https://bun.sh/)
- [Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/) for your platform

### Development

```bash
bun install      # install frontend dependencies
just dev         # Tauri dev server (frontend + backend)
just dev-web     # frontend only (no Rust compilation)
```

### Build

```bash
just build       # release build for current platform
```

### Other commands

```bash
just check          # TypeScript check + OpenAPI lint
just lint-openapi   # validate docs/openapi.yaml
just preview-docs   # render API docs in a browser
```

## API

The full Tauri IPC interface is documented as an OpenAPI 3.1 spec at [`docs/openapi.yaml`](docs/openapi.yaml).
