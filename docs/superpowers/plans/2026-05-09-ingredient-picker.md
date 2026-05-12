# Ingredient Picker Dialog — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the inline ingredient add forms with a searchable master/detail modal dialog that displays full BeerMaverick library data for hops, fermentables, and yeasts.

**Architecture:** A single shared `IngredientPicker.svelte` component accepts a `type` prop (`'hop' | 'fermentable' | 'yeast'`) and renders a native `<dialog>` overlay. The left panel is a search-filtered scrollable list; the right panel shows full ingredient detail with recipe-specific inputs pinned at the bottom. Each ingredient table component (`HopsTable`, `FermentablesTable`, `YeastsTable`) replaces its inline form with `<IngredientPicker>`. The Rust backend already serializes all fields — only the TypeScript interfaces need expanding.

**Tech Stack:** Svelte 5, TypeScript, TailwindCSS 4, inline CSS variables (midnight theme), native HTML `<dialog>` element.

---

## Files

| Action | Path |
|--------|------|
| Modify | `src/lib/api.ts` |
| Create | `src/lib/components/ingredients/IngredientPicker.svelte` |
| Modify | `src/lib/components/ingredients/HopsTable.svelte` |
| Modify | `src/lib/components/ingredients/FermentablesTable.svelte` |
| Modify | `src/lib/components/ingredients/YeastsTable.svelte` |

---

## Task 1: Expand TypeScript interfaces for Hop and Fermentable

The Rust backend already serializes all columns. Only the TypeScript interfaces are thin.

**Files:**
- Modify: `src/lib/api.ts`

- [ ] **Step 1: Replace the `Hop` interface**

In `src/lib/api.ts`, replace:

```typescript
export interface Hop {
  id: string;
  name: string;
  alpha_pct: number;
  form: string;
}
```

With:

```typescript
export interface Hop {
  id: string;
  name: string;
  alpha_pct: number;
  beta_pct: number | null;
  form: string;
  type_: string | null;
  origin: string | null;
  year: string | null;
  notes: string | null;
  substitutes: string | null;
  hsi_pct: number | null;
  humulene_pct: number | null;
  caryophyllene_pct: number | null;
  cohumulone_pct: number | null;
  myrcene_pct: number | null;
}
```

- [ ] **Step 2: Replace the `Fermentable` interface**

In `src/lib/api.ts`, replace:

```typescript
export interface Fermentable {
  id: string;
  name: string;
  type_: string;
  yield_pct: number;
  color_lovibond: number;
}
```

With:

```typescript
export interface Fermentable {
  id: string;
  name: string;
  type_: string;
  yield_pct: number;
  color_lovibond: number;
  origin: string | null;
  supplier: string | null;
  notes: string | null;
  add_after_boil: boolean;
  coarse_fine_diff_pct: number | null;
  moisture_pct: number | null;
  diastatic_power_lintner: number | null;
  protein_pct: number | null;
  max_in_batch_pct: number | null;
  recommend_mash: boolean | null;
  ibu_gal_per_lb: number | null;
}
```

- [ ] **Step 3: Verify types pass**

```bash
just check
```

Expected: no TypeScript errors.

- [ ] **Step 4: Commit**

```bash
git add src/lib/api.ts
git commit -m "Expand Hop and Fermentable TypeScript interfaces to full schema"
```

---

## Task 2: Build IngredientPicker.svelte

**Files:**
- Create: `src/lib/components/ingredients/IngredientPicker.svelte`

- [ ] **Step 1: Create the component**

Create `src/lib/components/ingredients/IngredientPicker.svelte` with the following content:

