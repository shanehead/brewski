# FloatInput Component Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create a `FloatInput` Svelte component that centralises the `toFixed` display, `parseFloat` parsing, `onblur` save, and `use:escRevert` cancel behaviour so float fields can never regress to the cursor-jumping `oninput` pattern.

**Architecture:** A single thin wrapper component (`FloatInput.svelte`) accepts a numeric value and a decimal count, formats for display internally, and fires `oncommit(v: number | null)` on blur. All unit conversion stays in the caller (via helper functions already present in each file, whose signatures change from Event-based to number-based). The component is then substituted for every raw `<input type="number">` that currently uses `toFixed` for display.

**Tech Stack:** Svelte 5 runes, TypeScript, Vitest + @testing-library/svelte, `use:escRevert` action at `src/lib/actions/escRevert.ts`.

---

## File Map

| Status | Path | Purpose |
|--------|------|---------|
| **Create** | `src/lib/components/FloatInput.svelte` | New reusable component |
| **Create** | `tests/FloatInput.test.ts` | Component unit tests |
| **Modify** | `tests/BatchCarbonationSection.test.ts:134-169` | Fix test that now requires blur to trigger commit |
| **Modify** | `src/lib/components/batch/BatchCarbonationSection.svelte` | Replace 1 temp input |
| **Modify** | `src/lib/components/EquipmentProfileModal.svelte` | Refactor 8 helpers + replace 26 inputs |
| **Modify** | `src/lib/components/tabs/MashTab.svelte` | Replace 6 float inputs |
| **Modify** | `src/lib/desktop/IngredientPicker.svelte` | Replace 3 float inputs |
| **Modify** | `src/lib/components/ingredients/IngredientPicker.svelte` | Replace 3 float inputs |
| **Modify** | `src/lib/mobile/IngredientPicker.svelte` | Replace 3 float inputs |
| **Modify** | `src/lib/components/ingredients/FermentablesTable.svelte` | Replace 1 float input + update handler signature |
| **Modify** | `src/lib/components/ingredients/HopsTable.svelte` | Replace 2 float inputs |
| **Modify** | `src/lib/components/batch/BatchOverviewTab.svelte` | Replace 1 float input |

---

## Task 1: Create `FloatInput.svelte` and its tests (TDD)

**Files:**
- Create: `tests/FloatInput.test.ts`
- Create: `src/lib/components/FloatInput.svelte`

- [ ] **Step 1.1 — Write the failing tests**

Create `tests/FloatInput.test.ts`:

```ts
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import FloatInput from "$lib/components/FloatInput.svelte";

describe("FloatInput", () => {
  it("displays value formatted to specified decimals", () => {
    render(FloatInput, { value: 5.789, decimals: 2, oncommit: vi.fn() });
    const input = screen.getByRole("spinbutton") as HTMLInputElement;
    expect(input.value).toBe("5.79");
  });

  it("defaults to 2 decimal places", () => {
    render(FloatInput, { value: 1, oncommit: vi.fn() });
    const input = screen.getByRole("spinbutton") as HTMLInputElement;
    expect(input.value).toBe("1.00");
  });

  it("shows empty string when value is null", () => {
    render(FloatInput, { value: null, oncommit: vi.fn() });
    const input = screen.getByRole("spinbutton") as HTMLInputElement;
    expect(input.value).toBe("");
  });

  it("renders with type=number and inputmode=decimal", () => {
    render(FloatInput, { value: 1, oncommit: vi.fn() });
    const input = screen.getByRole("spinbutton");
    expect(input).toHaveAttribute("type", "number");
    expect(input).toHaveAttribute("inputmode", "decimal");
  });

  it("calls oncommit with parsed number on blur", async () => {
    const oncommit = vi.fn();
    const user = userEvent.setup();
    render(FloatInput, { value: 1, decimals: 2, oncommit });
    const input = screen.getByRole("spinbutton");
    await user.clear(input);
    await user.type(input, "3.14");
    await user.tab();
    expect(oncommit).toHaveBeenCalledWith(3.14);
  });

  it("calls oncommit with null when field is cleared and blurred", async () => {
    const oncommit = vi.fn();
    const user = userEvent.setup();
    render(FloatInput, { value: 5, decimals: 1, oncommit });
    const input = screen.getByRole("spinbutton");
    await user.clear(input);
    await user.tab();
    expect(oncommit).toHaveBeenCalledWith(null);
  });

  it("does not call oncommit with the edited value after Escape", async () => {
    const oncommit = vi.fn();
    const user = userEvent.setup();
    render(FloatInput, { value: 5, decimals: 1, oncommit });
    const input = screen.getByRole("spinbutton") as HTMLInputElement;
    await user.clear(input);
    await user.type(input, "99");
    await user.keyboard("{Escape}");
    // escRevert restores original value and triggers blur with original
    expect(input.value).toBe("5.0");
    expect(oncommit).not.toHaveBeenCalledWith(99);
  });

  it("forwards style prop to the input", () => {
    render(FloatInput, { value: 1, oncommit: vi.fn(), style: "color: red;" });
    const input = screen.getByRole("spinbutton");
    expect(input).toHaveAttribute("style", "color: red;");
  });

  it("forwards id, step, min, placeholder, class props to the input", () => {
    render(FloatInput, {
      value: 1,
      oncommit: vi.fn(),
      id: "my-input",
      step: "0.1",
      min: "0",
      placeholder: "enter value",
      class: "my-class",
    });
    const input = screen.getByRole("spinbutton");
    expect(input).toHaveAttribute("id", "my-input");
    expect(input).toHaveAttribute("step", "0.1");
    expect(input).toHaveAttribute("min", "0");
    expect(input).toHaveAttribute("placeholder", "enter value");
    expect(input).toHaveClass("my-class");
  });
});
```

