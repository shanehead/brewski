<script lang="ts">
  import { onMount } from "svelte";
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
  import Card from "$lib/components/Card.svelte";
  import FieldLabel from "$lib/components/FieldLabel.svelte";
  import Tooltip from "$lib/components/Tooltip.svelte";
  import DocLink from "$lib/components/DocLink.svelte";
  import { DOCS } from "$lib/docs-urls";
  import TabContent from "$lib/components/tabs/TabContent.svelte";

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
    const updated = await ipc(setRecipeWaterSources(recipe.id, mash_water_id, recipe.sparge_water_id ?? null));
    adjustments = updated?.water_adjustments ?? [];
    await refreshProfile();
    onchange();
  }

  async function handleSpargeWaterChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    const sparge_water_id = target.value || null;
    const updated = await ipc(setRecipeWaterSources(recipe.id, recipe.mash_water_id ?? null, sparge_water_id));
    adjustments = updated?.water_adjustments ?? [];
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

  onMount(() => {
    loadData();
  });
</script>

<TabContent width="3xl">
  <div class="flex justify-end mb-2">
    <DocLink label="Water chemistry guide" url={DOCS.waterChemistry} />
  </div>
  {#if loading}
    <div style="color: var(--color-text-secondary);">Loading water data…</div>
  {:else}
    <Card title="Source Water">
      <div class="flex flex-col gap-3">
        <div class="flex flex-col gap-1">
          <FieldLabel for="mash-water">Mash Water</FieldLabel>
          <select id="mash-water" value={recipe.mash_water_id ?? ""} onchange={handleMashWaterChange}
                  class="px-2 py-1.5 rounded text-sm"
                  style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
            <option value="">— Select water profile —</option>
            {#each waters as water}
              <option value={water.id}>{water.name}</option>
            {/each}
          </select>
        </div>
        <div class="flex flex-col gap-1">
          <FieldLabel for="sparge-water">Sparge Water</FieldLabel>
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
    </Card>

    <Card title="Mineral Adjustments">
      <div class="flex flex-col gap-5">
        {#each ["mash", "sparge"] as target}
          <div class="flex flex-col gap-2">
            <h4 class="text-xs font-semibold uppercase tracking-wider capitalize" style="color: var(--color-text-muted);">{target} Additions</h4>
            <div class="flex flex-col gap-1">
              {#each adjustments.filter(a => a.target === target) as adj}
                <div class="flex items-center gap-2">
                  <span class="text-sm flex-1" style="color: var(--color-text-secondary);">{getAdditionLabel(adj.addition)}</span>
                  <input type="number" inputmode="decimal" step="0.1" min="0" value={adj.amount}
                         aria-label="{getAdditionLabel(adj.addition)} amount in grams"
                         onchange={(e) => handleUpdateAddition(adj.id, parseFloat((e.target as HTMLInputElement).value) || 0)}
                         class="w-20 px-2 py-1 rounded text-sm"
                         style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
                  <span class="text-sm" style="color: var(--color-text-muted); width: 30px;">g</span>
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
    </Card>

    {#if profile}
      <Card title="Adjusted Profile">
        <div class="flex flex-col gap-3">
          <div class="grid grid-cols-3 gap-4">
            {#each [
              { label: "Calcium", key: "calcium_ppm", tooltip: "Promotes enzyme activity, yeast health, and clarity. Target 50–150 ppm for most styles." },
              { label: "Magnesium", key: "magnesium_ppm", tooltip: "A yeast nutrient at low levels. Tastes harsh above 30 ppm — keep it under that." },
              { label: "Sodium", key: "sodium_ppm", tooltip: "Adds roundness and fullness at under 150 ppm. Salty and harsh above it." },
              { label: "Chloride", key: "chloride_ppm", tooltip: "Accentuates malt character and body. Higher Cl:SO₄ ratio makes beer taste maltier." },
              { label: "Sulfate", key: "sulfate_ppm", tooltip: "Accentuates hop dryness and bitterness. Higher SO₄:Cl ratio makes beer taste drier." },
              { label: "Bicarbonate", key: "bicarbonate_ppm", tooltip: "Raises mash pH. Useful for dark malts. Keep it low for pale styles." }
            ] as item}
              <div class="flex flex-col gap-0.5">
                <span class="text-xs inline-flex items-center gap-1" style="color: var(--color-text-secondary);">{item.label} <Tooltip text={item.tooltip} /></span>
                <span class="text-base font-semibold font-mono" style="color: var(--color-text-primary);">
                  {profile.combined[item.key as keyof typeof profile.combined].toFixed(1)}<span class="text-xs font-normal ml-0.5" style="color: var(--color-text-muted);">ppm</span>
                </span>
              </div>
            {/each}
          </div>
          <div class="pt-2 border-t flex items-center gap-3" style="border-color: var(--color-border);">
            <div class="flex flex-col gap-0.5">
              <span class="text-xs" style="color: var(--color-text-secondary);">Cl:SO₄ Ratio</span>
              <span class="text-base font-semibold font-mono" style="color: var(--color-text-primary);">{profile.combined.cl_so4_ratio.toFixed(2)}</span>
            </div>
            <span class="text-sm px-2 py-1 rounded"
                  style="background: var(--color-bg-elevated); color: var(--color-text-secondary);">
              {getRatioLabel(profile.combined.cl_so4_ratio)}
            </span>
          </div>
        </div>
      </Card>
    {/if}
  {/if}
</TabContent>