```svelte
<script lang="ts">
  import type { Hop, Fermentable, Yeast } from '$lib/api';
  import { listHopLibrary, listFermentableLibrary, listYeastLibrary } from '$lib/api';
  import { ipc } from '$lib/stores/error';
  import { settings } from '$lib/stores/settings';
  import {
    kgToHopDisplay, hopDisplayToKg, hopWeightLabel,
    kgToLb, lbToKg, weightLabel,
    type Units,
  } from '$lib/units';
  import { onMount } from 'svelte';

  export type AddPayload =
    | { type: 'hop'; item: Hop; amount_kg: number; use_: string; time_min: number }
    | { type: 'fermentable'; item: Fermentable; amount_kg: number }
    | { type: 'yeast'; item: Yeast; amount: number };

  const HOP_USES = ['boil', 'aroma', 'dry hop', 'first wort', 'whirlpool'] as const;

  let {
    type,
    open,
    onclose,
    onadd,
  }: {
    type: 'hop' | 'fermentable' | 'yeast';
    open: boolean;
    onclose: () => void;
    onadd: (payload: AddPayload) => void;
  } = $props();

  let dialog = $state<HTMLDialogElement | null>(null);
  let query = $state('');
  let library = $state<(Hop | Fermentable | Yeast)[]>([]);
  let selected = $state<Hop | Fermentable | Yeast | null>(null);
  let amount = $state(0);
  let use_ = $state('boil');
  let time = $state(60);

  const units = $derived<Units>($settings.units === 'imperial' ? 'imperial' : 'metric');

  onMount(async () => {
    if (type === 'hop') library = (await ipc(listHopLibrary())) ?? [];
    else if (type === 'fermentable') library = (await ipc(listFermentableLibrary())) ?? [];
    else library = (await ipc(listYeastLibrary())) ?? [];
  });

  $effect(() => {
    if (!dialog) return;
    if (open) {
      dialog.showModal();
      query = '';
      selected = null;
    } else if (dialog.open) {
      dialog.close();
    }
  });

  $effect(() => {
    if (!selected) return;
    if (type === 'hop') { amount = 0.028; use_ = 'boil'; time = 60; }
    else if (type === 'fermentable') { amount = 1.0; }
    else { amount = 1; }
  });

  const filtered = $derived(
    query.trim() === ''
      ? library
      : library.filter(item =>
          item.name.toLowerCase().includes(query.trim().toLowerCase())
        )
  );

  const canAdd = $derived(selected !== null && amount > 0);

  function handleAdd() {
    if (!selected || amount <= 0) return;
    if (type === 'hop') {
      onadd({ type: 'hop', item: selected as Hop, amount_kg: amount, use_, time_min: time });
    } else if (type === 'fermentable') {
      onadd({ type: 'fermentable', item: selected as Fermentable, amount_kg: amount });
    } else {
      onadd({ type: 'yeast', item: selected as Yeast, amount });
    }
    onclose();
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === dialog) onclose();
  }

  function rowSubtext(item: Hop | Fermentable | Yeast): string {
    if (type === 'hop') return `${(item as Hop).alpha_pct}% AA`;
    if (type === 'fermentable') {
      const f = item as Fermentable;
      return `${f.yield_pct.toFixed(0)}% · ${f.color_lovibond}°L`;
    }
    const y = item as Yeast;
    return y.laboratory ?? y.form;
  }

  const SRM_STOPS: [number, string][] = [
    [1, '#f8f753'], [2, '#f6f513'], [3, '#ece61a'], [4, '#d5bc26'],
    [6, '#c1a331'], [8, '#a58c43'], [10, '#8d7537'], [13, '#7a5c34'],
    [17, '#5d3d2b'], [20, '#4a2c24'], [24, '#3d1f1d'], [29, '#2d1616'],
    [35, '#1e0f0f'], [40, '#160b0b'],
  ];
  function srmToHex(srm: number): string {
    const clamp = Math.min(Math.max(srm, 1), 40);
    for (let i = SRM_STOPS.length - 1; i >= 0; i--) {
      if (clamp >= SRM_STOPS[i][0]) return SRM_STOPS[i][1];
    }
    return SRM_STOPS[0][1];
  }

  function fmt(val: number | null, digits = 1): string {
    return val == null ? '—' : val.toFixed(digits);
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<dialog
  bind:this={dialog}
  onclick={handleBackdropClick}
  onclose={onclose}
  style="
    width: 80vw; max-width: 960px; min-width: 560px; height: 75vh;
    background: var(--color-bg-surface); border: 1px solid var(--color-border);
    border-radius: 10px; padding: 0; color: var(--color-text-primary); overflow: hidden;
  "
>
  <div style="display: flex; height: 100%;">

    <!-- Left: search + list -->
    <div style="width: 38%; min-width: 200px; display: flex; flex-direction: column; border-right: 1px solid var(--color-border); padding: 12px; gap: 8px;">
      <input
        bind:value={query}
        placeholder="Search {type === 'hop' ? 'hops' : type === 'fermentable' ? 'fermentables' : 'yeasts'}…"
        autofocus
        style="
          background: var(--color-bg-elevated); border: 1px solid var(--color-border);
          border-radius: 6px; padding: 7px 10px; font-size: 13px;
          color: var(--color-text-primary); outline: none; width: 100%; box-sizing: border-box;
        "
      />
      <div style="flex: 1; overflow-y: auto; display: flex; flex-direction: column; gap: 2px;">
        {#if filtered.length === 0}
          <p style="color: var(--color-text-muted); font-size: 12px; text-align: center; margin-top: 24px;">
            No results for "{query}"
          </p>
        {:else}
          {#each filtered as item (item.id)}
            {@const isSelected = selected?.id === item.id}
            <button
              onclick={() => { selected = item; }}
              style="
                text-align: left; border-radius: 5px; padding: 6px 9px; cursor: pointer; width: 100%;
                background: {isSelected ? 'color-mix(in srgb, var(--color-accent) 15%, transparent)' : 'transparent'};
                border: 1px solid {isSelected ? 'var(--color-accent)' : 'transparent'};
                display: flex; justify-content: space-between; align-items: center; gap: 8px;
              "
            >
              <span style="font-size: 13px; color: var(--color-text-primary);">{item.name}</span>
              <span style="font-size: 11px; color: var(--color-text-muted); white-space: nowrap;">{rowSubtext(item)}</span>
            </button>
          {/each}
        {/if}
      </div>
    </div>

    <!-- Right: detail -->
    <div style="flex: 1; display: flex; flex-direction: column; overflow: hidden; min-width: 0;">
      {#if !selected}
        <div style="display: flex; align-items: center; justify-content: center; height: 100%; color: var(--color-text-muted); font-size: 13px;">
          Select an ingredient to see details
        </div>

      {:else if type === 'hop'}
        {@const hop = selected as Hop}
        <div style="flex: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 10px;">
          <div>
            <h2 style="font-size: 18px; font-weight: 700; margin: 0;">{hop.name}</h2>
            <div style="display: flex; gap: 6px; margin-top: 6px; flex-wrap: wrap;">
              {#if hop.origin}
                <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 2px 8px; border-radius: 99px; font-size: 11px;">{hop.origin}</span>
              {/if}
              {#if hop.type_}
                <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 2px 8px; border-radius: 99px; font-size: 11px;">{hop.type_}</span>
              {/if}
              <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 2px 8px; border-radius: 99px; font-size: 11px;">{hop.form}</span>
            </div>
          </div>
          <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 6px;">
            {#each [['Alpha', hop.alpha_pct + '%'], ['Beta', fmt(hop.beta_pct) + '%'], ['Cohumulone', fmt(hop.cohumulone_pct) + '%'], ['Myrcene', fmt(hop.myrcene_pct) + '%'], ['Humulene', fmt(hop.humulene_pct) + '%'], ['Caryophyllene', fmt(hop.caryophyllene_pct) + '%']] as [label, value]}
              <div style="background: var(--color-bg-elevated); border-radius: 5px; padding: 6px 8px;">
                <div style="font-size: 10px; color: var(--color-text-muted);">{label}</div>
                <div style="font-size: 13px; font-weight: 600;">{value}</div>
              </div>
            {/each}
          </div>
          {#if hop.notes}
            <p style="font-size: 12px; color: var(--color-text-secondary); line-height: 1.5; margin: 0;">{hop.notes}</p>
          {/if}
          {#if hop.substitutes}
            <p style="font-size: 11px; color: var(--color-text-muted); margin: 0;">
              <span style="color: var(--color-text-secondary);">Substitutes:</span> {hop.substitutes}
            </p>
          {/if}
        </div>
        <div style="border-top: 1px solid var(--color-border); padding: 12px 16px; display: flex; gap: 10px; align-items: flex-end; background: var(--color-bg-surface); flex-shrink: 0;">
          <div>
            <div style="font-size: 10px; color: var(--color-text-muted); margin-bottom: 4px;">Amount ({hopWeightLabel(units)})</div>
            <input type="number" step={units === 'imperial' ? 0.1 : 1}
              value={kgToHopDisplay(amount, units).toFixed(units === 'imperial' ? 2 : 0)}
              oninput={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); if (!isNaN(v) && v > 0) amount = hopDisplayToKg(v, units); }}
              min="0.001"
              style="width: 70px; background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 5px; padding: 5px 8px; color: var(--color-text-primary); font-size: 13px;" />
          </div>
          <div>
            <div style="font-size: 10px; color: var(--color-text-muted); margin-bottom: 4px;">Use</div>
            <select bind:value={use_} style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 5px; padding: 5px 8px; color: var(--color-text-primary); font-size: 13px;">
              {#each HOP_USES as u}<option value={u}>{u}</option>{/each}
            </select>
          </div>
          <div>
            <div style="font-size: 10px; color: var(--color-text-muted); margin-bottom: 4px;">Time (min)</div>
            <input type="number" step="5" bind:value={time} min="0"
              style="width: 60px; background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 5px; padding: 5px 8px; color: var(--color-text-primary); font-size: 13px;" />
          </div>
          <button onclick={handleAdd} disabled={!canAdd}
            style="margin-left: auto; background: {canAdd ? 'var(--color-accent)' : 'var(--color-bg-elevated)'}; color: {canAdd ? '#fff' : 'var(--color-text-muted)'}; border: none; border-radius: 6px; padding: 8px 18px; font-size: 13px; font-weight: 600; cursor: {canAdd ? 'pointer' : 'default'};">
            Add to Recipe
          </button>
        </div>

      {:else if type === 'fermentable'}
        {@const ferm = selected as Fermentable}
        <div style="flex: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 10px;">
          <div>
            <h2 style="font-size: 18px; font-weight: 700; margin: 0;">{ferm.name}</h2>
            <div style="display: flex; gap: 6px; margin-top: 6px; flex-wrap: wrap;">
              <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 2px 8px; border-radius: 99px; font-size: 11px;">{ferm.type_}</span>
              {#if ferm.notes}
                <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 2px 8px; border-radius: 99px; font-size: 11px;">{ferm.notes}</span>
              {/if}
            </div>
          </div>
          <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 6px;">
            <div style="background: var(--color-bg-elevated); border-radius: 5px; padding: 6px 8px;">
              <div style="font-size: 10px; color: var(--color-text-muted);">Yield</div>
              <div style="font-size: 13px; font-weight: 600;">{ferm.yield_pct.toFixed(1)}%</div>
            </div>
            <div style="background: var(--color-bg-elevated); border-radius: 5px; padding: 6px 8px;">
              <div style="font-size: 10px; color: var(--color-text-muted);">Color</div>
              <div style="font-size: 13px; font-weight: 600; display: flex; align-items: center; gap: 5px;">
                <span style="display: inline-block; width: 12px; height: 12px; border-radius: 2px; background: {srmToHex(ferm.color_lovibond)};"></span>
                {ferm.color_lovibond}°L
              </div>
            </div>
            <div style="background: var(--color-bg-elevated); border-radius: 5px; padding: 6px 8px;">
              <div style="font-size: 10px; color: var(--color-text-muted);">Diastatic Power</div>
              <div style="font-size: 13px; font-weight: 600;">{ferm.diastatic_power_lintner != null ? ferm.diastatic_power_lintner + '°L' : '—'}</div>
            </div>
            <div style="background: var(--color-bg-elevated); border-radius: 5px; padding: 6px 8px;">
              <div style="font-size: 10px; color: var(--color-text-muted);">Max in Batch</div>
              <div style="font-size: 13px; font-weight: 600;">{ferm.max_in_batch_pct != null ? ferm.max_in_batch_pct + '%' : '—'}</div>
            </div>
          </div>
        </div>
        <div style="border-top: 1px solid var(--color-border); padding: 12px 16px; display: flex; gap: 10px; align-items: flex-end; background: var(--color-bg-surface); flex-shrink: 0;">
          <div>
            <div style="font-size: 10px; color: var(--color-text-muted); margin-bottom: 4px;">Amount ({weightLabel(units)})</div>
            <input type="number" step={units === 'imperial' ? 0.1 : 0.05}
              value={(units === 'imperial' ? kgToLb(amount) : amount).toFixed(2)}
              oninput={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); if (!isNaN(v) && v > 0) amount = units === 'imperial' ? lbToKg(v) : v; }}
              min="0.01"
              style="width: 80px; background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 5px; padding: 5px 8px; color: var(--color-text-primary); font-size: 13px;" />
          </div>
          <button onclick={handleAdd} disabled={!canAdd}
            style="margin-left: auto; background: {canAdd ? 'var(--color-accent)' : 'var(--color-bg-elevated)'}; color: {canAdd ? '#fff' : 'var(--color-text-muted)'}; border: none; border-radius: 6px; padding: 8px 18px; font-size: 13px; font-weight: 600; cursor: {canAdd ? 'pointer' : 'default'};">
            Add to Recipe
          </button>
        </div>

      {:else}
        {@const yeast = selected as Yeast}
        <div style="flex: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 10px;">
          <div>
            <h2 style="font-size: 18px; font-weight: 700; margin: 0;">{yeast.name}</h2>
            {#if yeast.laboratory || yeast.product_id}
              <p style="font-size: 13px; color: var(--color-text-secondary); margin: 2px 0 0;">
                {[yeast.laboratory, yeast.product_id].filter(Boolean).join(' · ')}
              </p>
            {/if}
            <div style="display: flex; gap: 6px; margin-top: 6px; flex-wrap: wrap;">
              <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 2px 8px; border-radius: 99px; font-size: 11px;">{yeast.type_}</span>
              <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 2px 8px; border-radius: 99px; font-size: 11px;">{yeast.form}</span>
              {#if yeast.species}
                <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 2px 8px; border-radius: 99px; font-size: 11px; font-style: italic;">{yeast.species}</span>
              {/if}
            </div>
          </div>
          <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 6px;">
            <div style="background: var(--color-bg-elevated); border-radius: 5px; padding: 6px 8px;">
              <div style="font-size: 10px; color: var(--color-text-muted);">Attenuation</div>
              <div style="font-size: 13px; font-weight: 600;">
                {yeast.min_attenuation_pct != null && yeast.max_attenuation_pct != null
                  ? `${yeast.min_attenuation_pct}–${yeast.max_attenuation_pct}%`
                  : yeast.attenuation_pct != null ? `${yeast.attenuation_pct}%` : '—'}
              </div>
            </div>
            <div style="background: var(--color-bg-elevated); border-radius: 5px; padding: 6px 8px;">
              <div style="font-size: 10px; color: var(--color-text-muted);">Temperature</div>
              <div style="font-size: 13px; font-weight: 600;">
                {yeast.min_temperature_c != null && yeast.max_temperature_c != null
                  ? `${yeast.min_temperature_c}–${yeast.max_temperature_c}°C` : '—'}
              </div>
            </div>
            <div style="background: var(--color-bg-elevated); border-radius: 5px; padding: 6px 8px;">
              <div style="font-size: 10px; color: var(--color-text-muted);">Flocculation</div>
              <div style="font-size: 13px; font-weight: 600;">{yeast.flocculation ?? '—'}</div>
            </div>
            <div style="background: var(--color-bg-elevated); border-radius: 5px; padding: 6px 8px;">
              <div style="font-size: 10px; color: var(--color-text-muted);">Alcohol Tolerance</div>
              <div style="font-size: 13px; font-weight: 600;">{yeast.alcohol_tolerance ?? '—'}</div>
            </div>
            <div style="background: var(--color-bg-elevated); border-radius: 5px; padding: 6px 8px;">
              <div style="font-size: 10px; color: var(--color-text-muted);">Phenolic</div>
              <div style="font-size: 13px; font-weight: 600;">
                {yeast.pof_positive == null ? '—' : yeast.pof_positive ? 'Yes' : 'No'}
              </div>
            </div>
            <div style="background: var(--color-bg-elevated); border-radius: 5px; padding: 6px 8px;">
              <div style="font-size: 10px; color: var(--color-text-muted);">Diastaticus</div>
              <div style="font-size: 13px; font-weight: 600;">
                {yeast.sta1_positive == null ? '—' : yeast.sta1_positive ? 'Yes' : 'No'}
              </div>
            </div>
          </div>
          {#if yeast.flavor_profile}
            <p style="font-size: 12px; color: var(--color-text-secondary); line-height: 1.5; margin: 0;">{yeast.flavor_profile}</p>
          {/if}
          {#if yeast.styles}
            <p style="font-size: 11px; color: var(--color-text-muted); margin: 0;">
              <span style="color: var(--color-text-secondary);">Styles:</span> {yeast.styles}
            </p>
          {/if}
          {#if yeast.substitutes}
            <p style="font-size: 11px; color: var(--color-text-muted); margin: 0;">
              <span style="color: var(--color-text-secondary);">Comparables:</span> {yeast.substitutes}
            </p>
          {/if}
        </div>
        <div style="border-top: 1px solid var(--color-border); padding: 12px 16px; display: flex; gap: 10px; align-items: flex-end; background: var(--color-bg-surface); flex-shrink: 0;">
          <div>
            <div style="font-size: 10px; color: var(--color-text-muted); margin-bottom: 4px;">Packages</div>
            <input type="number" step="1" bind:value={amount} min="1"
              style="width: 60px; background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 5px; padding: 5px 8px; color: var(--color-text-primary); font-size: 13px;" />
          </div>
          <button onclick={handleAdd} disabled={!canAdd}
            style="margin-left: auto; background: {canAdd ? 'var(--color-accent)' : 'var(--color-bg-elevated)'}; color: {canAdd ? '#fff' : 'var(--color-text-muted)'}; border: none; border-radius: 6px; padding: 8px 18px; font-size: 13px; font-weight: 600; cursor: {canAdd ? 'pointer' : 'default'};">
            Add to Recipe
          </button>
        </div>
      {/if}
    </div>
  </div>
</dialog>

<style>
  dialog::backdrop {
    background: rgba(0, 0, 0, 0.7);
  }
</style>
```