- [ ] **Step 1.2 — Run tests to confirm they fail**

```bash
cd /Users/shead/Documents/code/brewski && npx vitest run tests/FloatInput.test.ts
```

Expected: all 8 tests fail with "Cannot find module '$lib/components/FloatInput.svelte'".

- [ ] **Step 1.3 — Implement `FloatInput.svelte`**

Create `src/lib/components/FloatInput.svelte`:

```svelte
<script lang="ts">
  import { escRevert } from "$lib/actions/escRevert";

  let {
    value,
    decimals = 2,
    oncommit,
    id,
    step,
    min,
    placeholder,
    class: className,
    style,
    disabled = false,
  }: {
    value: number | null;
    decimals?: number;
    oncommit: (v: number | null) => void;
    id?: string;
    step?: number | string;
    min?: number | string;
    placeholder?: string;
    class?: string;
    style?: string;
    disabled?: boolean;
  } = $props();

  const display = $derived(value != null ? value.toFixed(decimals) : "");
</script>

<input
  {id}
  type="number"
  inputmode="decimal"
  {step}
  {min}
  {placeholder}
  {disabled}
  class={className}
  {style}
  value={display}
  use:escRevert
  onblur={(e) => {
    const v = parseFloat((e.target as HTMLInputElement).value);
    oncommit(isNaN(v) ? null : v);
  }}
/>
```

- [ ] **Step 1.4 — Run tests to confirm they pass**

```bash
cd /Users/shead/Documents/code/brewski && npx vitest run tests/FloatInput.test.ts
```

Expected: all 8 tests pass.

- [ ] **Step 1.5 — Commit**

```bash
cd /Users/shead/Documents/code/brewski && git add src/lib/components/FloatInput.svelte tests/FloatInput.test.ts && git commit -m "feat: add FloatInput component with onblur/escRevert baked in"
```

---

## Task 2: Fix the `BatchCarbonationSection` temp test

The `BatchCarbonationSection` temp input was already changed from `oninput` to `onblur` in a prior fix. The test at line 134 types into the field but never triggers blur, so `onUpdate` is never called. Add `user.tab()` after typing.

**Files:**
- Modify: `tests/BatchCarbonationSection.test.ts:154-162`

- [ ] **Step 2.1 — Run the current failing test to confirm the failure**

```bash
cd /Users/shead/Documents/code/brewski && npx vitest run tests/BatchCarbonationSection.test.ts
```

Expected: "calls onUpdate after temperature change" fails with `Number of calls: 0`.

- [ ] **Step 2.2 — Add `user.tab()` to trigger blur**

In `tests/BatchCarbonationSection.test.ts`, find the test "calls onUpdate after temperature change" and update the interaction block (around line 154):

```ts
    const input = screen.getByLabelText(/Packaging Temp/i) as HTMLInputElement;
    await user.clear(input);
    await user.type(input, "18");
    await user.tab(); // trigger blur so oncommit fires
```

The three lines that follow (tick/setTimeout/tick) stay unchanged.

- [ ] **Step 2.3 — Run tests to confirm they pass**

```bash
cd /Users/shead/Documents/code/brewski && npx vitest run tests/BatchCarbonationSection.test.ts
```

Expected: all 5 tests pass.

- [ ] **Step 2.4 — Commit**

```bash
cd /Users/shead/Documents/code/brewski && git add tests/BatchCarbonationSection.test.ts && git commit -m "fix(test): trigger blur after typing in carbonation temp test"
```

---

## Task 3: Migrate `BatchCarbonationSection.svelte`

**Files:**
- Modify: `src/lib/components/batch/BatchCarbonationSection.svelte`

- [ ] **Step 3.1 — Add the `FloatInput` import**

In the `<script>` block, add to the existing imports:

```ts
import FloatInput from "$lib/components/FloatInput.svelte";
```

- [ ] **Step 3.2 — Update `updateTemp` to accept `number | null`**

Find `updateTemp` (currently takes `value: string`) and replace it:

```ts
  function updateTemp(v: number | null) {
    hasInteracted = true;
    if (v == null) return;
    tempC = units === "imperial" ? fToC(v) : v;
  }
```

- [ ] **Step 3.3 — Replace the temp input element**

Find the `<input id="carb-temp" ...>` block and replace the entire element with:

```svelte
      <FloatInput
        id="carb-temp"
        value={tempDisplay}
        decimals={1}
        step="0.1"
        oncommit={updateTemp}
        class="w-full bg-transparent text-sm outline-none text-text-primary"
      />
```

Note: `tempDisplay` is `$derived(units === "imperial" ? cToF(tempC) : tempC)` — already in display units — so `decimals={1}` and no further conversion is needed on the `value` side. `oncommit` receives the user's typed number in display units; `updateTemp` reverses the conversion when saving.

- [ ] **Step 3.4 — Run all tests**

```bash
cd /Users/shead/Documents/code/brewski && npx vitest run tests/BatchCarbonationSection.test.ts
```

