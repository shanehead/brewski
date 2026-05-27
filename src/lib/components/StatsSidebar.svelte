<!-- src/lib/components/StatsSidebar.svelte -->
<script lang="ts">
  import type { RecipeStats } from "$lib/api";
  import { convertGravity } from "$lib/api";
  import { settings } from "$lib/stores/settings";
  import { type Units, lToGal, volumeLabel } from "$lib/units";
  import { srmToHex } from "$lib/utils/srm";
  import { formatGravity } from "$lib/gravity-display";
  import { ipc } from "$lib/stores/error";

  let { stats }: { stats: RecipeStats | null } = $props();

  const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");
  const gravityUnit = $derived($settings.gravity_unit ?? "sg");

  let displayOg = $state("—");
  let displayFg = $state("—");
  let displayPreBoil = $state("—");

  $effect(() => {
    const unit = gravityUnit;
    if (stats?.og != null) {
      ipc(convertGravity(stats.og, "sg")).then(r => { if (r) displayOg = formatGravity(r, unit); });
    } else {
      displayOg = "—";
    }
  });

  $effect(() => {
    const unit = gravityUnit;
    if (stats?.fg != null) {
      ipc(convertGravity(stats.fg, "sg")).then(r => { if (r) displayFg = formatGravity(r, unit); });
    } else {
      displayFg = "—";
    }
  });

  $effect(() => {
    const unit = gravityUnit;
    if (stats?.pre_boil_gravity != null) {
      ipc(convertGravity(stats.pre_boil_gravity, "sg")).then(r => {
        if (r) displayPreBoil = formatGravity(r, unit);
      });
    } else {
      displayPreBoil = "—";
    }
  });

  function fmt(val: number | undefined | null, decimals = 3): string {
    if (val === undefined || val === null) return "—";
    return val.toFixed(decimals);
  }

  function pct(value: number, min: number, max: number): number {
    return Math.min(100, Math.max(0, ((value - min) / (max - min)) * 100));
  }
</script>

<aside class="w-44 flex-shrink-0 flex flex-col border-l overflow-y-auto p-3 gap-1.5"
       style="background: var(--color-bg-surface); border-color: var(--color-border);">
  <p class="text-xs font-semibold uppercase tracking-wider mb-1"
     style="color: var(--color-text-muted);">Stats</p>

  {#if stats}
    <!-- OG -->
    <div class="rounded-lg p-2.5" style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
      <p class="text-xs mb-0.5" style="color: var(--color-text-muted);">OG</p>
      <p class="text-xl font-bold font-mono leading-none" style="color: var(--color-text-primary);">{displayOg}</p>
      <div class="mt-1.5 h-1 rounded-full" style="background: var(--color-border);">
        <div class="h-full rounded-full" style="width: {pct(stats.og, 1.000, 1.120)}%; background: var(--color-accent);"></div>
      </div>
    </div>

    <!-- FG -->
    <div class="rounded-lg p-2.5" style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
      <p class="text-xs mb-0.5" style="color: var(--color-text-muted);">FG</p>
      <p class="text-xl font-bold font-mono leading-none" style="color: var(--color-text-primary);">{displayFg}</p>
      <div class="mt-1.5 h-1 rounded-full" style="background: var(--color-border);">
        <div class="h-full rounded-full" style="width: {pct(stats.fg, 1.000, 1.030)}%; background: var(--color-accent);"></div>
      </div>
    </div>

    <!-- ABV -->
    <div class="rounded-lg p-2.5" style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
      <p class="text-xs mb-0.5" style="color: var(--color-text-muted);">ABV</p>
      <p class="text-xl font-bold font-mono leading-none" style="color: var(--color-text-primary);">
        {fmt(stats.abv_pct, 1)}<span class="text-sm font-normal" style="color: var(--color-text-muted);">%</span>
      </p>
      <div class="mt-1.5 h-1 rounded-full" style="background: var(--color-border);">
        <div class="h-full rounded-full" style="width: {pct(stats.abv_pct, 0, 12)}%; background: #a6e3a1;"></div>
      </div>
    </div>

    <!-- IBU -->
    <div class="rounded-lg p-2.5" style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
      <p class="text-xs mb-0.5" style="color: var(--color-text-muted);">IBU</p>
      <p class="text-xl font-bold font-mono leading-none" style="color: var(--color-text-primary);">{fmt(stats.ibu, 0)}</p>
      <div class="mt-1.5 h-1 rounded-full" style="background: var(--color-border);">
        <div class="h-full rounded-full" style="width: {pct(stats.ibu, 0, 120)}%; background: #fab387;"></div>
      </div>
    </div>

    <!-- SRM -->
    <div class="rounded-lg p-2.5" style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
      <p class="text-xs mb-0.5" style="color: var(--color-text-muted);">SRM</p>
      <div class="flex items-center gap-2">
        <div class="w-5 h-5 rounded flex-shrink-0"
             style="background: {srmToHex(stats.srm)}; border: 1px solid rgba(255,255,255,0.15);"></div>
        <p class="text-xl font-bold font-mono leading-none" style="color: var(--color-text-primary);">{fmt(stats.srm, 1)}</p>
      </div>
    </div>

    <!-- BU:GU -->
    <div class="rounded-lg p-2.5" style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
      <p class="text-xs mb-0.5" style="color: var(--color-text-muted);">BU:GU</p>
      <p class="text-xl font-bold font-mono leading-none" style="color: var(--color-text-primary);">{fmt(stats.bu_gu_ratio, 2)}</p>
    </div>

    <!-- Calories -->
    <div class="rounded-lg p-2.5" style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
      <p class="text-xs mb-0.5" style="color: var(--color-text-muted);">Cal / 12oz</p>
      <p class="text-xl font-bold font-mono leading-none" style="color: var(--color-text-primary);">{fmt(stats.calories_per_355ml, 0)}</p>
    </div>

    <!-- Volumes divider -->
    <p class="text-xs font-semibold uppercase tracking-wider mt-1 mb-0.5"
       style="color: var(--color-text-muted);">Volumes</p>

    <!-- Pre-boil -->
    <div class="rounded-lg p-2.5" style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
      <p class="text-xs mb-0.5" style="color: var(--color-text-muted);">Pre-boil</p>
      <p class="text-base font-bold font-mono leading-none" style="color: var(--color-text-primary);">
        {fmt(units === "imperial" ? lToGal(stats.pre_boil_volume_l) : stats.pre_boil_volume_l, 1)}<span class="text-xs font-normal ml-0.5" style="color: var(--color-text-muted);">{volumeLabel(units)}</span>
      </p>
    </div>

    <!-- Post-boil -->
    <div class="rounded-lg p-2.5" style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
      <p class="text-xs mb-0.5" style="color: var(--color-text-muted);">Post-boil</p>
      <p class="text-base font-bold font-mono leading-none" style="color: var(--color-text-primary);">
        {fmt(units === "imperial" ? lToGal(stats.post_boil_volume_l) : stats.post_boil_volume_l, 1)}<span class="text-xs font-normal ml-0.5" style="color: var(--color-text-muted);">{volumeLabel(units)}</span>
      </p>
    </div>

    <!-- Pre-boil gravity -->
    <div class="rounded-lg p-2.5" style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
      <p class="text-xs mb-0.5" style="color: var(--color-text-muted);">Pre-boil G</p>
      <p class="text-base font-bold font-mono leading-none" style="color: var(--color-text-primary);">{displayPreBoil}</p>
    </div>

  {:else}
    <p class="text-xs" style="color: var(--color-text-muted);">Add ingredients to see stats</p>
  {/if}
</aside>
