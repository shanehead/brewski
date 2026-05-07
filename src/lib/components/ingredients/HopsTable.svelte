<script lang="ts">
  import type { Recipe, Hop } from "$lib/api";
  import { listHopLibrary, createRecipeHop, deleteRecipeHop } from "$lib/api";
  import { onMount } from "svelte";
  import { settings } from "$lib/stores/settings";
  import { type Units, kgToHopDisplay, hopDisplayToKg, hopWeightLabel } from "$lib/units";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let library = $state<Hop[]>([]);
  let adding = $state(false);
  let selectedLibId = $state("");
  let amount = $state(0.028); // always kg internally

  const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");
  let use_ = $state("boil");
  let time = $state(60);

  onMount(async () => { library = await listHopLibrary(); });

  const selectedLib = $derived(library.find((h) => h.id === selectedLibId));

  async function handleAdd() {
    if (!selectedLib) return;
    await createRecipeHop(recipe.id, {
      hop_id: selectedLib.id,
      name: selectedLib.name,
      alpha_pct: selectedLib.alpha_pct,
      form: selectedLib.form,
      amount_kg: amount,
      use_,
      time_min: time,
    });
    adding = false;
    selectedLibId = "";
    onchange();
  }

  async function handleDelete(id: string) {
    await deleteRecipeHop(id);
    onchange();
  }

  const HOP_USES = ["boil", "aroma", "dry hop", "first wort", "whirlpool"] as const;
</script>

<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Hops</h3>
    <button onclick={() => adding = !adding} class="text-xs px-2 py-1 rounded"
            style="background: var(--color-accent); color: #fff;">+ Add</button>
  </div>

  {#if adding}
    <div class="flex flex-wrap gap-2 items-end p-2 rounded" style="background: var(--color-bg-elevated);">
      <div class="flex-1 min-w-32">
        <label for="hop-select" class="text-xs mb-1 block" style="color: var(--color-text-secondary);">Hop</label>
        <select id="hop-select" bind:value={selectedLibId} class="w-full px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="">Choose…</option>
          {#each library as h}
            <option value={h.id}>{h.name} ({h.alpha_pct}% AA)</option>
          {/each}
        </select>
      </div>
      <div class="w-20">
        <label for="hop-amount" class="text-xs mb-1 block" style="color: var(--color-text-secondary);">Amount ({hopWeightLabel(units)})</label>
        <input id="hop-amount" type="number" step={units === "imperial" ? 0.1 : 1}
               value={kgToHopDisplay(amount, units).toFixed(units === "imperial" ? 2 : 0)}
               oninput={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); if (!isNaN(v) && v > 0) amount = hopDisplayToKg(v, units); }}
               min="0.001"
               class="w-full px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>
      <div class="w-28">
        <label for="hop-use" class="text-xs mb-1 block" style="color: var(--color-text-secondary);">Use</label>
        <select id="hop-use" bind:value={use_} class="w-full px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          {#each HOP_USES as u}
            <option value={u}>{u}</option>
          {/each}
        </select>
      </div>
      <div class="w-16">
        <label for="hop-time" class="text-xs mb-1 block" style="color: var(--color-text-secondary);">Time (min)</label>
        <input id="hop-time" type="number" step="5" bind:value={time} min="0"
               class="w-full px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>
      <button onclick={handleAdd} class="text-xs px-3 py-1.5 rounded self-end"
              style="background: var(--color-accent); color: #fff;">Add</button>
      <button onclick={() => adding = false} class="text-xs px-2 py-1.5 rounded self-end"
              style="color: var(--color-text-secondary);">Cancel</button>
    </div>
  {/if}

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