- [ ] **Step 2: Verify types**

```bash
just check
```

Expected: no errors.

- [ ] **Step 3: Commit**

```bash
git add src/lib/components/ingredients/IngredientPicker.svelte
git commit -m "Add IngredientPicker dialog component"
```

---

## Task 3: Wire HopsTable to use IngredientPicker

**Files:**
- Modify: `src/lib/components/ingredients/HopsTable.svelte`

- [ ] **Step 1: Replace the script block**

Replace the entire `<script lang="ts">` block in `HopsTable.svelte` with:

```svelte
<script lang="ts">
  import type { Recipe } from "$lib/api";
  import { createRecipeHop, deleteRecipeHop } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { type Units, kgToHopDisplay, hopWeightLabel } from "$lib/units";
  import IngredientPicker, { type AddPayload } from "./IngredientPicker.svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let adding = $state(false);
  const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");

  async function handlePickerAdd(payload: AddPayload) {
    if (payload.type !== "hop") return;
    await ipc(createRecipeHop(recipe.id, {
      hop_id: payload.item.id,
      name: payload.item.name,
      alpha_pct: payload.item.alpha_pct,
      form: payload.item.form,
      amount_kg: payload.amount_kg,
      use_: payload.use_,
      time_min: payload.time_min,
    }));
    onchange();
  }

  async function handleDelete(id: string) {
    await ipc(deleteRecipeHop(id));
    onchange();
  }
</script>
```

