<!-- src/lib/components/tabs/FermentationTab.svelte -->
<script lang="ts">
  import type { Recipe, UpdateRecipeInput } from "$lib/api";
  import { updateRecipe } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import Card from "$lib/components/Card.svelte";
  import FieldLabel from "$lib/components/FieldLabel.svelte";
  import TabContent from "$lib/components/tabs/TabContent.svelte";
  import { escRevert } from "$lib/actions/escRevert";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  type FermField = 'primary_age_days' | 'primary_temp_c' | 'secondary_age_days' | 'secondary_temp_c' | 'tertiary_age_days' | 'tertiary_temp_c';

  const FERM_ROWS: Array<{ field: FermField; label: string }> = [
    { field: "primary_age_days", label: "Primary (days)" },
    { field: "primary_temp_c", label: "Primary Temp (°C)" },
    { field: "secondary_age_days", label: "Secondary (days)" },
    { field: "secondary_temp_c", label: "Secondary Temp (°C)" },
    { field: "tertiary_age_days", label: "Tertiary (days)" },
    { field: "tertiary_temp_c", label: "Tertiary Temp (°C)" },
  ];

  async function save<K extends keyof UpdateRecipeInput>(field: K, value: UpdateRecipeInput[K] | null) {
    const result = await ipc(updateRecipe(recipe.id, { [field]: value } as UpdateRecipeInput));
    if (!result) return;
    onchange();
  }
</script>

<TabContent>
  <Card title="Fermentation Schedule">
    <div class="grid grid-cols-2 gap-x-4 gap-y-3">
      {#each FERM_ROWS as row}
        <div class="flex flex-col gap-1">
          <FieldLabel for="ferm-{row.field}">{row.label}</FieldLabel>
          <input id="ferm-{row.field}" type="number" inputmode="decimal" step="1"
                 value={recipe[row.field] ?? ""}
                 use:escRevert
                 onblur={(e) => {
                   const v = (e.target as HTMLInputElement).value;
                   save(row.field, v ? parseFloat(v) : null);
                 }}
                 class="px-2 py-1.5 rounded text-sm bg-bg-elevated text-text-primary border border-border"
                 />
        </div>
      {/each}
    </div>
  </Card>

  <Card title="Carbonation">
    <div class="grid grid-cols-2 gap-x-4 gap-y-3">
      <div class="flex flex-col gap-1">
        <FieldLabel for="ferm-carb-vols">CO₂ Volumes</FieldLabel>
        <input id="ferm-carb-vols" type="number" inputmode="decimal" step="0.1" value={recipe.carbonation_vols ?? ""}
               use:escRevert
               onblur={(e) => {
                 const v = (e.target as HTMLInputElement).value;
                 save("carbonation_vols", v ? parseFloat(v) : null);
               }}
               class="px-2 py-1.5 rounded text-sm bg-bg-elevated text-text-primary border border-border"
               />
      </div>

      <div class="flex flex-col gap-1 justify-end">
        <FieldLabel>Forced Carbonation</FieldLabel>
        <label class="flex items-center gap-2 py-1.5 cursor-pointer">
          <input type="checkbox" checked={recipe.forced_carbonation}
                 onchange={(e) => save("forced_carbonation", (e.target as HTMLInputElement).checked)}
                 class="rounded" />
          <span class="text-sm text-text-secondary">Yes (kegged)</span>
        </label>
      </div>
    </div>
  </Card>
</TabContent>
