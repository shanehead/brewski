<script lang="ts">
  import { convertGravity, type GravityConversionResult, type GravityUnit } from "$lib/api";
  import { ipc } from "$lib/stores/error";

  let fromUnit = $state<GravityUnit>("sg");
  let value = $state(1.05);
  let result = $state<GravityConversionResult | null>(null);

  $effect(() => {
    const currentUnit = fromUnit;
    const currentValue = value;

    if (Number.isNaN(currentValue) || currentValue <= 0) {
      result = null;
      return;
    }

    void (async () => {
      const next = await ipc(convertGravity(currentValue, currentUnit));
      if (next && fromUnit === currentUnit && value === currentValue) result = next;
    })();
  });
</script>

<div class="p-6 md:p-8">
  <h2 class="text-xl font-semibold text-text-primary">Gravity Conversions</h2>
  <p class="mt-2 max-w-2xl text-sm text-text-secondary">
    Convert between specific gravity, Plato, and Brix using the same backend formulas as the other calculators.
  </p>

  <div class="mt-6 grid gap-6 lg:grid-cols-[minmax(0,20rem)_minmax(0,1fr)]">
    <section class="rounded-xl border p-4 bg-bg-surface border-border">
      <label class="block text-sm font-medium text-text-primary">
        Input Unit
        <select bind:value={fromUnit}
                class="mt-2 w-full rounded px-3 py-2 text-sm bg-bg-elevated text-text-primary border border-border"
               >
          <option value="sg">Specific gravity</option>
          <option value="plato">Plato</option>
          <option value="brix">Brix</option>
        </select>
      </label>

      <label class="mt-4 block text-sm font-medium text-text-primary">
        Value
        <input bind:value={value} type="number" min="0" step="0.001"
               class="mt-2 w-full rounded px-3 py-2 text-sm bg-bg-elevated text-text-primary border border-border"
               />
      </label>
    </section>

    <section class="rounded-xl border p-5 bg-bg-surface border-accent">
      {#if result}
        <div class="grid gap-4 md:grid-cols-3">
          <div>
            <div class="text-xs uppercase tracking-wide text-text-secondary">SG</div>
            <div class="mt-2 text-3xl font-semibold text-text-primary">{result.sg.toFixed(3)}</div>
          </div>
          <div>
            <div class="text-xs uppercase tracking-wide text-text-secondary">Plato</div>
            <div class="mt-2 text-3xl font-semibold text-text-primary">{result.plato.toFixed(1)}°P</div>
          </div>
          <div>
            <div class="text-xs uppercase tracking-wide text-text-secondary">Brix</div>
            <div class="mt-2 text-3xl font-semibold text-text-primary">{result.brix.toFixed(1)}°Bx</div>
          </div>
        </div>
      {/if}
    </section>
  </div>
</div>
