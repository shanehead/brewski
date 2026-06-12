<script lang="ts">
  import { untrack } from "svelte";
  import { createEquipmentProfile, updateEquipmentProfile } from "$lib/api";
  import type { EquipmentProfile, CreateEquipmentProfileInput, UpdateEquipmentProfileInput } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { lToGal, galToL, volumeLabel, kgToLb, lbToKg, weightLabel, cToF, fToC, tempLabel, type Units } from "$lib/units";
  import FieldLabel from "$lib/components/FieldLabel.svelte";
  import FloatInput from "$lib/components/FloatInput.svelte";

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
  // untrack() intentional: modal remounts fresh each time it opens
  let name = $state(untrack(() => profile?.name ?? ""));
  let notes = $state(untrack(() => profile?.notes ?? ""));
  let boilTimeMin = $state(untrack(() => profile?.boil_time_min ?? 60));

  // Volumes
  let batchVolumeTarget = $state(untrack(() => profile?.batch_volume_target ?? "fermenter"));
  let batchSizeL = $state(untrack(() => profile?.batch_size_l ?? 23));
  let calcBoilVolume = $state(untrack(() => profile?.calc_boil_volume ?? true));
  let boilSizeL = $state(untrack(() => profile?.boil_size_l ?? 27));
  let evapRateLHr = $state(untrack(() => profile?.evap_rate_l_hr ?? 3.8));
  let trubChillerLossL = $state(untrack(() => profile?.trub_chiller_loss_l ?? 0));
  let mashTunDeadspaceL = $state(untrack(() => profile?.mash_tun_deadspace_l ?? 0));
  let mashTunLossL = $state(untrack(() => profile?.mash_tun_loss_l ?? 0));
  let hltDeadspaceL = $state<number | null>(untrack(() => profile?.hlt_deadspace_l ?? null));
  let fermenterLossL = $state(untrack(() => profile?.fermenter_loss_l ?? 0));
  let topUpWaterL = $state(untrack(() => profile?.top_up_water_l ?? 0));
  let coolingShrinkagePct = $state(untrack(() => profile?.cooling_shrinkage_pct ?? 4));

  // Efficiency
  let efficiencyPct = $state(untrack(() => profile?.efficiency_pct ?? 72));
  let calcMashEfficiency = $state(untrack(() => profile?.calc_mash_efficiency ?? true));
  let mashEfficiencyPct = $state<number | null>(untrack(() => profile?.mash_efficiency_pct ?? null));

  // Hops
  let hopUtilizationPct = $state(untrack(() => profile?.hop_utilization_pct ?? 100));
  let calcAromaHopUtilization = $state(untrack(() => profile?.calc_aroma_hop_utilization ?? true));
  let aromaHopUtilizationPct = $state(untrack(() => profile?.aroma_hop_utilization_pct ?? 23));
  let whirlpoolTimeMin = $state<number | null>(untrack(() => profile?.whirlpool_time_min ?? null));

  // Boil temperature
  let altitudeAdjustment = $state(untrack(() => profile?.altitude_adjustment ?? false));
  let boilTempF = $state<number | null>(untrack(() => profile?.boil_temp_f ?? null));

  // Mash / Sparge
  let spargeMethod = $state(untrack(() => profile?.sparge_method ?? "no_sparge"));
  let mashVolumeMinL = $state<number | null>(untrack(() => profile?.mash_volume_min_l ?? null));
  let mashVolumeMaxL = $state<number | null>(untrack(() => profile?.mash_volume_max_l ?? null));
  let spargeVolumeMinL = $state<number | null>(untrack(() => profile?.sparge_volume_min_l ?? null));
  let spargeVolumeMaxL = $state<number | null>(untrack(() => profile?.sparge_volume_max_l ?? null));
  let calcStrikeWaterTemp = $state(untrack(() => profile?.calc_strike_water_temp ?? false));

  // New parity fields
  let tunHeatCapacityL = $state(untrack(() => profile?.tun_heat_capacity_l ?? 0));
  let hopstandTempF = $state(untrack(() => profile?.hopstand_temp_f ?? 176));
  let grainAbsorptionRateLPerKg = $state(untrack(() => profile?.grain_absorption_rate_l_per_kg ?? 1.04));
  let waterGrainRatioLPerKg = $state(untrack(() => profile?.water_grain_ratio_l_per_kg ?? 3.12));
  let includeGrainVolumeInMashLimits = $state(untrack(() => profile?.include_grain_volume_in_mash_limits ?? true));
  let overflowTarget = $state(untrack(() => profile?.overflow_target ?? "mash"));
  let hltWaterLimitMinL = $state<number | null>(untrack(() => profile?.hlt_water_limit_min_l ?? null));
  let roomTempF = $state(untrack(() => profile?.room_temp_f ?? 68));
  let grainTempF = $state(untrack(() => profile?.grain_temp_f ?? 68));
  let spargeTempF = $state<number | null>(untrack(() => profile?.sparge_temp_f ?? null));

  let saving = $state(false);

  let units = $derived(($settings.units ?? "metric") as Units);

  // ── unit conversion helpers (number-in / number-out for FloatInput) ──────
  function volVal(l: number): number { return units === "imperial" ? lToGal(l) : l; }
  function volSave(v: number | null): number { return units === "imperial" ? galToL(v ?? 0) : (v ?? 0); }
  function volValNull(l: number | null): number | null { return l != null ? volVal(l) : null; }
  function volSaveNull(v: number | null): number | null { return v != null ? (units === "imperial" ? galToL(v) : v) : null; }

  function tempVal(f: number | null): number | null { return f != null ? (units === "imperial" ? f : fToC(f)) : null; }
  function tempSave(v: number | null): number | null { return v != null ? (units === "imperial" ? v : cToF(v)) : null; }

  function ratioVal(r: number): number { return units === "imperial" ? r * 0.4796 : r; }
  function ratioSave(v: number | null): number { return units === "imperial" ? (v ?? 0) / 0.4796 : (v ?? 0); }

  // ── derived display values ───────────────────────────────────────────────
  let postBoilColdL = $derived(batchSizeL + trubChillerLossL + fermenterLossL - topUpWaterL);
  let boilHours = $derived(boilTimeMin / 60);
  let preBoilColdL = $derived(postBoilColdL + evapRateLHr * boilHours + mashTunLossL);
  let preBoilHotL = $derived(preBoilColdL * (1 + coolingShrinkagePct / 100));
  let postBoilHotL = $derived(postBoilColdL * (1 + coolingShrinkagePct / 100));
  let calcEvapPct = $derived(preBoilColdL > 0 ? (evapRateLHr * boilHours / preBoilColdL) * 100 : 0);
  let ratioLabel = $derived(units === "imperial" ? "qt/lb" : "L/kg");
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
      evap_rate_l_hr: evapRateLHr,
      tun_heat_capacity_l: tunHeatCapacityL,
      hopstand_temp_f: hopstandTempF,
      grain_absorption_rate_l_per_kg: grainAbsorptionRateLPerKg,
      water_grain_ratio_l_per_kg: waterGrainRatioLPerKg,
      include_grain_volume_in_mash_limits: includeGrainVolumeInMashLimits,
      overflow_target: overflowTarget,
      hlt_water_limit_min_l: hltWaterLimitMinL ?? undefined,
      room_temp_f: roomTempF,
      grain_temp_f: grainTempF,
      sparge_temp_f: spargeTempF ?? undefined,
      trub_chiller_loss_l: trubChillerLossL,
      mash_tun_deadspace_l: mashTunDeadspaceL,
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


