<script lang="ts">
  import { convertColor, type ColorConversionResult, type ColorUnit } from "$lib/api";
  import { ipc } from "$lib/stores/error";

  let fromUnit = $state<ColorUnit>("srm");
  let value = $state(10);
  let result = $state<ColorConversionResult | null>(null);

  $effect(() => {
    const currentUnit = fromUnit;
    const currentValue = value;

    if (Number.isNaN(currentValue) || currentValue < 0) {
      result = null;
      return;
    }

    void (async () => {
      const next = await ipc(convertColor(currentValue, currentUnit));
      if (next && fromUnit === currentUnit && value === currentValue) result = next;
    })();
  });
</script>

<div class="p-6 md:p-8">
  <h2 class="text-xl font-semibold text-text-primary">Color Conversion</h2>
  <p class="mt-2 max-w-2xl text-sm text-text-secondary">
    Convert malt or beer color values between SRM, EBC, and Lovibond.
  </p>

  <div class="mt-6 grid gap-6 lg:grid-cols-[minmax(0,20rem)_minmax(0,1fr)]">
    <section class="rounded-xl border p-4 bg-bg-surface border-border">
      <label class="block text-sm font-medium text-text-primary">
        Input Unit
        <select bind:value={fromUnit}
                class="mt-2 w-full rounded px-3 py-2 text-sm bg-bg-elevated text-text-primary border border-border"
               >
          <option value="srm">SRM</option>
          <option value="ebc">EBC</option>
          <option value="lovibond">Lovibond</option>
        </select>
      </label>

      <label class="mt-4 block text-sm font-medium text-text-primary">
        Value
        <input bind:value={value} type="number" min="0" step="0.1"
               class="mt-2 w-full rounded px-3 py-2 text-sm bg-bg-elevated text-text-primary border border-border"
               />
      </label>
    </section>

    <section class="rounded-xl border p-5 bg-bg-surface border-accent">
      {#if result}
        <div class="grid gap-4 md:grid-cols-3">
          <div>
            <div class="text-xs uppercase tracking-wide text-text-secondary">SRM</div>
            <div class="mt-2 text-3xl font-semibold text-text-primary">{result.srm.toFixed(1)}</div>
          </div>
          <div>
            <div class="text-xs uppercase tracking-wide text-text-secondary">EBC</div>
            <div class="mt-2 text-3xl font-semibold text-text-primary">{result.ebc.toFixed(1)}</div>
          </div>
          <div>
            <div class="text-xs uppercase tracking-wide text-text-secondary">Lovibond</div>
            <div class="mt-2 text-3xl font-semibold text-text-primary">{result.lovibond.toFixed(1)}</div>
          </div>
        </div>
      {/if}
    </section>
  </div>
</div>