Expected: all 5 tests pass.

- [ ] **Step 3.5 — Commit**

```bash
cd /Users/shead/Documents/code/brewski && git add src/lib/components/batch/BatchCarbonationSection.svelte && git commit -m "refactor: use FloatInput in BatchCarbonationSection"
```

---

## Task 4: Migrate `EquipmentProfileModal.svelte` — helper functions

The modal has event-based helpers that read from `Event.target.value`. These need to become number-in/number-out functions because `FloatInput` delivers a parsed `number | null` to `oncommit`, not a DOM event.

**Files:**
- Modify: `src/lib/components/EquipmentProfileModal.svelte` (script section, lines ~79–180)

- [ ] **Step 4.1 — Add `FloatInput` import**

In the `<script>` section, add:

```ts
import FloatInput from "$lib/components/FloatInput.svelte";
```

- [ ] **Step 4.2 — Replace the helper functions**

Delete the entire block of conversion helpers (`volDisp`, `volDispNull`, `volIn`, `volInNull`, `weightDispNull`, `weightInNull`, `tempDispNull`, `tempInNull`, `ratioDisp`, `ratioIn`, `numInput`, `nullableNumInput`) and replace with these number-in/number-out equivalents:

```ts
  // ── unit conversion helpers (number-in / number-out for FloatInput) ──────
  function volVal(l: number): number { return units === "imperial" ? lToGal(l) : l; }
  function volSave(v: number | null): number { return units === "imperial" ? galToL(v ?? 0) : (v ?? 0); }
  function volValNull(l: number | null): number | null { return l != null ? volVal(l) : null; }
  function volSaveNull(v: number | null): number | null { return v != null ? (units === "imperial" ? galToL(v) : v) : null; }

  function tempVal(f: number | null): number | null { return f != null ? (units === "imperial" ? f : fToC(f)) : null; }
  function tempSave(v: number | null): number | null { return v != null ? (units === "imperial" ? v : cToF(v)) : null; }

  function ratioVal(r: number): number { return units === "imperial" ? r * 0.4796 : r; }
  function ratioSave(v: number | null): number { return units === "imperial" ? (v ?? 0) / 0.4796 : (v ?? 0); }
```

Note: `numInput` and `nullableNumInput` are not replaced — they become unnecessary because `FloatInput` handles parsing internally.

- [ ] **Step 4.3 — Run the type checker to catch any regressions before touching the template**

```bash
cd /Users/shead/Documents/code/brewski && npx svelte-check --tsconfig tsconfig.json 2>&1 | grep -i "EquipmentProfileModal"
```

Expected: errors in the template about old helper names (`volDisp`, `volIn`, etc.) no longer being defined. No errors in the script block itself. These template errors are expected and will be fixed in Task 5.

- [ ] **Step 4.4 — Commit helper refactor**

```bash
cd /Users/shead/Documents/code/brewski && git add src/lib/components/EquipmentProfileModal.svelte && git commit -m "refactor: update EquipmentProfileModal helpers to number-in/out for FloatInput"
```

---

## Task 5: Migrate `EquipmentProfileModal.svelte` — template inputs

Replace all 26 `<input type="number">` elements in the template. The changes follow four patterns depending on which helper they use. Work through each section of the modal.

**Files:**
- Modify: `src/lib/components/EquipmentProfileModal.svelte` (template section, lines ~214–468)

- [ ] **Step 5.1 — Replace boil time input (plain integer, no helper needed)**

Find:
```svelte
<input id="eq-boil-time" type="number" inputmode="decimal" value={boilTimeMin} onblur={(e) => boilTimeMin = numInput(e)} class="eq-field-input" />
```
Replace with:
```svelte
<FloatInput id="eq-boil-time" decimals={0} value={boilTimeMin} oncommit={(v) => boilTimeMin = v ?? boilTimeMin} class="eq-field-input" />
```

- [ ] **Step 5.2 — Replace all volume inputs**

Apply this pattern for each volume field. `volVal`/`volSave` for required fields; `volValNull`/`volSaveNull` for optional fields (those that currently use `volDispNull`/`volInNull`).

**Batch size:**
```svelte
<FloatInput id="eq-batch-size" step="0.1" decimals={2} value={volVal(batchSizeL)} oncommit={(v) => batchSizeL = volSave(v)} class="eq-field-input" />
```

**Pre-boil size (inside the `{:else}` branch):**
```svelte
<FloatInput id="eq-boil-size" step="0.1" decimals={2} value={volVal(boilSizeL)} oncommit={(v) => boilSizeL = volSave(v)} class="eq-field-input" />
```

**Boil off rate:**
```svelte
<FloatInput id="eq-evap-rate" step="0.01" decimals={2} value={volVal(evapRateLHr)} oncommit={(v) => evapRateLHr = volSave(v)} class="eq-field-input" />
```

**Trub/chiller loss:**
```svelte
<FloatInput id="eq-trub-loss" step="0.01" decimals={2} value={volVal(trubChillerLossL)} oncommit={(v) => trubChillerLossL = volSave(v)} class="eq-field-input" />
```

**Mash-tun deadspace:**
```svelte
<FloatInput id="eq-lauter-dead" step="0.01" decimals={2} value={volVal(mashTunDeadspaceL)} oncommit={(v) => mashTunDeadspaceL = volSave(v)} class="eq-field-input" />
```