- [ ] **Step 2: Replace the template**

Replace the entire template (everything after `</script>`) with:

```svelte
<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Hops</h3>
    <button onclick={() => adding = true} class="text-xs px-2 py-1 rounded"
            style="background: var(--color-accent); color: #fff;">+ Add</button>
  </div>

  <IngredientPicker
    type="hop"
    open={adding}
    onclose={() => adding = false}
    onadd={handlePickerAdd}
  />

  {#if recipe.hops.length > 0}
    <table class="w-full text-sm">
      <thead>
        <tr style="color: var(--color-text-muted);">
          <th class="text-left py-1 font-medium text-xs">Name</th>
          <th class="text-right py-1 font-medium text-xs">AA%</th>
          <th class="text-right py-1 font-medium text-xs">{hopWeightLabel(units)}</th>
          <th class="text-right py-1 font-medium text-xs">Use</th>
          <th class="text-right py-1 font-medium text-xs">Time</th>
          <th class="w-6"></th>
        </tr>
      </thead>
      <tbody>
        {#each recipe.hops as h (h.id)}
          <tr class="border-t" style="border-color: var(--color-border);">
            <td class="py-1.5" style="color: var(--color-text-primary);">{h.name}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{h.alpha_pct}%</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{kgToHopDisplay(h.amount_kg, units).toFixed(units === "imperial" ? 2 : 0)}{hopWeightLabel(units)}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{h.use_}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{h.time_min}min</td>
            <td class="pl-1">
              <button onclick={() => handleDelete(h.id)} class="text-xs opacity-40 hover:opacity-100"
                      style="color: var(--color-text-secondary);">×</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
```

