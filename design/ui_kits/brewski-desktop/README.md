# Brewski — Desktop UI kit

A pixel-faithful React recreation of the Brewski desktop Tauri shell, built
straight from `shanehead/brewski`'s SvelteKit components.

Open `index.html` directly to interact:
- Click between the 5 sample recipes in the sidebar
- Switch the 7 recipe tabs (Overview + Ingredients are wired; the rest
  fall back to a "see the repo for the full implementation" hint —
  matching the design intent, not faking content)
- Use the **palette** icon in the bottom of the rail to swap between
  all 10 themes live (Midnight, Tokyo Night, Dracula, Catppuccin, Nord,
  Monokai, Catppuccin Latte, Solarized Light, Ayu Light, GitHub Light)
- Switch to **Batches**, **Tools**, **Equipment**, **Library**, **Settings**
  from the icon rail — each is a faithful recreation of the matching
  Svelte route

## Component map

| File                | Source equivalent in [shanehead/brewski](https://github.com/shanehead/brewski) |
| ------------------- | ------------------------------------------------------------------------------ |
| `theme.css`         | `src/app.css` + `src/themes/*` + `tailwind` resets                             |
| `BrewingIcon.jsx`   | `src/lib/components/BrewingIcon.svelte` + `src/lib/icons.ts`                   |
| `Card.jsx`          | `src/lib/components/Card.svelte`                                               |
| `StatPill.jsx`      | the pill block inside `src/lib/components/BatchList.svelte`                    |
| `TabBar.jsx`        | `src/lib/components/TabBar.svelte`                                             |
| `StatsSidebar.jsx`  | `src/lib/components/StatsSidebar.svelte`                                       |
| `RecipeList.jsx`    | `src/lib/components/RecipeList.svelte`                                         |
| `OverviewTab.jsx`   | `src/lib/components/tabs/OverviewTab.svelte`                                   |
| `IngredientsTab.jsx`| derived from `src/lib/components/ingredients/*` + `tabs/IngredientsTab.svelte` |
| `RecipeView.jsx`    | `src/lib/desktop/RecipeView.svelte`                                            |
| `BatchesView.jsx`   | `src/lib/desktop/BatchesHome.svelte` + `BatchView.svelte`                      |
| `ToolView.jsx`      | `src/routes/tools/+layout.svelte` + `tools/abv-calories/+page.svelte`          |
| `SettingsView.jsx`  | `src/routes/settings/+page.svelte` + `src/routes/library/+page.svelte`         |
| `AppShell.jsx`      | `src/lib/desktop/AppShell.svelte`                                              |
| `App.jsx`           | top-level state + route switcher (replaces `+layout.svelte` + `+page.svelte`)  |
| `data.js`           | sample recipes / batches / tools / theme list                                  |
| `icons.js`          | exact copy of `src/lib/icons.ts`, exposed on `window.BREWSKI_ICONS`            |

## Caveats

- The Mash, Water, Fermentation, Notes, and recipe-level Batches tabs are
  stubbed with a "see the repo" hint. The originals are large and
  feature-specific; this kit shows the chrome (tab bar, icon, header
  treatment) and leaves the body to the source.
- The Carbonation, Color, Gravity, Hydrometer, Pitch, Refractometer, and
  Unit-Conversion tools also stub. The ABV / Calories tool is fully
  wired and computes results live from the standard formulas.
- This is a visual kit. Settings persist only in component state; nothing
  writes to disk. The real app uses Tauri IPC to a Rust/SQLite backend.
- Imports use the in-browser Babel transformer — fine for a design
  prototype, not appropriate for production code.