**Mash-tun loss:**
```svelte
<FloatInput id="eq-mash-loss" step="0.01" decimals={2} value={volVal(mashTunLossL)} oncommit={(v) => mashTunLossL = volSave(v)} class="eq-field-input" />
```

**HLT deadspace (optional):**
```svelte
<FloatInput id="eq-hlt-dead" step="0.01" decimals={2} placeholder="optional" value={volValNull(hltDeadspaceL)} oncommit={(v) => hltDeadspaceL = volSaveNull(v)} class="eq-field-input" />
```

**Fermenter loss:**
```svelte
<FloatInput id="eq-ferm-loss" step="0.01" decimals={2} value={volVal(fermenterLossL)} oncommit={(v) => fermenterLossL = volSave(v)} class="eq-field-input" />
```

**HLT water limit min (optional):**
```svelte
<FloatInput id="eq-hlt-limit-min" step="0.1" decimals={2} placeholder="optional" value={volValNull(hltWaterLimitMinL)} oncommit={(v) => hltWaterLimitMinL = volSaveNull(v)} class="eq-field-input" />
```

**Fermenter top-up (optional):**
```svelte
<FloatInput id="eq-topup" step="0.01" decimals={2} placeholder="optional" value={volValNull(topUpWaterL)} oncommit={(v) => topUpWaterL = volSave(v)} class="eq-field-input" />
```

**Mash-tun heat capacity:**
```svelte
<FloatInput id="eq-tun-heat-cap" step="0.1" decimals={2} value={volVal(tunHeatCapacityL)} oncommit={(v) => tunHeatCapacityL = volSave(v)} class="eq-field-input" />
```

**Mash volume min/max (optional, 4 inputs):**
```svelte
<FloatInput id="eq-mash-vol-min" step="0.1" decimals={2} placeholder="optional" value={volValNull(mashVolumeMinL)} oncommit={(v) => mashVolumeMinL = volSaveNull(v)} class="eq-field-input" />
<FloatInput id="eq-mash-vol-max" step="0.1" decimals={2} placeholder="optional" value={volValNull(mashVolumeMaxL)} oncommit={(v) => mashVolumeMaxL = volSaveNull(v)} class="eq-field-input" />
```

**Sparge volume min/max (optional, 4 inputs):**
```svelte
<FloatInput id="eq-sparge-vol-min" step="0.1" decimals={2} placeholder="optional" value={volValNull(spargeVolumeMinL)} oncommit={(v) => spargeVolumeMinL = volSaveNull(v)} class="eq-field-input" />
<FloatInput id="eq-sparge-vol-max" step="0.1" decimals={2} placeholder="optional" value={volValNull(spargeVolumeMaxL)} oncommit={(v) => spargeVolumeMaxL = volSaveNull(v)} class="eq-field-input" />
```

- [ ] **Step 5.3 — Replace all temperature inputs**

**Hopstand temperature:**
```svelte
<FloatInput id="eq-hopstand-temp" step="1" decimals={1} value={tempVal(hopstandTempF)} oncommit={(v) => { const s = tempSave(v); if (s != null) hopstandTempF = s; }} class="eq-field-input" />
```

**Boil temperature (optional):**
```svelte
<FloatInput id="eq-boil-temp" step="1" decimals={1} placeholder={units === "imperial" ? "212" : "100"} value={tempVal(boilTempF)} oncommit={(v) => boilTempF = tempSave(v)} class="eq-field-input" />
```

**Room temperature:**
```svelte
<FloatInput id="eq-room-temp" step="1" decimals={1} value={tempVal(roomTempF)} oncommit={(v) => { const s = tempSave(v); if (s != null) roomTempF = s; }} class="eq-field-input" />
```

**Grain temperature:**
```svelte
<FloatInput id="eq-grain-temp" step="1" decimals={1} value={tempVal(grainTempF)} oncommit={(v) => { const s = tempSave(v); if (s != null) grainTempF = s; }} class="eq-field-input" />
```

**Sparge temperature (optional):**
```svelte
<FloatInput id="eq-sparge-temp" step="1" decimals={1} placeholder="optional" value={tempVal(spargeTempF)} oncommit={(v) => spargeTempF = tempSave(v)} class="eq-field-input" />
```

- [ ] **Step 5.4 — Replace ratio inputs**

**Grain absorption rate:**
```svelte
<FloatInput id="eq-grain-abs" step="0.01" decimals={2} value={ratioVal(grainAbsorptionRateLPerKg)} oncommit={(v) => grainAbsorptionRateLPerKg = ratioSave(v)} class="eq-field-input" />
```

**Water/grain ratio:**
```svelte
<FloatInput id="eq-water-grain" step="0.01" decimals={2} value={ratioVal(waterGrainRatioLPerKg)} oncommit={(v) => waterGrainRatioLPerKg = ratioSave(v)} class="eq-field-input" />
```

- [ ] **Step 5.5 — Replace plain number inputs (no unit conversion)**

**Cooling shrinkage %:**
```svelte
<FloatInput id="eq-cooling" step="0.1" decimals={1} value={coolingShrinkagePct} oncommit={(v) => coolingShrinkagePct = v ?? coolingShrinkagePct} class="eq-field-input" />
```

**Brewhouse efficiency %:**
```svelte
<FloatInput id="eq-efficiency" step="0.1" decimals={1} value={efficiencyPct} oncommit={(v) => efficiencyPct = v ?? efficiencyPct} class="eq-field-input" />
```

