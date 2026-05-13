<script lang="ts">
  import type { Recipe, Water, CalculatedWaterProfile, RecipeWaterAdjustment } from "$lib/api";
  import { 
    listWaterLibrary, 
    setRecipeWaterSources, 
    calculateWaterProfile,
    createWaterAdjustment,
    updateWaterAdjustment,
    deleteWaterAdjustment
  } from "$lib/api";
  import { ipc } from "$lib/stores/error";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let waters = $state<Water[]>([]);
  let profile = $state<CalculatedWaterProfile | null>(null);
  let adjustments = $state<RecipeWaterAdjustment[]>([]);
  let loading = $state(true);

  const ADDITIONS = [
    { value: "gypsum", label: "Gypsum" },
    { value: "calcium_chloride", label: "Calcium Chloride" },
    { value: "epsom_salt", label: "Epsom Salt" },
    { value: "table_salt", label: "Table Salt" },
    { value: "baking_soda", label: "Baking Soda" },
    { value: "chalk", label: "Chalk" },
    { value: "lactic_acid", label: "Lactic Acid" },
    { value: "phosphoric_acid", label: "Phosphoric Acid" },
  ];

  async function loadData() {
    loading = true;
    waters = await ipc(listWaterLibrary()) ?? [];
    adjustments = recipe.water_adjustments ?? [];
    await refreshProfile();
    loading = false;
  }

  async function refreshProfile() {
    profile = await ipc(calculateWaterProfile(recipe.id)) ?? null;
  }

  async function handleMashWaterChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    const mash_water_id = target.value || null;
    const updated = await ipc(setRecipeWaterSources(recipe.id, mash_water_id, recipe.sparge_water_id ?? null)) ?? recipe;
    recipe = updated;
    adjustments = updated.water_adjustments ?? [];
    await refreshProfile();
    onchange();
  }

  async function handleSpargeWaterChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    const sparge_water_id = target.value || null;
    const updated = await ipc(setRecipeWaterSources(recipe.id, recipe.mash_water_id ?? null, sparge_water_id)) ?? recipe;
    recipe = updated;
    adjustments = updated.water_adjustments ?? [];
    await refreshProfile();
    onchange();
  }

  async function handleAddAddition(target: "mash" | "sparge", addition: string) {
    const adj = await ipc(createWaterAdjustment(recipe.id, { addition: addition as any, target, amount: 0 })) ?? null;
    if (adj) {
      adjustments = [...adjustments, adj];
      await refreshProfile();
      onchange();
    }
  }

  async function handleUpdateAddition(id: string, amount: number) {
    const adj = await ipc(updateWaterAdjustment(id, { amount })) ?? null;
    if (adj) {
      adjustments = adjustments.map(a => a.id === id ? adj : a);
      await refreshProfile();
      onchange();
    }
  }

  async function handleDeleteAddition(id: string) {
    await ipc(deleteWaterAdjustment(id));
    adjustments = adjustments.filter(a => a.id !== id);
    await refreshProfile();
    onchange();
  }

  function getAdditionLabel(addition: string): string {
    return ADDITIONS.find(a => a.value === addition)?.label ?? addition;
  }

  function getRatioLabel(ratio: number): string {
    if (ratio < 0.5) return "Hoppy";
    if (ratio <= 1.5) return "Balanced";
    return "Malty";
  }

  $effect(() => {
    loadData();
  });
</script>

