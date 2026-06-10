<script lang="ts">
  import { untrack } from "svelte";
  import { createEquipmentProfile, updateEquipmentProfile } from "$lib/api";
  import type { EquipmentProfile, CreateEquipmentProfileInput, UpdateEquipmentProfileInput } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { lToGal, galToL, volumeLabel, kgToLb, lbToKg, weightLabel, cToF, fToC, tempLabel, type Units } from "$lib/units";
  import FieldLabel from "$lib/components/FieldLabel.svelte";

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

  // ── unit conversion helpers ──────────────────────────────────────────────
  function volDisp(l: number): string { return (units === "imperial" ? lToGal(l) : l).toFixed(2); }
  function volDispNull(l: number | null): string { return l != null ? volDisp(l) : ""; }
  function volIn(e: Event): number {
    const v = parseFloat((e.target as HTMLInputElement).value) || 0;
    return units === "imperial" ? galToL(v) : v;
  }
  function volInNull(e: Event): number | null {
    const v = parseFloat((e.target as HTMLInputElement).value);
    return isNaN(v) ? null : (units === "imperial" ? galToL(v) : v);
  }
  function weightDispNull(kg: number | null): string {
    return kg != null ? (units === "imperial" ? kgToLb(kg) : kg).toFixed(2) : "";
  }
  function weightInNull(e: Event): number | null {
    const v = parseFloat((e.target as HTMLInputElement).value);
    return isNaN(v) ? null : (units === "imperial" ? lbToKg(v) : v);
  }
  function tempDispNull(f: number | null): string {
    return f != null ? (units === "imperial" ? f : fToC(f)).toFixed(1) : "";
  }
  function tempInNull(e: Event): number | null {
    const v = parseFloat((e.target as HTMLInputElement).value);
    return isNaN(v) ? null : (units === "imperial" ? v : cToF(v));
  }
  function ratioDisp(lPerKg: number): string {
    return (units === "imperial" ? lPerKg * 0.4796 : lPerKg).toFixed(2);
  }
  function ratioIn(e: Event): number {
    const v = parseFloat((e.target as HTMLInputElement).value) || 0;
    return units === "imperial" ? v / 0.4796 : v;
  }

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
     tabindex="-1"
     onclick={(e) => e.target === e.currentTarget && oncancel()}
     onkeydown={(e) => e.key === "Escape" && oncancel()}>

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
          <label for="eq-name" class="text-xs" style="color: var(--color-text-secondary);">Name</label>
          <input id="eq-name" type="text" bind:value={name} class="eq-field-input" />
        </div>
        <div class="flex flex-col gap-1">
          <label for="eq-boil-time" class="text-xs" style="color: var(--color-text-secondary);">Boil Time <span style="color: var(--color-text-tertiary);">min</span></label>
          <input id="eq-boil-time" type="number" inputmode="decimal" value={boilTimeMin} oninput={(e) => boilTimeMin = numInput(e)} class="eq-field-input" />
        </div>
        <div class="col-span-2 flex flex-col gap-1">
          <label for="eq-notes" class="text-xs" style="color: var(--color-text-secondary);">Description</label>
          <input id="eq-notes" type="text" bind:value={notes} class="eq-field-input" />
        </div>
      </div>

      <!-- Volumes -->
      <section>
        <h3 class="eq-section-label">Volumes</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-1">
            <label for="eq-batch-target" class="text-xs" style="color: var(--color-text-secondary);">Batch Volume Target</label>
            <select id="eq-batch-target" bind:value={batchVolumeTarget} class="eq-field-input">
              <option value="fermenter">Fermenter</option>
              <option value="kettle">Kettle</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-batch-size" class="text-xs" style="color: var(--color-text-secondary);">{batchLabel} <span style="color: var(--color-text-tertiary);">{volumeLabel(units)}</span></label>
            <input id="eq-batch-size" type="number" inputmode="decimal" step="0.1" value={volDisp(batchSizeL)} oninput={(e) => batchSizeL = volIn(e)} class="eq-field-input" />
          </div>

          <div class="flex items-center gap-2">
            <input type="checkbox" id="calc-boil" bind:checked={calcBoilVolume} />
            <label for="calc-boil" class="text-sm" style="color: var(--color-text-primary);">Calc boil volume</label>
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-boil-size" class="text-xs" style="color: var(--color-text-secondary);">Pre-Boil Volume* <span style="color: var(--color-text-tertiary);">{volumeLabel(units)}</span></label>
            {#if calcBoilVolume}
              <div id="eq-boil-size" class="eq-field-display">{(units === "imperial" ? lToGal(preBoilHotL) : preBoilHotL).toFixed(2)} <span style="color: var(--color-text-tertiary);">(hot)</span></div>
            {:else}
              <input id="eq-boil-size" type="number" inputmode="decimal" step="0.1" value={volDisp(boilSizeL)} oninput={(e) => boilSizeL = volIn(e)} class="eq-field-input" />
            {/if}
          </div>

          <div class="flex flex-col gap-1">
            <label for="eq-evap-rate" class="text-xs" style="color: var(--color-text-secondary);">Boil Off <span style="color: var(--color-text-tertiary);">({calcEvapPct.toFixed(1)}%) {volumeLabel(units)}/hr</span></label>
            <input id="eq-evap-rate" type="number" inputmode="decimal" step="0.01" value={volDisp(evapRateLHr)} oninput={(e) => evapRateLHr = volIn(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-trub-loss" class="text-xs" style="color: var(--color-text-secondary);">Trub/Chiller Loss <span style="color: var(--color-text-tertiary);">{volumeLabel(units)}</span></label>
            <input id="eq-trub-loss" type="number" inputmode="decimal" step="0.01" value={volDisp(trubChillerLossL)} oninput={(e) => trubChillerLossL = volIn(e)} class="eq-field-input" />
          </div>

          <div class="flex flex-col gap-1">
            <label for="eq-lauter-dead" class="text-xs" style="color: var(--color-text-secondary);">Mash-Tun Deadspace <span style="color: var(--color-text-tertiary);">{volumeLabel(units)}</span></label>
            <input id="eq-lauter-dead" type="number" inputmode="decimal" step="0.01" value={volDisp(mashTunDeadspaceL)} oninput={(e) => mashTunDeadspaceL = volIn(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-mash-loss" class="text-xs" style="color: var(--color-text-secondary);">Mash-Tun Loss <span style="color: var(--color-text-tertiary);">{volumeLabel(units)}</span></label>
            <input id="eq-mash-loss" type="number" inputmode="decimal" step="0.01" value={volDisp(mashTunLossL)} oninput={(e) => mashTunLossL = volIn(e)} class="eq-field-input" />
          </div>

          <div class="flex flex-col gap-1">
            <label for="eq-hlt-dead" class="text-xs" style="color: var(--color-text-secondary);">HLT Deadspace <span style="color: var(--color-text-tertiary);">{volumeLabel(units)}</span></label>
            <input id="eq-hlt-dead" type="number" inputmode="decimal" step="0.01" placeholder="optional"
                   value={volDispNull(hltDeadspaceL)} oninput={(e) => hltDeadspaceL = volInNull(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-ferm-loss" class="text-xs" style="color: var(--color-text-secondary);">Fermenter Loss <span style="color: var(--color-text-tertiary);">{volumeLabel(units)}</span></label>
            <input id="eq-ferm-loss" type="number" inputmode="decimal" step="0.01" value={volDisp(fermenterLossL)} oninput={(e) => fermenterLossL = volIn(e)} class="eq-field-input" />
          </div>
          <div></div>
          <div class="flex flex-col gap-1">
            <label for="eq-hlt-limit-min" class="text-xs" style="color: var(--color-text-secondary);">HLT Water Limit Min <span style="color: var(--color-text-tertiary);">{volumeLabel(units)}</span></label>
            <input id="eq-hlt-limit-min" type="number" inputmode="decimal" step="0.1" placeholder="optional"
                   value={volDispNull(hltWaterLimitMinL)} oninput={(e) => hltWaterLimitMinL = volInNull(e)} class="eq-field-input" />
          </div>

          <div class="flex flex-col gap-1">
            <label for="eq-topup" class="text-xs" style="color: var(--color-text-secondary);">Fermenter Top-Up <span style="color: var(--color-text-tertiary);">{volumeLabel(units)}</span></label>
            <input id="eq-topup" type="number" inputmode="decimal" step="0.01" placeholder="optional"
                   value={topUpWaterL ? volDisp(topUpWaterL) : ""} oninput={(e) => topUpWaterL = volIn(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-cooling" class="text-xs" style="color: var(--color-text-secondary);">Cooling Shrinkage <span style="color: var(--color-text-tertiary);">%</span></label>
            <input id="eq-cooling" type="number" inputmode="decimal" step="0.1" value={coolingShrinkagePct} oninput={(e) => coolingShrinkagePct = numInput(e)} class="eq-field-input" />
          </div>
        </div>
        <p class="text-xs mt-2 text-right" style="color: var(--color-text-tertiary);">
          Post-Boil Kettle: {(units === "imperial" ? lToGal(postBoilHotL) : postBoilHotL).toFixed(2)} {volumeLabel(units)} &nbsp;·&nbsp; *Pre-Boil is hot (incl. {coolingShrinkagePct}% expansion)
        </p>
      </section>

      <!-- Efficiency -->
      <section>
        <h3 class="eq-section-label">Efficiency</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-1">
            <label for="eq-efficiency" class="text-xs" style="color: var(--color-text-secondary);">Brewhouse Efficiency <span style="color: var(--color-text-tertiary);">%</span></label>
            <input id="eq-efficiency" type="number" inputmode="decimal" step="0.1" value={efficiencyPct} oninput={(e) => efficiencyPct = numInput(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-mash-eff" class="text-xs" style="color: var(--color-text-secondary);">Mash Efficiency <span style="color: var(--color-text-tertiary);">%</span></label>
            {#if calcMashEfficiency}
              <div id="eq-mash-eff" class="eq-field-display" style="color: var(--color-text-tertiary);">calculated</div>
            {:else}
              <input id="eq-mash-eff" type="number" inputmode="decimal" step="0.1" placeholder="optional"
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
            <label for="eq-hop-util" class="text-xs" style="color: var(--color-text-secondary);">Hop Utilization Multiplier <span style="color: var(--color-text-tertiary);">%</span></label>
            <input id="eq-hop-util" type="number" inputmode="decimal" step="1" value={hopUtilizationPct} oninput={(e) => hopUtilizationPct = numInput(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-aroma-util" class="text-xs" style="color: var(--color-text-secondary);">Aroma Hop Utilization <span style="color: var(--color-text-tertiary);">%</span></label>
            {#if calcAromaHopUtilization}
              <div id="eq-aroma-util" class="eq-field-display" style="color: var(--color-text-tertiary);">calculated</div>
            {:else}
              <input id="eq-aroma-util" type="number" inputmode="decimal" step="0.1" value={aromaHopUtilizationPct} oninput={(e) => aromaHopUtilizationPct = numInput(e)} class="eq-field-input" />
            {/if}
          </div>
          <div class="flex items-center gap-2">
            <input type="checkbox" id="calc-aroma" bind:checked={calcAromaHopUtilization} />
            <label for="calc-aroma" class="text-sm" style="color: var(--color-text-primary);">Calc aroma hop utilization</label>
          </div>
          {#if calcAromaHopUtilization}
            <div class="flex flex-col gap-1">
              <label for="eq-hopstand-temp" class="text-xs" style="color: var(--color-text-secondary);">Hopstand Temperature <span style="color: var(--color-text-tertiary);">{tempLabel(units)}</span></label>
              <input id="eq-hopstand-temp" type="number" inputmode="decimal" step="1"
                     value={tempDispNull(hopstandTempF)} oninput={(e) => hopstandTempF = tempInNull(e) ?? hopstandTempF} class="eq-field-input" />
            </div>
          {:else}
            <div></div>
          {/if}
          <div class="flex flex-col gap-1">
            <label for="eq-whirlpool" class="text-xs" style="color: var(--color-text-secondary);">Whirlpool / No-Chill Time <span style="color: var(--color-text-tertiary);">min</span></label>
            <input id="eq-whirlpool" type="number" inputmode="decimal" step="1" placeholder="optional"
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
            <label for="eq-boil-temp" class="text-xs" style="color: var(--color-text-secondary);">Boil Temperature <span style="color: var(--color-text-tertiary);">{tempLabel(units)}</span></label>
            {#if altitudeAdjustment}
              <div id="eq-boil-temp" class="eq-field-display" style="color: var(--color-text-tertiary);">calculated from altitude</div>
            {:else}
              <input id="eq-boil-temp" type="number" inputmode="decimal" step="1"
                     placeholder={units === "imperial" ? "212" : "100"}
                     value={tempDispNull(boilTempF)} oninput={(e) => boilTempF = tempInNull(e)} class="eq-field-input" />
            {/if}
          </div>
        </div>
      </section>

      <!-- Mash / Sparge Water -->
      <section>
        <h3 class="eq-section-label">Mash / Sparge Water</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-1">
            <label for="eq-tun-heat-cap" class="text-xs" style="color: var(--color-text-secondary);">Mash-Tun Heat Capacity <span style="color: var(--color-text-tertiary);">{volumeLabel(units)} equiv.</span></label>
            <input id="eq-tun-heat-cap" type="number" inputmode="decimal" step="0.1"
                   value={volDisp(tunHeatCapacityL)} oninput={(e) => tunHeatCapacityL = volIn(e)} class="eq-field-input" />
          </div>
          <div class="col-span-2 flex flex-col gap-1">
            <label for="eq-sparge-method" class="text-xs" style="color: var(--color-text-secondary);">Sparge Method</label>
            <select id="eq-sparge-method" bind:value={spargeMethod} class="eq-field-input">
              <option value="no_sparge">No Sparge</option>
              <option value="batch_sparge">Batch Sparge</option>
              <option value="fly_sparge">Fly Sparge</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-grain-abs" class="text-xs" style="color: var(--color-text-secondary);">Grain Absorption Rate <span style="color: var(--color-text-tertiary);">{ratioLabel}</span></label>
            <input id="eq-grain-abs" type="number" inputmode="decimal" step="0.01"
                   value={ratioDisp(grainAbsorptionRateLPerKg)} oninput={(e) => grainAbsorptionRateLPerKg = ratioIn(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-water-grain" class="text-xs" style="color: var(--color-text-secondary);">Water/Grain Ratio <span style="color: var(--color-text-tertiary);">{ratioLabel}</span></label>
            <input id="eq-water-grain" type="number" inputmode="decimal" step="0.01"
                   value={ratioDisp(waterGrainRatioLPerKg)} oninput={(e) => waterGrainRatioLPerKg = ratioIn(e)} class="eq-field-input" />
          </div>

          <div class="flex items-center gap-2 col-span-2 mt-1">
            <input type="checkbox" id="include-grain-vol" bind:checked={includeGrainVolumeInMashLimits} />
            <label for="include-grain-vol" class="text-sm" style="color: var(--color-text-primary);">Include grain volume in mash limits</label>
          </div>
          <FieldLabel class="col-span-2 mt-1">Mash Volume Limits</FieldLabel>
          <div class="flex flex-col gap-1">
            <label for="eq-mash-vol-min" class="text-xs" style="color: var(--color-text-secondary);">Min <span style="color: var(--color-text-tertiary);">{volumeLabel(units)}</span></label>
            <input id="eq-mash-vol-min" type="number" inputmode="decimal" step="0.1" placeholder="optional"
                   value={volDispNull(mashVolumeMinL)} oninput={(e) => mashVolumeMinL = volInNull(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-mash-vol-max" class="text-xs" style="color: var(--color-text-secondary);">Max <span style="color: var(--color-text-tertiary);">{volumeLabel(units)}</span></label>
            <input id="eq-mash-vol-max" type="number" inputmode="decimal" step="0.1" placeholder="optional"
                   value={volDispNull(mashVolumeMaxL)} oninput={(e) => mashVolumeMaxL = volInNull(e)} class="eq-field-input" />
          </div>

          <FieldLabel class="col-span-2 mt-1">Sparge Volume Limits</FieldLabel>
          <div class="flex flex-col gap-1">
            <label for="eq-sparge-vol-min" class="text-xs" style="color: var(--color-text-secondary);">Min <span style="color: var(--color-text-tertiary);">{volumeLabel(units)}</span></label>
            <input id="eq-sparge-vol-min" type="number" inputmode="decimal" step="0.1" placeholder="optional"
                   value={volDispNull(spargeVolumeMinL)} oninput={(e) => spargeVolumeMinL = volInNull(e)} class="eq-field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label for="eq-sparge-vol-max" class="text-xs" style="color: var(--color-text-secondary);">Max <span style="color: var(--color-text-tertiary);">{volumeLabel(units)}</span></label>
            <input id="eq-sparge-vol-max" type="number" inputmode="decimal" step="0.1" placeholder="optional"
                   value={volDispNull(spargeVolumeMaxL)} oninput={(e) => spargeVolumeMaxL = volInNull(e)} class="eq-field-input" />
          </div>

          <div class="col-span-2 flex flex-col gap-1 mt-1">
            <label for="eq-overflow-target" class="text-xs" style="color: var(--color-text-secondary);">Overflow Target</label>
            <select id="eq-overflow-target" bind:value={overflowTarget} class="eq-field-input">
              <option value="mash">Mash</option>
              <option value="sparge">Sparge</option>
              <option value="hlt">HLT</option>
            </select>
          </div>
          <div class="flex items-center gap-2 col-span-2 mt-1">
            <input type="checkbox" id="calc-strike" bind:checked={calcStrikeWaterTemp} />
            <label for="calc-strike" class="text-sm" style="color: var(--color-text-primary);">Calc strike water temperature</label>
          </div>
          {#if calcStrikeWaterTemp}
            <div class="flex flex-col gap-1">
              <label for="eq-room-temp" class="text-xs" style="color: var(--color-text-secondary);">Room Temperature <span style="color: var(--color-text-tertiary);">{tempLabel(units)}</span></label>
              <input id="eq-room-temp" type="number" inputmode="decimal" step="1"
                     value={tempDispNull(roomTempF)} oninput={(e) => roomTempF = tempInNull(e) ?? roomTempF} class="eq-field-input" />
            </div>
            <div class="flex flex-col gap-1">
              <label for="eq-grain-temp" class="text-xs" style="color: var(--color-text-secondary);">Grain Temperature <span style="color: var(--color-text-tertiary);">{tempLabel(units)}</span></label>
              <input id="eq-grain-temp" type="number" inputmode="decimal" step="1"
                     value={tempDispNull(grainTempF)} oninput={(e) => grainTempF = tempInNull(e) ?? grainTempF} class="eq-field-input" />
            </div>
            <p class="text-xs col-span-2" style="color: var(--color-text-tertiary);">
              Set heat capacity to 0 if your mash tun is pre-heated.
            </p>
          {/if}
          <div class="col-span-2 flex flex-col gap-1 mt-1">
            <label for="eq-sparge-temp" class="text-xs" style="color: var(--color-text-secondary);">Sparge Temperature <span style="color: var(--color-text-tertiary);">{tempLabel(units)}</span></label>
            <input id="eq-sparge-temp" type="number" inputmode="decimal" step="1" placeholder="optional"
                   value={tempDispNull(spargeTempF)} oninput={(e) => spargeTempF = tempInNull(e)} class="eq-field-input" />
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