**Mash efficiency % (optional):**
```svelte
<FloatInput id="eq-mash-eff" step="0.1" decimals={1} placeholder="optional" value={mashEfficiencyPct ?? null} oncommit={(v) => mashEfficiencyPct = v} class="eq-field-input" />
```

**Hop utilization %:**
```svelte
<FloatInput id="eq-hop-util" step="1" decimals={1} value={hopUtilizationPct} oncommit={(v) => hopUtilizationPct = v ?? hopUtilizationPct} class="eq-field-input" />
```

**Aroma hop utilization % (inside `{:else}` branch):**
```svelte
<FloatInput id="eq-aroma-util" step="0.1" decimals={1} value={aromaHopUtilizationPct} oncommit={(v) => aromaHopUtilizationPct = v ?? aromaHopUtilizationPct} class="eq-field-input" />
```

**Whirlpool time (optional):**
```svelte
<FloatInput id="eq-whirlpool" step="1" decimals={0} placeholder="optional" value={whirlpoolTimeMin ?? null} oncommit={(v) => whirlpoolTimeMin = v} class="eq-field-input" />
```

- [ ] **Step 5.6 — Run type checker to confirm no errors**

```bash
cd /Users/shead/Documents/code/brewski && npx svelte-check --tsconfig tsconfig.json 2>&1 | grep -c "Error"
```

Expected: 0 errors.

- [ ] **Step 5.7 — Run all tests**

```bash
cd /Users/shead/Documents/code/brewski && npx vitest run tests/EquipmentPage.test.ts
```

Expected: all tests pass.

- [ ] **Step 5.8 — Commit**

```bash
cd /Users/shead/Documents/code/brewski && git add src/lib/components/EquipmentProfileModal.svelte && git commit -m "refactor: use FloatInput for all 26 number inputs in EquipmentProfileModal"
```

---

## Task 6: Migrate `MashTab.svelte`

**Files:**
- Modify: `src/lib/components/tabs/MashTab.svelte`

- [ ] **Step 6.1 — Add `FloatInput` import**

```ts
import FloatInput from "$lib/components/FloatInput.svelte";
```

- [ ] **Step 6.2 — Replace the mash parameter inputs**

Find and replace each float input in the "Mash Parameters" card. The existing handler signatures (all `handleMashField`) stay unchanged; only the input elements change.

**Grain temp:**
```svelte
<FloatInput id="mash-grain-temp" step={units === "imperial" ? 1 : 0.5} decimals={1}
  value={units === "imperial" ? cToF(mash?.grain_temp_c ?? 21) : (mash?.grain_temp_c ?? 21)}
  oncommit={(v) => { if (v != null) handleMashField({ grain_temp_c: units === "imperial" ? fToC(v) : v }); }}
  class="px-2 py-1.5 rounded text-sm bg-bg-elevated text-text-primary border border-border"
/>
```

**Sparge temp (nullable):**
```svelte
<FloatInput id="mash-sparge-temp" step={units === "imperial" ? 1 : 0.5} decimals={1}
  placeholder={units === "imperial" ? "167" : "75"}
  value={mash?.sparge_temp_c != null ? (units === "imperial" ? cToF(mash.sparge_temp_c) : mash.sparge_temp_c) : null}
  oncommit={(v) => handleMashField({ sparge_temp_c: v != null ? (units === "imperial" ? fToC(v) : v) : undefined })}
  class="px-2 py-1.5 rounded text-sm bg-bg-elevated text-text-primary border border-border"
/>
```

**Mash pH (no unit conversion):**
```svelte
<FloatInput id="mash-ph" step="0.1" decimals={2} placeholder="5.4"
  value={mash?.ph ?? null}
  oncommit={(v) => handleMashField({ ph: v ?? undefined })}
  class="px-2 py-1.5 rounded text-sm bg-bg-elevated text-text-primary border border-border"
/>
```

**Water:Grain ratio (nullable, unit conversion):**
```svelte
<FloatInput id="mash-ratio" step="0.1" decimals={2}
  placeholder={units === "imperial" ? "1.5" : "3.0"}
  value={mash.ratio_l_per_kg != null ? (units === "imperial" ? lPerKgToQtPerLb(mash.ratio_l_per_kg) : mash.ratio_l_per_kg) : null}
  oncommit={(v) => { if (v != null) handleMashField({ ratio_l_per_kg: units === "imperial" ? qtPerLbToLPerKg(v) : v }); }}
  class="px-2 py-1.5 rounded text-sm bg-bg-elevated text-text-primary border border-border"
/>
```

- [ ] **Step 6.3 — Replace the "Add Step" form inputs**

**Add step temp:**
```svelte
<FloatInput step={units === "imperial" ? 1 : 0.5} decimals={1}
  value={units === "imperial" ? cToF(stepTemp) : stepTemp}
  oncommit={(v) => { if (v != null) stepTemp = units === "imperial" ? fToC(v) : v; }}
  class="h-9 px-2 rounded text-sm bg-bg-base text-text-primary border border-border"
/>
```

**Add step infuse amount (inside `{#if stepType === "infusion"}`):**
```svelte
<FloatInput step="0.1" decimals={1} placeholder={"Infuse " + volumeLabel(units)}
  value={stepInfuse != null ? (units === "imperial" ? lToGal(stepInfuse) : stepInfuse) : null}
  oncommit={(v) => stepInfuse = v != null ? (units === "imperial" ? galToL(v) : v) : null}
  class="h-9 px-2 rounded text-sm bg-bg-base text-text-primary border border-border"
/>
```

