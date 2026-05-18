<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { PageData } from "./$types";
  import type { Batch, UpdateBatchInput } from "$lib/api";
  import { getBatch, updateBatch } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { batchList, refreshBatchList } from "$lib/stores/batches";
  import BatchList from "$lib/components/BatchList.svelte";
  import TabBar from "$lib/components/TabBar.svelte";
  import BatchOverviewTab from "$lib/components/batch/BatchOverviewTab.svelte";
  import BatchGravityTab from "$lib/components/batch/BatchGravityTab.svelte";
  import BatchNotesTab from "$lib/components/batch/BatchNotesTab.svelte";
  import BatchTastingTab from "$lib/components/batch/BatchTastingTab.svelte";

  let { data }: { data: PageData } = $props();

  let batch = $state<Batch | null>(null);
  let activeTab = $state<"overview" | "gravity" | "notes" | "tasting">("overview");

  const TABS = [
    { key: "overview", label: "Overview" },
    { key: "gravity", label: "Gravity Log" },
    { key: "notes", label: "Notes" },
    { key: "tasting", label: "Tasting" },
  ] as const;

  async function loadBatch() {
    batch = await ipc(getBatch(data.id)) ?? null;
  }

  onMount(async () => {
    await refreshBatchList();
    await loadBatch();
  });

  $effect(() => {
    if (data?.id) loadBatch();
  });

  async function handleUpdate(input: UpdateBatchInput) {
    if (!batch) return;
    batch = await ipc(updateBatch(batch.id, input)) ?? batch;
  }
</script>

<aside class="w-56 flex flex-col flex-shrink-0 border-r overflow-hidden"
       style="background: var(--color-bg-surface); border-color: var(--color-border);">
  <div class="p-2 border-b" style="border-color: var(--color-border);">
    <button
      onclick={() => goto("/batches")}
      class="w-full px-2 py-1.5 rounded text-sm text-left"
      style="background: var(--color-accent); color: #fff;"
    >+ New Batch</button>
  </div>
  <div class="flex-1 overflow-y-auto">
    <BatchList batches={$batchList} onRefresh={async () => { await ipc(refreshBatchList()); }} />
  </div>
</aside>

<div class="flex flex-1 flex-col overflow-hidden">
  {#if batch}
    <!-- Header -->
    <div class="px-4 pt-3 pb-0 border-b flex-shrink-0" style="border-color: var(--color-border);">
      <div class="font-semibold text-base truncate">{batch.recipe_name}</div>
      <div class="text-xs mb-2" style="color: var(--color-text-muted);">
        {batch.name ?? "Batch"} · v{batch.recipe_version_id.slice(0, 6)}
      </div>
      <!-- Tab bar -->
      <TabBar tabs={TABS} active={activeTab} onchange={(key) => activeTab = key as typeof activeTab} flush />
    </div>

    <!-- Tab content -->
    <div class="flex-1 overflow-hidden">
      {#if activeTab === "overview"}
        <BatchOverviewTab {batch} onUpdate={handleUpdate} />
      {:else if activeTab === "gravity"}
        <BatchGravityTab {batch} onRefresh={loadBatch} />
      {:else if activeTab === "notes"}
        <BatchNotesTab {batch} onUpdate={handleUpdate} />
      {:else if activeTab === "tasting"}
        <BatchTastingTab {batch} onUpdate={handleUpdate} />
      {/if}
    </div>
  {:else}
    <div class="flex-1 flex items-center justify-center" style="color: var(--color-text-muted);">
      <p class="text-sm">Loading…</p>
    </div>
  {/if}
</div>
