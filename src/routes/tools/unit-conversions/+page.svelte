<script lang="ts">
  import { settings } from "$lib/stores/settings";
  import {
    cToF,
    fToC,
    kgToLb,
    lbToKg,
    lToGal,
    galToL,
    tempLabel,
    volumeLabel,
    weightLabel,
    type Units,
  } from "$lib/units";

  let volumeInput = $state(20);
  let weightInput = $state(5);
  let tempInput = $state(20);

  const units = $derived(($settings.units ?? "metric") as Units);
  const convertedVolume = $derived(units === "imperial" ? galToL(volumeInput) : lToGal(volumeInput));
  const convertedWeight = $derived(units === "imperial" ? lbToKg(weightInput) : kgToLb(weightInput));
  const convertedTemp = $derived(units === "imperial" ? fToC(tempInput) : cToF(tempInput));
  const volumeTargetLabel = $derived(units === "imperial" ? "L" : "gal");
  const weightTargetLabel = $derived(units === "imperial" ? "kg" : "lb");
  const tempTargetLabel = $derived(units === "imperial" ? "°C" : "°F");
</script>

<div class="p-6 md:p-8">
  <h2 class="text-xl font-semibold text-text-primary">Unit Conversions</h2>
  <p class="mt-2 max-w-2xl text-sm text-text-secondary">
    Quick frontend-only conversions using your current measurement system as the primary input side.
  </p>

  <div class="mt-6 grid gap-6 xl:grid-cols-3">
    <section class="rounded-xl border p-4 bg-bg-surface border-border">
      <div class="text-sm font-semibold text-text-primary">Volume</div>
      <label class="mt-4 block text-sm font-medium text-text-primary">
        Value ({volumeLabel(units)})
        <input bind:value={volumeInput} type="number" step="0.1"
               class="mt-2 w-full rounded px-3 py-2 text-sm bg-bg-elevated text-text-primary border border-border"
               />
      </label>
      <div class="mt-6 text-xs uppercase tracking-wide text-text-secondary">{volumeTargetLabel}</div>
      <div class="mt-2 text-4xl font-semibold text-text-primary">{convertedVolume.toFixed(2)}</div>
    </section>

    <section class="rounded-xl border p-4 bg-bg-surface border-border">
      <div class="text-sm font-semibold text-text-primary">Weight</div>
      <label class="mt-4 block text-sm font-medium text-text-primary">
        Value ({weightLabel(units)})
        <input bind:value={weightInput} type="number" step="0.1"
               class="mt-2 w-full rounded px-3 py-2 text-sm bg-bg-elevated text-text-primary border border-border"
               />
      </label>
      <div class="mt-6 text-xs uppercase tracking-wide text-text-secondary">{weightTargetLabel}</div>
      <div class="mt-2 text-4xl font-semibold text-text-primary">{convertedWeight.toFixed(2)}</div>
    </section>

    <section class="rounded-xl border p-4 bg-bg-surface border-border">
      <div class="text-sm font-semibold text-text-primary">Temperature</div>
      <label class="mt-4 block text-sm font-medium text-text-primary">
        Value ({tempLabel(units)})
        <input bind:value={tempInput} type="number" step="0.1"
               class="mt-2 w-full rounded px-3 py-2 text-sm bg-bg-elevated text-text-primary border border-border"
               />
      </label>
      <div class="mt-6 text-xs uppercase tracking-wide text-text-secondary">{tempTargetLabel}</div>
      <div class="mt-2 text-4xl font-semibold text-text-primary">{convertedTemp.toFixed(1)}</div>
    </section>
  </div>
</div>
