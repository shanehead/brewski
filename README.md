<p align="center">
  <img src="logo.png" alt="Brewski" width="320"/>
</p>

# Brewski

A homebrewing recipe manager — native desktop app for crafting, organizing, and refining beer recipes.

Built with Tauri 2, SvelteKit, and Rust. Runs on macOS, iOS, Android, Windows, and Linux.

## Architecture

<p align="center">
  <img src="docs/architecture.svg" alt="Architecture diagram" width="580"/>
</p>

The frontend is a SvelteKit app running inside Tauri's WebView. All backend access goes through Tauri's IPC bridge via `tauri.invoke()` — typed wrappers live in [`src/lib/api.gen.ts`](src/lib/api.gen.ts). The Rust side handles commands, delegates to a repository layer, and persists everything to an embedded SQLite database via SeaORM and sqlx.

C4 diagrams (System Context, Container, and Component levels) are in [`docs/c4.md`](docs/c4.md).

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
bun install        # install frontend dependencies
just dev           # Tauri dev server (frontend + backend)
just dev-web       # frontend only (no Rust compilation)
just dev-ios       # iOS simulator (set IOS_SIMULATOR env var to override device)
just dev-android   # Android emulator
```

### Build

```bash
just build           # release build for current platform (no bundling)
just build-macos     # macOS universal binary (arm64 + x86_64)
just build-ios       # iOS IPA
just build-android   # Android APK + AAB
just build-windows   # Windows installer
just build-linux     # Linux packages (.deb, .AppImage, .rpm)
just build-all       # all platforms
```

### Other commands

```bash
just check          # TypeScript check + OpenAPI lint
just test           # Rust + frontend tests
just lint-openapi   # validate docs/openapi/openapi.yaml
just preview-docs   # render API docs in a browser
```

## CI / CD

GitHub Actions workflows live in [`.github/workflows/`](.github/workflows/):

| Workflow | Trigger | What it does |
|---|---|---|
| `ci.yml` | Push to `main`, pull requests | Type check, OpenAPI lint, frontend tests, Rust tests |
| `release.yml` | Push a `v*` tag | Builds all platforms in parallel, uploads artifacts to a draft GitHub Release |

Release jobs are gated on the `release` environment — add signing secrets there (Apple certificates, Android keystore) before tagging a release.

## Database location

Brewski stores its SQLite database in the OS app data directory:

| Platform | Path |
|---|---|
| macOS | `~/Library/Application Support/brewski/brewski.db` |
| Windows | `%APPDATA%\brewski\brewski.db` |
| Linux | `~/.local/share/brewski/brewski.db` |

## API

The full Tauri IPC interface is documented as an OpenAPI 3.1 spec at [`docs/openapi/openapi.yaml`](docs/openapi/openapi.yaml).

## Contributing

Commits follow [Conventional Commits](https://www.conventionalcommits.org/) style: `type(scope): description` (e.g. `feat(recipes): add clone command`, `fix(ibu): correct rager formula`).

## Data

Hop, fermentable, and yeast library data sourced from [BeerMaverick](https://beermaverick.com).

## License

MIT — see [LICENSE](LICENSE).
