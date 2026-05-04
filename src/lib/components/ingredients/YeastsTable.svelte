<script lang="ts">
  import type { Recipe, Yeast } from "$lib/api";
  import { listYeastLibrary, createRecipeYeast, deleteRecipeYeast } from "$lib/api";
  import { onMount } from "svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let library = $state<Yeast[]>([]);
  let adding = $state(false);
  let selectedLibId = $state("");

  onMount(async () => { library = await listYeastLibrary(); });

  const selectedLib = $derived(library.find((y) => y.id === selectedLibId));

  async function handleAdd() {
    if (!selectedLib) return;
    await createRecipeYeast(recipe.id, {
      yeast_id: selectedLib.id,
      name: selectedLib.name,
      type_: selectedLib.type_,
      form: selectedLib.form,
      laboratory: selectedLib.laboratory,
      product_id: selectedLib.product_id,
      attenuation_pct: selectedLib.attenuation_pct,
      amount: 1,
    });
    adding = false;
    selectedLibId = "";
    onchange();
  }

  async function handleDelete(id: string) {
    await deleteRecipeYeast(id);
    onchange();
  }
</script>

<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Yeast</h3>
    <button onclick={() => adding = !adding} class="text-xs px-2 py-1 rounded"
            style="background: var(--color-accent); color: #fff;">+ Add</button>
  </div>

  {#if adding}
    <div class="flex gap-2 items-end p-2 rounded" style="background: var(--color-bg-elevated);">
      <div class="flex-1">
        <label for="yeast-select" class="text-xs mb-1 block" style="color: var(--color-text-secondary);">Yeast</label>
        <select id="yeast-select" bind:value={selectedLibId} class="w-full px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="">Choose…</option>
          {#each library as y}
            <option value={y.id}>{y.name} ({y.laboratory ?? y.form})</option>
          {/each}
        </select>
      </div>
      <button onclick={handleAdd} class="text-xs px-3 py-1.5 rounded"
              style="background: var(--color-accent); color: #fff;">Add</button>
      <button onclick={() => adding = false} class="text-xs px-2 py-1.5 rounded"
              style="color: var(--color-text-secondary);">Cancel</button>
    </div>
  {/if}

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
