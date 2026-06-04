<script lang="ts">
  import { calculateAbvCalories, type AbvCaloriesResult } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import Tooltip from "$lib/components/Tooltip.svelte";
  import DocLink from "$lib/components/DocLink.svelte";
  import { DOCS } from "$lib/docs-urls";

  let og = $state(1.052);
  let fg = $state(1.013);
  let result = $state<AbvCaloriesResult | null>(null);

  $effect(() => {
    const currentOg = og;
    const currentFg = fg;

    if (currentOg <= 1 || currentFg <= 0 || currentFg > currentOg) {
      result = null;
      return;
    }

    void (async () => {
      const next = await ipc(calculateAbvCalories(currentOg, currentFg));
      if (next && og === currentOg && fg === currentFg) result = next;
    })();
  });
</script>

<div class="p-6 md:p-8">
  <div class="flex items-center gap-2">
    <h2 class="text-xl font-semibold" style="color: var(--color-text-primary);">ABV / Attenuation / Calories</h2>
    <DocLink label="ABV & Calories reference" url={DOCS.calcAbv} />
  </div>
  <p class="mt-2 max-w-2xl text-sm" style="color: var(--color-text-secondary);">
    Estimate beer strength, apparent attenuation, and calories per 12 oz serving from original and final gravity.
  </p>

  <div class="mt-6 grid gap-6 lg:grid-cols-[minmax(0,20rem)_minmax(0,1fr)]">
    <section class="rounded-xl border p-4" style="background: var(--color-bg-surface); border-color: var(--color-border);">
      <div>
        <div class="flex items-center gap-1 mb-1">
          <span class="text-sm font-medium" style="color: var(--color-text-primary);">Original Gravity</span>
          <Tooltip text="Original gravity — the sugar content of your wort before fermentation. Typical ales: 1.040 to 1.080." />
        </div>
        <input bind:value={og} type="number" min="1" max="1.2" step="0.001"
               class="w-full rounded px-3 py-2 text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>

      <div class="mt-4">
        <div class="flex items-center gap-1 mb-1">
          <span class="text-sm font-medium" style="color: var(--color-text-primary);">Final Gravity</span>
          <Tooltip text="Final gravity — what's left after the yeast finishes. The difference from OG gives you ABV and attenuation." />
        </div>
        <input bind:value={fg} type="number" min="0.99" max="1.2" step="0.001"
               class="w-full rounded px-3 py-2 text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>
    </section>

    <section class="rounded-xl border p-5" style="background: var(--color-bg-surface); border-color: var(--color-accent);">
      {#if result}
        <div class="grid gap-4 md:grid-cols-3">
          <div>
            <div class="text-xs uppercase tracking-wide" style="color: var(--color-text-secondary);">ABV</div>
            <div class="mt-1 text-3xl font-semibold" style="color: var(--color-text-primary);">{result.abvPct.toFixed(1)}%</div>
          </div>
          <div>
            <div class="text-xs uppercase tracking-wide" style="color: var(--color-text-secondary);">Attenuation</div>
            <div class="mt-1 text-3xl font-semibold" style="color: var(--color-text-primary);">{result.attenuationPct.toFixed(1)}%</div>
          </div>
          <div>
            <div class="text-xs uppercase tracking-wide" style="color: var(--color-text-secondary);">Calories / 12 oz</div>
            <div class="mt-1 text-3xl font-semibold" style="color: var(--color-text-primary);">{result.caloriesPer355ml.toFixed(0)}</div>
          </div>
        </div>
      {:else}
        <p class="text-sm" style="color: var(--color-text-secondary);">Enter a valid OG and FG to calculate results.</p>
      {/if}
    </section>
  </div>
</div>
