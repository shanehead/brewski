<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { PageData } from "./$types";
  import {
    getRecipe,
    getRecipeStats,
    updateRecipe,
    listRecipeVersions,
    getRecipeVersion,
    saveRecipeVersion,
    branchFromVersion,
    deleteRecipeVersion,
  } from "$lib/api";
  import type { Recipe, RecipeStats, RecipeVersionSummary } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import RecipeList from "$lib/components/RecipeList.svelte";
  import StatsSidebar from "$lib/components/StatsSidebar.svelte";
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";
  import OverviewTab from "$lib/components/tabs/OverviewTab.svelte";
  import IngredientsTab from "$lib/components/tabs/IngredientsTab.svelte";
  import MashTab from "$lib/components/tabs/MashTab.svelte";
  import WaterTab from "$lib/components/tabs/WaterTab.svelte";
  import FermentationTab from "$lib/components/tabs/FermentationTab.svelte";
  import NotesTab from "$lib/components/tabs/NotesTab.svelte";
  import BatchesTab from "$lib/components/tabs/BatchesTab.svelte";
  import VersionHistoryPanel from "$lib/components/VersionHistoryPanel.svelte";
  import type { BrewingIconName } from "$lib/icons";

  let { data }: { data: PageData } = $props();

  let recipe = $state<Recipe | null>(null);
  let stats = $state<RecipeStats | null>(null);
  let activeTab = $state<"overview" | "ingredients" | "mash" | "water" | "fermentation" | "notes" | "batches">("overview");
  let saving = $state(false);

  // Version history state
  let showVersionPanel = $state(false);
  let versions = $state<RecipeVersionSummary[]>([]);
  let viewingVersion = $state<RecipeVersionSummary | null>(null);
  let viewingRecipe = $state<Recipe | null>(null);

  // Save Version popover state
  let showSavePopover = $state(false);
  let saveVersionName = $state("");
  let savingVersion = $state(false);

  // Delete confirmation modal state
  let showDeleteModal = $state(false);
  let deleteCandidate = $state<RecipeVersionSummary | null>(null);

  const TABS: { key: "overview" | "ingredients" | "mash" | "water" | "fermentation" | "notes" | "batches"; label: string; icon: BrewingIconName }[] = [
    { key: "overview", label: "Overview", icon: "overview" },
    { key: "ingredients", label: "Ingredients", icon: "ingredients" },
    { key: "mash", label: "Mash", icon: "mash" },
    { key: "water", label: "Water", icon: "water" },
    { key: "fermentation", label: "Fermentation", icon: "fermentation" },
    { key: "batches", label: "Batches", icon: "batches" },
    { key: "notes", label: "Notes", icon: "notes" },
  ] as const;

  async function loadRecipeById(id: string) {
    recipe = await ipc(getRecipe(id)) ?? null;
    if (recipe) {
      stats = await ipc(getRecipeStats(recipe.id)) ?? null;
    } else {
      stats = null;
    }
  }

  async function loadVersions(id: string) {
    const result = await ipc(listRecipeVersions(id));
    if (result) {
      versions = result.sort((a, b) => b.created_at - a.created_at);
    }
  }

  onMount(async () => {
    await loadRecipeById(data.id);
    await loadVersions(data.id);
  });

  $effect(() => {
    if (data?.id) {
      (async () => {
        await loadRecipeById(data.id);
        await loadVersions(data.id);
      })();
    }
  });

  async function refreshRecipe() {
    await loadRecipeById(data.id);
    await loadVersions(data.id);
    viewingVersion = null;
    viewingRecipe = null;
  }

  async function handleNameBlur(e: FocusEvent) {
    const target = e.currentTarget as HTMLInputElement;
    if (!recipe || target.value === recipe.name) return;
    saving = true;
    recipe = await ipc(updateRecipe(recipe.id, { name: target.value })) ?? recipe;
    saving = false;
  }

  async function handleViewVersion(version: RecipeVersionSummary) {
    if (viewingVersion?.id === version.id) {
      viewingVersion = null;
      viewingRecipe = null;
      return;
    }
    viewingVersion = version;
    viewingRecipe = await ipc(getRecipeVersion(version.id)) ?? null;
  }

  async function handleBranchFromVersion(version: RecipeVersionSummary) {
    if (!recipe) return;
    const confirmed = confirm(
      `This will replace your current recipe with v${version.version_number}'s data. Continue?`
    );
    if (!confirmed) return;
    await ipc(branchFromVersion(recipe.id, version.id));
    await refreshRecipe();
  }

  async function handleDeleteVersion(version: RecipeVersionSummary) {
    // Open in-app modal instead of native confirm to ensure it's visible
    deleteCandidate = version;
    showDeleteModal = true;
  }

  async function confirmDelete() {
    if (!deleteCandidate || !recipe) return;
    showDeleteModal = false;
    console.log('confirmDelete invoking ipc for', deleteCandidate.id);
    const result = await ipc(deleteRecipeVersion(deleteCandidate.id));
    console.log('deleteRecipeVersion result', result);
    deleteCandidate = null;
    await refreshRecipe();
  }

  function cancelDelete() {
    deleteCandidate = null;
    showDeleteModal = false;
  }

  async function handleSaveVersion() {
    if (!recipe || !saveVersionName.trim()) return;
    savingVersion = true;
    await ipc(saveRecipeVersion({ recipe_id: recipe.id, name: saveVersionName.trim() }));
    showSavePopover = false;
    saveVersionName = "";
    savingVersion = false;
    await loadVersions(data.id);
  }

  const displayRecipe = $derived(viewingRecipe ?? recipe);