- [ ] **Step 3: Verify types**

```bash
just check
```

Expected: no errors.

- [ ] **Step 4: Smoke-test manually**

Run `just dev`. Open a recipe → Ingredients tab → click "+ Add" under Hops. Verify:
- Dialog opens with search input focused
- Typing filters the list in real time
- Selecting a hop shows full detail panel (alpha, beta, oils, notes, substitutes)
- Amount/use/time inputs are visible and editable
- "Add to Recipe" adds the hop and closes the dialog
- Escape key closes the dialog
- Clicking the dark backdrop closes the dialog

- [ ] **Step 5: Commit**

```bash
git add src/lib/components/ingredients/HopsTable.svelte
git commit -m "Replace HopsTable inline form with IngredientPicker dialog"
```

---

## Task 4: Wire FermentablesTable to use IngredientPicker

**Files:**
- Modify: `src/lib/components/ingredients/FermentablesTable.svelte`

- [ ] **Step 1: Replace the script block**

Replace the entire `<script lang="ts">` block in `FermentablesTable.svelte` with:

```svelte
<script lang="ts">
  import type { Recipe, RecipeAdditionFermentable } from "$lib/api";
  import { createRecipeFermentable, updateRecipeFermentable, deleteRecipeFermentable } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { type Units, kgToLb, lbToKg, weightLabel } from "$lib/units";
  import IngredientPicker, { type AddPayload } from "./IngredientPicker.svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let adding = $state(false);
  const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");

  async function handlePickerAdd(payload: AddPayload) {
    if (payload.type !== "fermentable") return;
    await ipc(createRecipeFermentable(recipe.id, {
      fermentable_id: payload.item.id,
      name: payload.item.name,
      type_: payload.item.type_,
      yield_pct: payload.item.yield_pct,
      color_lovibond: payload.item.color_lovibond,
      amount_kg: payload.amount_kg,
    }));
    onchange();
  }

  async function handleAmountChange(f: RecipeAdditionFermentable, value: string) {
    const display = parseFloat(value);
    if (!isNaN(display) && display > 0) {
      await ipc(updateRecipeFermentable(f.id, { amount_kg: units === "imperial" ? lbToKg(display) : display }));
      onchange();
    }
  }

  async function handleDelete(id: string) {
    await ipc(deleteRecipeFermentable(id));
    onchange();
  }
</script>
```

