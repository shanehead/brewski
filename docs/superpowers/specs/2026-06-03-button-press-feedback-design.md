# Button press feedback

**Date:** 2026-06-03
**Status:** Approved

## Problem

Buttons have no visual response when clicked or tapped. On mobile especially, this makes the UI feel unresponsive.

## Solution

Add a single global `:active` rule to `src/app.css` that applies a scale + brightness press effect to all non-disabled buttons.

```css
button:not(:disabled):active {
  transform: scale(0.96);
  filter: brightness(0.82);
  transition: transform 80ms cubic-bezier(0.2, 0, 0, 1),
              filter 80ms cubic-bezier(0.2, 0, 0, 1);
}
```

## Scope

- **`src/app.css`** — add the rule above, after the existing mobile touch-target block.
- **`design/preview/components-buttons.html`** — add matching `:active` styles to keep the design system in sync.

## What this covers

All `<button>` elements across the app: primary, secondary, outline, danger, icon buttons in the nav rail, and markdown toolbar buttons. Disabled buttons (`button:disabled`) are excluded.

## What this does not change

No component files touched. No Tailwind classes added. No hover states changed.
