<script lang="ts">
  import { onMount } from "svelte";
  import { goto, afterNavigate } from "$app/navigation";
  import { page } from "$app/stores";
  import {
    getRecipe,
    getRecipeStats,
    updateRecipe,
    listRecipeVersions,
    getRecipeVersion,
    saveRecipeVersion,
    branchFromVersion,
    deleteRecipeVersion,
    writeRecipeBeerxml,
  } from "$lib/api";
  import type { Recipe, RecipeStats, RecipeVersionSummary } from "$lib/api";
  import { ipc, lastError } from "$lib/stores/error";
  import { refreshRecipeList } from "$lib/stores/recipes";
  import { save } from "@tauri-apps/plugin-dialog";
  import { appDataDir as getAppDataDir } from "@tauri-apps/api/path";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { srmToHex } from "$lib/utils/srm";
  import { open } from "@tauri-apps/plugin-dialog";
  import { uploadRecipeImage, deleteRecipeImage } from "$lib/api";
  import RecipeList from "$lib/components/RecipeList.svelte";
  import StatsSidebar from "$lib/components/StatsSidebar.svelte";
  import OverviewTab from "$lib/components/tabs/OverviewTab.svelte";
  import IngredientsTab from "$lib/components/tabs/IngredientsTab.svelte";
  import MashTab from "$lib/components/tabs/MashTab.svelte";
  import WaterTab from "$lib/components/tabs/WaterTab.svelte";
  import FermentationTab from "$lib/components/tabs/FermentationTab.svelte";
  import NotesTab from "$lib/components/tabs/NotesTab.svelte";
  import BatchesTab from "$lib/components/tabs/BatchesTab.svelte";
  import TabBar from "$lib/components/TabBar.svelte";
  import VersionHistoryPanel from "$lib/components/VersionHistoryPanel.svelte";
  import ConfirmModal from "$lib/components/ConfirmModal.svelte";
  import ScaleRecipeModal from "$lib/components/ScaleRecipeModal.svelte";
  import type { BrewingIconName } from "$lib/icons";

  let { id }: { id: string } = $props();

  let recipe = $state<Recipe | null>(null);
  let stats = $state<RecipeStats | null>(null);

  const VALID_TABS = ["overview", "ingredients", "mash", "water", "fermentation", "notes", "batches"] as const;
  type TabKey = typeof VALID_TABS[number];

  const activeTab = $derived.by<TabKey>(() => {
    const raw = $page.url.searchParams.get("tab") ?? "";
    return (VALID_TABS as readonly string[]).includes(raw) ? (raw as TabKey) : "overview";
  });
  let saving = $state(false);
  let appDataDir = $state("");

  // Version history state
  let showVersionPanel = $state(false);
  let versions = $state<RecipeVersionSummary[]>([]);
  let viewingVersion = $state<RecipeVersionSummary | null>(null);
  let viewingRecipe = $state<Recipe | null>(null);

  // Save Version popover state
  let showSavePopover = $state(false);
  let saveVersionName = $state("");
  let savingVersion = $state(false);

  // Image popover state
  let showImagePopover = $state(false);

  // Delete confirmation modal state
  let showDeleteModal = $state(false);
  let deleteCandidate = $state<RecipeVersionSummary | null>(null);

  // Branch confirmation modal state
  let showBranchModal = $state(false);
  let branchCandidate = $state<RecipeVersionSummary | null>(null);

  // Scale recipe modal state
  let showScaleModal = $state(false);

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
    appDataDir = await getAppDataDir();
    await loadRecipeById(id);
    await loadVersions(id);
  });

  $effect(() => {
    if (id) {
      (async () => {
        await loadRecipeById(id);
        await loadVersions(id);
      })();
    }
  });

  // Reload stats when navigating back from another section (e.g., after editing equipment).
  afterNavigate(({ from, to }) => {
    if (from && from.url.pathname !== to?.url.pathname) {
      loadRecipeById(id);
    }
  });

  async function refreshRecipe() {
    await loadRecipeById(id);
    await loadVersions(id);
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

  function handleBranchFromVersion(version: RecipeVersionSummary) {
    branchCandidate = version;
    showBranchModal = true;
  }

  async function confirmBranch() {
    if (!branchCandidate || !recipe) return;
    showBranchModal = false;
    const candidate = branchCandidate;
    branchCandidate = null;
    await ipc(branchFromVersion(recipe.id, candidate.id));
    await refreshRecipe();
  }

  function cancelBranch() {
    branchCandidate = null;
    showBranchModal = false;
  }

  async function handleDeleteVersion(version: RecipeVersionSummary) {
    // Open in-app modal instead of native confirm to ensure it's visible
    deleteCandidate = version;
    showDeleteModal = true;
  }

  async function confirmDelete() {
    if (!deleteCandidate || !recipe) return;
    showDeleteModal = false;
    const candidate = deleteCandidate;
    deleteCandidate = null;
    try {
      await deleteRecipeVersion(candidate.id);
      await refreshRecipe();
    } catch (e) {
      lastError.set(String(e));
    }
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
    await loadVersions(id);
  }

  async function handleExport() {
    if (!recipe) return;
    const path = await save({
      defaultPath: `${recipe.name}.xml`,
      filters: [{ name: "BeerXML", extensions: ["xml"] }],
    });
    if (!path) return;
    await ipc(writeRecipeBeerxml(recipe.id, path));
  }

  async function handleImageUpload() {
    if (!recipe) return;
    const path = await open({
      filters: [{ name: "Image", extensions: ["jpg", "jpeg", "png", "webp", "heic"] }],
    });
    if (!path || typeof path !== "string") return;
    recipe = await ipc(uploadRecipeImage({ recipe_id: recipe.id, source_path: path })) ?? recipe;
    refreshRecipeList();
  }

  async function handleImageRemove() {
    if (!recipe) return;
    recipe = await ipc(deleteRecipeImage({ recipe_id: recipe.id })) ?? recipe;
    refreshRecipeList();
  }

  const displayRecipe = $derived(viewingRecipe ?? recipe);
  const recipeImageSrc = $derived(
    recipe?.image_path
      ? convertFileSrc(`${appDataDir}/images/${recipe.image_path}`)
      : null
  );
  const srmColor1 = $derived(srmToHex(stats?.srm ?? 4));
  const srmColor2 = $derived(srmToHex(Math.min((stats?.srm ?? 4) + 12, 40)));
</script>

<RecipeList selectedId={id} />

{#if recipe}
  <div class="flex flex-1 flex-col overflow-hidden">
    <!-- Header -->
    <header
      class="flex items-center px-4 py-2 border-b gap-3 flex-shrink-0 bg-bg-surface border-border"
     
    >
      <button
        onclick={() => goto("/")}
        class="text-xs px-2 py-1 rounded text-text-secondary bg-bg-elevated border border-border"
       
      >
        ← Recipes
      </button>

      <!-- Recipe image thumbnail + popover -->
      <div class="relative flex-shrink-0">
        <button
          onclick={() => { showImagePopover = !showImagePopover; }}
          aria-label="Photo options"
          class="w-10 h-10 rounded overflow-hidden block border border-border"
         
        >
          {#if recipeImageSrc}
            <img src={recipeImageSrc} alt="" class="w-full h-full object-cover" />
          {:else}
            <div class="w-full h-full" style="background: linear-gradient(135deg, {srmColor1}, {srmColor2});"></div>
          {/if}
        </button>
        {#if showImagePopover}
          <div
            class="absolute left-0 top-full mt-1 rounded shadow-lg z-20 flex flex-col overflow-hidden bg-bg-elevated border border-border"
            style="min-width: 140px;"
          >
            <button
              onclick={() => { showImagePopover = false; handleImageUpload(); }}
              class="px-3 py-2 text-left text-sm hover:bg-[var(--color-bg-surface)] transition-colors text-text-primary"
             
            >{recipe?.image_path ? "Change photo" : "Add photo"}</button>
            {#if recipe?.image_path}
              <button
                onclick={() => { showImagePopover = false; handleImageRemove(); }}
                class="px-3 py-2 text-left text-sm hover:bg-[var(--color-bg-surface)] transition-colors text-text-danger"
              >Remove photo</button>
            {/if}
          </div>
          <!-- Click-away backdrop -->
          <button
            class="fixed inset-0 z-10"
            style="background: transparent;"
            onclick={() => { showImagePopover = false; }}
            aria-label="Close menu"
            tabindex="-1"
          ></button>
        {/if}
      </div>

      <input
        value={recipe.name}
        onblur={handleNameBlur}
        disabled={viewingVersion !== null}
        class="flex-1 text-base font-semibold bg-transparent outline-none text-text-primary"
       
      />
      {#if saving}
        <span class="text-xs text-text-muted">Saving…</span>
      {/if}

      <!-- Save Version button -->
      <div class="relative">
        <button
          onclick={() => { showSavePopover = !showSavePopover; }}
          class="text-xs px-2 py-1 rounded text-text-secondary bg-bg-elevated border border-border"
         
        >
          Save Version
        </button>
        {#if showSavePopover}
          <div
            class="absolute right-0 top-full mt-1 p-3 rounded shadow-lg z-10 flex flex-col gap-2 bg-bg-elevated border border-border"
            style="min-width: 200px;"
          >
            <input
              type="text"
              bind:value={saveVersionName}
              placeholder="Version name…"
              class="px-2 py-1 rounded text-sm outline-none bg-bg-surface text-text-primary border border-border"
             
              onkeydown={(e) => { if (e.key === "Enter") handleSaveVersion(); }}
            />
            <button
              onclick={handleSaveVersion}
              disabled={savingVersion || !saveVersionName.trim()}
              class="px-3 py-1 rounded text-sm bg-accent"
              style="color: #fff;"
            >
              {savingVersion ? "Saving…" : "Save"}
            </button>
          </div>
        {/if}
      </div>

      <!-- Scale Recipe button -->
      <button
        onclick={() => { showScaleModal = true; }}
        class="text-xs px-2 py-1 rounded text-text-secondary bg-bg-elevated border border-border"
       
      >
        Scale Recipe
      </button>

      <!-- Export BeerXML button -->
      <button
        onclick={handleExport}
        class="flex items-center gap-1 text-xs px-2 py-1 rounded transition-colors text-text-secondary bg-bg-elevated border border-border"
       
      >
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
          <polyline points="7 10 12 15 17 10"/>
          <line x1="12" y1="15" x2="12" y2="3"/>
        </svg>
        Export BeerXML
      </button>

      <!-- Version history toggle -->
      <button
        onclick={() => { showVersionPanel = !showVersionPanel; }}
        class="text-xs px-2 py-1 rounded"
        style="
          color: {showVersionPanel ? '#fff' : 'var(--color-text-secondary)'};
          background: {showVersionPanel ? 'var(--color-accent)' : 'var(--color-bg-elevated)'};
          border: 1px solid {showVersionPanel ? 'transparent' : 'var(--color-border)'};
        "
      >
        History ({versions.length})
      </button>
    </header>

    <!-- Read-only version banner -->
    {#if viewingVersion}
      <div
        class="flex items-center gap-3 px-4 py-2 text-sm flex-shrink-0 bg-bg-elevated border-b border-border"
       
      >
        <span class="text-text-secondary">
          Viewing v{viewingVersion.version_number}
          {viewingVersion.name ? `· ${viewingVersion.name}` : ""}
          · {new Date(viewingVersion.created_at * 1000).toLocaleDateString()}
        </span>
        <button
          onclick={() => handleBranchFromVersion(viewingVersion!)}
          class="px-3 py-1 rounded text-sm bg-accent"
          style="color: #fff;"
        >
          Branch from here
        </button>
        <button
          onclick={() => { viewingVersion = null; viewingRecipe = null; }}
          class="text-xs text-text-muted"
         
        >
          Back to current
        </button>
      </div>
    {/if}

    <!-- Tab bar -->
    <nav class="px-4 pt-1 flex-shrink-0 bg-bg-surface"
        >
      <TabBar tabs={TABS} active={activeTab} onchange={(key) => goto(`/recipe/${id}?tab=${key}`, { replaceState: true, noScroll: true })} />
    </nav>

    <!-- Tab content + stats sidebar + version panel -->
    <div class="flex flex-1 overflow-hidden">
      <div class="flex-1 overflow-y-auto p-4" style={viewingVersion ? "opacity: 0.85;" : ""} inert={viewingVersion !== null || undefined}>
        {#if displayRecipe}
          {#if activeTab === "overview"}
            <OverviewTab recipe={displayRecipe} onchange={refreshRecipe} />
          {:else if activeTab === "ingredients"}
            <IngredientsTab recipe={displayRecipe} {stats} onchange={refreshRecipe} />
          {:else if activeTab === "mash"}
            <MashTab recipe={displayRecipe} {stats} onchange={refreshRecipe} />
          {:else if activeTab === "water"}
            <WaterTab recipe={displayRecipe} {stats} onchange={refreshRecipe} />
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
      <ConfirmModal
        message="Delete v{deleteCandidate.version_number}{deleteCandidate.name ? ` \"${deleteCandidate.name}\"` : ''}? This cannot be undone."
        confirmLabel="Delete"
        dangerous={true}
        onconfirm={confirmDelete}
        oncancel={cancelDelete}
      />
    {/if}
    {#if showBranchModal && branchCandidate}
      <ConfirmModal
        message="Replace your current recipe with v{branchCandidate.version_number}'s data? This cannot be undone."
        confirmLabel="Branch from here"
        onconfirm={confirmBranch}
        oncancel={cancelBranch}
      />
    {/if}
    {#if showScaleModal && recipe}
      <ScaleRecipeModal
        recipeId={recipe.id}
        currentBatchSizeL={recipe.batch_size_l}
        onClose={() => { showScaleModal = false; }}
      />
    {/if}

  </div>
{:else}
  <div class="flex-1 flex items-center justify-center">
    <p class="text-sm text-text-muted">Loading…</p>
  </div>
{/if}