- [ ] **Step 6.4 — Replace the "Edit Step" inline inputs**

**Edit step temp (inside `{#if editingStepId === step.id}`):**
```svelte
<FloatInput step={units === "imperial" ? 1 : 0.5} decimals={1}
  value={units === "imperial" ? cToF(step.step_temp_c) : step.step_temp_c}
  oncommit={(v) => { if (v != null) handleUpdateStepField(step.id, 'step_temp_c', units === 'imperial' ? fToC(v) : v); }}
  class="w-20 px-2 py-1.5 h-10 rounded text-sm bg-bg-base text-text-primary border border-border"
/>
```

**Edit step infuse amount (inside `{#if step.type_ === 'infusion'}`):**
```svelte
<FloatInput step="0.1" decimals={1}
  value={step.infuse_amount_l != null ? (units === 'imperial' ? lToGal(step.infuse_amount_l) : step.infuse_amount_l) : null}
  oncommit={(v) => { if (v != null) handleUpdateStepField(step.id, 'infuse_amount_l', units === 'imperial' ? galToL(v) : v); }}
  class="w-24 px-2 py-1.5 h-10 rounded text-sm bg-bg-base text-text-primary border border-border"
/>
```

- [ ] **Step 6.5 — Run tests**

```bash
cd /Users/shead/Documents/code/brewski && npx vitest run tests/MashTab.test.ts
```

Expected: all tests pass.

- [ ] **Step 6.6 — Commit**

```bash
cd /Users/shead/Documents/code/brewski && git add src/lib/components/tabs/MashTab.svelte && git commit -m "refactor: use FloatInput in MashTab"
```

---

## Task 7: Migrate `IngredientPicker` (desktop, shared, mobile)

All three files contain the same three float input patterns (hop amount, hopstand temp, fermentable amount). Apply the same changes to all three.

**Files:**
- Modify: `src/lib/desktop/IngredientPicker.svelte`
- Modify: `src/lib/components/ingredients/IngredientPicker.svelte`
- Modify: `src/lib/mobile/IngredientPicker.svelte`

- [ ] **Step 7.1 — Add `FloatInput` import to each file**

In each file's `<script>` block add:
```ts
import FloatInput from "$lib/components/FloatInput.svelte";
```

- [ ] **Step 7.2 — Replace hop amount input in each file**

Find the input with `value={kgToHopDisplay(amount, units).toFixed(...)}` and replace:

```svelte
<FloatInput
  step={units === 'imperial' ? 0.1 : 1}
  decimals={units === 'imperial' ? 2 : 0}
  value={kgToHopDisplay(amount, units)}
  oncommit={(v) => { if (v != null && !isNaN(v)) amount = hopDisplayToKg(v, units); }}
  class="bg-bg-elevated border border-border text-text-primary"
  style="width: 70px; border-radius: 5px; padding: 5px 8px; font-size: 13px;"
/>
```

Note: the `style` attribute (if any) is forwarded as-is. In `src/lib/mobile/IngredientPicker.svelte` the class/style attributes may differ slightly — match the class/style values from the original input in each file.

- [ ] **Step 7.3 — Replace hopstand temp input in each file**

Find the input with `value={units === 'imperial' ? cToF(hopstand_temp_c).toFixed(0) : hopstand_temp_c}` and replace:

```svelte
<FloatInput
  step="1"
  decimals={0}
  value={units === 'imperial' ? cToF(hopstand_temp_c) : hopstand_temp_c}
  oncommit={(v) => { if (v != null) hopstand_temp_c = units === 'imperial' ? fToC(v) : v; }}
  class="bg-bg-elevated border border-border text-text-primary"
  style="width: 70px; border-radius: 5px; padding: 5px 8px; font-size: 13px;"
/>
```

- [ ] **Step 7.4 — Replace fermentable amount input in each file**

Find the input with `value={(units === 'imperial' ? kgToLb(amount) : amount).toFixed(2)}` and replace:

```svelte
<FloatInput
  step="0.1"
  decimals={2}
  value={units === 'imperial' ? kgToLb(amount) : amount}
  oncommit={(v) => { if (v != null && !isNaN(v)) amount = units === 'imperial' ? lbToKg(v) : v; }}
  class="bg-bg-elevated border border-border text-text-primary"
  style="width: 70px; border-radius: 5px; padding: 5px 8px; font-size: 13px;"
/>
```

- [ ] **Step 7.5 — Run type checker**

```bash
cd /Users/shead/Documents/code/brewski && npx svelte-check --tsconfig tsconfig.json 2>&1 | grep "Error" | head -20
```

Expected: 0 errors related to the three IngredientPicker files.

- [ ] **Step 7.6 — Commit**

```bash
cd /Users/shead/Documents/code/brewski && git add src/lib/desktop/IngredientPicker.svelte src/lib/components/ingredients/IngredientPicker.svelte src/lib/mobile/IngredientPicker.svelte && git commit -m "refactor: use FloatInput in all IngredientPicker variants"
```

---

## Task 8: Migrate `FermentablesTable.svelte`

**Files:**
- Modify: `src/lib/components/ingredients/FermentablesTable.svelte`

