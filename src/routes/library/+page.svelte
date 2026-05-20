<script lang="ts">
  import { onMount, tick } from 'svelte';
  import type { Hop, Fermentable, Yeast, Misc, Water } from '$lib/api';
  import {
    listHopLibrary, deleteHop,
    listFermentableLibrary, deleteFermentable,
    listYeastLibrary, deleteYeast,
    listMiscLibrary, deleteMisc,
    listWaterLibrary, deleteWater,
  } from '$lib/api';
  import { ipc } from '$lib/stores/error';
  import IngredientEditModal from '$lib/components/ingredients/IngredientEditModal.svelte';
  import ConfirmModal from '$lib/components/ConfirmModal.svelte';

  type Tab = 'hop' | 'fermentable' | 'yeast' | 'misc' | 'water';
  type AnyIngredient = Hop | Fermentable | Yeast | Misc | Water;

  let activeTab = $state<Tab>('hop');
  let query = $state('');

  let hops = $state<Hop[]>([]);
  let fermentables = $state<Fermentable[]>([]);
  let yeasts = $state<Yeast[]>([]);
  let miscs = $state<Misc[]>([]);
  let waters = $state<Water[]>([]);
  let loaded = $state<Record<Tab, boolean>>({ hop: false, fermentable: false, yeast: false, misc: false, water: false });

  let editModalOpen = $state(false);
  let editType = $state<Tab>('hop');
  let editIngredient = $state<AnyIngredient | null>(null);

  let deleteModalOpen = $state(false);
  let deleteCandidate = $state<AnyIngredient | null>(null);
  let deleteType = $state<Tab>('hop');

  const TAB_LABELS: Record<Tab, string> = { hop: 'Hops', fermentable: 'Fermentables', yeast: 'Yeasts', misc: 'Misc', water: 'Water' };

  async function loadTab(tab: Tab) {
    if (loaded[tab]) return;
    if (tab === 'hop') hops = (await ipc(listHopLibrary())) ?? [];
    else if (tab === 'fermentable') fermentables = (await ipc(listFermentableLibrary())) ?? [];
    else if (tab === 'yeast') yeasts = (await ipc(listYeastLibrary())) ?? [];
    else if (tab === 'misc') miscs = (await ipc(listMiscLibrary())) ?? [];
    else waters = (await ipc(listWaterLibrary())) ?? [];
    loaded[tab] = true;
  }

  async function refreshTab(tab: Tab) {
    loaded[tab] = false;
    await loadTab(tab);
  }

  onMount(() => loadTab('hop'));

  $effect(() => { loadTab(activeTab); });

  function switchTab(tab: Tab) {
    activeTab = tab;
    query = '';
  }

  const currentList = $derived<AnyIngredient[]>(
    activeTab === 'hop' ? hops :
    activeTab === 'fermentable' ? fermentables :
    activeTab === 'yeast' ? yeasts :
    activeTab === 'misc' ? miscs :
    waters
  );

  const filtered = $derived(
    query.trim() === ''
      ? currentList
      : currentList.filter(i => i.name.toLowerCase().includes(query.trim().toLowerCase()))
  );

  const existingNames = $derived(currentList.map(i => i.name));

  function rowSubtext(item: AnyIngredient): string {
    if (activeTab === 'hop') return `${(item as Hop).alpha_pct}% AA · ${(item as Hop).form}`;
    if (activeTab === 'fermentable') return `${(item as Fermentable).type_} · ${(item as Fermentable).yield_pct.toFixed(0)}% yield · ${(item as Fermentable).color_lovibond}°L`;
    if (activeTab === 'yeast') return `${(item as Yeast).type_} · ${(item as Yeast).form}`;
    if (activeTab === 'misc') return `${(item as Misc).type_} · ${(item as Misc).use_}`;
    const w = item as Water;
    return `Ca:${w.calcium_ppm} Mg:${w.magnesium_ppm} Na:${w.sodium_ppm} SO₄:${w.sulfate_ppm} Cl:${w.chloride_ppm} HCO₃:${w.bicarbonate_ppm}`;
  }

  function openCreate() {
    editIngredient = null;
    editType = activeTab;
    editModalOpen = true;
  }

  function openEdit(item: AnyIngredient) {
    editIngredient = item;
    editType = activeTab;
    editModalOpen = true;
  }

  function openDelete(item: AnyIngredient) {
    deleteCandidate = item;
    deleteType = activeTab;
    deleteModalOpen = true;
  }

  async function confirmDelete() {
    if (!deleteCandidate) return;
    deleteModalOpen = false;
    const id = deleteCandidate.id;
    if (deleteType === 'hop') await ipc(deleteHop(id));
    else if (deleteType === 'fermentable') await ipc(deleteFermentable(id));
    else if (deleteType === 'yeast') await ipc(deleteYeast(id));
    else if (deleteType === 'misc') await ipc(deleteMisc(id));
    else await ipc(deleteWater(id));
    deleteCandidate = null;
    await refreshTab(deleteType);
  }

  async function handleSave() {
    editModalOpen = false;
    await refreshTab(editType);
    editIngredient = null;
  }

  async function handleDuplicate(saved: AnyIngredient) {
    editModalOpen = false;
    await refreshTab(editType);
    await tick();
    editIngredient = saved;
    editModalOpen = true;
  }
