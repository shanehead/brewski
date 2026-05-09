<script lang="ts">
  import type { Recipe } from "$lib/api";
  import { updateRecipe } from "$lib/api";
  import { ipc } from "$lib/stores/error";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  async function save(field: string, value: unknown) {
    await ipc(updateRecipe(recipe.id, { [field]: value } as any));
    onchange();
  }
</script>

<div class="grid grid-cols-2 gap-4 max-w-xl">
  <div class="col-span-2">
    <h3 class="text-sm font-semibold mb-3" style="color: var(--color-text-primary);">Fermentation Schedule</h3>
  </div>

  {#each [
    { field: "primary_age_days", label: "Primary (days)" },
    { field: "primary_temp_c", label: "Primary Temp (°C)" },
    { field: "secondary_age_days", label: "Secondary (days)" },
    { field: "secondary_temp_c", label: "Secondary Temp (°C)" },
    { field: "tertiary_age_days", label: "Tertiary (days)" },
    { field: "tertiary_temp_c", label: "Tertiary Temp (°C)" },
  ] as row}
    <div class="flex flex-col gap-1">
      <label for="ferm-{row.field}" class="text-xs font-medium" style="color: var(--color-text-secondary);">{row.label}</label>
      <input id="ferm-{row.field}" type="number" step="1"
             value={(recipe as any)[row.field] ?? ""}
             onblur={(e) => {
               const v = (e.target as HTMLInputElement).value;
               save(row.field, v ? parseFloat(v) : null);
             }}
             class="px-2 py-1.5 rounded text-sm"
             style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
    </div>
  {/each}

  <div class="col-span-2 border-t pt-4" style="border-color: var(--color-border);">
    <h3 class="text-sm font-semibold mb-3" style="color: var(--color-text-primary);">Carbonation</h3>
  </div>

  <div class="flex flex-col gap-1">
    <label for="ferm-carb-vols" class="text-xs font-medium" style="color: var(--color-text-secondary);">CO₂ Volumes</label>
    <input id="ferm-carb-vols" type="number" step="0.1" value={recipe.carbonation_vols ?? ""}
           onblur={(e) => {
             const v = (e.target as HTMLInputElement).value;
             save("carbonation_vols", v ? parseFloat(v) : null);
           }}
           class="px-2 py-1.5 rounded text-sm"
           style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
  </div>

  <div class="flex flex-col gap-1">
    <p class="text-xs font-medium" style="color: var(--color-text-secondary);">Forced Carbonation</p>
    <label class="flex items-center gap-2 mt-1 cursor-pointer">
      <input type="checkbox" checked={recipe.forced_carbonation}
             onchange={(e) => save("forced_carbonation", (e.target as HTMLInputElement).checked)}
             class="rounded" />
      <span class="text-sm" style="color: var(--color-text-secondary);">Yes (kegged)</span>
    </label>
  </div>
</div>