- [ ] **Step 8.1 — Add `FloatInput` import**

```ts
import FloatInput from "$lib/components/FloatInput.svelte";
```

- [ ] **Step 8.2 — Update `handleAmountChange` to accept `number | null`**

Find (around line 34):
```ts
  async function handleAmountChange(f: RecipeAdditionFermentable, value: string) {
    const display = parseFloat(value);
    if (!isNaN(display) && display > 0) {
      await ipc(updateRecipeFermentable(f.id, { amount_kg: units === "imperial" ? lbToKg(display) : display }));
      onchange();
    }
  }
```

Replace with:
```ts
  async function handleAmountChange(f: RecipeAdditionFermentable, v: number | null) {
    if (v != null && v > 0) {
      await ipc(updateRecipeFermentable(f.id, { amount_kg: units === "imperial" ? lbToKg(v) : v }));
      onchange();
    }
  }
```

- [ ] **Step 8.3 — Replace the amount input**

Find (around line 86):
```svelte
              <input type="number" inputmode="decimal" step={units === "imperial" ? 0.1 : 0.05}
                     value={(units === "imperial" ? kgToLb(f.amount_kg) : f.amount_kg).toFixed(2)}
                     use:escRevert
                     onblur={(e) => handleAmountChange(f, (e.target as HTMLInputElement).value)}
                     class="w-16 text-right px-1 rounded bg-bg-elevated text-text-primary"
                     style="border: 1px solid transparent;" />
```

Replace with:
```svelte
              <FloatInput
                step={units === "imperial" ? 0.1 : 0.05}
                decimals={2}
                value={units === "imperial" ? kgToLb(f.amount_kg) : f.amount_kg}
                oncommit={(v) => handleAmountChange(f, v)}
                class="w-16 text-right px-1 rounded bg-bg-elevated text-text-primary"
              />
```

Note: the `style="border: 1px solid transparent;"` cannot be forwarded via the `class` prop. You can either add a wrapper `<span>` or pass it via the `style` attribute: since `FloatInput` doesn't forward `style`, add `style` as a prop to `FloatInput` (`style?: string`, forwarded to the input element). Alternatively, move it to the `class` prop using Tailwind: `class="w-16 text-right px-1 rounded bg-bg-elevated text-text-primary border-transparent border"`.

Use the Tailwind approach (remove the inline style):
```svelte
              <FloatInput
                step={units === "imperial" ? 0.1 : 0.05}
                decimals={2}
                value={units === "imperial" ? kgToLb(f.amount_kg) : f.amount_kg}
                oncommit={(v) => handleAmountChange(f, v)}
                class="w-16 text-right px-1 rounded bg-bg-elevated text-text-primary border border-transparent"
              />
```

- [ ] **Step 8.4 — Run tests**

```bash
cd /Users/shead/Documents/code/brewski && npx vitest run 2>&1 | tail -10
```

Expected: all tests pass.

- [ ] **Step 8.5 — Commit**

```bash
cd /Users/shead/Documents/code/brewski && git add src/lib/components/ingredients/FermentablesTable.svelte && git commit -m "refactor: use FloatInput in FermentablesTable"
```

---

## Task 9: Migrate `HopsTable.svelte`

**Files:**
- Modify: `src/lib/components/ingredients/HopsTable.svelte`

- [ ] **Step 9.1 — Add `FloatInput` import**

```ts
import FloatInput from "$lib/components/FloatInput.svelte";
```

- [ ] **Step 9.2 — Replace hop amount input**

Find the amount input (around line 106):
```svelte
              <input type="number" inputmode="decimal"
                     step={units === "imperial" ? 0.1 : 1}
                     value={kgToHopDisplay(h.amount_kg, units).toFixed(units === "imperial" ? 2 : 0)}
                     use:escRevert
                     onblur={(e) => {
                       const v = parseFloat((e.target as HTMLInputElement).value);
                       if (!isNaN(v) && v > 0) handleUpdate(h.id, { amount_kg: hopDisplayToKg(v, units) });
                     }}
                     class="w-16 text-right px-1 rounded bg-bg-elevated text-text-primary"
                     style="border: 1px solid transparent;" />
```

Replace with:
```svelte
              <FloatInput
                step={units === "imperial" ? 0.1 : 1}
                decimals={units === "imperial" ? 2 : 0}
                value={kgToHopDisplay(h.amount_kg, units)}
                oncommit={(v) => { if (v != null && v > 0) handleUpdate(h.id, { amount_kg: hopDisplayToKg(v, units) }); }}
                class="w-16 text-right px-1 rounded bg-bg-elevated text-text-primary border border-transparent"
              />
```

- [ ] **Step 9.3 — Replace hopstand temp input**

Find the hopstand temp input (around line 135):
```svelte
                  <input type="number" inputmode="decimal" step={units === "imperial" ? 1 : 1}
                         value={h.hopstand_temp_c != null
                           ? (units === "imperial" ? cToF(h.hopstand_temp_c).toFixed(0) : h.hopstand_temp_c.toFixed(0))
                           : ""}
                         placeholder={units === "imperial" ? "170°F" : "80°C"}
                         use:escRevert
                         onblur={(e) => {
                           const v = parseFloat((e.target as HTMLInputElement).value);
                           if (!isNaN(v)) handleUpdate(h.id, { hopstand_temp_c: units === "imperial" ? fToC(v) : v });
                         }}
                         class="w-16 text-right px-1 rounded bg-bg-elevated text-text-secondary"
                         style="border: 1px solid transparent;" />
```

