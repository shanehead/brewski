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
  import { srmToHex } from "$lib/utils/srm";
  export type AddPayload =
    | { type: 'hop'; item: Hop; amount_kg: number; use_: string; time_min: number; hopstand_temp_c: number | null }
    | { type: 'fermentable'; item: Fermentable; amount_kg: number }
    | { type: 'yeast'; item: Yeast; amount: number };

  const HOP_USES = ['boil', 'aroma', 'dry hop', 'first wort', 'hopstand'] as const;

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
  let searchInput = $state<HTMLInputElement | null>(null);
  let query = $state('');
  let library = $state<(Hop | Fermentable | Yeast)[]>([]);
  let libraryLoaded = $state(false);
  let selected = $state<Hop | Fermentable | Yeast | null>(null);
  let amount = $state(0);
  let use_ = $state('boil');
  let time = $state(60);
  let hopstand_temp_c = $state(80);

  const units = $derived<Units>($settings.units === 'imperial' ? 'imperial' : 'metric');

  async function loadLibrary() {
    if (libraryLoaded) return;
    if (type === 'hop') library = (await ipc(listHopLibrary())) ?? [];
    else if (type === 'fermentable') library = (await ipc(listFermentableLibrary())) ?? [];
    else library = (await ipc(listYeastLibrary())) ?? [];
    libraryLoaded = true;
  }

  $effect(() => {
    if (!dialog) return;
    if (open) {
      loadLibrary();
      dialog.showModal();
      query = '';
      selected = null;
      // autofocus only fires once per element; explicitly focus on each open
      setTimeout(() => searchInput?.focus(), 0);
    } else if (dialog.open) {
      dialog.close();
    }
  });

  $effect(() => {
    if (!selected) return;
    if (type === 'hop') { amount = hopDisplayToKg(units === 'imperial' ? 1 : 28, units); use_ = 'boil'; time = 60; hopstand_temp_c = 80; }
    else if (type === 'fermentable') { amount = units === 'imperial' ? lbToKg(2) : 1.0; }
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
      onadd({ type: 'hop', item: selected as Hop, amount_kg: amount, use_, time_min: time, hopstand_temp_c: use_ === 'hopstand' ? hopstand_temp_c : null });
    } else if (type === 'fermentable') {
      onadd({ type: 'fermentable', item: selected as Fermentable, amount_kg: amount });
    } else {
      onadd({ type: 'yeast', item: selected as Yeast, amount });
    }
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

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<dialog
  bind:this={dialog}
  onclick={handleBackdropClick}
  onclose={onclose}
  class="ingredient-picker-dialog bg-bg-surface border border-border text-text-primary"
  style="border-radius: 10px; padding: 0; overflow: hidden; position: relative;"
>
  <button
    onclick={onclose}
    class="text-text-muted" style="position: absolute; top: 10px; right: 12px; background: none; border: none; cursor: pointer; font-size: 18px; line-height: 1; padding: 2px 6px; border-radius: 4px;"
    onmouseenter={(e) => (e.currentTarget as HTMLButtonElement).style.color = 'var(--color-text-primary)'}
    onmouseleave={(e) => (e.currentTarget as HTMLButtonElement).style.color = 'var(--color-text-muted)'}
  >×</button>

  <div style="display: flex; height: 100%;">

    <!-- Left: search + list -->
    <div class="border-r border-border" style="width: 38%; min-width: 200px; display: flex; flex-direction: column; padding: 12px; gap: 8px;">
      <div style="display: flex; align-items: center; gap: 8px; padding-right: 28px; min-height: 28px;">
        <span style="font-size: 18px; line-height: 1; display: inline-flex; align-items: center;">
          <BrewingIcon name={headerIcon} />
        </span>
        <h2 class="text-text-primary" style="font-size: 15px; font-weight: 700; margin: 0;">
          {headerTitle}
        </h2>
      </div>
      <div style="position: relative;">
        <svg class="text-text-muted" style="position: absolute; left: 8px; top: 50%; transform: translateY(-50%); pointer-events: none;" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
        <input
          bind:this={searchInput}
          bind:value={query}
          placeholder="Search {type === 'hop' ? 'hops' : type === 'fermentable' ? 'fermentables' : 'yeasts'}…"
          class="bg-bg-elevated border border-border text-text-primary" style="border-radius: 6px; padding: 7px 10px 7px 28px; font-size: 13px; outline: none; width: 100%; box-sizing: border-box;"
        />
      </div>
      <div style="flex: 1; overflow-y: auto; display: flex; flex-direction: column; gap: 2px;">
        {#if filtered.length === 0}
          <p class="text-text-muted" style="font-size: 12px; text-align: center; margin-top: 24px;">
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
              <span class="text-text-primary" style="font-size: 13px;">{item.name}</span>
              <span class="text-text-secondary" style="font-size: 12px; white-space: nowrap;">{rowSubtext(item)}</span>
            </button>
          {/each}
        {/if}
      </div>
    </div>

    <!-- Right: detail -->
    <div style="flex: 1; display: flex; flex-direction: column; overflow: hidden; min-width: 0;">
      {#if !selected}
        <div class="text-text-muted" style="display: flex; align-items: center; justify-content: center; height: 100%; font-size: 13px;">
          Select an ingredient to see details
        </div>

      {:else if type === 'hop'}
        {@const hop = selected as Hop}
        <div style="flex: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 10px;">
          <div>
            <h2 style="font-size: 18px; font-weight: 700; margin: 0;">{hop.name}</h2>
            <div style="display: flex; gap: 6px; margin-top: 6px; flex-wrap: wrap;">
              {#if hop.origin}
                <span class="bg-bg-elevated text-text-secondary" style="padding: 2px 8px; border-radius: 99px; font-size: 11px;">{hop.origin}</span>
              {/if}
              {#if hop.type_}
                <span class="bg-bg-elevated text-text-secondary" style="padding: 2px 8px; border-radius: 99px; font-size: 11px;">{hop.type_}</span>
              {/if}
              <span class="bg-bg-elevated text-text-secondary" style="padding: 2px 8px; border-radius: 99px; font-size: 11px;">{hop.form}</span>
            </div>
          </div>
          <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 6px;">
            {#each [['Alpha', hop.alpha_pct + '%'], ['Beta', fmt(hop.beta_pct ?? null) + '%'], ['Cohumulone', fmt(hop.cohumulone_pct ?? null) + '%'], ['Myrcene', fmt(hop.myrcene_pct ?? null) + '%'], ['Humulene', fmt(hop.humulene_pct ?? null) + '%'], ['Caryophyllene', fmt(hop.caryophyllene_pct ?? null) + '%']] as [label, value]}
              <div class="bg-bg-elevated" style="border-radius: 5px; padding: 6px 8px;">
                <div class="text-text-secondary" style="font-size: 11px;">{label}</div>
                <div style="font-size: 13px; font-weight: 600;">{value}</div>
              </div>
            {/each}
          </div>
          {#if hop.notes}
            <p class="text-text-secondary" style="font-size: 12px; line-height: 1.5; margin: 0;">{hop.notes}</p>
          {/if}
          {#if hop.substitutes}
            <p class="text-text-muted" style="font-size: 11px; margin: 0;">
              <span class="text-text-secondary">Substitutes:</span> {hop.substitutes}
            </p>
          {/if}
        </div>
        <div class="border-t border-border bg-bg-surface" style="padding: 12px 16px; display: flex; gap: 10px; align-items: flex-end; flex-shrink: 0;">
          <div>
            <div class="text-text-secondary" style="font-size: 11px; margin-bottom: 4px;">Amount ({hopWeightLabel(units)})</div>
            <input type="number" inputmode="decimal" step={units === 'imperial' ? 0.1 : 1}
              value={kgToHopDisplay(amount, units).toFixed(units === 'imperial' ? 2 : 0)}
              onblur={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); if (!isNaN(v)) amount = hopDisplayToKg(v, units); }}
              min="0.001"
              class="bg-bg-elevated border border-border text-text-primary" style="width: 70px; border-radius: 5px; padding: 5px 8px; font-size: 13px;" />
          </div>
          <div>
            <div class="text-text-secondary" style="font-size: 11px; margin-bottom: 4px;">Use</div>
            <select bind:value={use_} class="bg-bg-elevated border border-border text-text-primary" style="border-radius: 5px; padding: 5px 8px; font-size: 13px;">
              {#each HOP_USES as u}<option value={u}>{u}</option>{/each}
            </select>
          </div>
          <div>
            <div class="text-text-secondary" style="font-size: 11px; margin-bottom: 4px;">Time (min)</div>
            <input type="number" inputmode="decimal" step="5" bind:value={time} min="0"
              class="bg-bg-elevated border border-border text-text-primary" style="width: 60px; border-radius: 5px; padding: 5px 8px; font-size: 13px;" />
          </div>
          {#if use_ === 'hopstand'}
          <div>
            <div class="text-text-secondary" style="font-size: 11px; margin-bottom: 4px;">Temp ({tempLabel(units)})</div>
            <input type="number" inputmode="decimal" step="1"
              value={units === 'imperial' ? cToF(hopstand_temp_c).toFixed(0) : hopstand_temp_c}
              onblur={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); if (!isNaN(v)) hopstand_temp_c = units === 'imperial' ? fToC(v) : v; }}
              min="0"
              class="bg-bg-elevated border border-border text-text-primary" style="width: 60px; border-radius: 5px; padding: 5px 8px; font-size: 13px;" />
          </div>
          {/if}
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
              <span class="bg-bg-elevated text-text-secondary" style="padding: 2px 8px; border-radius: 99px; font-size: 11px;">{ferm.type_}</span>
              {#if ferm.notes}
                <span class="bg-bg-elevated text-text-secondary" style="padding: 2px 8px; border-radius: 99px; font-size: 11px;">{ferm.notes}</span>
              {/if}
            </div>
          </div>
          <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 6px;">
            <div class="bg-bg-elevated" style="border-radius: 5px; padding: 6px 8px;">
              <div class="text-text-secondary" style="font-size: 11px;">Yield</div>
              <div style="font-size: 13px; font-weight: 600;">{ferm.yield_pct.toFixed(1)}%</div>
            </div>
            <div class="bg-bg-elevated" style="border-radius: 5px; padding: 6px 8px;">
              <div class="text-text-secondary" style="font-size: 11px;">Color</div>
              <div style="font-size: 13px; font-weight: 600; display: flex; align-items: center; gap: 5px;">
                <span style="display: inline-block; width: 12px; height: 12px; border-radius: 2px; background: {srmToHex(ferm.color_lovibond)};"></span>
                {ferm.color_lovibond}°L
              </div>
            </div>
            <div class="bg-bg-elevated" style="border-radius: 5px; padding: 6px 8px;">
              <div class="text-text-secondary" style="font-size: 11px;">Diastatic Power</div>
              <div style="font-size: 13px; font-weight: 600;">{ferm.diastatic_power_lintner != null ? ferm.diastatic_power_lintner + '°L' : '—'}</div>
            </div>
            <div class="bg-bg-elevated" style="border-radius: 5px; padding: 6px 8px;">
              <div class="text-text-secondary" style="font-size: 11px;">Max in Batch</div>
              <div style="font-size: 13px; font-weight: 600;">{ferm.max_in_batch_pct != null ? ferm.max_in_batch_pct + '%' : '—'}</div>
            </div>
          </div>
        </div>
        <div class="border-t border-border bg-bg-surface" style="padding: 12px 16px; display: flex; gap: 10px; align-items: flex-end; flex-shrink: 0;">
          <div>
            <div class="text-text-secondary" style="font-size: 11px; margin-bottom: 4px;">Amount ({weightLabel(units)})</div>
            <input type="number" inputmode="decimal" step="0.1"
              value={(units === 'imperial' ? kgToLb(amount) : amount).toFixed(2)}
              onblur={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); if (!isNaN(v)) amount = units === 'imperial' ? lbToKg(v) : v; }}
              min="0.01"
              class="bg-bg-elevated border border-border text-text-primary" style="width: 80px; border-radius: 5px; padding: 5px 8px; font-size: 13px;" />
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
              <p class="text-text-secondary" style="font-size: 13px; margin: 2px 0 0;">
                {[yeast.laboratory, yeast.product_id].filter(Boolean).join(' · ')}
              </p>
            {/if}
            <div style="display: flex; gap: 6px; margin-top: 6px; flex-wrap: wrap;">
              <span class="bg-bg-elevated text-text-secondary" style="padding: 2px 8px; border-radius: 99px; font-size: 11px;">{yeast.type_}</span>
              <span class="bg-bg-elevated text-text-secondary" style="padding: 2px 8px; border-radius: 99px; font-size: 11px;">{yeast.form}</span>
              {#if yeast.species}
                <span class="bg-bg-elevated text-text-secondary" style="padding: 2px 8px; border-radius: 99px; font-size: 11px; font-style: italic;">{yeast.species}</span>
              {/if}
            </div>
          </div>
          <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 6px;">
            <div class="bg-bg-elevated" style="border-radius: 5px; padding: 6px 8px;">
              <div class="text-text-secondary" style="font-size: 11px;">Attenuation</div>
              <div style="font-size: 13px; font-weight: 600;">
                {yeast.min_attenuation_pct != null && yeast.max_attenuation_pct != null
                  ? `${yeast.min_attenuation_pct}–${yeast.max_attenuation_pct}%`
                  : yeast.attenuation_pct != null ? `${yeast.attenuation_pct}%` : '—'}
              </div>
            </div>
            <div class="bg-bg-elevated" style="border-radius: 5px; padding: 6px 8px;">
              <div class="text-text-secondary" style="font-size: 11px;">Temperature</div>
              <div style="font-size: 13px; font-weight: 600;">
                {yeast.min_temperature_c != null && yeast.max_temperature_c != null
                  ? `${yeast.min_temperature_c}–${yeast.max_temperature_c}°C` : '—'}
              </div>
            </div>
            <div class="bg-bg-elevated" style="border-radius: 5px; padding: 6px 8px;">
              <div class="text-text-secondary" style="font-size: 11px;">Flocculation</div>
              <div style="font-size: 13px; font-weight: 600;">{yeast.flocculation ?? '—'}</div>
            </div>
            <div class="bg-bg-elevated" style="border-radius: 5px; padding: 6px 8px;">
              <div class="text-text-secondary" style="font-size: 11px;">Alcohol Tolerance</div>
              <div style="font-size: 13px; font-weight: 600;">{yeast.alcohol_tolerance ?? '—'}</div>
            </div>
            <div class="bg-bg-elevated" style="border-radius: 5px; padding: 6px 8px;">
              <div class="text-text-secondary" style="font-size: 11px;">Phenolic</div>
              <div style="font-size: 13px; font-weight: 600;">
                {yeast.pof_positive == null ? '—' : yeast.pof_positive ? 'Yes' : 'No'}
              </div>
            </div>
            <div class="bg-bg-elevated" style="border-radius: 5px; padding: 6px 8px;">
              <div class="text-text-secondary" style="font-size: 11px;">Diastaticus</div>
              <div style="font-size: 13px; font-weight: 600;">
                {yeast.sta1_positive == null ? '—' : yeast.sta1_positive ? 'Yes' : 'No'}
              </div>
            </div>
          </div>
          {#if yeast.flavor_profile}
            <p class="text-text-secondary" style="font-size: 12px; line-height: 1.5; margin: 0;">{yeast.flavor_profile}</p>
          {/if}
          {#if yeast.styles}
            <p class="text-text-muted" style="font-size: 11px; margin: 0;">
              <span class="text-text-secondary">Styles:</span> {yeast.styles}
            </p>
          {/if}
          {#if yeast.substitutes}
            <p class="text-text-muted" style="font-size: 11px; margin: 0;">
              <span class="text-text-secondary">Comparables:</span> {yeast.substitutes}
            </p>
          {/if}
        </div>
        <div class="border-t border-border bg-bg-surface" style="padding: 12px 16px; display: flex; gap: 10px; align-items: flex-end; flex-shrink: 0;">
          <div>
            <div class="text-text-secondary" style="font-size: 11px; margin-bottom: 4px;">Packages</div>
            <input type="number" inputmode="decimal" step="1"
              value={amount}
              oninput={(e) => { const v = parseInt((e.target as HTMLInputElement).value, 10); if (!isNaN(v) && v > 0) amount = v; }}
              min="1"
              class="bg-bg-elevated border border-border text-text-primary" style="width: 60px; border-radius: 5px; padding: 5px 8px; font-size: 13px;" />
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
  dialog {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    margin: 0;
  }
  dialog::backdrop {
    background: rgba(0, 0, 0, 0.7);
  }
</style>
