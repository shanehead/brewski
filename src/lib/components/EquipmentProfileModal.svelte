<script lang="ts">
  import { createEquipmentProfile, updateEquipmentProfile } from "$lib/api";
  import type { EquipmentProfile, CreateEquipmentProfileInput, UpdateEquipmentProfileInput } from "$lib/api";
  import { ipc } from "$lib/stores/error";

  let {
    profile = null,
    onsave,
    oncancel,
  }: {
    profile?: EquipmentProfile | null;
    onsave: (saved: EquipmentProfile) => void;
    oncancel: () => void;
  } = $props();

  // ── form state ──────────────────────────────────────────────────────────
  let name = $state(profile?.name ?? "");
  let notes = $state(profile?.notes ?? "");
  let boilTimeMin = $state(profile?.boil_time_min ?? 60);

  // Volumes
  let batchVolumeTarget = $state(profile?.batch_volume_target ?? "fermenter");
  let batchSizeL = $state(profile?.batch_size_l ?? 23);
  let calcBoilVolume = $state(profile?.calc_boil_volume ?? true);
  let boilSizeL = $state(profile?.boil_size_l ?? 27);
  let evapRatePctHr = $state(profile?.evap_rate_pct_hr ?? 10);
  let trubChillerLossL = $state(profile?.trub_chiller_loss_l ?? 0);
  let lauterDeadspaceL = $state(profile?.lauter_deadspace_l ?? 0);
  let mashTunLossL = $state(profile?.mash_tun_loss_l ?? 0);
  let hltDeadspaceL = $state<number | null>(profile?.hlt_deadspace_l ?? null);
  let fermenterLossL = $state(profile?.fermenter_loss_l ?? 0);
  let topUpWaterL = $state(profile?.top_up_water_l ?? 0);
  let coolingShrinkagePct = $state(profile?.cooling_shrinkage_pct ?? 4);

  // Efficiency
  let efficiencyPct = $state(profile?.efficiency_pct ?? 72);
  let calcMashEfficiency = $state(profile?.calc_mash_efficiency ?? true);
  let mashEfficiencyPct = $state<number | null>(profile?.mash_efficiency_pct ?? null);

  // Hops
  let hopUtilizationPct = $state(profile?.hop_utilization_pct ?? 100);
  let calcAromaHopUtilization = $state(profile?.calc_aroma_hop_utilization ?? true);
  let aromaHopUtilizationPct = $state(profile?.aroma_hop_utilization_pct ?? 23);
  let whirlpoolTimeMin = $state<number | null>(profile?.whirlpool_time_min ?? null);

  // Boil temperature
  let altitudeAdjustment = $state(profile?.altitude_adjustment ?? false);
  let boilTempF = $state<number | null>(profile?.boil_temp_f ?? null);

  // Mash / Sparge
  let tunVolumeL = $state<number | null>(profile?.tun_volume_l ?? null);
  let tunWeightKg = $state<number | null>(profile?.tun_weight_kg ?? null);
  let spargeMethod = $state(profile?.sparge_method ?? "no_sparge");
  let mashVolumeMinL = $state<number | null>(profile?.mash_volume_min_l ?? null);
  let mashVolumeMaxL = $state<number | null>(profile?.mash_volume_max_l ?? null);
  let spargeVolumeMinL = $state<number | null>(profile?.sparge_volume_min_l ?? null);
  let spargeVolumeMaxL = $state<number | null>(profile?.sparge_volume_max_l ?? null);
  let calcStrikeWaterTemp = $state(profile?.calc_strike_water_temp ?? false);

  let saving = $state(false);

  // ── derived display values ───────────────────────────────────────────────
  let postBoilColdL = $derived(batchSizeL + trubChillerLossL + fermenterLossL - topUpWaterL);
  let boilHours = $derived(boilTimeMin / 60);
  let evapFraction = $derived(evapRatePctHr / 100 * boilHours);
  let preBoilColdL = $derived(postBoilColdL / (1 - evapFraction) + mashTunLossL);
  let preBoilHotL = $derived(preBoilColdL * (1 + coolingShrinkagePct / 100));
  let postBoilHotL = $derived(postBoilColdL * (1 + coolingShrinkagePct / 100));
  let evapPct = $derived(evapRatePctHr * boilHours);
  let batchLabel = $derived(batchVolumeTarget === "kettle" ? "Batch Volume (Kettle)" : "Batch Volume (Fermenter)");

  async function handleSave() {
    saving = true;
    const base = {
      name,
      notes: notes || undefined,
      boil_time_min: boilTimeMin,
      batch_volume_target: batchVolumeTarget,
      batch_size_l: batchSizeL,
      calc_boil_volume: calcBoilVolume,
      boil_size_l: calcBoilVolume ? preBoilColdL : boilSizeL,
      evap_rate_pct_hr: evapRatePctHr,
      trub_chiller_loss_l: trubChillerLossL,
      lauter_deadspace_l: lauterDeadspaceL,
      mash_tun_loss_l: mashTunLossL,
      hlt_deadspace_l: hltDeadspaceL ?? undefined,
      fermenter_loss_l: fermenterLossL,
      top_up_water_l: topUpWaterL,
      cooling_shrinkage_pct: coolingShrinkagePct,
      efficiency_pct: efficiencyPct,
      calc_mash_efficiency: calcMashEfficiency,
      mash_efficiency_pct: mashEfficiencyPct ?? undefined,
      hop_utilization_pct: hopUtilizationPct,
      calc_aroma_hop_utilization: calcAromaHopUtilization,
      aroma_hop_utilization_pct: aromaHopUtilizationPct,
      whirlpool_time_min: whirlpoolTimeMin ?? undefined,
      altitude_adjustment: altitudeAdjustment,
      boil_temp_f: boilTempF ?? undefined,
      tun_volume_l: tunVolumeL ?? undefined,
      tun_weight_kg: tunWeightKg ?? undefined,
      sparge_method: spargeMethod,
      mash_volume_min_l: mashVolumeMinL ?? undefined,
      mash_volume_max_l: mashVolumeMaxL ?? undefined,
      sparge_volume_min_l: spargeVolumeMinL ?? undefined,
      sparge_volume_max_l: spargeVolumeMaxL ?? undefined,
      calc_strike_water_temp: calcStrikeWaterTemp,
    };

    const saved = profile
      ? await ipc(updateEquipmentProfile(profile.id, base as UpdateEquipmentProfileInput))
      : await ipc(createEquipmentProfile(base as CreateEquipmentProfileInput));

    saving = false;
    if (saved) onsave(saved);
  }

  function numInput(e: Event) {
    return parseFloat((e.target as HTMLInputElement).value) || 0;
  }
  function nullableNumInput(e: Event) {
    const v = parseFloat((e.target as HTMLInputElement).value);
    return isNaN(v) ? null : v;
  }
