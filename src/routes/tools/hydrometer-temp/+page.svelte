<script lang="ts">
  import { correctHydrometerTemp } from "$lib/api";
  import { settings } from "$lib/stores/settings";
  import { ipc } from "$lib/stores/error";
  import { cToF, fToC, tempLabel, type Units } from "$lib/units";
  import Tooltip from "$lib/components/Tooltip.svelte";
  import DocLink from "$lib/components/DocLink.svelte";
  import { DOCS } from "$lib/docs-urls";

  let measuredSg = $state(1.05);
  let measuredTempC = $state(20);
  let calibrationTempC = $state(20);
  let correctedSg = $state<number | null>(null);

  const units = $derived(($settings.units ?? "metric") as Units);
  const measuredTempDisplay = $derived(units === "imperial" ? cToF(measuredTempC) : measuredTempC);
  const calibrationTempDisplay = $derived(units === "imperial" ? cToF(calibrationTempC) : calibrationTempC);

  function updateMeasuredTemp(value: string) {
    const next = Number(value);
    measuredTempC = units === "imperial" ? fToC(next) : next;
  }

  function updateCalibrationTemp(value: string) {
    const next = Number(value);
    calibrationTempC = units === "imperial" ? fToC(next) : next;
  }

  $effect(() => {
    const currentSg = measuredSg;
    const currentMeasuredTemp = measuredTempC;
    const currentCalibrationTemp = calibrationTempC;

    if (currentSg <= 1 || Number.isNaN(currentMeasuredTemp) || Number.isNaN(currentCalibrationTemp)) {
      correctedSg = null;
      return;
    }

    void (async () => {
      const next = await ipc(correctHydrometerTemp(currentSg, currentMeasuredTemp, currentCalibrationTemp));
      if (next !== undefined && measuredSg === currentSg && measuredTempC === currentMeasuredTemp && calibrationTempC === currentCalibrationTemp) {
        correctedSg = next;
      }
    })();
  });
</script>

<div class="p-6 md:p-8">
  <div class="flex items-center gap-2">
    <h2 class="text-xl font-semibold text-text-primary">Hydrometer Temperature Correction</h2>
    <DocLink label="Hydrometer correction reference" url={DOCS.calcHydrometer} />
  </div>
  <p class="mt-2 max-w-2xl text-sm text-text-secondary">
    Adjust a hydrometer reading when your wort sample is warmer or cooler than the hydrometer calibration point.
  </p>

  <div class="mt-6 grid gap-6 lg:grid-cols-[minmax(0,20rem)_minmax(0,1fr)]">
    <section class="rounded-xl border p-4 bg-bg-surface border-border">
      <label class="block text-sm font-medium text-text-primary">
        Measured Gravity
        <input bind:value={measuredSg} type="number" min="1" max="1.2" step="0.001"
               class="mt-2 w-full rounded px-3 py-2 text-sm bg-bg-elevated text-text-primary border border-border"
               />
      </label>

      <label class="mt-4 block text-sm font-medium text-text-primary">
        Sample Temperature ({tempLabel(units)})
        <input value={measuredTempDisplay} oninput={(e) => updateMeasuredTemp((e.target as HTMLInputElement).value)}
               type="number" step="0.1"
               class="mt-2 w-full rounded px-3 py-2 text-sm bg-bg-elevated text-text-primary border border-border"
               />
      </label>

      <div class="mt-4">
        <div class="flex items-center gap-1 mb-1">
          <span class="text-sm font-medium text-text-primary">Calibration Temperature ({tempLabel(units)})</span>
          <Tooltip text="The temperature your hydrometer is calibrated for. It's usually printed on the label or the instructions. Most hydrometers are calibrated at 20°C (68°F)." />
        </div>
        <input value={calibrationTempDisplay} oninput={(e) => updateCalibrationTemp((e.target as HTMLInputElement).value)}
               type="number" step="0.1"
               class="w-full rounded px-3 py-2 text-sm bg-bg-elevated text-text-primary border border-border"
               />
      </div>
    </section>

    <section class="rounded-xl border p-5 bg-bg-surface border-accent">
      {#if correctedSg !== null}
        <div class="text-xs uppercase tracking-wide text-text-secondary">Corrected Gravity</div>
        <div class="mt-2 text-4xl font-semibold text-text-primary">{correctedSg.toFixed(3)}</div>
      {:else}
        <p class="text-sm text-text-secondary">Enter a valid gravity and temperature values to calculate the correction.</p>
      {/if}
    </section>
  </div>
</div>
