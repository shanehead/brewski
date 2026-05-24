---
name: brewski-design
description: Use this skill to generate well-branded interfaces and assets for Brewski, either for production or throwaway prototypes/mocks/etc. Contains essential design guidelines, colors, type, fonts, assets, and UI kit components for prototyping.
user-invocable: true
---

# Brewski design skill

Brewski is a sleek, modern, free and open-source home-brewing app — built as
a single Tauri 2 + SvelteKit + Rust codebase targeting **macOS, iOS, Android,
Windows and Linux**. The brand identity is **dark by default**, **vivid
accents**, and **fully themable** (10 built-in color schemes).

## Where to start

1. **Read `README.md`** — it carries the brand voice, content fundamentals,
   visual foundations, iconography rules, and the file index.
2. **Open `preview/*.html`** — every preview card is a real, copy-pasteable
   reference for one design token or one component.
3. **Look at `ui_kits/brewski-desktop/`** — full clickable recreation of the
   Tauri desktop shell, with all 10 themes wired up. The component files
   match the original Svelte source one-to-one.

## What to use

| Need                    | Use this                                                    |
| ----------------------- | ----------------------------------------------------------- |
| Design tokens           | `colors_and_type.css` (CSS vars + 10 themes)                |
| Brand logo              | `assets/brewski-logo.png` (1024×1024, PNG with transparency) |
| Favicon                 | `assets/favicon.png`                                        |
| Icons (15 brewing icons)| `assets/icons.ts` / `assets/icons.js` / `assets/icons.html` |
| Visual reference cards  | `preview/*.html`                                            |
| Live components         | `ui_kits/brewski-desktop/*.jsx`                             |

## When making something

- **HTML artifact / throwaway prototype / mock**: link
  `colors_and_type.css`, copy the icons/logo you need into the same folder,
  set `data-theme="midnight"` on `<body>` (or any theme you prefer), and
  build with the tokens. Use Tailwind via CDN if you want utility classes —
  the existing styles are Tailwind-aligned (4px grid, same radii).
- **React prototype**: copy the relevant `.jsx` files from
  `ui_kits/brewski-desktop/` and adapt. Components attach themselves to
  `window`, so you can compose them across files.
- **Production code**: copy the icon set into your repo, follow the same 9
  CSS variables for theming, and copy the [Brewski repo's
  `src/themes/`](https://github.com/shanehead/brewski/tree/main/src/themes)
  CSS files verbatim. Match the SvelteKit conventions (`$platform` alias
  for desktop vs mobile, see `AGENTS.md` in the source repo).

## Hard rules — do not break

- **Never hard-code a brand color.** Use `var(--color-bg-base)` etc.
  Anything that's truly fixed (SRM stops, batch status colors, ingredient
  category colors) lives outside the theme and is documented in
  `colors_and_type.css`.
- **No gradients in product UI.** The only allowed gradient is on the app
  icon itself.
- **No emoji in UI.** Use the brewing icons or Feather-style line icons.
- **Sentence case for buttons and tabs.** ALL CAPS only for `OG`, `FG`,
  `ABV`, `IBU`, `SRM`, `BU:GU`, and the small UPPERCASE eyebrow labels with
  wide tracking on cards.
- **Borders separate surfaces, not shadows.** Shadows only on floating UI
  (popovers, modals, toasts).
- **44px minimum tap targets on mobile.** `height: 100dvh` on the shell.
- **Use "addition", not "ingredient"**, for line items in a recipe (a hop
  *addition* at 60 minutes is the brewing term).

## If the user invokes this skill cold

Ask what they want to build. Some good prompts:
- "A landing page for Brewski"
- "An onboarding flow for first-time users"
- "A native macOS prefs panel for sync settings"
- "A blog post template for a Brewski release announcement"
- "A printable BeerXML cheat-sheet"

Then ask the questions you'd ask any client: target platform, audience
fidelity (mock vs production), variations they want explored. Treat the
preview cards as the *single source of truth* for visual decisions — when
in doubt, open the matching card and copy its tokens.
