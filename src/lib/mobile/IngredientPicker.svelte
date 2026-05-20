<script lang="ts">
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";
  import type { Hop, Fermentable, Yeast } from '$lib/api';
  import { listHopLibrary, listFermentableLibrary, listYeastLibrary } from '$lib/api';
  import { ipc } from '$lib/stores/error';
  import { settings } from '$lib/stores/settings';
  import {
    kgToHopDisplay, hopDisplayToKg, hopWeightLabel,
    kgToLb, lbToKg, weightLabel,
    cToF, fToC, tempLabel,
    type Units,
  } from '$lib/units';
  import type { BrewingIconName } from "$lib/icons";

  export type AddPayload =
    | { type: 'hop'; item: Hop; form: string; amount_kg: number; use_: string; time_min: number; hopstand_temp_c: number | null }
    | { type: 'fermentable'; item: Fermentable; amount_kg: number }
    | { type: 'yeast'; item: Yeast; amount: number };

  const HOP_USES = ['boil', 'aroma', 'dry hop', 'first wort', 'hopstand'] as const;
  const HOP_FORMS = ['Pellet', 'Cryo', 'CO2 Extract', 'Plug', 'Leaf'] as const;

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

  let screen = $state<'list' | 'detail'>('list');
  let query = $state('');
  let library = $state<(Hop | Fermentable | Yeast)[]>([]);
  let libraryLoaded = $state(false);
  let selected = $state<Hop | Fermentable | Yeast | null>(null);
  let amount = $state(0);
  let use_ = $state('boil');
  let time = $state(60);
  let hopstand_temp_c = $state(80);
  let hopForm = $state('Pellet');

  const units = $derived<Units>($settings.units === 'imperial' ? 'imperial' : 'metric');

  async function loadLibrary() {
    if (libraryLoaded) return;
    if (type === 'hop') library = (await ipc(listHopLibrary())) ?? [];
    else if (type === 'fermentable') library = (await ipc(listFermentableLibrary())) ?? [];
    else library = (await ipc(listYeastLibrary())) ?? [];
    libraryLoaded = true;
  }

  $effect(() => {
    if (open) {
      loadLibrary();
      query = '';
      selected = null;
      screen = 'list';
    }
  });

  $effect(() => {
    if (!selected) return;
    if (type === 'hop') {
      const h = selected as Hop;
      amount = hopDisplayToKg(units === 'imperial' ? 1 : 28, units);
      use_ = 'boil';
      time = 60;
      hopstand_temp_c = 80;
      hopForm = h.form;
    }
    else if (type === 'fermentable') { amount = units === 'imperial' ? lbToKg(2) : 1.0; }
    else { amount = 1; }
  });

  const filtered = $derived(
    query.trim() === ''
      ? library
      : library.filter(item => item.name.toLowerCase().includes(query.trim().toLowerCase()))
  );

  const canAdd = $derived(selected !== null && amount > 0);

  function selectItem(item: Hop | Fermentable | Yeast) {
    selected = item;
    screen = 'detail';
  }

  function handleAdd() {
    if (!selected || amount <= 0) return;
    if (type === 'hop') {
      onadd({ type: 'hop', item: selected as Hop, form: hopForm, amount_kg: amount, use_, time_min: time, hopstand_temp_c: use_ === 'hopstand' ? hopstand_temp_c : null });
    } else if (type === 'fermentable') {
      onadd({ type: 'fermentable', item: selected as Fermentable, amount_kg: amount });
    } else {
      onadd({ type: 'yeast', item: selected as Yeast, amount });
    }
    onclose();
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

  function fmt(val: number | null, digits = 1): string {
    return val == null ? '—' : val.toFixed(digits);
  }

  const headerIcon = $derived<BrewingIconName>(
    type === "hop" ? "hop" : type === "fermentable" ? "fermentable" : "yeast"
  );
  const headerTitle = $derived(
    type === "hop" ? "Add Hop" : type === "fermentable" ? "Add Fermentable" : "Add Yeast"
  );
</script>

{#if open}
  <div class="fixed inset-0 flex flex-col" style="z-index: 200; background: var(--color-bg-base); color: var(--color-text-primary);">

    {#if screen === 'list'}
      <!-- List screen -->
      <div class="flex items-center gap-3 px-4 py-3 flex-shrink-0"
           style="border-bottom: 1px solid var(--color-border); background: var(--color-bg-surface);">
        <button onclick={onclose} style="background: none; border: none; cursor: pointer; color: var(--color-text-secondary); padding: 4px; font-size: 20px; line-height: 1;">×</button>
        <span style="font-size: 16px; display: inline-flex;"><BrewingIcon name={headerIcon} /></span>
        <h2 class="text-base font-semibold flex-1" style="color: var(--color-text-primary); margin: 0;">{headerTitle}</h2>
      </div>
      <div class="px-4 py-2 flex-shrink-0">
        <div class="relative">
          <svg style="position: absolute; left: 8px; top: 50%; transform: translateY(-50%); pointer-events: none; color: var(--color-text-muted);"
               width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
          </svg>
          <input bind:value={query}
                 aria-label="Search ingredients"
                 placeholder="Search {type === 'hop' ? 'hops' : type === 'fermentable' ? 'fermentables' : 'yeasts'}…"
                 class="w-full text-sm"
                 style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); border-radius: 8px; padding: 8px 12px 8px 28px; color: var(--color-text-primary); outline: none;" />
        </div>
      </div>
      <div class="flex-1 overflow-y-auto px-4 pb-4 flex flex-col gap-1">
        {#if filtered.length === 0}
          <p class="text-sm text-center mt-8" style="color: var(--color-text-muted);">No results for "{query}"</p>
        {:else}
          {#each filtered as item (item.id)}
            <button onclick={() => selectItem(item)}
                    class="flex items-center justify-between w-full text-left px-3 py-3 rounded-lg"
                    style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
              <span class="text-sm" style="color: var(--color-text-primary);">{item.name}</span>
              <span class="text-xs flex-shrink-0 ml-3" style="color: var(--color-text-secondary);">{rowSubtext(item)}</span>
            </button>
          {/each}
        {/if}
      </div>

    {:else}
      <!-- Detail screen -->
      <div class="flex items-center gap-3 px-4 py-3 flex-shrink-0"
           style="border-bottom: 1px solid var(--color-border); background: var(--color-bg-surface);">
        <button onclick={() => { screen = 'list'; selected = null; }}
                style="background: none; border: none; cursor: pointer; color: var(--color-accent); font-size: 14px; padding: 4px 0; display: flex; align-items: center; gap: 4px;">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
          Back
        </button>
        <h2 class="text-base font-semibold flex-1 truncate" style="color: var(--color-text-primary); margin: 0;">{selected?.name ?? ''}</h2>
      </div>

      <div class="flex-1 overflow-y-auto p-4 flex flex-col gap-3">
        {#if selected && type === 'hop'}
          {@const hop = selected as Hop}
          <div style="display: flex; gap: 6px; flex-wrap: wrap;">
            {#if hop.origin}<span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 3px 10px; border-radius: 99px; font-size: 12px;">{hop.origin}</span>{/if}
            {#if hop.type_}<span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 3px 10px; border-radius: 99px; font-size: 12px;">{hop.type_}</span>{/if}
            <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 3px 10px; border-radius: 99px; font-size: 12px;">{hop.form}</span>
          </div>
          <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 6px;">
            {#each [['Alpha', hop.alpha_pct + '%'], ['Beta', fmt(hop.beta_pct ?? null) + '%'], ['Cohumulone', fmt(hop.cohumulone_pct ?? null) + '%']] as [label, value]}
              <div style="background: var(--color-bg-elevated); border-radius: 6px; padding: 8px 10px;">
                <div style="font-size: 11px; color: var(--color-text-secondary);">{label}</div>
                <div style="font-size: 14px; font-weight: 600;">{value}</div>
              </div>
            {/each}
          </div>
          {#if hop.notes}<p style="font-size: 13px; color: var(--color-text-secondary); line-height: 1.5; margin: 0;">{hop.notes}</p>{/if}

        {:else if selected && type === 'fermentable'}
          {@const ferm = selected as Fermentable}
          <div style="display: flex; gap: 6px; flex-wrap: wrap;">
            <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 3px 10px; border-radius: 99px; font-size: 12px;">{ferm.type_}</span>
          </div>
          <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 6px;">
            {#each [['Yield', ferm.yield_pct.toFixed(0) + '%'], ['Color', ferm.color_lovibond + '°L'], ['Origin', ferm.origin ?? '—']] as [label, value]}
              <div style="background: var(--color-bg-elevated); border-radius: 6px; padding: 8px 10px;">
                <div style="font-size: 11px; color: var(--color-text-secondary);">{label}</div>
                <div style="font-size: 14px; font-weight: 600;">{value}</div>
              </div>
            {/each}
          </div>
          {#if ferm.notes}<p style="font-size: 13px; color: var(--color-text-secondary); line-height: 1.5; margin: 0;">{ferm.notes}</p>{/if}

        {:else if selected && type === 'yeast'}
          {@const yeast = selected as Yeast}
          <div style="display: flex; gap: 6px; flex-wrap: wrap;">
            <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 3px 10px; border-radius: 99px; font-size: 12px;">{yeast.type_}</span>
            <span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 3px 10px; border-radius: 99px; font-size: 12px;">{yeast.form}</span>
            {#if yeast.laboratory}<span style="background: var(--color-bg-elevated); color: var(--color-text-secondary); padding: 3px 10px; border-radius: 99px; font-size: 12px;">{yeast.laboratory}</span>{/if}
          </div>
          <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 6px;">
            {#each [['Attenuation', fmt(yeast.attenuation_pct ?? null, 0) + '%'], ['Flocculation', yeast.flocculation ?? '—']] as [label, value]}
              <div style="background: var(--color-bg-elevated); border-radius: 6px; padding: 8px 10px;">
                <div style="font-size: 11px; color: var(--color-text-secondary);">{label}</div>
                <div style="font-size: 14px; font-weight: 600;">{value}</div>
              </div>
            {/each}
          </div>
          {#if yeast.notes}<p style="font-size: 13px; color: var(--color-text-secondary); line-height: 1.5; margin: 0;">{yeast.notes}</p>{/if}
        {/if}
      </div>

      <!-- Add controls footer -->
      <div class="flex-shrink-0 p-4 flex flex-col gap-3"
           style="border-top: 1px solid var(--color-border); background: var(--color-bg-surface);">
        {#if type === 'hop' && selected}
          <div class="flex gap-3">
            <label class="flex flex-col gap-1 flex-1 text-xs" style="color: var(--color-text-secondary);">
              Amount ({hopWeightLabel(units)})
              <input type="number" inputmode="decimal" step={units === 'imperial' ? 0.1 : 1}
                     value={kgToHopDisplay(amount, units).toFixed(units === 'imperial' ? 2 : 0)}
                     onblur={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); if (!isNaN(v)) amount = hopDisplayToKg(v, units); }}
                     min="0.001" class="px-3 py-2 rounded-lg text-sm"
                     style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">
              Form
              <select bind:value={hopForm} class="px-3 py-2 rounded-lg text-sm"
                      style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); color: var(--color-text-primary);">
                {#each HOP_FORMS as f}<option value={f}>{f}</option>{/each}
              </select>
            </label>
            <label class="flex flex-col gap-1 flex-1 text-xs" style="color: var(--color-text-secondary);">
              Use
              <select bind:value={use_} class="px-3 py-2 rounded-lg text-sm"
                      style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); color: var(--color-text-primary);">
                {#each HOP_USES as u}<option value={u}>{u}</option>{/each}
              </select>
            </label>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary); width: 70px;">
              Time (min)
              <input type="number" inputmode="decimal" step="5" bind:value={time} min="0" class="px-3 py-2 rounded-lg text-sm"
                     style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          {#if use_ === 'hopstand'}
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary); max-width: 120px;">
              Temp ({tempLabel(units)})
              <input type="number" inputmode="decimal" step="1"
                     value={units === 'imperial' ? cToF(hopstand_temp_c).toFixed(0) : hopstand_temp_c}
                     onblur={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); if (!isNaN(v)) hopstand_temp_c = units === 'imperial' ? fToC(v) : v; }}
                     min="0" class="px-3 py-2 rounded-lg text-sm"
                     style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          {/if}
        {:else if type === 'fermentable' && selected}
          <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">
            Amount ({weightLabel(units)})
            <input type="number" inputmode="decimal" step="0.1" min="0"
                   value={units === 'imperial' ? kgToLb(amount).toFixed(2) : amount.toFixed(2)}
                   onblur={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); if (!isNaN(v)) amount = units === 'imperial' ? lbToKg(v) : v; }}
                   class="px-3 py-2 rounded-lg text-sm"
                   style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); color: var(--color-text-primary); max-width: 150px;" />
          </label>
        {:else if type === 'yeast' && selected}
          <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">
            Packs / Units
            <input type="number" inputmode="decimal" step="1" min="1" bind:value={amount}
                   class="px-3 py-2 rounded-lg text-sm"
                   style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); color: var(--color-text-primary); max-width: 100px;" />
          </label>
        {/if}
        <button onclick={handleAdd} disabled={!canAdd}
                class="w-full py-3 rounded-lg text-sm font-semibold"
                style="background: {canAdd ? 'var(--color-accent)' : 'var(--color-bg-elevated)'}; color: {canAdd ? '#fff' : 'var(--color-text-muted)'}; border: none; cursor: {canAdd ? 'pointer' : 'default'};">
          Add to Recipe
        </button>
      </div>
    {/if}
  </div>
{/if}
