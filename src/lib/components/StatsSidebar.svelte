<script lang="ts">
  import type { RecipeStats } from "$lib/api";

  let { stats }: { stats: RecipeStats | null } = $props();

  function fmt(val: number | undefined, decimals = 3): string {
    if (val === undefined || val === null) return "—";
    return val.toFixed(decimals);
  }

  function srmToHex(srm: number): string {
    const clamp = Math.min(Math.max(srm, 1), 40);
    const stops: [number, string][] = [
      [1, "#FFE699"], [2, "#FFD878"], [3, "#FFCA5A"], [4, "#FFBF42"],
      [6, "#FBB123"], [8, "#F8A600"], [10, "#F39C00"], [13, "#EA8F00"],
      [17, "#D77200"], [20, "#CF6900"], [24, "#BB5100"], [29, "#A13600"],
      [35, "#8D1D00"], [40, "#611200"],
    ];
    for (let i = stops.length - 1; i >= 0; i--) {
      if (clamp >= stops[i][0]) return stops[i][1];
    }
    return "#FFE699";
  }
</script>

<aside class="w-40 flex-shrink-0 flex flex-col border-l p-3 gap-3"
       style="background: var(--color-bg-surface); border-color: var(--color-border);">
  <h3 class="text-xs font-semibold uppercase tracking-wider" style="color: var(--color-text-muted);">Stats</h3>

  {#if stats}
    <div class="flex flex-col gap-2">
      <div class="flex items-center justify-between">
        <span class="text-xs" style="color: var(--color-text-secondary);">OG</span>
        <span class="text-xs font-mono" style="color: var(--color-text-primary);">{fmt(stats.og, 3)}</span>
      </div>
      <div class="flex items-center justify-between">
        <span class="text-xs" style="color: var(--color-text-secondary);">FG</span>
        <span class="text-xs font-mono" style="color: var(--color-text-primary);">{fmt(stats.fg, 3)}</span>
      </div>
      <div class="flex items-center justify-between">
        <span class="text-xs" style="color: var(--color-text-secondary);">ABV</span>
        <span class="text-xs font-mono" style="color: var(--color-text-primary);">{fmt(stats.abv_pct, 1)}%</span>
      </div>
      <div class="flex items-center justify-between">
        <span class="text-xs" style="color: var(--color-text-secondary);">IBU</span>
        <span class="text-xs font-mono" style="color: var(--color-text-primary);">{fmt(stats.ibu, 0)}</span>
      </div>
      <div class="flex items-center justify-between">
        <span class="text-xs" style="color: var(--color-text-secondary);">SRM</span>
        <div class="flex items-center gap-1.5">
          <div class="w-3 h-3 rounded-full border border-white/20"
               style="background: {srmToHex(stats.srm)};"></div>
          <span class="text-xs font-mono" style="color: var(--color-text-primary);">{fmt(stats.srm, 1)}</span>
        </div>
      </div>
      <div class="flex items-center justify-between">
        <span class="text-xs" style="color: var(--color-text-secondary);">BU:GU</span>
        <span class="text-xs font-mono" style="color: var(--color-text-primary);">{fmt(stats.bu_gu_ratio, 2)}</span>
      </div>
      <div class="flex items-center justify-between">
        <span class="text-xs" style="color: var(--color-text-secondary);">Cal</span>
        <span class="text-xs font-mono" style="color: var(--color-text-primary);">{fmt(stats.calories_per_355ml, 0)} kcal</span>
      </div>
    </div>

    <div class="border-t pt-2" style="border-color: var(--color-border);">
      <h4 class="text-xs font-semibold uppercase tracking-wider mb-2" style="color: var(--color-text-muted);">Volumes</h4>
      <div class="flex items-center justify-between">
        <span class="text-xs" style="color: var(--color-text-secondary);">Pre-boil</span>
        <span class="text-xs font-mono" style="color: var(--color-text-primary);">{fmt(stats.pre_boil_volume_l, 1)}L</span>
      </div>
      <div class="flex items-center justify-between">
        <span class="text-xs" style="color: var(--color-text-secondary);">Post-boil</span>
        <span class="text-xs font-mono" style="color: var(--color-text-primary);">{fmt(stats.post_boil_volume_l, 1)}L</span>
      </div>
      <div class="flex items-center justify-between">
        <span class="text-xs" style="color: var(--color-text-secondary);">Pre-boil G</span>
        <span class="text-xs font-mono" style="color: var(--color-text-primary);">{fmt(stats.pre_boil_gravity, 3)}</span>
      </div>
    </div>
  {:else}
    <p class="text-xs" style="color: var(--color-text-muted);">Add ingredients to see stats</p>
  {/if}
</aside>
