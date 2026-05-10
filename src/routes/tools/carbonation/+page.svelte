<script lang="ts">
  import { calculateCo2Pressure, calculatePrimingSugar, type SugarType } from "$lib/api";
  import { settings } from "$lib/stores/settings";
  import { ipc } from "$lib/stores/error";
  import {
    cToF,
    fToC,
    hopWeightLabel,
    kgToHopDisplay,
    lToGal,
    galToL,
    tempLabel,
    volumeLabel,
    type Units,
  } from "$lib/units";

  let targetVols = $state(2.4);
  let batchSizeL = $state(19);
  let tempC = $state(20);
  let sugarType = $state<SugarType>("corn_sugar");
  let primingSugar = $state<number | null>(null);
  let pressureKpa = $state<number | null>(null);

  const units = $derived(($settings.units ?? "metric") as Units);
  const batchSizeDisplay = $derived(units === "imperial" ? lToGal(batchSizeL) : batchSizeL);
  const tempDisplay = $derived(units === "imperial" ? cToF(tempC) : tempC);
  const primingSugarDisplay = $derived(primingSugar === null ? null : kgToHopDisplay(primingSugar / 1000, units));

  function updateBatchSize(value: string) {
    const next = Number(value);
    batchSizeL = units === "imperial" ? galToL(next) : next;
  }

  function updateTemp(value: string) {
    const next = Number(value);
    tempC = units === "imperial" ? fToC(next) : next;
  }

  $effect(() => {
    const currentTargetVols = targetVols;
    const currentBatchSize = batchSizeL;
    const currentTempC = tempC;
    const currentSugarType = sugarType;

    if (currentTargetVols <= 0 || currentBatchSize <= 0) {
      primingSugar = null;
      pressureKpa = null;
      return;
    }

    void (async () => {
      const [sugar, pressure] = await Promise.all([
        ipc(calculatePrimingSugar(currentTargetVols, currentBatchSize, currentTempC, currentSugarType)),
        ipc(calculateCo2Pressure(currentTargetVols, currentTempC)),
      ]);

      if (
        sugar !== undefined &&
        pressure !== undefined &&
        targetVols === currentTargetVols &&
        batchSizeL === currentBatchSize &&
        tempC === currentTempC &&
        sugarType === currentSugarType
      ) {
        primingSugar = sugar;
        pressureKpa = pressure;
      }
    })();
  });
</script>

<div class="p-6 md:p-8">
  <h2 class="text-xl font-semibold" style="color: var(--color-text-primary);">Carbonation</h2>
  <p class="mt-2 max-w-2xl text-sm" style="color: var(--color-text-secondary);">
    Calculate bottle priming sugar or keg pressure for a target carbonation level at a given beer temperature.
  </p>

  <div class="mt-6 grid gap-6 lg:grid-cols-[minmax(0,20rem)_minmax(0,1fr)]">
    <section class="rounded-xl border p-4" style="background: var(--color-bg-surface); border-color: var(--color-border);">
      <label class="block text-sm font-medium" style="color: var(--color-text-primary);">
        Target CO2 Volumes
        <input bind:value={targetVols} type="number" min="0.5" max="5" step="0.1"
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
        Beer Temperature ({tempLabel(units)})
        <input value={tempDisplay} oninput={(e) => updateTemp((e.target as HTMLInputElement).value)}
               type="number" step="0.1"
               class="mt-2 w-full rounded px-3 py-2 text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </label>

      <label class="mt-4 block text-sm font-medium" style="color: var(--color-text-primary);">
        Priming Sugar Type
        <select bind:value={sugarType}
                class="mt-2 w-full rounded px-3 py-2 text-sm"
                style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="corn_sugar">Corn sugar</option>
          <option value="table_sugar">Table sugar</option>
          <option value="dry_malt_extract">Dry malt extract</option>
        </select>
      </label>
    </section>

    <section class="rounded-xl border p-5" style="background: var(--color-bg-surface); border-color: var(--color-accent);">
      {#if primingSugarDisplay !== null && pressureKpa !== null}
        <div class="grid gap-4 md:grid-cols-2">
          <div>
            <div class="text-xs uppercase tracking-wide" style="color: var(--color-text-secondary);">Priming Sugar</div>
            <div class="mt-2 text-4xl font-semibold" style="color: var(--color-text-primary);">{primingSugarDisplay.toFixed(1)}</div>
            <div class="mt-1 text-sm" style="color: var(--color-text-secondary);">{hopWeightLabel(units)}</div>
          </div>
          <div>
            <div class="text-xs uppercase tracking-wide" style="color: var(--color-text-secondary);">Serving Pressure</div>
            <div class="mt-2 text-4xl font-semibold" style="color: var(--color-text-primary);">{pressureKpa.toFixed(0)}</div>
            <div class="mt-1 text-sm" style="color: var(--color-text-secondary);">kPa</div>
          </div>
        </div>
      {:else}
        <p class="text-sm" style="color: var(--color-text-secondary);">Enter a valid target, batch size, and temperature to calculate carbonation.</p>
      {/if}
    </section>
  </div>
</div>
