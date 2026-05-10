<script lang="ts">
  import { calculatePitchRate, type PitchRateResult } from "$lib/api";
  import { settings } from "$lib/stores/settings";
  import { ipc } from "$lib/stores/error";
  import { galToL, lToGal, volumeLabel, type Units } from "$lib/units";

  let og = $state(1.05);
  let batchSizeL = $state(20);
  let pitchRate = $state(0.75);
  let yeastPackCells = $state(100);
  let viabilityPct = $state(85);
  let result = $state<PitchRateResult | null>(null);

  const units = $derived(($settings.units ?? "metric") as Units);
  const batchSizeDisplay = $derived(units === "imperial" ? lToGal(batchSizeL) : batchSizeL);
  const starterDisplay = $derived(result ? (units === "imperial" ? lToGal(result.starterVolumeL) : result.starterVolumeL) : null);

  function updateBatchSize(value: string) {
    const next = Number(value);
    batchSizeL = units === "imperial" ? galToL(next) : next;
  }

  $effect(() => {
    const currentOg = og;
    const currentBatchSize = batchSizeL;
    const currentPitchRate = pitchRate;
    const currentPackCells = yeastPackCells;
    const currentViability = viabilityPct;

    if (currentOg <= 1 || currentBatchSize <= 0 || currentPitchRate <= 0 || currentPackCells <= 0 || currentViability <= 0) {
      result = null;
      return;
    }

    void (async () => {
      const next = await ipc(calculatePitchRate(
        currentOg,
        currentBatchSize,
        currentPitchRate,
        currentPackCells,
        currentViability,
      ));
      if (
        next &&
        og === currentOg &&
        batchSizeL === currentBatchSize &&
        pitchRate === currentPitchRate &&
        yeastPackCells === currentPackCells &&
        viabilityPct === currentViability
      ) {
        result = next;
      }
    })();
  });
</script>

<div class="p-6 md:p-8">
  <h2 class="text-xl font-semibold" style="color: var(--color-text-primary);">Yeast Pitch Rate</h2>
  <p class="mt-2 max-w-2xl text-sm" style="color: var(--color-text-secondary);">
    Estimate total cells needed for the batch and a simple starter size based on pack count and viability.
  </p>

  <div class="mt-6 grid gap-6 lg:grid-cols-[minmax(0,20rem)_minmax(0,1fr)]">
    <section class="rounded-xl border p-4" style="background: var(--color-bg-surface); border-color: var(--color-border);">
      <label class="block text-sm font-medium" style="color: var(--color-text-primary);">
        Original Gravity
        <input bind:value={og} type="number" min="1" max="1.2" step="0.001"
               class="mt-2 w-full rounded px-3 py-2 text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </label>

      <label class="mt-4 block text-sm font-medium" style="color: var(--color-text-primary);">
        Batch Size ({volumeLabel(units)})
        <input value={batchSizeDisplay} oninput={(e) => updateBatchSize((e.target as HTMLInputElement).value)}
               type="number" min="0" step="0.1"
               class="mt-2 w-full rounded px-3 py-2 text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </label>

      <label class="mt-4 block text-sm font-medium" style="color: var(--color-text-primary);">
        Pitch Rate (M cells / mL / °P)
        <input bind:value={pitchRate} type="number" min="0" step="0.05"
               class="mt-2 w-full rounded px-3 py-2 text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </label>

      <label class="mt-4 block text-sm font-medium" style="color: var(--color-text-primary);">
        Yeast Pack Cells (billions)
        <input bind:value={yeastPackCells} type="number" min="0" step="1"
               class="mt-2 w-full rounded px-3 py-2 text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </label>

      <label class="mt-4 block text-sm font-medium" style="color: var(--color-text-primary);">
        Viability (%)
        <input bind:value={viabilityPct} type="number" min="0" max="100" step="1"
               class="mt-2 w-full rounded px-3 py-2 text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </label>
    </section>

    <section class="rounded-xl border p-5" style="background: var(--color-bg-surface); border-color: var(--color-accent);">
      {#if result && starterDisplay !== null}
        <div class="grid gap-4 md:grid-cols-2">
          <div>
            <div class="text-xs uppercase tracking-wide" style="color: var(--color-text-secondary);">Required Cells</div>
            <div class="mt-2 text-4xl font-semibold" style="color: var(--color-text-primary);">{result.requiredCells.toFixed(0)}</div>
            <div class="mt-1 text-sm" style="color: var(--color-text-secondary);">billion cells</div>
          </div>
          <div>
            <div class="text-xs uppercase tracking-wide" style="color: var(--color-text-secondary);">Starter Volume</div>
            <div class="mt-2 text-4xl font-semibold" style="color: var(--color-text-primary);">{starterDisplay.toFixed(2)}</div>
            <div class="mt-1 text-sm" style="color: var(--color-text-secondary);">{volumeLabel(units)}</div>
          </div>
        </div>
      {:else}
        <p class="text-sm" style="color: var(--color-text-secondary);">Enter a valid gravity, batch size, and yeast profile to calculate pitch needs.</p>
      {/if}
    </section>
  </div>
</div>