<div class="flex flex-col gap-6 max-w-4xl">
  {#if loading}
    <div style="color: var(--color-text-secondary);">Loading water data…</div>
  {:else}
    <!-- Source Water Section -->
    <div class="flex flex-col gap-3 p-4 rounded" style="background: var(--color-bg-surface); border: 1px solid var(--color-border);">
      <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Source Water</h3>
      
      <div class="flex flex-col gap-2">
        <label for="mash-water" class="text-xs font-medium" style="color: var(--color-text-secondary);">Mash Water</label>
        <select id="mash-water" value={recipe.mash_water_id ?? ""} onchange={handleMashWaterChange}
                class="px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="">— Select water profile —</option>
          {#each waters as water}
            <option value={water.id}>{water.name}</option>
          {/each}
        </select>
      </div>

      <div class="flex flex-col gap-2">
        <label for="sparge-water" class="text-xs font-medium" style="color: var(--color-text-secondary);">Sparge Water</label>
        <div class="flex gap-2 items-center">
          <select id="sparge-water" value={recipe.sparge_water_id ?? ""} onchange={handleSpargeWaterChange}
                  class="flex-1 px-2 py-1.5 rounded text-sm"
                  style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
            <option value="">— Same as mash —</option>
            {#each waters as water}
              <option value={water.id}>{water.name}</option>
            {/each}
          </select>
        </div>
      </div>
    </div>

    <!-- Additions Section -->
    <div class="flex flex-col gap-3 p-4 rounded" style="background: var(--color-bg-surface); border: 1px solid var(--color-border);">
      <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Mineral Adjustments</h3>
      
      {#each ["mash", "sparge"] as target}
        <div class="flex flex-col gap-2">
          <h4 class="text-xs font-medium capitalize" style="color: var(--color-text-secondary);">{target} Additions</h4>
          
          <div class="flex flex-col gap-1">
            {#each adjustments.filter(a => a.target === target) as adj}
              <div class="flex items-center gap-2">
                <span class="text-xs flex-1" style="color: var(--color-text-secondary);">{getAdditionLabel(adj.addition)}</span>
                <input type="number" step="0.1" min="0" value={adj.amount}
                       onchange={(e) => handleUpdateAddition(adj.id, parseFloat((e.target as HTMLInputElement).value) || 0)}
                       class="w-20 px-2 py-1 rounded text-xs"
                       style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
                <span class="text-xs" style="color: var(--color-text-muted); width: 30px;">g</span>
                <button onclick={() => handleDeleteAddition(adj.id)}
                        class="px-2 py-1 rounded text-xs"
                        style="background: var(--color-bg-elevated); color: var(--color-text-secondary); border: 1px solid var(--color-border);">
                  ✕
                </button>
              </div>
            {/each}
          </div>

          <div class="flex gap-1 flex-wrap">
            {#each ADDITIONS as addition}
              {#if !adjustments.some(a => a.target === target && a.addition === addition.value)}
                <button onclick={() => handleAddAddition(target as any, addition.value)}
                        class="px-2 py-1 rounded text-xs"
                        style="background: var(--color-bg-elevated); color: var(--color-text-secondary); border: 1px solid var(--color-border);">
                  + {addition.label}
                </button>
              {/if}
            {/each}
          </div>
        </div>
      {/each}
    </div>

    <!-- Profile Summary -->
    {#if profile}
      <div class="flex flex-col gap-3 p-4 rounded" style="background: var(--color-bg-surface); border: 1px solid var(--color-border);">
        <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Adjusted Profile</h3>
        
        <div class="grid grid-cols-3 gap-4">
          {#each [
            { label: "Calcium", key: "calcium_ppm" },
            { label: "Magnesium", key: "magnesium_ppm" },
            { label: "Sodium", key: "sodium_ppm" },
            { label: "Chloride", key: "chloride_ppm" },
            { label: "Sulfate", key: "sulfate_ppm" },
            { label: "Bicarbonate", key: "bicarbonate_ppm" }
          ] as item}
            <div class="flex flex-col gap-1">
              <span class="text-xs font-medium" style="color: var(--color-text-secondary);">{item.label}</span>
              <span class="text-sm font-semibold" style="color: var(--color-text-primary);">
                {profile.combined[item.key as keyof typeof profile.combined].toFixed(1)} ppm
              </span>
            </div>
          {/each}
        </div>

        <div class="flex flex-col gap-1 pt-2 border-t" style="border-color: var(--color-border);">
          <span class="text-xs font-medium" style="color: var(--color-text-secondary);">Cl:SO₄ Ratio</span>
          <div class="flex items-center gap-2">
            <span class="text-sm font-semibold" style="color: var(--color-text-primary);">
              {profile.combined.cl_so4_ratio.toFixed(2)}
            </span>
            <span class="text-xs px-2 py-1 rounded" 
                  style="background: var(--color-bg-elevated); color: var(--color-text-secondary);">
              {getRatioLabel(profile.combined.cl_so4_ratio)}
            </span>
          </div>
        </div>
      </div>
    {/if}
  {/if}
</div>
