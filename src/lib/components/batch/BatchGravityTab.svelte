<!-- src/lib/components/batch/BatchGravityTab.svelte -->
<script lang="ts">
  import type { Batch, CreateGravityReadingInput } from "$lib/api";
  import { addGravityReading, deleteGravityReading, convertGravity } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { formatSg, gravityStep, gravityPlaceholder } from "$lib/gravity-display";
  import Tooltip from "$lib/components/Tooltip.svelte";
  import DocLink from "$lib/components/DocLink.svelte";
  import { DOCS } from "$lib/docs-urls";

  let { batch, onRefresh }: { batch: Batch; onRefresh: () => void } = $props();

  const gravityUnit = $derived($settings.gravity_unit ?? "sg");

  let newGravity = $state("");
  let newTemp = $state("");
  let newDate = $state(new Date().toISOString().slice(0, 10));
  let newNotes = $state("");

  const displayReadings = $derived(
    batch.gravity_readings.map(r => formatSg(r.gravity, gravityUnit)),
  );

  function formatDate(ts: number): string {
    return new Date(ts * 1000).toLocaleDateString();
  }

  async function handleAdd() {
    if (!newGravity || !newDate) return;
    const unit = gravityUnit;
    const converted = await ipc(convertGravity(parseFloat(newGravity), unit));
    if (!converted) return;
    const input: CreateGravityReadingInput = {
      recorded_at: Math.floor(new Date(newDate).getTime() / 1000),
      gravity: converted.sg,
      temp_c: newTemp ? parseFloat(newTemp) : null,
      notes: newNotes || null,
    };
    await ipc(addGravityReading(batch.id, input));
    newGravity = "";
    newTemp = "";
    newNotes = "";
    onRefresh();
  }

  async function handleDelete(id: string) {
    await ipc(deleteGravityReading(id));
    onRefresh();
  }
</script>

<div class="p-4 flex flex-col gap-4 overflow-y-auto">
  {#if batch.gravity_readings.length > 0}
    <table class="w-full text-sm">
      <thead>
        <tr class="text-text-muted">
          <th class="text-left py-1 font-normal">Date</th>
          <th class="text-left py-1 font-normal">Gravity</th>
          <th class="text-left py-1 font-normal">Temp (°C)</th>
          <th class="text-left py-1 font-normal">Notes</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        {#each batch.gravity_readings as r, i (r.id)}
          <tr class="border-t border-border">
            <td class="py-1.5">{formatDate(r.recorded_at)}</td>
            <td class="py-1.5">{displayReadings[i] ?? r.gravity.toFixed(3)}</td>
            <td class="py-1.5">{r.temp_c != null ? r.temp_c + "°" : "—"}</td>
            <td class="py-1.5 text-xs text-text-muted">{r.notes ?? ""}</td>
            <td class="py-1.5">
              <button
                onclick={() => handleDelete(r.id)}
                class="opacity-40 hover:opacity-100 text-xs text-text-muted"
               
              >✕</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {:else}
    <p class="text-sm text-text-muted">No readings yet.</p>
  {/if}

  <div class="flex flex-col gap-2 pt-2 border-t border-border">
    <div class="flex items-center justify-between">
      <div class="text-xs text-text-muted">ADD READING</div>
      <DocLink label="Gravity tracking guide" url={DOCS.gravityTracking} />
    </div>
    <div class="flex gap-2 flex-wrap">
      <input type="date" bind:value={newDate}
        class="px-2 py-1.5 rounded text-sm outline-none bg-bg-elevated text-text-primary border border-border"
        />
      <div class="flex items-center gap-1">
        <input type="number" inputmode="decimal" step={gravityStep(gravityUnit)}
          placeholder={gravityPlaceholder(gravityUnit)} bind:value={newGravity}
          class="px-2 py-1.5 rounded text-sm outline-none w-40 bg-bg-elevated text-text-primary border border-border"
          />
        <Tooltip text="Enter your current gravity reading. If using a refractometer post-fermentation, correct the reading first in Tools → Refractometer before logging it." />
      </div>
      <input type="number" inputmode="decimal" step="0.1" placeholder="Temp °C" bind:value={newTemp}
        class="px-2 py-1.5 rounded text-sm outline-none w-24 bg-bg-elevated text-text-primary border border-border"
        />
      <input type="text" placeholder="Notes" bind:value={newNotes}
        class="px-2 py-1.5 rounded text-sm outline-none flex-1 bg-bg-elevated text-text-primary border border-border"
        />
      <button
        onclick={handleAdd}
        class="px-3 py-1.5 rounded text-sm bg-accent"
        style="color: #fff;"
      >Add</button>
    </div>
  </div>
</div>
