# UI Polish — Card-Based Design System

**Date:** 2026-05-21  
**Status:** Approved  
**Approach:** Systematic card-based polish across all surfaces (Approach A)

---

## Goal

Transform the app from a raw form/text UI into a polished, modern desktop app. The visual direction is "polished & modern" (Linear/Notion-style): clean cards, subtle depth, strong typography hierarchy. All work operates within the existing CSS variable theme system — no new themes, no variable changes.

---

## Design Principles

1. **Cards as the primary grouping unit.** Related fields are wrapped in a card (`bg-surface`, border, `border-radius: 10px`) with a labeled header. This replaces the current pattern of bare fields floating in a padded div.
2. **Big numbers in the stats sidebar.** Each stat becomes a mini-card with a large primary value, small label, and a thin colored progress bar for visual range context.
3. **Consistent shell chrome.** Icon rail buttons get rounded corners and a subtle glow on active. Tab bar gets an underline indicator. Recipe list items get improved hover/selected states.
4. **Theme-safe.** All styles use existing CSS variables (`--color-bg-surface`, `--color-bg-elevated`, `--color-border`, `--color-accent`, `--color-text-*`). No hard-coded colors.

---

## Component Changes

### 1. Shared Card Component (`src/lib/components/Card.svelte`)

A new reusable wrapper:

```
Card
  ├── card-header: icon slot + title string
  └── card-body: default slot (grid layout provided by child)
```

Props: `title: string`, `icon?: BrewingIconName`  
Styles: `bg-surface` background, `border` + `rounded-xl`, header with bottom border, `p-3` body padding.

### 2. Stats Sidebar (`StatsSidebar.svelte`)

**Current:** Label/value rows with a small color dot for SRM.  
**New:** Each stat in a `StatCard` sub-component with:
- Large value (`text-2xl font-bold`)  
- Small label above  
- Thin 3px progress bar below (color-coded: ABV=green, IBU=orange, OG/FG=accent, SRM=srm color)
- SRM swatch enlarged to 16×16px

Progress bar ranges: OG 1.000–1.120, FG 1.000–1.030, ABV 0–12%, IBU 0–120, SRM 1–40.

### 3. Overview Tab (`tabs/OverviewTab.svelte`)

Fields reorganized into two cards:
- **Recipe Details** card: Type, Brewer, Style, Date, Equipment Profile
- **Volumes & Timing** card: Batch Size, Boil Size, Boil Time, Efficiency

### 4. Fermentation Tab (`tabs/FermentationTab.svelte`)

Two cards:
- **Fermentation Schedule** card: Primary/Secondary/Tertiary age + temp fields
- **Carbonation** card: CO₂ Volumes, Forced Carbonation toggle

### 5. Mash Tab (`tabs/MashTab.svelte`)

Two cards:
- **Mash Parameters** card: existing top-level mash fields
- **Mash Steps** card: existing steps table (unchanged table markup, wrapped in card shell)

### 6. Water Tab (`tabs/WaterTab.svelte`)

Already has distinct sections; wrap each existing section in a Card.

### 7. Ingredients Tab (`tabs/IngredientsTab.svelte`)

Each ingredient table (Fermentables, Hops, Yeasts) wrapped in a Card. Table markup unchanged.

### 8. Recipe List (`components/RecipeList.svelte`)

- List items: `rounded-lg` hover background, slightly more padding
- Active item: `bg-elevated` with left accent border (`border-l-2 border-accent`)
- Delete button: already hover-only, keep as-is

### 9. Desktop AppShell (`desktop/AppShell.svelte`)

- Icon rail buttons: add `rounded-lg` (already `rounded`), add `shadow-sm` glow effect on active using `box-shadow: 0 0 8px var(--color-accent-hover)`

### 10. TabBar (`components/TabBar.svelte`)

- Active tab: replace background highlight with bottom border (`border-b-2 border-accent`) + accent text color
- Inactive tabs: `hover:text-primary`

### 11. RecipeView header (`desktop/RecipeView.svelte`)

- "Save Version" and "History" buttons: add `border border-[color-border]` for more definition

---

## Mobile

Mobile views (`src/lib/mobile/`) share the same tab components and `StatsSidebar`. The Card component will be used identically. Mobile-specific shell components (AppShell, BottomTabBar) are not changing layout — only the shared inner components change, which automatically applies to mobile.

---

## Non-goals

- No changes to theme CSS files
- No changes to any Rust backend, API, or data model
- No new routes or navigation
- Notes tab and Batches tab: no structural changes (Notes is a textarea, Batches is a list)
- No animation or transition work (hover transitions already exist)

---

## Files Changed

| File | Change |
|------|--------|
| `src/lib/components/Card.svelte` | **New** — reusable card shell |
| `src/lib/components/StatsSidebar.svelte` | Rebuild with stat mini-cards + progress bars |
| `src/lib/components/tabs/OverviewTab.svelte` | Wrap fields in 2 cards |
| `src/lib/components/tabs/FermentationTab.svelte` | Wrap fields in 2 cards |
| `src/lib/components/tabs/MashTab.svelte` | Wrap in cards |
| `src/lib/components/tabs/WaterTab.svelte` | Wrap sections in cards |
| `src/lib/components/tabs/IngredientsTab.svelte` | Wrap tables in cards |
| `src/lib/components/RecipeList.svelte` | Polish item hover/active states |
| `src/lib/components/TabBar.svelte` | Underline indicator for active tab |
| `src/lib/desktop/AppShell.svelte` | Icon rail active glow |
| `src/lib/desktop/RecipeView.svelte` | Header button border polish |