Replace with:
```svelte
                  <FloatInput
                    step="1"
                    decimals={0}
                    placeholder={units === "imperial" ? "170°F" : "80°C"}
                    value={h.hopstand_temp_c != null ? (units === "imperial" ? cToF(h.hopstand_temp_c) : h.hopstand_temp_c) : null}
                    oncommit={(v) => { if (v != null) handleUpdate(h.id, { hopstand_temp_c: units === "imperial" ? fToC(v) : v }); }}
                    class="w-16 text-right px-1 rounded bg-bg-elevated text-text-secondary border border-transparent"
                  />
```

Note: the `time_min` integer input in this file does NOT need to change — leave it as a raw `<input type="number">` since it uses `parseInt`, not `parseFloat`, and has no `toFixed` formatting.

- [ ] **Step 9.4 — Run tests**

```bash
cd /Users/shead/Documents/code/brewski && npx vitest run 2>&1 | tail -10
```

Expected: all tests pass.

- [ ] **Step 9.5 — Commit**

```bash
cd /Users/shead/Documents/code/brewski && git add src/lib/components/ingredients/HopsTable.svelte && git commit -m "refactor: use FloatInput in HopsTable"
```

---

## Task 10: Migrate `BatchOverviewTab.svelte`

**Files:**
- Modify: `src/lib/components/batch/BatchOverviewTab.svelte`

- [ ] **Step 10.1 — Add `FloatInput` import**

```ts
import FloatInput from "$lib/components/FloatInput.svelte";
```

- [ ] **Step 10.2 — Replace the float gravity/reading input**

Find (around line 221):
```svelte
            <input
              id="batch-{row.field}"
              type="number" inputmode="decimal"
              step="0.1"
              value={rawValue != null ? rawValue.toFixed(1) : ""}
              use:escRevert
              onblur={(e) => {
                const v = e.currentTarget.value;
                onUpdate({ [row.field]: v ? parseFloat(v) : null } as UpdateBatchInput);
              }}
              placeholder="—"
              class="w-full bg-transparent text-sm outline-none"
              style="color: {highlighted ? 'var(--color-accent)' : 'var(--color-text-primary)'}; font-weight: {highlighted ? '600' : '400'};"
            />
```

Replace with:
```svelte
            <FloatInput
              id="batch-{row.field}"
              step="0.1"
              decimals={1}
              placeholder="—"
              value={rawValue}
              oncommit={(v) => onUpdate({ [row.field]: v } as UpdateBatchInput)}
              class="w-full bg-transparent text-sm outline-none"
            />
```

Note: the inline `style` with dynamic colour cannot be forwarded via `class`. Add a wrapping `<span>` with the style around the `FloatInput`, or add a `style` prop to `FloatInput`. The simplest fix is to add `style` forwarding to `FloatInput` (one line change):

In `FloatInput.svelte`, add `style` to both the props destructuring and the `<input>` element:

```svelte
  let {
    ...
    style,           // add this line
  }: {
    ...
    style?: string;  // add this line
  } = $props();
```

```svelte
<input
  ...
  {style}    <!-- add this attribute -->
  ...
/>
```

Then `BatchOverviewTab` can pass `style="color: ..."` directly to `FloatInput`.

Full replacement after the `FloatInput` style prop is added:
```svelte
            <FloatInput
              id="batch-{row.field}"
              step="0.1"
              decimals={1}
              placeholder="—"
              value={rawValue}
              oncommit={(v) => onUpdate({ [row.field]: v } as UpdateBatchInput)}
              class="w-full bg-transparent text-sm outline-none"
              style="color: {highlighted ? 'var(--color-accent)' : 'var(--color-text-primary)'}; font-weight: {highlighted ? '600' : '400'};"
            />
```

- [ ] **Step 10.3 — Run all tests**

```bash
cd /Users/shead/Documents/code/brewski && npx vitest run 2>&1 | tail -15
```

Expected: all tests pass.

- [ ] **Step 10.4 — Commit**

```bash
cd /Users/shead/Documents/code/brewski && git add src/lib/components/batch/BatchOverviewTab.svelte && git commit -m "refactor: use FloatInput in BatchOverviewTab"
```

---

## Task 11: Final verification

- [ ] **Step 11.1 — Run the full test suite**

```bash
cd /Users/shead/Documents/code/brewski && npx vitest run
```

Expected: all tests pass, 0 failures.

- [ ] **Step 11.2 — Run svelte-check across the whole project**

```bash
cd /Users/shead/Documents/code/brewski && npx svelte-check --tsconfig tsconfig.json 2>&1 | grep -E "Error|Warning" | head -20
```

Expected: 0 errors. Warnings about unused CSS selectors are acceptable.

- [ ] **Step 11.3 — Confirm no raw `<input type="number">` with `toFixed` in display value remains**

```bash
grep -rn "value=.*toFixed" src/lib/components src/lib/desktop src/lib/mobile --include="*.svelte" | grep -v "//\|label\|:else\|calcEvap\|preBoil\|postBoil\|displayReadings\|strike_temp\|ibu\|toFixed(0)\|toFixed(1)\|toFixed(3)" | grep "input"
```

Expected: no output (all `toFixed` in input `value=` props have been replaced).
