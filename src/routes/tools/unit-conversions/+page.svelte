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
  <h2 class="text-xl font-semibold" style="color: var(--color-text-primary);">Unit Conversions</h2>
  <p class="mt-2 max-w-2xl text-sm" style="color: var(--color-text-secondary);">
    Quick frontend-only conversions using your current measurement system as the primary input side.
  </p>

  <div class="mt-6 grid gap-6 xl:grid-cols-3">
    <section class="rounded-xl border p-4" style="background: var(--color-bg-surface); border-color: var(--color-border);">
      <div class="text-sm font-semibold" style="color: var(--color-text-primary);">Volume</div>
      <label class="mt-4 block text-sm font-medium" style="color: var(--color-text-primary);">
        Value ({volumeLabel(units)})
        <input bind:value={volumeInput} type="number" step="0.1"
               class="mt-2 w-full rounded px-3 py-2 text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </label>
      <div class="mt-6 text-xs uppercase tracking-wide" style="color: var(--color-text-secondary);">{volumeTargetLabel}</div>
      <div class="mt-2 text-4xl font-semibold" style="color: var(--color-text-primary);">{convertedVolume.toFixed(2)}</div>
    </section>

    <section class="rounded-xl border p-4" style="background: var(--color-bg-surface); border-color: var(--color-border);">
      <div class="text-sm font-semibold" style="color: var(--color-text-primary);">Weight</div>
      <label class="mt-4 block text-sm font-medium" style="color: var(--color-text-primary);">
        Value ({weightLabel(units)})
        <input bind:value={weightInput} type="number" step="0.1"
               class="mt-2 w-full rounded px-3 py-2 text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </label>
      <div class="mt-6 text-xs uppercase tracking-wide" style="color: var(--color-text-secondary);">{weightTargetLabel}</div>
      <div class="mt-2 text-4xl font-semibold" style="color: var(--color-text-primary);">{convertedWeight.toFixed(2)}</div>
    </section>

    <section class="rounded-xl border p-4" style="background: var(--color-bg-surface); border-color: var(--color-border);">
      <div class="text-sm font-semibold" style="color: var(--color-text-primary);">Temperature</div>
      <label class="mt-4 block text-sm font-medium" style="color: var(--color-text-primary);">
        Value ({tempLabel(units)})
        <input bind:value={tempInput} type="number" step="0.1"
               class="mt-2 w-full rounded px-3 py-2 text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </label>
      <div class="mt-6 text-xs uppercase tracking-wide" style="color: var(--color-text-secondary);">{tempTargetLabel}</div>
      <div class="mt-2 text-4xl font-semibold" style="color: var(--color-text-primary);">{convertedTemp.toFixed(1)}</div>
    </section>
  </div>
</div>
