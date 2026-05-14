<!-- src/lib/components/batch/BatchOverviewTab.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { Batch, UpdateBatchInput, RecipeVersionSummary } from "$lib/api";
  import { listRecipeVersions } from "$lib/api";
  import { ipc } from "$lib/stores/error";

  let { batch, onUpdate }: { batch: Batch; onUpdate: (input: UpdateBatchInput) => void } = $props();

  const STATUSES = ["planned", "brewing", "fermenting", "packaged", "complete"] as const;

  let batchVersion = $state<RecipeVersionSummary | null>(null);

  onMount(async () => {
    const versions = await ipc(listRecipeVersions(batch.recipe_id));
    if (versions) {
      batchVersion = versions.find((v) => v.id === batch.recipe_version_id) ?? null;
    }
  });

  function toDateInput(ts: number | null | undefined): string {
    if (!ts) return "";
    return new Date(ts * 1000).toISOString().slice(0, 10);
  }

  function fromDateInput(val: string): number | null {
    if (!val) return null;
    return Math.floor(new Date(val).getTime() / 1000);
  }
</script>

<div class="p-4 flex flex-col gap-6 overflow-y-auto">
  {#if batchVersion}
    <div class="text-xs" style="color: var(--color-text-muted);">
      Brewed with
      <button
        onclick={() => goto(`/recipe/${batch.recipe_id}`)}
        class="underline"
        style="color: var(--color-accent);"
      >
        Recipe v{batchVersion.version_number}{batchVersion.name ? ` · ${batchVersion.name}` : ""}
      </button>
    </div>
  {/if}

  <!-- Status -->
  <div>
    <div class="text-xs mb-2" style="color: var(--color-text-secondary);">STATUS</div>
    <div class="flex gap-2 flex-wrap">
      {#each STATUSES as s}
        <button
          onclick={() => onUpdate({ status: s })}
          class="px-3 py-1 rounded text-sm transition-colors"
          style={batch.status === s
            ? "background: var(--color-accent); color: #fff;"
            : "background: var(--color-bg-elevated); color: var(--color-text-secondary);"}
        >
          {s.charAt(0).toUpperCase() + s.slice(1)}
        </button>
      {/each}
    </div>
  </div>

  <!-- Dates -->
  <div class="grid grid-cols-3 gap-4">
    {#each [
      { label: "Brew Date", field: "brew_date", value: batch.brew_date },
      { label: "Into Fermenter", field: "fermenter_date", value: batch.fermenter_date },
      { label: "Packaging Date", field: "packaging_date", value: batch.packaging_date },
    ] as item}
      <div>
        <label class="text-xs block mb-1" style="color: var(--color-text-secondary);">{item.label}</label>
        <input
          type="date"
          value={toDateInput(item.value)}
          onchange={(e) => onUpdate({ [item.field]: fromDateInput(e.currentTarget.value) })}
          class="w-full px-2 py-1.5 rounded text-sm outline-none"
          style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
        />
      </div>
    {/each}
  </div>

  <!-- Actual vs Target -->
  <div>
    <div class="text-xs mb-2" style="color: var(--color-text-secondary);">MEASUREMENTS</div>
    <div class="grid grid-cols-2 gap-3">
      {#each [
        { label: "Pre-Boil Gravity", field: "actual_pre_boil_gravity", value: batch.actual_pre_boil_gravity, prominent: batch.status === "brewing" },
        { label: "Original Gravity (OG)", field: "actual_og", value: batch.actual_og, prominent: batch.status === "brewing" },
        { label: "Final Gravity (FG)", field: "actual_fg", value: batch.actual_fg, prominent: batch.status === "fermenting" },
        { label: "Pre-Boil Volume (L)", field: "actual_pre_boil_volume_l", value: batch.actual_pre_boil_volume_l, prominent: false },
        { label: "Post-Boil Volume (L)", field: "actual_post_boil_volume_l", value: batch.actual_post_boil_volume_l, prominent: false },
        { label: "Batch Size (L)", field: "actual_batch_size_l", value: batch.actual_batch_size_l, prominent: false },
      ] as row}
        <div
          class="p-3 rounded"
          style="background: var(--color-bg-elevated); {row.prominent ? 'border: 1px solid var(--color-accent);' : ''}"
        >
          <label class="text-xs block mb-1" style="color: var(--color-text-secondary);">{row.label}</label>
          <input
            type="number"
            step="0.001"
            value={row.value ?? ""}
            onblur={(e) => {
              const v = e.currentTarget.value;
              onUpdate({ [row.field]: v ? parseFloat(v) : null });
            }}
            placeholder="—"
            class="w-full bg-transparent text-sm outline-none"
            style="color: var(--color-text-primary);"
          />
        </div>
      {/each}
    </div>
    {#if batch.actual_og && batch.actual_fg}
      <div class="mt-3 text-sm" style="color: var(--color-text-muted);">
        Actual ABV: {((batch.actual_og - batch.actual_fg) * 131.25).toFixed(1)}%
      </div>
    {/if}
  </div>
</div>
