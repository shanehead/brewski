---
title: FloatInput Component
date: 2026-06-11
status: approved
---

# FloatInput Component

## Problem

Float input fields across the app use `oninput` to update state on every keystroke. Because the `value` prop is also set to a formatted string (e.g. `volDisp(x)` which calls `.toFixed(2)`), each keystroke triggers a re-render that resets the input's displayed value and moves the cursor. Typing "1" produces "1.00" with the cursor after the zero.

The correct pattern is `onblur`, which delays the state update until focus leaves the field. Several files already use `onblur` correctly, but `EquipmentProfileModal.svelte` and `BatchCarbonationSection.svelte` did not. The fix was applied manually to 26 inputs. A centralized component prevents this from recurring.

## Design

### Component: `FloatInput.svelte`

Location: `src/lib/components/FloatInput.svelte`

**Props:**

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `number \| null` | required | Numeric value to display. Caller applies any unit conversion before passing. |
| `decimals` | `number` | `2` | Decimal places for `toFixed` display formatting. |
| `oncommit` | `(v: number \| null) => void` | required | Called on blur with the parsed number, or `null` if the field is empty or unparseable. |
| `id` | `string` | — | Forwarded to `<input>`. |
| `step` | `number \| string` | — | Forwarded to `<input>`. |
| `min` | `number \| string` | — | Forwarded to `<input>`. |
| `placeholder` | `string` | — | Forwarded to `<input>`. |
| `class` | `string` | — | Forwarded to `<input>`. |
| `disabled` | `boolean` | `false` | Forwarded to `<input>`. |

**Behavior:**

- Always renders `<input type="number" inputmode="decimal">`.
- Displays `value.toFixed(decimals)` when `value` is non-null, `""` when null.
- On blur: parses `parseFloat(input.value)`, calls `oncommit(isNaN(result) ? null : result)`.
- `use:escRevert` is baked in — ESC cancels the in-progress edit and restores the previous displayed value, consistent with the app-wide ESC dismiss convention.

**The component does not handle unit conversion.** Unit conversion is the caller's responsibility, keeping the component focused and reusable across all field types.

### Helper function pattern

Callers that need unit conversion keep local helpers. The helpers change signature: instead of taking `Event` and returning `string`, they take and return `number` (the `toFixed` and `parseFloat` steps move into the component).

```ts
// Old (Event-based)
function volDisp(l: number): string { return (units === "imperial" ? lToGal(l) : l).toFixed(2); }
function volIn(e: Event): number { ... parseFloat(e.target.value) ... }

// New (number-in / number-out)
function volVal(l: number): number { return units === "imperial" ? lToGal(l) : l; }
function volSave(v: number | null): number { return units === "imperial" ? galToL(v ?? 0) : (v ?? 0); }
```

Call site example:

```svelte
<FloatInput
  id="eq-batch-size"
  value={volVal(batchSizeL)}
  decimals={2}
  step="0.1"
  oncommit={(v) => batchSizeL = volSave(v)}
  class="eq-field-input"
/>
```

For nullable fields, the pattern extends naturally:

```svelte
<FloatInput
  value={hltDeadspaceL != null ? volVal(hltDeadspaceL) : null}
  decimals={2}
  step="0.01"
  placeholder="optional"
  oncommit={(v) => hltDeadspaceL = v != null ? volSave(v) : null}
  class="eq-field-input"
/>
```

## Files to Update

| File | Changes |
|------|---------|
| `src/lib/components/FloatInput.svelte` | **New file** |
| `src/lib/components/EquipmentProfileModal.svelte` | Replace all 26 number inputs; refactor `volDisp`/`volIn`/`tempDispNull`/`tempInNull`/`ratioDisp`/`ratioIn`/`numInput`/`nullableNumInput` helpers to number-in/number-out |
| `src/lib/components/batch/BatchCarbonationSection.svelte` | Replace 1 temp input |
| `src/lib/components/tabs/MashTab.svelte` | Replace temp inputs (`grain_temp_c`, `sparge_temp_c`, `step_temp_c`) |
| `src/lib/desktop/IngredientPicker.svelte` | Replace hop and fermentable amount inputs |
| `src/lib/components/ingredients/IngredientPicker.svelte` | Same as above (shared component) |
| `src/lib/mobile/IngredientPicker.svelte` | Same as above (mobile variant) |
| `src/lib/components/ingredients/FermentablesTable.svelte` | Replace amount inputs |
| `src/lib/components/ingredients/HopsTable.svelte` | Replace amount inputs |
| `src/lib/components/batch/BatchOverviewTab.svelte` | Replace gravity/reading inputs |

## Out of Scope

- Tool pages (`pitch-rate`, `carbonation`, `hydrometer-temp`): these use live-updating calculator inputs where the cursor-jump is acceptable given the calculator UX. They are not primary editing surfaces.
- Integer fields: fields like boil time or hop time in whole minutes don't use `toFixed` and aren't affected by the cursor bug.