</script>

<div class="fixed inset-0 z-50 flex items-start justify-center overflow-y-auto py-8"
     style="background: rgba(0,0,0,0.6);"
     role="dialog"
     aria-modal="true"
     onclick={(e) => e.target === e.currentTarget && oncancel()}>

  <div class="w-full max-w-2xl rounded-lg shadow-xl flex flex-col"
       style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">

    <!-- Header row -->
    <div class="flex items-center justify-between px-6 py-4 border-b" style="border-color: var(--color-border);">
      <h2 class="text-base font-semibold" style="color: var(--color-text-primary);">
        {profile ? "Edit Equipment Profile" : "New Equipment Profile"}
      </h2>
      <button onclick={oncancel} class="text-lg leading-none" style="color: var(--color-text-secondary);">✕</button>
    </div>

    <!-- Body -->
    <div class="px-6 py-4 flex flex-col gap-6 overflow-y-auto">

      <!-- Name / Boil Time / Description -->
      <div class="grid grid-cols-2 gap-4">
        <div class="flex flex-col gap-1">
          <label class="text-xs" style="color: var(--color-text-secondary);">Name</label>
          <input type="text" bind:value={name} class="eq-field-input" />
        </div>
        <div class="flex flex-col gap-1">
          <label class="text-xs" style="color: var(--color-text-secondary);">Boil Time <span style="color: var(--color-text-tertiary);">min</span></label>
          <input type="number" value={boilTimeMin} oninput={(e) => boilTimeMin = numInput(e)} class="eq-field-input" />
        </div>
        <div class="col-span-2 flex flex-col gap-1">
          <label class="text-xs" style="color: var(--color-text-secondary);">Description</label>
          <input type="text" bind:value={notes} class="eq-field-input" />
        </div>
      </div>

      <!-- Volumes -->
      <section>
        <h3 class="eq-section-label">Volumes</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Batch Volume Target</label>
            <select bind:value={batchVolumeTarget} class="eq-field-input">
              <option value="fermenter">Fermenter</option>
              <option value="kettle">Kettle</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">{batchLabel} <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.1" value={batchSizeL} oninput={(e) => batchSizeL = numInput(e)} class="eq-field-input" />
          </div>

          <div class="flex items-center gap-2">
            <input type="checkbox" id="calc-boil" bind:checked={calcBoilVolume} />
            <label for="calc-boil" class="text-sm" style="color: var(--color-text-primary);">Calc boil volume</label>
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Pre-Boil Volume* <span style="color: var(--color-text-tertiary);">L</span></label>
            {#if calcBoilVolume}
              <div class="eq-field-display">{preBoilHotL.toFixed(2)} <span style="color: var(--color-text-tertiary);">(hot)</span></div>
            {:else}
              <input type="number" step="0.1" value={boilSizeL} oninput={(e) => boilSizeL = numInput(e)} class="eq-field-input" />
            {/if}
          </div>

          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Boil Off <span style="color: var(--color-text-tertiary);">({evapPct.toFixed(1)}%) L/hr</span></label>
            <input type="number" step="0.1" value={evapRatePctHr} oninput={(e) => evapRatePctHr = numInput(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Trub/Chiller Loss <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.01" value={trubChillerLossL} oninput={(e) => trubChillerLossL = numInput(e)} class="eq-field-input" />
          </div>

          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Mash-Tun Deadspace <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.01" value={lauterDeadspaceL} oninput={(e) => lauterDeadspaceL = numInput(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Mash-Tun Loss <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.01" value={mashTunLossL} oninput={(e) => mashTunLossL = numInput(e)} class="eq-field-input" />
          </div>

          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">HLT Deadspace <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.01" placeholder="optional"
                   value={hltDeadspaceL ?? ""} oninput={(e) => hltDeadspaceL = nullableNumInput(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Fermenter Loss <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.01" value={fermenterLossL} oninput={(e) => fermenterLossL = numInput(e)} class="eq-field-input" />
          </div>

          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Fermenter Top-Up <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.01" placeholder="optional"
                   value={topUpWaterL || ""} oninput={(e) => topUpWaterL = numInput(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Cooling Shrinkage <span style="color: var(--color-text-tertiary);">%</span></label>
            <input type="number" step="0.1" value={coolingShrinkagePct} oninput={(e) => coolingShrinkagePct = numInput(e)} class="eq-field-input" />
          </div>
        </div>
        <p class="text-xs mt-2 text-right" style="color: var(--color-text-tertiary);">
          Post-Boil Kettle: {postBoilHotL.toFixed(2)} L &nbsp;·&nbsp; *Pre-Boil is hot (incl. {coolingShrinkagePct}% expansion)
        </p>
      </section>

      <!-- Efficiency -->
      <section>
        <h3 class="eq-section-label">Efficiency</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Brewhouse Efficiency <span style="color: var(--color-text-tertiary);">%</span></label>
            <input type="number" step="0.1" value={efficiencyPct} oninput={(e) => efficiencyPct = numInput(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Mash Efficiency <span style="color: var(--color-text-tertiary);">%</span></label>
            {#if calcMashEfficiency}
              <div class="eq-field-display" style="color: var(--color-text-tertiary);">calculated</div>
            {:else}
              <input type="number" step="0.1" placeholder="optional"
                     value={mashEfficiencyPct ?? ""} oninput={(e) => mashEfficiencyPct = nullableNumInput(e)} class="eq-field-input" />
            {/if}
          </div>
          <div class="flex items-center gap-2 col-span-2">
            <input type="checkbox" id="calc-mash-eff" bind:checked={calcMashEfficiency} />
            <label for="calc-mash-eff" class="text-sm" style="color: var(--color-text-primary);">Calc mash efficiency</label>
          </div>
        </div>
      </section>

      <!-- Hops -->
      <section>
        <h3 class="eq-section-label">Hops</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Hop Utilization Multiplier <span style="color: var(--color-text-tertiary);">%</span></label>
            <input type="number" step="1" value={hopUtilizationPct} oninput={(e) => hopUtilizationPct = numInput(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Aroma Hop Utilization <span style="color: var(--color-text-tertiary);">%</span></label>
            {#if calcAromaHopUtilization}
              <div class="eq-field-display" style="color: var(--color-text-tertiary);">calculated</div>
            {:else}
              <input type="number" step="0.1" value={aromaHopUtilizationPct} oninput={(e) => aromaHopUtilizationPct = numInput(e)} class="eq-field-input" />
            {/if}
          </div>
          <div class="flex items-center gap-2 col-span-2">
            <input type="checkbox" id="calc-aroma" bind:checked={calcAromaHopUtilization} />
            <label for="calc-aroma" class="text-sm" style="color: var(--color-text-primary);">Calc aroma hop utilization</label>
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Whirlpool / No-Chill Time <span style="color: var(--color-text-tertiary);">min</span></label>
            <input type="number" step="1" placeholder="optional"
                   value={whirlpoolTimeMin ?? ""} oninput={(e) => whirlpoolTimeMin = nullableNumInput(e)} class="eq-field-input" />
          </div>
        </div>
      </section>

      <!-- Boil Temperature -->
      <section>
        <h3 class="eq-section-label">Boil Temperature</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex items-center gap-2 col-span-2">
            <input type="checkbox" id="altitude-adj" bind:checked={altitudeAdjustment} />
            <label for="altitude-adj" class="text-sm" style="color: var(--color-text-primary);">Altitude adjustment</label>
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Boil Temperature <span style="color: var(--color-text-tertiary);">°F</span></label>
            {#if altitudeAdjustment}
              <div class="eq-field-display" style="color: var(--color-text-tertiary);">calculated from altitude</div>
            {:else}
              <input type="number" step="1" placeholder="212"
                     value={boilTempF ?? ""} oninput={(e) => boilTempF = nullableNumInput(e)} class="eq-field-input" />
            {/if}
          </div>
        </div>
      </section>

      <!-- Mash / Sparge Water -->
      <section>
        <h3 class="eq-section-label">Mash / Sparge Water</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Tun Volume <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.1" placeholder="optional"
                   value={tunVolumeL ?? ""} oninput={(e) => tunVolumeL = nullableNumInput(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Tun Weight <span style="color: var(--color-text-tertiary);">kg</span></label>
            <input type="number" step="0.1" placeholder="optional"
                   value={tunWeightKg ?? ""} oninput={(e) => tunWeightKg = nullableNumInput(e)} class="eq-field-input" />
          </div>
          <div class="col-span-2 flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Sparge Method</label>
            <select bind:value={spargeMethod} class="eq-field-input">
              <option value="no_sparge">No Sparge</option>
              <option value="batch_sparge">Batch Sparge</option>
              <option value="fly_sparge">Fly Sparge</option>
            </select>
          </div>

          <div class="col-span-2 text-xs font-medium mt-1" style="color: var(--color-text-secondary);">Mash Volume Limits</div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Min <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.1" placeholder="optional"
                   value={mashVolumeMinL ?? ""} oninput={(e) => mashVolumeMinL = nullableNumInput(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Max <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.1" placeholder="optional"
                   value={mashVolumeMaxL ?? ""} oninput={(e) => mashVolumeMaxL = nullableNumInput(e)} class="eq-field-input" />
          </div>

          <div class="col-span-2 text-xs font-medium mt-1" style="color: var(--color-text-secondary);">Sparge Volume Limits</div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Min <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.1" placeholder="optional"
                   value={spargeVolumeMinL ?? ""} oninput={(e) => spargeVolumeMinL = nullableNumInput(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Max <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.1" placeholder="optional"
                   value={spargeVolumeMaxL ?? ""} oninput={(e) => spargeVolumeMaxL = nullableNumInput(e)} class="eq-field-input" />
          </div>

          <div class="flex items-center gap-2 col-span-2 mt-1">
            <input type="checkbox" id="calc-strike" bind:checked={calcStrikeWaterTemp} />
            <label for="calc-strike" class="text-sm" style="color: var(--color-text-primary);">Calc strike water temperature</label>
          </div>
        </div>
      </section>

    </div>

    <!-- Footer -->
    <div class="flex justify-end gap-3 px-6 py-4 border-t" style="border-color: var(--color-border);">
      <button onclick={oncancel} class="px-4 py-2 rounded text-sm"
              style="background: var(--color-bg-base); color: var(--color-text-secondary); border: 1px solid var(--color-border);">
        Cancel
      </button>
      <button onclick={handleSave} disabled={saving || !name.trim()} class="px-4 py-2 rounded text-sm"
              style="background: var(--color-accent); color: #fff; opacity: {saving || !name.trim() ? 0.5 : 1};">
        {saving ? "Saving…" : "Save"}
      </button>
    </div>

  </div>
</div>

<style>
  .eq-field-input {
    width: 100%;
    padding: 0.375rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.875rem;
    background: var(--color-bg-base);
    color: var(--color-text-primary);
    border: 1px solid var(--color-border);
  }
  .eq-field-display {
    padding: 0.375rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.875rem;
    background: var(--color-bg-base);
    border: 1px solid var(--color-border);
    opacity: 0.6;
  }
  .eq-section-label {
    font-size: 0.7rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--color-text-secondary);
    margin-bottom: 0.75rem;
  }
</style>
