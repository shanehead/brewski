<!-- src/lib/components/tabs/OverviewTab.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import type { Recipe, Style, EquipmentProfile, UpdateRecipeInput } from "$lib/api";
  import { updateRecipe, listStyles, listEquipmentProfiles } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { type Units, lToGal, galToL, volumeLabel } from "$lib/units";
  import Card from "$lib/components/Card.svelte";
  import FieldLabel from "$lib/components/FieldLabel.svelte";
  import TabContent from "$lib/components/tabs/TabContent.svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let styles = $state<Style[]>([]);
  let equipmentProfiles = $state<EquipmentProfile[]>([]);

  onMount(async () => {
    [styles, equipmentProfiles] = await Promise.all([
      ipc(listStyles()).then(r => r ?? []),
      ipc(listEquipmentProfiles()).then(r => r ?? []),
    ]);
  });

  async function save<K extends keyof UpdateRecipeInput>(field: K, value: UpdateRecipeInput[K] | null) {
    const result = await ipc(updateRecipe(recipe.id, { [field]: value } as UpdateRecipeInput));
    if (!result) return;
    onchange();
  }

  const RECIPE_TYPES = ["all_grain", "extract", "partial_mash"] as const;
  const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");
</script>

<TabContent>
  <Card title="Recipe Details">
    <div class="grid grid-cols-2 gap-x-4 gap-y-3">
      <div class="flex flex-col gap-1">
        <FieldLabel for="overview-type">Recipe Type</FieldLabel>
        <select id="overview-type" value={recipe.type_} onchange={(e) => save("type_", (e.target as HTMLSelectElement).value)}
                class="w-full px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          {#each RECIPE_TYPES as t}
            <option value={t}>{t.replaceAll("_", " ")}</option>
          {/each}
        </select>
      </div>

      <div class="flex flex-col gap-1">
        <FieldLabel for="overview-brewer">Brewer</FieldLabel>
        <input id="overview-brewer" type="text" value={recipe.brewer ?? ""}
               onblur={(e) => save("brewer", (e.target as HTMLInputElement).value)}
               class="w-full px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>

      <div class="flex flex-col gap-1">
        <FieldLabel for="overview-style">Style</FieldLabel>
        <select id="overview-style" value={recipe.style_id ?? ""}
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
        <FieldLabel for="overview-date">Date</FieldLabel>
        <input id="overview-date" type="date" value={recipe.date ?? ""}
               onblur={(e) => save("date", (e.target as HTMLInputElement).value || null)}
               class="w-full px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>

      <div class="flex flex-col gap-1 col-span-2">
        <FieldLabel for="overview-equipment">Equipment Profile</FieldLabel>
        <select id="overview-equipment" value={recipe.equipment_profile_id ?? ""}
                onchange={(e) => save("equipment_profile_id", (e.target as HTMLSelectElement).value || null)}
                class="w-full px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="">None</option>
          {#each equipmentProfiles as ep}
            <option value={ep.id}>{ep.name}</option>
          {/each}
        </select>
      </div>
    </div>
  </Card>

  <Card title="Volumes & Timing">
    <div class="grid grid-cols-2 gap-x-4 gap-y-3">
      <div class="flex flex-col gap-1">
        <FieldLabel for="overview-batch-size">Batch Size ({volumeLabel(units)})</FieldLabel>
        <input id="overview-batch-size" type="number" inputmode="decimal" step="0.1"
               value={(units === "imperial" ? lToGal(recipe.batch_size_l) : recipe.batch_size_l).toFixed(1)}
               onblur={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); save("batch_size_l", units === "imperial" ? galToL(v) : v); }}
               class="w-full px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>

      <div class="flex flex-col gap-1">
        <FieldLabel for="overview-boil-size">Boil Size ({volumeLabel(units)})</FieldLabel>
        <input id="overview-boil-size" type="number" inputmode="decimal" step="0.1"
               value={(units === "imperial" ? lToGal(recipe.boil_size_l) : recipe.boil_size_l).toFixed(1)}
               onblur={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); save("boil_size_l", units === "imperial" ? galToL(v) : v); }}
               class="w-full px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>

      <div class="flex flex-col gap-1">
        <FieldLabel for="overview-boil-time">Boil Time (min)</FieldLabel>
        <input id="overview-boil-time" type="number" inputmode="decimal" step="5" value={recipe.boil_time_min}
               onblur={(e) => save("boil_time_min", parseFloat((e.target as HTMLInputElement).value))}
               class="w-full px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>

      <div class="flex flex-col gap-1">
        <FieldLabel for="overview-efficiency">Efficiency (%)</FieldLabel>
        <input id="overview-efficiency" type="number" inputmode="decimal" step="1" value={recipe.efficiency_pct ?? ""}
               placeholder="From equipment profile"
               onblur={(e) => {
                 const v = (e.target as HTMLInputElement).value;
                 save("efficiency_pct", v ? parseFloat(v) : null);
               }}
               class="w-full px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>
    </div>
  </Card>
</TabContent>
