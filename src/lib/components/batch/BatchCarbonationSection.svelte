<script lang="ts">
  import type { Batch, UpdateBatchInput } from "$lib/api";
  import { calculatePrimingSugar, calculateCo2Pressure, type SugarType } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { untrack } from "svelte";
  import { cToF, fToC, tempLabel, type Units } from "$lib/units";

  let {
    batch,
    recipePrimaryTempC,
    recipeCarbonationVols,
    onUpdate,
  }: {
    batch: Batch;
    recipePrimaryTempC: number | null;
    recipeCarbonationVols: number | null;
    onUpdate: (input: UpdateBatchInput) => void;
  } = $props();

  const units = $derived(($settings.units ?? "metric") as Units);
  const targetVols = $derived(recipeCarbonationVols ?? 2.4);
  const batchSizeL = $derived(batch.actual_batch_size_l ?? batch.planned_batch_size_l ?? null);

  let tempC = $state(batch.packaging_temp_c ?? recipePrimaryTempC ?? 20);
  let sugarType = $state<SugarType>(
    (batch.carbonation_sugar_type as SugarType) ?? "corn_sugar"
  );

  const tempDisplay = $derived(units === "imperial" ? cToF(tempC) : tempC);

  let primingSugarG = $state<number | null>(null);
  let pressureKpa = $state<number | null>(null);
  let hasInteracted = $state(false);

  $effect(() => {
    const vols = targetVols;
    const size = batchSizeL;
    const temp = tempC;
    const sugar = sugarType;

    if (!size || size <= 0 || vols <= 0) {
      primingSugarG = null;
      pressureKpa = null;
      return;
    }

    void (async () => {
      const [sg, kpa] = await Promise.all([
        ipc(calculatePrimingSugar(vols, size, temp, sugar)),
        ipc(calculateCo2Pressure(vols, temp)),
      ]);
      if (targetVols === vols && batchSizeL === size && tempC === temp && sugarType === sugar) {
        primingSugarG = sg ?? null;
        pressureKpa = kpa ?? null;
        // Read hasInteracted via untrack so it's not a reactive dependency —
        // the effect should only re-run when inputs change, not when interaction flag changes
        if (untrack(() => hasInteracted) && sg != null && kpa != null) {
          onUpdate({
            packaging_temp_c: temp,
            carbonation_sugar_type: sugar,
            priming_sugar_g: sg,
            serving_pressure_kpa: kpa,
          });
        }
      }
    })();
  });

  function updateTemp(value: string) {
    const next = Number(value);
    tempC = units === "imperial" ? fToC(next) : next;
  }
</script>

<div>
  <div
    class="text-xs mb-3 mt-4 pt-4 border-t font-semibold uppercase tracking-wide"
    style="color: var(--color-text-secondary); border-color: var(--color-border);"
  >
    Carbonation
  </div>

  <div class="grid grid-cols-2 md:grid-cols-3 gap-3 mb-4">
    <!-- Target vols — read only from recipe -->
    <div
      class="p-3 rounded"
      style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); opacity: 0.7;"
    >
      <div class="text-xs mb-1" style="color: var(--color-text-secondary);">Target CO₂ (vols)</div>
      <div class="text-sm font-medium" style="color: var(--color-text-primary);">{targetVols.toFixed(1)}</div>
      <div class="text-xs mt-0.5" style="color: var(--color-text-muted);">from recipe</div>
    </div>

    <!-- Packaging temp -->
    <div class="p-3 rounded" style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
      <label
        for="carb-temp"
        class="text-xs block mb-1"
        style="color: var(--color-text-secondary);"
      >
        Packaging Temp ({tempLabel(units)})
      </label>
      <input
        id="carb-temp"
        type="number"
        inputmode="decimal"
        step="0.1"
        value={tempDisplay.toFixed(1)}
        oninput={(e) => updateTemp((e.target as HTMLInputElement).value)}
        onblur={() => { hasInteracted = true; }}
        class="w-full bg-transparent text-sm outline-none"
        style="color: var(--color-text-primary);"
      />
    </div>

    <!-- Sugar type -->
    <div
      class="p-3 rounded col-span-2 md:col-span-1"
      style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);"
    >
      <label for="carb-sugar" class="text-xs block mb-1" style="color: var(--color-text-secondary);">
        Sugar Type
      </label>
      <select
        id="carb-sugar"
        value={sugarType}
        onchange={(e) => {
          hasInteracted = true;
          sugarType = (e.target as HTMLSelectElement).value as SugarType;
        }}
        class="w-full bg-transparent text-sm outline-none"
        style="color: var(--color-text-primary);"
      >
        <option value="corn_sugar">Corn sugar</option>
        <option value="table_sugar">Table sugar</option>
        <option value="dry_malt_extract">Dry malt extract</option>
      </select>
    </div>
  </div>

  {#if batchSizeL === null}
    <p class="text-sm" style="color: var(--color-text-muted);">
      Enter batch size in Measurements to calculate carbonation.
    </p>
  {:else if primingSugarG !== null && pressureKpa !== null}
    <div class="grid grid-cols-2 gap-3">
      <div
        class="p-4 rounded-lg"
        style="background: rgba(99,102,241,0.1); border: 1px solid rgba(99,102,241,0.3);"
      >
        <div
          class="text-xs font-bold uppercase tracking-wide mb-2"
          style="color: var(--color-text-secondary);"
        >
          Bottle Priming
        </div>
        <div class="text-3xl font-semibold" style="color: var(--color-text-primary);">
          {primingSugarG.toFixed(0)}<span
            class="text-base font-normal ml-1"
            style="color: var(--color-text-secondary);">g</span>
        </div>
        <div class="text-xs mt-1" style="color: var(--color-text-muted);">
          {sugarType.replace(/_/g, " ")} · {batchSizeL.toFixed(1)} L
        </div>
      </div>

      <div
        class="p-4 rounded-lg"
        style="background: rgba(99,102,241,0.1); border: 1px solid rgba(99,102,241,0.3);"
      >
        <div
          class="text-xs font-bold uppercase tracking-wide mb-2"
          style="color: var(--color-text-secondary);"
        >
          Keg Pressure
        </div>
        <div class="text-3xl font-semibold" style="color: var(--color-text-primary);">
          {pressureKpa.toFixed(0)}<span
            class="text-base font-normal ml-1"
            style="color: var(--color-text-secondary);">kPa</span>
        </div>
        <div class="text-xs mt-1" style="color: var(--color-text-muted);">
          {(pressureKpa * 0.145038).toFixed(1)} PSI
        </div>
      </div>
    </div>
  {/if}
</div>