</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && oncancel()} />

<div class="fixed inset-0 z-50 flex items-start justify-center overflow-y-auto py-8"
     style="background: rgba(0,0,0,0.6);"
     role="none"
     onclick={(e) => e.target === e.currentTarget && oncancel()}
     onkeydown={(e) => e.key === "Escape" && oncancel()}>

  <div class="w-full max-w-2xl rounded-lg shadow-xl flex flex-col bg-bg-elevated border border-border"
       role="dialog" aria-modal="true">

    <!-- Header row -->
    <div class="flex items-center justify-between px-6 py-4 border-b border-border">
      <h2 class="text-base font-semibold text-text-primary">
        {profile ? "Edit Equipment Profile" : "New Equipment Profile"}
      </h2>
      <button onclick={oncancel} class="text-lg leading-none text-text-secondary">✕</button>
    </div>

    <!-- Body -->
    <div class="px-6 py-4 flex flex-col gap-6 overflow-y-auto">

      <!-- Name / Boil Time / Description -->
      <div class="grid grid-cols-2 gap-4">
        <div class="flex flex-col gap-1">
          <label for="eq-name" class="text-xs text-text-secondary">Name</label>
          <input id="eq-name" type="text" bind:value={name} class="eq-field-input" />
        </div>
        <div class="flex flex-col gap-1">
          <label for="eq-boil-time" class="text-xs text-text-secondary">Boil Time <span class="text-text-tertiary">min</span></label>
          <FloatInput id="eq-boil-time" decimals={0} value={boilTimeMin} oncommit={(v) => boilTimeMin = v ?? boilTimeMin} class="eq-field-input" />
        </div>
        <div class="col-span-2 flex flex-col gap-1">
          <label for="eq-notes" class="text-xs text-text-secondary">Description</label>
          <input id="eq-notes" type="text" bind:value={notes} class="eq-field-input" />
        </div>
      </div>

      <!-- Volumes -->
      <section>
        <h3 class="eq-section-label">Volumes</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-1">
            <label for="eq-batch-target" class="text-xs text-text-secondary">Batch Volume Target</label>
            <select id="eq-batch-target" bind:value={batchVolumeTarget} class="eq-field-input">
              <option value="fermenter">Fermenter</option>
              <option value="kettle">Kettle</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-batch-size" class="text-xs text-text-secondary">{batchLabel} <span class="text-text-tertiary">{volumeLabel(units)}</span></label>
            <FloatInput id="eq-batch-size" step="0.1" decimals={2} value={volVal(batchSizeL)} oncommit={(v) => batchSizeL = volSave(v)} class="eq-field-input" />
          </div>

          <div class="flex items-center gap-2">
            <input type="checkbox" id="calc-boil" bind:checked={calcBoilVolume} />
            <label for="calc-boil" class="text-sm text-text-primary">Calc boil volume</label>
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-boil-size" class="text-xs text-text-secondary">Pre-Boil Volume* <span class="text-text-tertiary">{volumeLabel(units)}</span></label>
            {#if calcBoilVolume}
              <div id="eq-boil-size" class="eq-field-display">{(units === "imperial" ? lToGal(preBoilHotL) : preBoilHotL).toFixed(2)} <span class="text-text-tertiary">(hot)</span></div>
            {:else}
              <FloatInput id="eq-boil-size" step="0.1" decimals={2} value={volVal(boilSizeL)} oncommit={(v) => boilSizeL = volSave(v)} class="eq-field-input" />
            {/if}
          </div>

          <div class="flex flex-col gap-1">
            <label for="eq-evap-rate" class="text-xs text-text-secondary">Boil Off <span class="text-text-tertiary">({calcEvapPct.toFixed(1)}%) {volumeLabel(units)}/hr</span></label>
            <FloatInput id="eq-evap-rate" step="0.01" decimals={2} value={volVal(evapRateLHr)} oncommit={(v) => evapRateLHr = volSave(v)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-trub-loss" class="text-xs text-text-secondary">Trub/Chiller Loss <span class="text-text-tertiary">{volumeLabel(units)}</span></label>
            <FloatInput id="eq-trub-loss" step="0.01" decimals={2} value={volVal(trubChillerLossL)} oncommit={(v) => trubChillerLossL = volSave(v)} class="eq-field-input" />
          </div>

          <div class="flex flex-col gap-1">
            <label for="eq-lauter-dead" class="text-xs text-text-secondary">Mash-Tun Deadspace <span class="text-text-tertiary">{volumeLabel(units)}</span></label>
            <FloatInput id="eq-lauter-dead" step="0.01" decimals={2} value={volVal(mashTunDeadspaceL)} oncommit={(v) => mashTunDeadspaceL = volSave(v)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-mash-loss" class="text-xs text-text-secondary">Mash-Tun Loss <span class="text-text-tertiary">{volumeLabel(units)}</span></label>
            <FloatInput id="eq-mash-loss" step="0.01" decimals={2} value={volVal(mashTunLossL)} oncommit={(v) => mashTunLossL = volSave(v)} class="eq-field-input" />
          </div>

          <div class="flex flex-col gap-1">
            <label for="eq-hlt-dead" class="text-xs text-text-secondary">HLT Deadspace <span class="text-text-tertiary">{volumeLabel(units)}</span></label>
            <FloatInput id="eq-hlt-dead" step="0.01" decimals={2} placeholder="optional" value={volValNull(hltDeadspaceL)} oncommit={(v) => hltDeadspaceL = volSaveNull(v)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-ferm-loss" class="text-xs text-text-secondary">Fermenter Loss <span class="text-text-tertiary">{volumeLabel(units)}</span></label>
            <FloatInput id="eq-ferm-loss" step="0.01" decimals={2} value={volVal(fermenterLossL)} oncommit={(v) => fermenterLossL = volSave(v)} class="eq-field-input" />
          </div>
          <div></div>
          <div class="flex flex-col gap-1">
            <label for="eq-hlt-limit-min" class="text-xs text-text-secondary">HLT Water Limit Min <span class="text-text-tertiary">{volumeLabel(units)}</span></label>
            <FloatInput id="eq-hlt-limit-min" step="0.1" decimals={2} placeholder="optional" value={volValNull(hltWaterLimitMinL)} oncommit={(v) => hltWaterLimitMinL = volSaveNull(v)} class="eq-field-input" />
          </div>

          <div class="flex flex-col gap-1">
            <label for="eq-topup" class="text-xs text-text-secondary">Fermenter Top-Up <span class="text-text-tertiary">{volumeLabel(units)}</span></label>
            <FloatInput id="eq-topup" step="0.01" decimals={2} placeholder="optional" value={volValNull(topUpWaterL)} oncommit={(v) => topUpWaterL = volSave(v)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-cooling" class="text-xs text-text-secondary">Cooling Shrinkage <span class="text-text-tertiary">%</span></label>
            <FloatInput id="eq-cooling" step="0.1" decimals={1} value={coolingShrinkagePct} oncommit={(v) => coolingShrinkagePct = v ?? coolingShrinkagePct} class="eq-field-input" />
          </div>
        </div>
        <p class="text-xs mt-2 text-right text-text-tertiary">
          Post-Boil Kettle: {(units === "imperial" ? lToGal(postBoilHotL) : postBoilHotL).toFixed(2)} {volumeLabel(units)} &nbsp;·&nbsp; *Pre-Boil is hot (incl. {coolingShrinkagePct}% expansion)
        </p>
      </section>

      <!-- Efficiency -->
      <section>
        <h3 class="eq-section-label">Efficiency</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-1">
            <label for="eq-efficiency" class="text-xs text-text-secondary">Brewhouse Efficiency <span class="text-text-tertiary">%</span></label>
            <FloatInput id="eq-efficiency" step="0.1" decimals={1} value={efficiencyPct} oncommit={(v) => efficiencyPct = v ?? efficiencyPct} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-mash-eff" class="text-xs text-text-secondary">Mash Efficiency <span class="text-text-tertiary">%</span></label>
            {#if calcMashEfficiency}
              <div id="eq-mash-eff" class="eq-field-display text-text-tertiary">calculated</div>
            {:else}
              <FloatInput id="eq-mash-eff" step="0.1" decimals={1} placeholder="optional" value={mashEfficiencyPct ?? null} oncommit={(v) => mashEfficiencyPct = v} class="eq-field-input" />
            {/if}
          </div>
          <div class="flex items-center gap-2 col-span-2">
            <input type="checkbox" id="calc-mash-eff" bind:checked={calcMashEfficiency} />
            <label for="calc-mash-eff" class="text-sm text-text-primary">Calc mash efficiency</label>
          </div>
        </div>
      </section>

      <!-- Hops -->
      <section>
        <h3 class="eq-section-label">Hops</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-1">
            <label for="eq-hop-util" class="text-xs text-text-secondary">Hop Utilization Multiplier <span class="text-text-tertiary">%</span></label>
            <FloatInput id="eq-hop-util" step="1" decimals={1} value={hopUtilizationPct} oncommit={(v) => hopUtilizationPct = v ?? hopUtilizationPct} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-aroma-util" class="text-xs text-text-secondary">Aroma Hop Utilization <span class="text-text-tertiary">%</span></label>
            {#if calcAromaHopUtilization}
              <div id="eq-aroma-util" class="eq-field-display text-text-tertiary">calculated</div>
            {:else}
              <FloatInput id="eq-aroma-util" step="0.1" decimals={1} value={aromaHopUtilizationPct} oncommit={(v) => aromaHopUtilizationPct = v ?? aromaHopUtilizationPct} class="eq-field-input" />
            {/if}
          </div>
          <div class="flex items-center gap-2">
            <input type="checkbox" id="calc-aroma" bind:checked={calcAromaHopUtilization} />
            <label for="calc-aroma" class="text-sm text-text-primary">Calc aroma hop utilization</label>
          </div>
          {#if calcAromaHopUtilization}
            <div class="flex flex-col gap-1">
              <label for="eq-hopstand-temp" class="text-xs text-text-secondary">Hopstand Temperature <span class="text-text-tertiary">{tempLabel(units)}</span></label>
              <FloatInput id="eq-hopstand-temp" step="1" decimals={1} value={tempVal(hopstandTempF)} oncommit={(v) => { const s = tempSave(v); if (s != null) hopstandTempF = s; }} class="eq-field-input" />
            </div>
          {:else}
            <div></div>
          {/if}
          <div class="flex flex-col gap-1">
            <label for="eq-whirlpool" class="text-xs text-text-secondary">Whirlpool / No-Chill Time <span class="text-text-tertiary">min</span></label>
            <FloatInput id="eq-whirlpool" step="1" decimals={0} placeholder="optional" value={whirlpoolTimeMin ?? null} oncommit={(v) => whirlpoolTimeMin = v} class="eq-field-input" />
          </div>
        </div>
      </section>

      <!-- Boil Temperature -->
      <section>
        <h3 class="eq-section-label">Boil Temperature</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex items-center gap-2 col-span-2">
            <input type="checkbox" id="altitude-adj" bind:checked={altitudeAdjustment} />
            <label for="altitude-adj" class="text-sm text-text-primary">Altitude adjustment</label>
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-boil-temp" class="text-xs text-text-secondary">Boil Temperature <span class="text-text-tertiary">{tempLabel(units)}</span></label>
            {#if altitudeAdjustment}
              <div id="eq-boil-temp" class="eq-field-display text-text-tertiary">calculated from altitude</div>
            {:else}
              <FloatInput id="eq-boil-temp" step="1" decimals={1} placeholder={units === "imperial" ? "212" : "100"} value={tempVal(boilTempF)} oncommit={(v) => boilTempF = tempSave(v)} class="eq-field-input" />
            {/if}
          </div>
        </div>
      </section>

      <!-- Mash / Sparge Water -->
      <section>
        <h3 class="eq-section-label">Mash / Sparge Water</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-1">
            <label for="eq-tun-heat-cap" class="text-xs text-text-secondary">Mash-Tun Heat Capacity <span class="text-text-tertiary">{volumeLabel(units)} equiv.</span></label>
            <FloatInput id="eq-tun-heat-cap" step="0.1" decimals={2} value={volVal(tunHeatCapacityL)} oncommit={(v) => tunHeatCapacityL = volSave(v)} class="eq-field-input" />
          </div>
          <div class="col-span-2 flex flex-col gap-1">
            <label for="eq-sparge-method" class="text-xs text-text-secondary">Sparge Method</label>
            <select id="eq-sparge-method" bind:value={spargeMethod} class="eq-field-input">
              <option value="no_sparge">No Sparge</option>
              <option value="batch_sparge">Batch Sparge</option>
              <option value="fly_sparge">Fly Sparge</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-grain-abs" class="text-xs text-text-secondary">Grain Absorption Rate <span class="text-text-tertiary">{ratioLabel}</span></label>
            <FloatInput id="eq-grain-abs" step="0.01" decimals={2} value={ratioVal(grainAbsorptionRateLPerKg)} oncommit={(v) => grainAbsorptionRateLPerKg = ratioSave(v)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-water-grain" class="text-xs text-text-secondary">Water/Grain Ratio <span class="text-text-tertiary">{ratioLabel}</span></label>
            <FloatInput id="eq-water-grain" step="0.01" decimals={2} value={ratioVal(waterGrainRatioLPerKg)} oncommit={(v) => waterGrainRatioLPerKg = ratioSave(v)} class="eq-field-input" />
          </div>

          <div class="flex items-center gap-2 col-span-2 mt-1">
            <input type="checkbox" id="include-grain-vol" bind:checked={includeGrainVolumeInMashLimits} />
            <label for="include-grain-vol" class="text-sm text-text-primary">Include grain volume in mash limits</label>
          </div>
          <FieldLabel class="col-span-2 mt-1">Mash Volume Limits</FieldLabel>
          <div class="flex flex-col gap-1">
            <label for="eq-mash-vol-min" class="text-xs text-text-secondary">Min <span class="text-text-tertiary">{volumeLabel(units)}</span></label>
            <FloatInput id="eq-mash-vol-min" step="0.1" decimals={2} placeholder="optional" value={volValNull(mashVolumeMinL)} oncommit={(v) => mashVolumeMinL = volSaveNull(v)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-mash-vol-max" class="text-xs text-text-secondary">Max <span class="text-text-tertiary">{volumeLabel(units)}</span></label>
            <FloatInput id="eq-mash-vol-max" step="0.1" decimals={2} placeholder="optional" value={volValNull(mashVolumeMaxL)} oncommit={(v) => mashVolumeMaxL = volSaveNull(v)} class="eq-field-input" />
          </div>

          <FieldLabel class="col-span-2 mt-1">Sparge Volume Limits</FieldLabel>
          <div class="flex flex-col gap-1">
            <label for="eq-sparge-vol-min" class="text-xs text-text-secondary">Min <span class="text-text-tertiary">{volumeLabel(units)}</span></label>
            <FloatInput id="eq-sparge-vol-min" step="0.1" decimals={2} placeholder="optional" value={volValNull(spargeVolumeMinL)} oncommit={(v) => spargeVolumeMinL = volSaveNull(v)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-sparge-vol-max" class="text-xs text-text-secondary">Max <span class="text-text-tertiary">{volumeLabel(units)}</span></label>
            <FloatInput id="eq-sparge-vol-max" step="0.1" decimals={2} placeholder="optional" value={volValNull(spargeVolumeMaxL)} oncommit={(v) => spargeVolumeMaxL = volSaveNull(v)} class="eq-field-input" />
          </div>

          <div class="col-span-2 flex flex-col gap-1 mt-1">
            <label for="eq-overflow-target" class="text-xs text-text-secondary">Overflow Target</label>
            <select id="eq-overflow-target" bind:value={overflowTarget} class="eq-field-input">
              <option value="mash">Mash</option>
              <option value="sparge">Sparge</option>
              <option value="hlt">HLT</option>
            </select>
          </div>
          <div class="flex items-center gap-2 col-span-2 mt-1">
            <input type="checkbox" id="calc-strike" bind:checked={calcStrikeWaterTemp} />
            <label for="calc-strike" class="text-sm text-text-primary">Calc strike water temperature</label>
          </div>
          {#if calcStrikeWaterTemp}
            <div class="flex flex-col gap-1">
              <label for="eq-room-temp" class="text-xs text-text-secondary">Room Temperature <span class="text-text-tertiary">{tempLabel(units)}</span></label>
              <FloatInput id="eq-room-temp" step="1" decimals={1} value={tempVal(roomTempF)} oncommit={(v) => { const s = tempSave(v); if (s != null) roomTempF = s; }} class="eq-field-input" />
            </div>
            <div class="flex flex-col gap-1">
              <label for="eq-grain-temp" class="text-xs text-text-secondary">Grain Temperature <span class="text-text-tertiary">{tempLabel(units)}</span></label>
              <FloatInput id="eq-grain-temp" step="1" decimals={1} value={tempVal(grainTempF)} oncommit={(v) => { const s = tempSave(v); if (s != null) grainTempF = s; }} class="eq-field-input" />
            </div>
            <p class="text-xs col-span-2 text-text-tertiary">
              Set heat capacity to 0 if your mash tun is pre-heated.
            </p>
          {/if}
          <div class="col-span-2 flex flex-col gap-1 mt-1">
            <label for="eq-sparge-temp" class="text-xs text-text-secondary">Sparge Temperature <span class="text-text-tertiary">{tempLabel(units)}</span></label>
            <FloatInput id="eq-sparge-temp" step="1" decimals={1} placeholder="optional" value={tempVal(spargeTempF)} oncommit={(v) => spargeTempF = tempSave(v)} class="eq-field-input" />
          </div>
        </div>
      </section>

    </div>

    <!-- Footer -->
    <div class="flex justify-end gap-3 px-6 py-4 border-t border-border">
      <button onclick={oncancel} class="px-4 py-2 rounded text-sm bg-bg-base text-text-secondary border border-border"
             >
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
