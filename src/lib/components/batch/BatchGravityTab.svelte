<!-- src/lib/components/batch/BatchGravityTab.svelte -->
<script lang="ts">
  import type { Batch, CreateGravityReadingInput } from "$lib/api";
  import { addGravityReading, deleteGravityReading, convertGravity } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { formatGravity, gravityStep, gravityPlaceholder } from "$lib/gravity-display";

  let { batch, onRefresh }: { batch: Batch; onRefresh: () => void } = $props();

  const gravityUnit = $derived($settings.gravity_unit ?? "sg");

  let newGravity = $state("");
  let newTemp = $state("");
  let newDate = $state(new Date().toISOString().slice(0, 10));
  let newNotes = $state("");

  let displayReadings = $state<string[]>([]);

  $effect(() => {
    let cancelled = false;
    const unit = gravityUnit;
    const readings = batch.gravity_readings;
    if (readings.length === 0) { displayReadings = []; return () => { cancelled = true; }; }
    Promise.all(readings.map(r => convertGravity(r.gravity, "sg")))
      .then(results => { if (!cancelled) displayReadings = results.map(r => formatGravity(r, unit)); });
    return () => { cancelled = true; };
  });

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
        <tr style="color: var(--color-text-muted);">
          <th class="text-left py-1 font-normal">Date</th>
          <th class="text-left py-1 font-normal">Gravity</th>
          <th class="text-left py-1 font-normal">Temp (°C)</th>
          <th class="text-left py-1 font-normal">Notes</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        {#each batch.gravity_readings as r, i (r.id)}
          <tr class="border-t" style="border-color: var(--color-border);">
            <td class="py-1.5">{formatDate(r.recorded_at)}</td>
            <td class="py-1.5">{displayReadings[i] ?? r.gravity.toFixed(3)}</td>
            <td class="py-1.5">{r.temp_c != null ? r.temp_c + "°" : "—"}</td>
            <td class="py-1.5 text-xs" style="color: var(--color-text-muted);">{r.notes ?? ""}</td>
            <td class="py-1.5">
              <button
                onclick={() => handleDelete(r.id)}
                class="opacity-40 hover:opacity-100 text-xs"
                style="color: var(--color-text-muted);"
              >✕</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {:else}
    <p class="text-sm" style="color: var(--color-text-muted);">No readings yet.</p>
  {/if}

  <div class="flex flex-col gap-2 pt-2 border-t" style="border-color: var(--color-border);">
    <div class="text-xs" style="color: var(--color-text-muted);">ADD READING</div>
    <div class="flex gap-2 flex-wrap">
      <input type="date" bind:value={newDate}
        class="px-2 py-1.5 rounded text-sm outline-none"
        style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      <input type="number" inputmode="decimal" step={gravityStep(gravityUnit)}
        placeholder={gravityPlaceholder(gravityUnit)} bind:value={newGravity}
        class="px-2 py-1.5 rounded text-sm outline-none w-40"
        style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      <input type="number" inputmode="decimal" step="0.1" placeholder="Temp °C" bind:value={newTemp}
        class="px-2 py-1.5 rounded text-sm outline-none w-24"
        style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      <input type="text" placeholder="Notes" bind:value={newNotes}
        class="px-2 py-1.5 rounded text-sm outline-none flex-1"
        style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      <button
        onclick={handleAdd}
        class="px-3 py-1.5 rounded text-sm"
        style="background: var(--color-accent); color: #fff;"
      >Add</button>
    </div>
  </div>
</div>
