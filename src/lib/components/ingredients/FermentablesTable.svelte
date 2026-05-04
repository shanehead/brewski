<script lang="ts">
  import type { Recipe, RecipeAdditionFermentable, Fermentable } from "$lib/api";
  import { listFermentableLibrary, createRecipeFermentable, updateRecipeFermentable, deleteRecipeFermentable } from "$lib/api";
  import { onMount } from "svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let library = $state<Fermentable[]>([]);
  let adding = $state(false);
  let selectedLibId = $state("");
  let amount = $state(1.0);

  onMount(async () => { library = await listFermentableLibrary(); });

  const selectedLib = $derived(library.find((f) => f.id === selectedLibId));

  async function handleAdd() {
    if (!selectedLib) return;
    await createRecipeFermentable(recipe.id, {
      fermentable_id: selectedLib.id,
      name: selectedLib.name,
      type_: selectedLib.type_,
      yield_pct: selectedLib.yield_pct,
      color_lovibond: selectedLib.color_lovibond,
      amount_kg: amount,
    });
    adding = false;
    selectedLibId = "";
    amount = 1.0;
    onchange();
  }

  async function handleAmountChange(f: RecipeAdditionFermentable, value: string) {
    const kg = parseFloat(value);
    if (!isNaN(kg) && kg > 0) {
      await updateRecipeFermentable(f.id, { amount_kg: kg });
      onchange();
    }
  }

  async function handleDelete(id: string) {
    await deleteRecipeFermentable(id);
    onchange();
  }
</script>

<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Fermentables</h3>
    <button onclick={() => adding = !adding} class="text-xs px-2 py-1 rounded"
            style="background: var(--color-accent); color: #fff;">+ Add</button>
  </div>

  {#if adding}
    <div class="flex gap-2 items-end p-2 rounded" style="background: var(--color-bg-elevated);">
      <div class="flex-1">
        <label class="text-xs mb-1 block" style="color: var(--color-text-secondary);">Fermentable</label>
        <select bind:value={selectedLibId} class="w-full px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="">Choose…</option>
          {#each library as f}
            <option value={f.id}>{f.name}</option>
          {/each}
        </select>
      </div>
      <div class="w-24">
        <label class="text-xs mb-1 block" style="color: var(--color-text-secondary);">Amount (kg)</label>
        <input type="number" step="0.1" bind:value={amount} min="0.01"
               class="w-full px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>
      <button onclick={handleAdd} class="text-xs px-3 py-1.5 rounded"
              style="background: var(--color-accent); color: #fff;">Add</button>
      <button onclick={() => adding = false} class="text-xs px-2 py-1.5 rounded"
              style="color: var(--color-text-secondary);">Cancel</button>
    </div>
  {/if}

  {#if recipe.fermentables.length > 0}
    <table class="w-full text-sm">
      <thead>
        <tr style="color: var(--color-text-muted);">
          <th class="text-left py-1 font-medium text-xs">Name</th>
          <th class="text-right py-1 font-medium text-xs">Lovibond</th>
          <th class="text-right py-1 font-medium text-xs">kg</th>
          <th class="w-6"></th>
        </tr>
      </thead>
      <tbody>
        {#each recipe.fermentables as f (f.id)}
          <tr class="border-t" style="border-color: var(--color-border);">
            <td class="py-1.5" style="color: var(--color-text-primary);">{f.name}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{f.color_lovibond}°L</td>
            <td class="text-right py-1.5">
              <input type="number" step="0.05" value={f.amount_kg}
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