- [ ] **Step 2: Replace the template**

Replace the entire template (everything after `</script>`) with:

```svelte
<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Fermentables</h3>
    <button onclick={() => adding = true} class="text-xs px-2 py-1 rounded"
            style="background: var(--color-accent); color: #fff;">+ Add</button>
  </div>

  <IngredientPicker
    type="fermentable"
    open={adding}
    onclose={() => adding = false}
    onadd={handlePickerAdd}
  />

  {#if recipe.fermentables.length > 0}
    <table class="w-full text-sm">
      <thead>
        <tr style="color: var(--color-text-muted);">
          <th class="text-left py-1 font-medium text-xs">Name</th>
          <th class="text-right py-1 font-medium text-xs">Lovibond</th>
          <th class="text-right py-1 font-medium text-xs">{weightLabel(units)}</th>
          <th class="w-6"></th>
        </tr>
      </thead>
      <tbody>
        {#each recipe.fermentables as f (f.id)}
          <tr class="border-t" style="border-color: var(--color-border);">
            <td class="py-1.5" style="color: var(--color-text-primary);">{f.name}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{f.color_lovibond}°L</td>
            <td class="text-right py-1.5">
              <input type="number" step={units === "imperial" ? 0.1 : 0.05}
                     value={(units === "imperial" ? kgToLb(f.amount_kg) : f.amount_kg).toFixed(2)}
                     onblur={(e) => handleAmountChange(f, (e.target as HTMLInputElement).value)}
                     class="w-16 text-right px-1 rounded text-xs"
                     style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid transparent;" />
            </td>
            <td class="pl-1">
              <button onclick={() => handleDelete(f.id)} class="text-xs opacity-40 hover:opacity-100"
                      style="color: var(--color-text-secondary);">×</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
```

