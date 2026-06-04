# User Documentation — Design Spec

**Date:** 2026-06-04
**Status:** Approved

---

## Overview

Brewski needs extensive user-facing documentation covering all features of the app. The docs are the primary reference for new and experienced brewers alike, written in a colloquial, human tone. A secondary in-app help layer (tooltips + section links) surfaces the most important information without leaving the app.

---

## 1. Architecture

### 1.1 Docs Site

- **Platform:** VitePress (static site generator, Vite ecosystem)
- **Location:** `/docs` folder in the main Brewski repo
- **Deployment:** GitHub Actions builds and deploys to GitHub Pages on every push to `main`
- **Search:** VitePress built-in local search (MiniSearch) — no external service needed

### 1.2 In-App Help

- **Tooltips:** Small `?` icon on technical fields. Hover on desktop, tap on mobile. Tap/click anywhere to dismiss.
- **Section help links:** Each major tab or section has a small `↗` link in its header area pointing to the relevant guide page on the docs site.
- **User setting:** A "Show tooltips" toggle in Settings (default: on). When off, all `?` icons are hidden. This lets experienced users declutter the UI.

---

## 2. Content Structure

### 2.1 Site Navigation

```
Getting Started
├── What is Brewski?
├── Installation
├── Your first recipe
├── Your first batch
└── Importing an existing recipe (BeerXML)

Guides
├── Building a recipe from scratch
├── Working with fermentables
├── Working with hops
├── Adding yeast
├── Dialing in your mash
├── Water chemistry
├── Scaling a recipe
├── Recipe versions
├── Logging a brew day
├── Tracking gravity & fermentation
├── Carbonation & packaging
├── Using the ingredient library
└── Cloud sync

Reference
├── Equipment profiles
├── Styles
├── Ingredient library
├── Calculators
│   ├── ABV & calories
│   ├── Hydrometer correction
│   ├── Refractometer
│   ├── Gravity conversions
│   ├── Color conversions
│   ├── Pitch rate
│   ├── Carbonation
│   └── Unit conversions
├── Settings
└── BeerXML import & export

Concepts & Glossary
├── Understanding gravity (OG, FG, Plato, Brix)
├── IBU — bitterness explained
├── SRM & EBC — color explained
├── ABV & attenuation
├── Mash chemistry basics
├── Water ions & their effects
├── Hop forms & usage types
├── Yeast pitch rate & starters
└── Glossary A–Z

FAQ
└── FAQ (single page, ~15–20 Q&As)
```

### 2.2 Page Depth

Getting Started pages are the most important — they must be polished, friendly, and get a new user to a working recipe fast. Guides are step-by-step walkthroughs. Reference pages are denser and fact-forward. Concepts pages go into the brewing science. The FAQ is a single page covering the most common questions.

---

## 3. In-App Integration

### 3.1 Tooltip Behaviour

| Context | Behaviour |
|---|---|
| Desktop | Hover over `?` to show. Auto-dismisses when cursor leaves. |
| Mobile | Tap `?` to show. Tap anywhere to dismiss. |
| Rendering | Tooltip renders inline below the field (not absolutely positioned) to avoid overflow/clipping on mobile. |

**Fields that get tooltips** (technical terms a brewer might not know):
- Alpha %, Beta %
- IBU targets
- SRM, EBC
- Pitch rate, cell count
- Hopstand temp
- Wort correction factor (refractometer)
- Water ion targets (Ca, Mg, Na, Cl, SO4, HCO3)
- Mash ratio, tun deadspace, trub loss

**Fields that don't get tooltips** (obvious): Name, Amount, Date, Notes, any plain text field.

### 3.2 Section Help Links

Each major section tab shows a small `Hops guide ↗` or similar link aligned to the far right of the section header. Opens the docs page in the system browser. On mobile this sits below the tab content, not in the header, to avoid crowding the tab bar.

### 3.3 Settings Toggle

A `Show tooltips` toggle is added to Settings (boolean, default `true`). When `false`, all `?` icon elements are hidden via a global reactive setting. No tooltips fire. Help links remain visible regardless.

---

## 4. Writing Style Guide

All documentation is drafted by AI and reviewed/edited by the project author to ensure a consistent, human voice. The following rules apply when drafting and reviewing.

### Do

- Write directly to the brewer — use "you" and "your"
- Use contractions freely: "you're", "it's", "don't", "here's"
- Bold UI element names: **Add Hop**, **Save**, **Mash** tab
- Use `→` for navigation paths: Settings → Units → Gravity Unit
- Explain the *why*, not just the *how* — "Alpha % is how bitter your hops are, so don't leave it blank"
- Use analogies where they help — brewing is physical, keep explanations grounded
- Avoid overly long sentences. If a sentence needs a comma and a semicolon, split it into two.

### Don't

- No em-dashes (—). Use a comma, a period, or a new sentence instead.
- No "It is important to note that…"
- No "Navigate to the X section in order to…"
- No "This feature allows users to…"
- No passive voice where active works
- No excessive hedging ("you may want to consider")

### Tone Target

The docs should read like a knowledgeable friend walking you through the app, not a technical manual. If a sentence sounds like something a robot would write, rewrite it.

---

## 5. Deployment

### 5.1 VitePress Setup

- VitePress installed as a dev dependency
- Config at `/docs/.vitepress/config.ts`
- Sidebar generated from the structure in section 2.1
- Local search enabled (two-line config addition)
- Theme: VitePress default dark mode, with Brewski brand colors applied via CSS custom properties

### 5.2 GitHub Actions

A workflow at `.github/workflows/docs.yml` runs on push to `main`:
1. Install dependencies
2. Run `vitepress build docs`
3. Deploy built output to `gh-pages` branch via `peaceiris/actions-gh-pages` or the native GitHub Pages action

### 5.3 `.gitignore`

Add `.superpowers/` to `.gitignore` (brainstorm session files should not be committed).

---

## 6. Implementation Order

1. **VitePress scaffold** — install, config, sidebar, GitHub Actions workflow
2. **Getting Started** — four pages, highest priority
3. **Guides** — all 13 guides
4. **Reference** — equipment, calculators, settings, BeerXML
5. **Concepts & Glossary** — brewing science pages + A–Z glossary
6. **FAQ** — single page
7. **In-app tooltips** — tooltip component + `show_tooltips` setting + field annotations
8. **In-app help links** — section `↗` links wired to docs URLs
