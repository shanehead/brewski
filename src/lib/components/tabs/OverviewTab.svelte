<script lang="ts">
  import { onMount } from "svelte";
  import type { Recipe, Style, EquipmentProfile } from "$lib/api";
  import { updateRecipe, listStyles, listEquipmentProfiles } from "$lib/api";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let styles = $state<Style[]>([]);
  let equipmentProfiles = $state<EquipmentProfile[]>([]);

  onMount(async () => {
    [styles, equipmentProfiles] = await Promise.all([listStyles(), listEquipmentProfiles()]);
  });

  async function save(field: string, value: unknown) {
    await updateRecipe(recipe.id, { [field]: value } as any);
    onchange();
  }

  const RECIPE_TYPES = ["all_grain", "extract", "partial_mash"] as const;
</script>

<div class="grid grid-cols-2 gap-4 max-w-2xl">
  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Recipe Type</label>
    <select value={recipe.type_} onchange={(e) => save("type_", (e.target as HTMLSelectElement).value)}
            class="w-full px-2 py-1.5 rounded text-sm"
            style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
      {#each RECIPE_TYPES as t}
        <option value={t}>{t.replace("_", " ")}</option>
      {/each}
    </select>
  </div>

  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Brewer</label>
    <input type="text" value={recipe.brewer ?? ""}
           onblur={(e) => save("brewer", (e.target as HTMLInputElement).value)}
           class="w-full px-2 py-1.5 rounded text-sm"
           style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
  </div>

  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Batch Size (L)</label>
    <input type="number" step="0.1" value={recipe.batch_size_l}
           onblur={(e) => save("batch_size_l", parseFloat((e.target as HTMLInputElement).value))}
           class="w-full px-2 py-1.5 rounded text-sm"
           style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
  </div>

  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Boil Size (L)</label>
    <input type="number" step="0.1" value={recipe.boil_size_l}
           onblur={(e) => save("boil_size_l", parseFloat((e.target as HTMLInputElement).value))}
           class="w-full px-2 py-1.5 rounded text-sm"
           style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
  </div>

  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Boil Time (min)</label>
    <input type="number" step="5" value={recipe.boil_time_min}
           onblur={(e) => save("boil_time_min", parseFloat((e.target as HTMLInputElement).value))}
           class="w-full px-2 py-1.5 rounded text-sm"
           style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
  </div>

  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Efficiency (%)</label>
    <input type="number" step="1" value={recipe.efficiency_pct ?? ""}
           placeholder="From equipment profile"
           onblur={(e) => {
             const v = (e.target as HTMLInputElement).value;
             save("efficiency_pct", v ? parseFloat(v) : null);
           }}
           class="w-full px-2 py-1.5 rounded text-sm"
           style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
  </div>

  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Equipment Profile</label>
    <select value={recipe.equipment_profile_id ?? ""}
            onchange={(e) => save("equipment_profile_id", (e.target as HTMLSelectElement).value || null)}
            class="w-full px-2 py-1.5 rounded text-sm"
            style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
      <option value="">None</option>
      {#each equipmentProfiles as ep}
        <option value={ep.id}>{ep.name}</option>
      {/each}
    </select>
  </div>

  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Style</label>
    <select value={recipe.style_id ?? ""}
            onchange={(e) => save("style_id", (e.target as HTMLSelectElement).value || null)}
            class="w-full px-2 py-1.5 rounded text-sm"
            style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
      <option value="">None</option>
      {#each styles as s}
        <option value={s.id}>{s.name}</option>
      {/each}
    </select>
  </div>

  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Date</label>
    <input type="date" value={recipe.date ?? ""}
           onblur={(e) => save("date", (e.target as HTMLInputElement).value || null)}
           class="w-full px-2 py-1.5 rounded text-sm"
           style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
  </div>
</div>