- [ ] **Step 3: Verify types**

```bash
just check
```

Expected: no errors.

- [ ] **Step 4: Smoke-test manually**

Run `just dev`. Open a recipe → Ingredients tab → click "+ Add" under Fermentables. Verify:
- Dialog shows fermentables list with yield% and °L in row subtext
- Selecting a fermentable shows type badge, subcategory badge, yield, color swatch, diastatic power, max in batch
- Color swatch renders a beer-colored square next to the lovibond value
- Amount input respects the units setting
- Adding a fermentable works and closes the dialog

- [ ] **Step 5: Commit**

```bash
git add src/lib/components/ingredients/FermentablesTable.svelte
git commit -m "Replace FermentablesTable inline form with IngredientPicker dialog"
```

---

## Task 5: Wire YeastsTable to use IngredientPicker

**Files:**
- Modify: `src/lib/components/ingredients/YeastsTable.svelte`

- [ ] **Step 1: Replace the script block**

Replace the entire `<script lang="ts">` block in `YeastsTable.svelte` with:

```svelte
<script lang="ts">
  import type { Recipe } from "$lib/api";
  import { createRecipeYeast, deleteRecipeYeast } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import IngredientPicker, { type AddPayload } from "./IngredientPicker.svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let adding = $state(false);

  async function handlePickerAdd(payload: AddPayload) {
    if (payload.type !== "yeast") return;
    await ipc(createRecipeYeast(recipe.id, {
      yeast_id: payload.item.id,
      name: payload.item.name,
      type_: payload.item.type_,
      form: payload.item.form,
      laboratory: payload.item.laboratory,
      product_id: payload.item.product_id,
      attenuation_pct: payload.item.attenuation_pct,
      amount: payload.amount,
    }));
    onchange();
  }

  async function handleDelete(id: string) {
    await ipc(deleteRecipeYeast(id));
    onchange();
  }
</script>
```

