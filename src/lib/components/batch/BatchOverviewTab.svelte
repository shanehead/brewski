<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import TabBar from "$lib/components/TabBar.svelte";
  import type { Batch, UpdateBatchInput, RecipeVersionSummary, Recipe } from "$lib/api";
  import { listRecipeVersions, getRecipe, convertGravity } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { formatGravity, gravityStep } from "$lib/gravity-display";
  import BatchCarbonationSection from "$lib/components/batch/BatchCarbonationSection.svelte";

  let { batch, onUpdate }: { batch: Batch; onUpdate: (input: UpdateBatchInput) => void } = $props();

  const STATUSES = ["planned", "brewing", "fermenting", "conditioning", "packaged"] as const;

  let batchVersion = $state<RecipeVersionSummary | null>(null);
  let recipe = $state<Recipe | null>(null);

  const gravityUnit = $derived($settings.gravity_unit ?? "sg");

  let gravityDisplays = $state<Record<string, string>>({});

  $effect(() => {
    let cancelled = false;
    const unit = gravityUnit;
    const b = batch;
    const gravityFields = [
      "planned_og", "planned_fg", "planned_pre_boil_gravity",
      "actual_og", "actual_fg", "actual_pre_boil_gravity",
    ] as const;

    // Synchronous seed: show raw SG values immediately so the banner isn't blank
    const seed: Record<string, string> = {};
    for (const f of gravityFields) {
      const v = (b as Record<string, unknown>)[f];
      seed[f] = v != null ? (v as number).toFixed(3) : "";
    }
    gravityDisplays = seed;

    // If unit is SG, the seed is already correct — skip IPC
    if (unit === "sg") return () => { cancelled = true; };

    const toConvert = gravityFields.filter(f => (b as Record<string, unknown>)[f] != null);

    Promise.all(
      toConvert.map(f =>
        convertGravity((b as unknown as Record<string, number>)[f], "sg")
          .then(r => [f, formatGravity(r, unit)] as const)
      )
    ).then(entries => {
      if (cancelled) return;
      const next: Record<string, string> = { ...seed };
      for (const [f, v] of entries) next[f] = v;
      gravityDisplays = next;
    }).catch(() => {
      // IPC failed — seed values remain, which is better than blank
    });

    return () => { cancelled = true; };
  });

  onMount(async () => {
    const [versions, fetchedRecipe] = await Promise.all([
      ipc(listRecipeVersions(batch.recipe_id)),
      ipc(getRecipe(batch.recipe_id)),
    ]);
    if (versions) {
      batchVersion = versions.find((v) => v.id === batch.recipe_version_id) ?? null;
    }
    if (fetchedRecipe) {
      recipe = fetchedRecipe;
    }
  });

  function toDateInput(ts: number | null | undefined): string {
    if (ts == null) return "";
    return new Date(ts * 1000).toISOString().slice(0, 10);
  }

  function fromDateInput(val: string): number | null {
    if (!val) return null;
    return Math.floor(new Date(val).getTime() / 1000);
  }

  function onStatusChange(newStatus: string) {
    const update: UpdateBatchInput = { status: newStatus };
    const todayTs = Math.floor(Date.now() / 1000);
    if (newStatus === "brewing" && !batch.brew_date) update.brew_date = todayTs;
    if (newStatus === "fermenting" && !batch.fermenter_date) update.fermenter_date = todayTs;
    if (newStatus === "conditioning" && !batch.conditioning_date) update.conditioning_date = todayTs;
    if (newStatus === "packaged" && !batch.packaging_date) update.packaging_date = todayTs;
    onUpdate(update);
  }

  const HIGHLIGHTED: Record<string, string[]> = {
    planned: [],
    brewing: ["actual_pre_boil_gravity", "actual_og", "actual_post_boil_volume_l"],
    fermenting: ["actual_og", "actual_fg"],
    conditioning: ["actual_fg", "actual_batch_size_l"],
    packaged: ["actual_og", "actual_fg"],
  };

  const highlightedFields = $derived(new Set(HIGHLIGHTED[batch.status] ?? []));

  const stageTargets = $derived.by(() => {
    const { planned_og: og, planned_fg: fg, planned_pre_boil_gravity: pbg,
            planned_post_boil_volume_l: pbv, planned_batch_size_l: bs,
            actual_og, actual_fg } = batch;
    const targetAbv = og && fg ? ((og - fg) * 131.25).toFixed(1) : null;
    const actualAbv = actual_og && actual_fg ? ((actual_og - actual_fg) * 131.25).toFixed(1) : null;
    const items: { label: string; value: string }[] = [];
    switch (batch.status) {
      case "planned":
        if (og && gravityDisplays.planned_og) items.push({ label: "OG", value: gravityDisplays.planned_og });
        if (fg && gravityDisplays.planned_fg) items.push({ label: "FG", value: gravityDisplays.planned_fg });
        if (bs) items.push({ label: "Batch", value: `${bs.toFixed(1)} L` });
        break;
      case "brewing":
        if (pbg && gravityDisplays.planned_pre_boil_gravity) items.push({ label: "Pre-boil", value: gravityDisplays.planned_pre_boil_gravity });
        if (og && gravityDisplays.planned_og) items.push({ label: "OG", value: gravityDisplays.planned_og });
        if (pbv) items.push({ label: "Post-boil", value: `${pbv.toFixed(1)} L` });
        break;
      case "fermenting":
        if (actual_og && gravityDisplays.actual_og) items.push({ label: "Actual OG", value: gravityDisplays.actual_og });
        if (fg && gravityDisplays.planned_fg) items.push({ label: "Target FG", value: gravityDisplays.planned_fg });
        if (targetAbv) items.push({ label: "Target ABV", value: `${targetAbv}%` });
        break;
      case "conditioning":
      case "packaged":
        if (actual_og && gravityDisplays.actual_og) items.push({ label: "OG", value: gravityDisplays.actual_og });
        if (actual_fg && gravityDisplays.actual_fg) items.push({ label: "FG", value: gravityDisplays.actual_fg });
        if (actualAbv) items.push({ label: "ABV", value: `${actualAbv}%` });
        break;
    }
    return items;
  });
