<script lang="ts">
  import {
    calculateRefractometer,
    correctRefractometerFg,
    type RefractometerFgResult,
    type RefractometerResult,
  } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import Tooltip from "$lib/components/Tooltip.svelte";
  import DocLink from "$lib/components/DocLink.svelte";
  import { DOCS } from "$lib/docs-urls";

  let brix = $state(12);
  let ogBrix = $state(12);
  let fgBrix = $state(6);
  let wortCorrectionFactor = $state(1.04);
  let preResult = $state<RefractometerResult | null>(null);
  let postResult = $state<RefractometerFgResult | null>(null);

  $effect(() => {
    const currentBrix = brix;
    const currentWcf = wortCorrectionFactor;

    if (currentBrix < 0 || currentWcf <= 0) {
      preResult = null;
      return;
    }

    void (async () => {
      const next = await ipc(calculateRefractometer(currentBrix, currentWcf));
      if (next && brix === currentBrix && wortCorrectionFactor === currentWcf) preResult = next;
    })();
  });

  $effect(() => {
    const currentOgBrix = ogBrix;
    const currentFgBrix = fgBrix;
    const currentWcf = wortCorrectionFactor;

    if (currentOgBrix < 0 || currentFgBrix < 0 || currentWcf <= 0) {
      postResult = null;
      return;
    }

    void (async () => {
      const next = await ipc(correctRefractometerFg(currentOgBrix, currentFgBrix, currentWcf));
      if (next && ogBrix === currentOgBrix && fgBrix === currentFgBrix && wortCorrectionFactor === currentWcf) {
        postResult = next;
      }
    })();
  });
</script>

<div class="p-6 md:p-8">
  <div class="flex items-center gap-2">
    <h2 class="text-xl font-semibold" style="color: var(--color-text-primary);">Refractometer / Brix</h2>
    <DocLink label="Refractometer reference" url={DOCS.calcRefractometer} />
  </div>
  <p class="mt-2 max-w-2xl text-sm" style="color: var(--color-text-secondary);">
    Convert raw Brix into pre-fermentation SG, or correct an apparent final reading after alcohol is present.
  </p>

  <div class="mt-6 grid gap-6 xl:grid-cols-2">
    <section class="rounded-xl border p-4" style="background: var(--color-bg-surface); border-color: var(--color-border);">
      <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Pre-Fermentation</h3>
      <label class="mt-4 block text-sm font-medium" style="color: var(--color-text-primary);">
        Brix
        <input bind:value={brix} type="number" min="0" step="0.1"
               class="mt-2 w-full rounded px-3 py-2 text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </label>

      <div class="mt-4">
        <div class="flex items-center gap-1 mb-1">
          <span class="text-sm font-medium" style="color: var(--color-text-primary);">Wort Correction Factor</span>
          <Tooltip text="Wort refracts light slightly differently than pure sugar. The default (1.04) works for most beers. Check your refractometer's manual — some are pre-corrected and should use 1.00." />
        </div>
        <input bind:value={wortCorrectionFactor} type="number" min="0.9" max="1.2" step="0.01"
               class="w-full rounded px-3 py-2 text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>

      <div class="mt-6 rounded-lg border p-4" style="border-color: var(--color-accent);">
        {#if preResult}
          <div class="text-xs uppercase tracking-wide" style="color: var(--color-text-secondary);">Estimated SG</div>
          <div class="mt-2 text-4xl font-semibold" style="color: var(--color-text-primary);">{preResult.sg.toFixed(3)}</div>
        {/if}
      </div>
    </section>

    <section class="rounded-xl border p-4" style="background: var(--color-bg-surface); border-color: var(--color-border);">
      <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Post-Fermentation Correction</h3>
      <label class="mt-4 block text-sm font-medium" style="color: var(--color-text-primary);">
        Original Brix
        <input bind:value={ogBrix} type="number" min="0" step="0.1"
               class="mt-2 w-full rounded px-3 py-2 text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </label>

      <div class="mt-4">
        <div class="flex items-center gap-1 mb-1">
          <span class="text-sm font-medium" style="color: var(--color-text-primary);">Final Brix Reading</span>
          <Tooltip text="Your raw refractometer reading post-fermentation. Alcohol skews this reading — the corrected FG shown here uses the Novotný formula." />
        </div>
        <input bind:value={fgBrix} type="number" min="0" step="0.1"
               class="w-full rounded px-3 py-2 text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>

      <div class="mt-6 rounded-lg border p-4" style="border-color: var(--color-accent);">
        {#if postResult}
          <div class="text-xs uppercase tracking-wide" style="color: var(--color-text-secondary);">Corrected FG</div>
          <div class="mt-2 text-4xl font-semibold" style="color: var(--color-text-primary);">{postResult.fgSg.toFixed(3)}</div>
        {/if}
      </div>
    </section>
  </div>
</div>
