# Button Press Feedback Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a visible scale + brightness press effect to all non-disabled buttons app-wide.

**Architecture:** One global `button:not(:disabled):active` rule in `src/app.css` covers every button variant with no component changes. The design preview is updated in parallel so the system stays in sync.

**Tech Stack:** CSS (Tailwind + custom properties), SvelteKit

---

### Task 1: Add press-feedback rule to app.css

**Files:**
- Modify: `src/app.css` (after line 181 — end of the mobile touch-target block)

- [ ] **Step 1: Add the rule**

Append after the closing `}` of the `/* Minimum touch targets on mobile */` block:

```css
/* Press feedback for all non-disabled buttons */
button:not(:disabled):active {
  transform: scale(0.96);
  filter: brightness(0.82);
  transition: transform 80ms cubic-bezier(0.2, 0, 0, 1),
              filter 80ms cubic-bezier(0.2, 0, 0, 1);
}
```

- [ ] **Step 2: Verify quality gates still pass**

```bash
just check
```

Expected: `0 ERRORS` from svelte-check, `validated in Xms` from redocly.

- [ ] **Step 3: Commit**

```bash
git add src/app.css
git commit -m "feat: add scale+brightness press feedback to all buttons"
```

---

### Task 2: Sync the design preview

**Files:**
- Modify: `design/preview/components-buttons.html`

- [ ] **Step 1: Add `:active` styles to the preview's `.btn` rule**

In `design/preview/components-buttons.html`, the existing `.btn` rule is:

```css
.btn {
  padding: 6px 12px;
  border-radius: 6px;
  font: 500 13px var(--font-sans);
  border: 1px solid transparent;
  cursor: pointer; transition: background var(--motion-fast) var(--ease-standard);
}
```

Replace it with:

```css
.btn {
  padding: 6px 12px;
  border-radius: 6px;
  font: 500 13px var(--font-sans);
  border: 1px solid transparent;
  cursor: pointer;
  transition: background var(--motion-fast) var(--ease-standard),
              transform 80ms cubic-bezier(0.2, 0, 0, 1),
              filter 80ms cubic-bezier(0.2, 0, 0, 1);
}
.btn:not(:disabled):active {
  transform: scale(0.96);
  filter: brightness(0.82);
}
```

- [ ] **Step 2: Commit**

```bash
git add design/preview/components-buttons.html
git commit -m "chore: sync button press feedback to design preview"
```