- [ ] **Step 2: Replace the template**

Replace the entire template (everything after `</script>`) with:

```svelte
<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Yeast</h3>
    <button onclick={() => adding = true} class="text-xs px-2 py-1 rounded"
            style="background: var(--color-accent); color: #fff;">+ Add</button>
  </div>

  <IngredientPicker
    type="yeast"
    open={adding}
    onclose={() => adding = false}
    onadd={handlePickerAdd}
  />

  {#each recipe.yeasts as y (y.id)}
    <div class="flex items-center justify-between py-1.5 border-t" style="border-color: var(--color-border);">
      <div>
        <p class="text-sm" style="color: var(--color-text-primary);">{y.name}</p>
        <p class="text-xs" style="color: var(--color-text-secondary);">
          {y.laboratory ?? ""} {y.product_id ?? ""} · {y.attenuation_pct ?? "?"}% attenuation
        </p>
      </div>
      <button onclick={() => handleDelete(y.id)} class="text-xs opacity-40 hover:opacity-100"
              style="color: var(--color-text-secondary);">×</button>
    </div>
  {/each}
</div>
```

- [ ] **Step 3: Verify types**

```bash
just check
```

Expected: no errors.

- [ ] **Step 4: Smoke-test manually**

Run `just dev`. Open a recipe → Ingredients tab → click "+ Add" under Yeast. Verify:
- List shows yeast name + lab name in subtext
- Searching filters correctly (e.g. "wyeast" narrows to Wyeast strains)
- Detail panel shows attenuation range, temperature, flocculation, alcohol tolerance, Phenolic, Diastaticus
- Flavor profile, styles, and comparables appear when present
- Packages input defaults to 1
- Adding a yeast works and closes the dialog

- [ ] **Step 5: Final commit**

```bash
git add src/lib/components/ingredients/YeastsTable.svelte
git commit -m "Replace YeastsTable inline form with IngredientPicker dialog"
```
