<script lang="ts">
  import { onMount } from "svelte";
  import { goto, afterNavigate } from "$app/navigation";
  import { getRecipe, getRecipeStats, getRecipeBeerxml, uploadRecipeImage, deleteRecipeImage } from "$lib/api";
  import type { Recipe, RecipeStats } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { appDataDir as getAppDataDir } from "@tauri-apps/api/path";
  import RecipeHero from "$lib/components/RecipeHero.svelte";
  import ScaleRecipeModal from "$lib/components/ScaleRecipeModal.svelte";
  import { settings } from "$lib/stores/settings";
  import { formatSg } from "$lib/gravity-display";
  import OverviewTab from "$lib/components/tabs/OverviewTab.svelte";
  import IngredientsTab from "$lib/components/tabs/IngredientsTab.svelte";
  import MashTab from "$lib/components/tabs/MashTab.svelte";
  import WaterTab from "$lib/components/tabs/WaterTab.svelte";
  import FermentationTab from "$lib/components/tabs/FermentationTab.svelte";
  import NotesTab from "$lib/components/tabs/NotesTab.svelte";

  let { id }: { id: string } = $props();

  let recipe = $state<Recipe | null>(null);
  let stats = $state<RecipeStats | null>(null);
  let appDataDir = $state("");
  let fileInput = $state<HTMLInputElement | null>(null);
  let showScaleModal = $state(false);

  const gravityUnit = $derived($settings.gravity_unit ?? "sg");
  const displayOg = $derived(stats?.og != null ? formatSg(stats.og, gravityUnit) : "—");
  const displayFg = $derived(stats?.fg != null ? formatSg(stats.fg, gravityUnit) : "—");

  async function load() {
    recipe = await ipc(getRecipe(id)) ?? null;
    if (recipe) stats = await ipc(getRecipeStats(recipe.id)) ?? null;
  }

  onMount(async () => {
    appDataDir = await getAppDataDir();
    await load();
  });
  $effect(() => { if (id) load(); });

  // Reload stats when navigating back from another section (e.g., after editing equipment).
  afterNavigate(({ from, to }) => {
    if (from && from.url.pathname !== to?.url.pathname) {
      load();
    }
  });

  async function handleExport() {
    if (!recipe) return;
    const xml = await ipc(getRecipeBeerxml(recipe.id));
    if (!xml) return;
    const blob = new Blob([xml], { type: "application/xml" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `${recipe.name}.xml`;
    a.click();
    URL.revokeObjectURL(url);
  }

  async function handleImageUpload() {
    fileInput?.click();
  }

  async function handleFileSelected(event: Event) {
    if (!recipe) return;
    const file = (event.target as HTMLInputElement).files?.[0];
    if (!file) return;
    const path = (file as File & { path?: string }).path;
    if (!path) return;
    recipe = await ipc(uploadRecipeImage({ recipe_id: recipe.id, source_path: path })) ?? recipe;
    if (fileInput) fileInput.value = "";
  }

  async function handleImageRemove() {
    if (!recipe) return;
    recipe = await ipc(deleteRecipeImage({ recipe_id: recipe.id })) ?? recipe;
  }

  function fmt(val: number | undefined, decimals = 3): string {
    if (val === undefined || val === null) return "—";
    return val.toFixed(decimals);
  }
</script>

{#if recipe}
  <div class="flex flex-col h-full overflow-hidden bg-bg-base">
    <!-- Header with back button -->
    <div class="flex items-center gap-3 px-4 py-3 border-b flex-shrink-0 bg-bg-surface border-border"
        >
      <button
        onclick={() => goto("/")}
        class="text-sm text-accent"
       
      >‹ Recipes</button>
      <span class="flex-1 font-semibold text-base truncate text-text-primary"
           >{recipe.name}</span>
      <button
        onclick={handleExport}
        aria-label="Export BeerXML"
        class="flex items-center justify-center rounded flex-shrink-0 text-text-secondary bg-bg-elevated border border-border"
        style="width: 28px; height: 28px; border-radius: var(--radius-md);"
      >
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
          <polyline points="7 10 12 15 17 10"/>
          <line x1="12" y1="15" x2="12" y2="3"/>
        </svg>
      </button>
      <button
        onclick={() => { showScaleModal = true; }}
        class="flex items-center justify-center rounded flex-shrink-0 text-xs px-2 text-text-secondary bg-bg-elevated border border-border"
        style="height: 28px; border-radius: var(--radius-md);"
      >
        Scale
      </button>
    </div>

    <input
      type="file"
      accept="image/*"
      bind:this={fileInput}
      onchange={handleFileSelected}
      class="hidden"
    />

    {#if showScaleModal && recipe}
      <ScaleRecipeModal
        recipeId={recipe.id}
        currentBatchSizeL={recipe.batch_size_l}
        onClose={() => { showScaleModal = false; }}
      />
    {/if}

    <RecipeHero
      recipe={{ ...recipe, srm: stats?.srm ?? null }}
      {appDataDir}
      height="160px"
      onUploadClick={handleImageUpload}
      onRemoveClick={handleImageRemove}
    />

    <!-- Single scroll -->
    <div class="flex-1 overflow-y-auto">
      <div class="p-4 flex flex-col gap-6">

        <!-- Stats card -->
        {#if stats}
          <div class="rounded-lg p-4 grid grid-cols-5 gap-2 bg-bg-elevated border border-border"
              >
            {#each [
              { label: "OG", value: displayOg },
              { label: "FG", value: displayFg },
              { label: "ABV", value: fmt(stats.abv_pct, 1) + "%" },
              { label: "IBU", value: fmt(stats.ibu, 0) },
              { label: "SRM", value: fmt(stats.srm, 1) },
            ] as stat}
              <div class="text-center">
                <div class="text-sm font-semibold font-mono text-accent">{stat.value}</div>
                <div class="text-xs text-text-muted">{stat.label}</div>
              </div>
            {/each}
          </div>
        {/if}

        <!-- Overview -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3 text-text-secondary"
              >Overview</div>
          <OverviewTab {recipe} onchange={load} />
        </section>

        <!-- Ingredients -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3 text-text-secondary"
              >Ingredients</div>
          <IngredientsTab {recipe} {stats} onchange={load} />
        </section>

        <!-- Mash -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3 text-text-secondary"
              >Mash</div>
          <MashTab {recipe} {stats} onchange={load} />
        </section>

        <!-- Water -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3 text-text-secondary"
              >Water</div>
          <WaterTab {recipe} {stats} onchange={load} />
        </section>

        <!-- Fermentation -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3 text-text-secondary"
              >Fermentation</div>
          <FermentationTab {recipe} onchange={load} />
        </section>

        <!-- Notes -->
        <section>
          <div class="text-xs font-semibold uppercase tracking-wider mb-3 text-text-secondary"
              >Notes</div>
          <NotesTab {recipe} onchange={load} />
        </section>

      </div>
    </div>
  </div>
{:else}
  <div class="flex-1 flex items-center justify-center">
    <p class="text-sm text-text-muted">Loading…</p>
  </div>
{/if}
