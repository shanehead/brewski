# Brewfather API Comparison

> Analysis date: 2026-05-20  
> Brewfather API reference: https://docs.brewfather.app/api

## Fundamental Difference

Brewfather's API is a **web REST API** (HTTP, Basic Auth, rate-limited, public-facing). Brewski's "API" is **Tauri IPC** — desktop-only, all-POST to `/commands/*`, no auth. These aren't the same kind of thing, which shapes all the options below.

---

## Feature Coverage

| Area | Brewski | Brewfather API |
|------|---------|----------------|
| Recipe CRUD | Full (create/update/delete) | Read-only |
| Ingredient library (CRUD) | Full — fermentables, hops, yeasts, misc, water | Inventory amounts only (PATCH) |
| Mash schedules | Full | None |
| Water chemistry | Full (adjustments, profiles) | None |
| Equipment profiles | Full | None |
| BeerXML import/export | Yes | None |
| Recipe versioning | Yes (branch/snapshot) | None |
| Batch status workflow | Simple | Rich (Planning→Brewing→Fermenting→Conditioning→Completed→Archived) |
| Measured brew values | None | Yes (OG/FG/pH/volumes as actuals vs. planned) |
| Inventory tracking (on-hand qty) | None | Yes — per-ingredient stock amounts |
| Sensor/gravity readings | Basic (add/delete readings) | Rich (iSpindel/Tilt integration, live readings) |
| Brew tracker | None | Yes (live session position) |
| Pagination/filtering | None | Yes (`start_after`, `order_by`, `limit`, `status` filter) |
| Authentication/scopes | None (desktop) | Basic Auth + granular scopes |

---

## Gaps

### Brewfather has, Brewski doesn't

- **Inventory tracking** — on-hand quantities per ingredient (`inventory`, `inventory_adjust`)
- **Measured batch values** — actual vs. planned mash pH, boil volumes, pre/post-boil gravities, FG
- **Batch status lifecycle** — 6-stage workflow (Planning → Brewing → Fermenting → Conditioning → Completed → Archived)
- **Sensor readings** — live fermentation monitoring from hardware devices (iSpindel, Tilt, Rapt)
- **Brew tracker** — real-time session tracking based on stage start time
- **Pagination** — Brewski returns everything with no pagination

### Brewski has, Brewfather API doesn't

- Recipe creation/editing (Brewfather API is read-only for recipes)
- Mash schedule management
- Water chemistry (profiles, adjustments, mineral calculations)
- Equipment profiles
- BeerXML import/export
- Recipe versioning and branching
- Application settings

---

## Options

### Option A: Adapt Brewski's API to match Brewfather

Map Brewski commands to Brewfather-style HTTP endpoints. Works cleanly for recipe reads and basic batch operations. Hard parts:

- **Inventory**: Brewski has no on-hand quantity concept — requires a new data model
- **Measured batch values**: Brewski's batch model is simpler; requires new fields
- **Sensor readings**: No hardware device integration exists
- **Pagination**: Missing, but easy to add

Also requires **exposing an HTTP server from the Tauri backend**, which is non-trivial and adds network attack surface to a desktop app.

### Option B: Brewfather API compatibility layer (recommended)

Add a small HTTP server (Tauri plugin or sidecar) that translates Brewfather API calls to Brewski Tauri IPC:

```
GET /v2/recipes        → invoke('list_recipes')
GET /v2/recipes/:id    → invoke('get_recipe', { id })
GET /v2/batches        → invoke('list_batches')
GET /v2/batches/:id    → invoke('get_batch', { id })
PATCH /v2/batches/:id  → invoke('update_batch', { id, ... })
```

Return `501 Not Implemented` for sensor readings, inventory, brew tracker.

**Why this is better:**

1. Non-destructive — Tauri IPC stays as-is; the layer is additive
2. The cleanly-mappable subset (recipe reads + batch status updates) is what most third-party tools actually use
3. Enables compatibility with tools like Rapt, iSpindel dashboards, and custom automation that speak the Brewfather API
4. Honest about gaps via 501 responses rather than pretending full compatibility

---

## Prerequisite Question

Before building either option, identify the **concrete use case**: which specific tool or integration is the driver? The inventory and sensor gaps are real data model features to build, not just a translation problem — the answer determines scope.