</script>

<div class="flex flex-col flex-1 overflow-hidden" style="background: var(--color-bg-base);">
  <!-- Header -->
  <div class="flex items-center justify-between px-6 py-4 flex-shrink-0"
       style="border-bottom: 1px solid var(--color-border);">
    <h1 class="text-lg font-semibold" style="color: var(--color-text-primary);">Ingredient Library</h1>
    <button onclick={openCreate} class="px-3 py-1.5 rounded text-sm font-medium"
            style="background: var(--color-accent); color: #fff; border: none; cursor: pointer;">
      + New {TAB_LABELS[activeTab].replace(/s$/, '')}
    </button>
  </div>

  <!-- Tabs -->
  <div class="flex gap-0 flex-shrink-0 px-6 pt-3"
       style="border-bottom: 1px solid var(--color-border);">
    {#each (['hop', 'fermentable', 'yeast', 'misc', 'water'] as Tab[]) as tab}
      <button onclick={() => switchTab(tab)}
              class="px-4 py-2 text-sm font-medium transition-colors"
              style="
                border: none; background: none; cursor: pointer;
                color: {activeTab === tab ? 'var(--color-accent)' : 'var(--color-text-secondary)'};
                border-bottom: 2px solid {activeTab === tab ? 'var(--color-accent)' : 'transparent'};
                margin-bottom: -1px;
              ">
        {TAB_LABELS[tab]}
      </button>
    {/each}
  </div>

  <!-- Search -->
  <div class="px-6 py-3 flex-shrink-0">
    <div class="relative max-w-xs">
      <svg style="position: absolute; left: 8px; top: 50%; transform: translateY(-50%); pointer-events: none; color: var(--color-text-muted);"
           width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
      </svg>
      <input bind:value={query}
             placeholder="Search {TAB_LABELS[activeTab].toLowerCase()}…"
             class="pl-8 pr-3 py-1.5 rounded text-sm w-full"
             style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); color: var(--color-text-primary); outline: none;" />
    </div>
  </div>

  <!-- List -->
  <div class="flex-1 overflow-y-auto px-6 pb-6">
    {#if filtered.length === 0}
      <p class="text-sm mt-8 text-center" style="color: var(--color-text-muted);">
        {query ? `No results for "${query}"` : `No ${TAB_LABELS[activeTab].toLowerCase()} yet.`}
      </p>
    {:else}
      <div class="flex flex-col gap-1">
        {#each filtered as item (item.id)}
          {@const isSeeded = item.source === 'seeded'}
          <div class="flex items-center gap-3 px-3 py-2.5 rounded"
               role="button" tabindex="0"
               onclick={() => openEdit(item)}
               onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') openEdit(item); }}
               style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); cursor: pointer;">
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2">
                <span class="text-sm font-medium truncate" style="color: var(--color-text-primary);">{item.name}</span>
                <span class="text-xs px-1.5 py-0.5 rounded-full flex-shrink-0"
                      style="background: {isSeeded ? 'var(--color-bg-surface)' : 'color-mix(in srgb, var(--color-accent) 15%, transparent)'}; color: {isSeeded ? 'var(--color-text-muted)' : 'var(--color-accent)'}; border: 1px solid {isSeeded ? 'var(--color-border)' : 'color-mix(in srgb, var(--color-accent) 40%, transparent)'};">
                  {isSeeded ? 'built-in' : 'custom'}
                </span>
              </div>
              <div class="text-xs mt-0.5 truncate" style="color: var(--color-text-secondary);">{rowSubtext(item)}</div>
            </div>
            {#if !isSeeded}
              <button onclick={(e) => { e.stopPropagation(); openEdit(item); }}
                      class="text-xs px-2 py-1 rounded flex-shrink-0"
                      style="background: var(--color-bg-surface); color: var(--color-text-secondary); border: 1px solid var(--color-border); cursor: pointer;">
                Edit
              </button>
              <button onclick={(e) => { e.stopPropagation(); openDelete(item); }}
                      class="text-xs px-2 py-1 rounded flex-shrink-0"
                      style="background: var(--color-bg-surface); color: #f87171; border: 1px solid var(--color-border); cursor: pointer;">
                Delete
              </button>
            {/if}
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

{#if editModalOpen}
  <IngredientEditModal
    type={editType}
    ingredient={editIngredient}
    existingNames={existingNames}
    onsave={handleSave}
    onduplicate={handleDuplicate}
    oncancel={() => { editModalOpen = false; editIngredient = null; }}
  />
{/if}

{#if deleteModalOpen && deleteCandidate}
  <ConfirmModal
    message="Delete {deleteCandidate.name}? This cannot be undone."
    confirmLabel="Delete"
    dangerous={true}
    onconfirm={confirmDelete}
    oncancel={() => { deleteModalOpen = false; deleteCandidate = null; }}
  />
{/if}