</script>

<RecipeList selectedId={data.id} />

{#if recipe}
  <div class="flex flex-1 flex-col overflow-hidden">
    <!-- Header -->
    <header
      class="flex items-center px-4 py-2 border-b gap-3 flex-shrink-0"
      style="background: var(--color-bg-surface); border-color: var(--color-border);"
    >
      <button
        onclick={() => goto("/")}
        class="text-xs px-2 py-1 rounded"
        style="color: var(--color-text-secondary); background: var(--color-bg-elevated);"
      >
        ← Recipes
      </button>
      <input
        value={recipe.name}
        onblur={handleNameBlur}
        disabled={viewingVersion !== null}
        class="flex-1 text-base font-semibold bg-transparent outline-none"
        style="color: var(--color-text-primary);"
      />
      {#if saving}
        <span class="text-xs" style="color: var(--color-text-muted);">Saving…</span>
      {/if}

      <!-- Save Version button -->
      <div class="relative">
        <button
          onclick={() => { showSavePopover = !showSavePopover; }}
          class="text-xs px-2 py-1 rounded"
          style="color: var(--color-text-secondary); background: var(--color-bg-elevated);"
        >
          Save Version
        </button>
        {#if showSavePopover}
          <div
            class="absolute right-0 top-full mt-1 p-3 rounded shadow-lg z-10 flex flex-col gap-2"
            style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); min-width: 200px;"
          >
            <input
              type="text"
              bind:value={saveVersionName}
              placeholder="Version name…"
              class="px-2 py-1 rounded text-sm outline-none"
              style="background: var(--color-bg-surface); color: var(--color-text-primary); border: 1px solid var(--color-border);"
              onkeydown={(e) => { if (e.key === "Enter") handleSaveVersion(); }}
            />
            <button
              onclick={handleSaveVersion}
              disabled={savingVersion || !saveVersionName.trim()}
              class="px-3 py-1 rounded text-sm"
              style="background: var(--color-accent); color: #fff;"
            >
              {savingVersion ? "Saving…" : "Save"}
            </button>
          </div>
        {/if}
      </div>

      <!-- Version history toggle -->
      <button
        onclick={() => { showVersionPanel = !showVersionPanel; }}
        class="text-xs px-2 py-1 rounded"
        style="
          color: {showVersionPanel ? '#fff' : 'var(--color-text-secondary)'};
          background: {showVersionPanel ? 'var(--color-accent)' : 'var(--color-bg-elevated)'};
        "
      >
        History ({versions.length})
      </button>
    </header>

    <!-- Read-only version banner -->
    {#if viewingVersion}
      <div
        class="flex items-center gap-3 px-4 py-2 text-sm flex-shrink-0"
        style="background: var(--color-bg-elevated); border-bottom: 1px solid var(--color-border);"
      >
        <span style="color: var(--color-text-secondary);">
          Viewing v{viewingVersion.version_number}
          {viewingVersion.name ? `· ${viewingVersion.name}` : ""}
          · {new Date(viewingVersion.created_at * 1000).toLocaleDateString()}
        </span>
        <button
          onclick={() => handleBranchFromVersion(viewingVersion!)}
          class="px-3 py-1 rounded text-sm"
          style="background: var(--color-accent); color: #fff;"
        >
          Branch from here
        </button>
        <button
          onclick={() => { viewingVersion = null; viewingRecipe = null; }}
          class="text-xs"
          style="color: var(--color-text-muted);"
        >
          Back to current
        </button>
      </div>
    {/if}

    <!-- Tab bar -->
    <nav
      class="flex border-b flex-shrink-0"
      style="background: var(--color-bg-surface); border-color: var(--color-border);"
    >
      {#each TABS as tab}
        <button
          onclick={() => activeTab = tab.key}
          class="px-4 py-2 text-sm border-b-2 transition-colors inline-flex items-center gap-2"
          style={activeTab === tab.key
            ? "border-color: var(--color-accent); color: var(--color-text-primary);"
            : "border-color: transparent; color: var(--color-text-secondary);"}
        >
          <BrewingIcon name={tab.icon} />
          {tab.label}
        </button>
      {/each}
    </nav>

    <!-- Tab content + stats sidebar + version panel -->
    <div class="flex flex-1 overflow-hidden">
      <div class="flex-1 overflow-y-auto p-4" style={viewingVersion ? "opacity: 0.85;" : ""} inert={viewingVersion !== null || undefined}>
        {#if displayRecipe}
          {#if activeTab === "overview"}
            <OverviewTab recipe={displayRecipe} onchange={refreshRecipe} />
          {:else if activeTab === "ingredients"}
            <IngredientsTab recipe={displayRecipe} onchange={refreshRecipe} />
          {:else if activeTab === "mash"}
            <MashTab recipe={displayRecipe} {stats} onchange={refreshRecipe} />
          {:else if activeTab === "water"}
            <WaterTab recipe={displayRecipe} onchange={refreshRecipe} />
          {:else if activeTab === "fermentation"}
            <FermentationTab recipe={displayRecipe} onchange={refreshRecipe} />
          {:else if activeTab === "notes"}
            <NotesTab recipe={displayRecipe} onchange={refreshRecipe} />
          {:else if activeTab === "batches"}
            <BatchesTab recipeId={recipe.id} />
          {/if}
        {/if}
      </div>
      <StatsSidebar {stats} />
      {#if showVersionPanel}
        <VersionHistoryPanel
          {versions}
          viewingVersionId={viewingVersion?.id ?? null}
          onview={handleViewVersion}
          onbranch={handleBranchFromVersion}
          ondelete={handleDeleteVersion}
          onclose={() => showVersionPanel = false}
        />
      {/if}
    </div>
    {#if showDeleteModal && deleteCandidate}
      <div class="fixed inset-0 flex items-center justify-center" style="z-index: 1000;">
        <div class="absolute inset-0" style="background: rgba(0,0,0,0.4);"></div>
        <div class="bg-var p-4 rounded" style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); z-index: 1001; min-width: 320px;">
          <div class="text-sm mb-3" style="color: var(--color-text-primary);">Delete v{deleteCandidate.version_number}{deleteCandidate.name ? ` \"${deleteCandidate.name}\"` : ""}?</div>
          <div class="flex justify-end gap-2">
            <button onclick={cancelDelete} class="px-3 py-1 rounded" style="background: var(--color-bg-surface); color: var(--color-text-primary); border: 1px solid var(--color-border);">Cancel</button>
            <button onclick={confirmDelete} class="px-3 py-1 rounded" style="background: var(--color-accent); color: #fff;">Delete</button>
          </div>
        </div>
      </div>
    {/if}

  </div>
{:else}
  <div class="flex-1 flex items-center justify-center">
    <p class="text-sm" style="color: var(--color-text-muted);">Loading…</p>
  </div>
{/if}