</script>

<div class="p-4 flex flex-col gap-6">
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
    <!-- Mobile: native select -->
    <select
      class="md:hidden w-full px-3 py-2 rounded text-sm outline-none"
      style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
      value={batch.status}
      onchange={(e) => onStatusChange(e.currentTarget.value)}
    >
      {#each STATUSES as s}
        <option value={s}>{s.charAt(0).toUpperCase() + s.slice(1)}</option>
      {/each}
    </select>
    <!-- Desktop: tab bar -->
    <div class="hidden md:block">
      <TabBar
        tabs={STATUSES.map(s => ({ key: s, label: s.charAt(0).toUpperCase() + s.slice(1) }))}
        active={batch.status}
        onchange={(key) => onStatusChange(key)}
      />
    </div>
  </div>

  <!-- Stage callout banner -->
  {#if stageTargets.length > 0}
    <div
      class="flex items-center gap-4 flex-wrap px-3 py-2 rounded-lg text-sm"
      style="background: rgba(99,102,241,0.12); border: 1px solid rgba(99,102,241,0.25);"
    >
      <span class="text-xs font-bold uppercase tracking-wide" style="color: var(--color-text-secondary); min-width: 48px;">
        {batch.status === "conditioning" || batch.status === "packaged" ? "Actuals" : batch.status === "fermenting" ? "Progress" : "Targets"}
      </span>
      {#each stageTargets as t}
        <span style="color: var(--color-text-secondary);">
          {t.label} <strong style="color: var(--color-text-primary);">{t.value}</strong>
        </span>
      {/each}
    </div>
  {/if}

  <!-- Measurements -->
  <div>
    <div class="text-xs mb-2" style="color: var(--color-text-secondary);">MEASUREMENTS</div>
    <div class="grid grid-cols-2 md:grid-cols-3 gap-3">
      {#each [
        { label: "Pre-Boil Gravity", field: "actual_pre_boil_gravity", isGravity: true },
        { label: "Original Gravity (OG)", field: "actual_og", isGravity: true },
        { label: "Final Gravity (FG)", field: "actual_fg", isGravity: true },
        { label: "Pre-Boil Volume (L)", field: "actual_pre_boil_volume_l" },
        { label: "Post-Boil Volume (L)", field: "actual_post_boil_volume_l" },
        { label: "Batch Size (L)", field: "actual_batch_size_l" },
      ] as row}
        {@const rawValue = (batch as unknown as Record<string, number | null>)[row.field]}
        {@const highlighted = highlightedFields.has(row.field)}
        <div
          class="p-3 rounded"
          style="background: var(--color-bg-elevated);
                 border: 1px solid {highlighted ? 'rgba(99,102,241,0.4)' : 'var(--color-border)'};
                 opacity: {highlighted || rawValue != null ? '1' : '0.55'};"
        >
          <label for="batch-{row.field}" class="text-xs block mb-1" style="color: var(--color-text-secondary);">{row.label}</label>
          {#if row.isGravity}
            <input
              id="batch-{row.field}"
              type="number" inputmode="decimal"
              step={gravityStep(gravityUnit)}
              value={gravityDisplays[row.field] ?? ""}
              onblur={async (e) => {
                const v = e.currentTarget.value;
                if (!v) { onUpdate({ [row.field]: null } as UpdateBatchInput); return; }
                const converted = await ipc(convertGravity(parseFloat(v), gravityUnit));
                if (converted) onUpdate({ [row.field]: converted.sg } as UpdateBatchInput);
              }}
              placeholder="—"
              class="w-full bg-transparent text-sm outline-none"
              style="color: {highlighted ? 'var(--color-accent)' : 'var(--color-text-primary)'}; font-weight: {highlighted ? '600' : '400'};"
            />
          {:else}
            <input
              id="batch-{row.field}"
              type="number" inputmode="decimal"
              step="0.1"
              value={rawValue != null ? rawValue.toFixed(1) : ""}
              onblur={(e) => {
                const v = e.currentTarget.value;
                onUpdate({ [row.field]: v ? parseFloat(v) : null } as UpdateBatchInput);
              }}
              placeholder="—"
              class="w-full bg-transparent text-sm outline-none"
              style="color: {highlighted ? 'var(--color-accent)' : 'var(--color-text-primary)'}; font-weight: {highlighted ? '600' : '400'};"
            />
          {/if}
        </div>
      {/each}
    </div>
    {#if batch.actual_og && batch.actual_fg}
      <div class="mt-3 text-sm" style="color: var(--color-text-muted);">
        Actual ABV: {((batch.actual_og - batch.actual_fg) * 131.25).toFixed(1)}%
      </div>
    {/if}
  </div>

  <!-- Dates -->
  <div>
    <div class="text-xs mb-2" style="color: var(--color-text-secondary);">DATES</div>
    <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
      {#each [
        { label: "Brew Date", field: "brew_date", value: batch.brew_date },
        { label: "Into Fermenter", field: "fermenter_date", value: batch.fermenter_date },
        { label: "Conditioning", field: "conditioning_date", value: batch.conditioning_date },
        { label: "Packaging", field: "packaging_date", value: batch.packaging_date },
      ] as item}
        <div>
          <label for="batch-{item.field}" class="text-xs block mb-1" style="color: var(--color-text-secondary);">{item.label}</label>
          <input
            id="batch-{item.field}"
            type="date"
            value={toDateInput(item.value)}
            onchange={(e) => onUpdate({ [item.field]: fromDateInput(e.currentTarget.value) } as UpdateBatchInput)}
            class="w-full px-2 py-1.5 rounded text-sm outline-none"
            style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border); opacity: {item.value ? '1' : '0.55'};"
          />
        </div>
      {/each}
    </div>
  </div>

  {#if batch.status === "conditioning" || batch.status === "packaged"}
    <BatchCarbonationSection
      {batch}
      recipePrimaryTempC={recipe?.primary_temp_c ?? null}
      recipeCarbonationVols={recipe?.carbonation_vols ?? null}
      {onUpdate}
    />
  {/if}

  <!-- Notes -->
  <div>
    <div class="text-xs mb-2" style="color: var(--color-text-secondary);">NOTES</div>
    <textarea
      value={batch.notes ?? ""}
      onblur={(e) => onUpdate({ notes: e.currentTarget.value || null })}
      placeholder="Brew day observations, gravity readings, anything worth remembering…"
      rows="4"
      class="w-full px-3 py-2 rounded text-sm outline-none resize-y"
      style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border); font-family: inherit;"
    ></textarea>
  </div>
</div>
