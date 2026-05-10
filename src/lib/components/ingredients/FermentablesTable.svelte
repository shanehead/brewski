<script lang="ts">
  import type { Recipe, RecipeAdditionFermentable } from "$lib/api";
  import { createRecipeFermentable, updateRecipeFermentable, deleteRecipeFermentable } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { type Units, kgToLb, lbToKg, weightLabel } from "$lib/units";
  import IngredientPicker, { type AddPayload } from "./IngredientPicker.svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let adding = $state(false);
  const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");

  async function handlePickerAdd(payload: AddPayload) {
    if (payload.type !== "fermentable") return;
    const result = await ipc(createRecipeFermentable(recipe.id, {
      fermentable_id: payload.item.id,
      name: payload.item.name,
      type_: payload.item.type_,
      yield_pct: payload.item.yield_pct,
      color_lovibond: payload.item.color_lovibond,
      amount_kg: payload.amount_kg,
    }));
    if (result === undefined) return;
    adding = false;
    onchange();
  }

  async function handleAmountChange(f: RecipeAdditionFermentable, value: string) {
    const display = parseFloat(value);
    if (!isNaN(display) && display > 0) {
      await ipc(updateRecipeFermentable(f.id, { amount_kg: units === "imperial" ? lbToKg(display) : display }));
      onchange();
    }
  }

  async function handleDelete(id: string) {
    await ipc(deleteRecipeFermentable(id));
    onchange();
  }
</script>

<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Fermentables</h3>
    <button onclick={() => adding = true} class="text-xs px-2 py-1 rounded"
            style="background: var(--color-accent); color: #fff;">+ Add</button>
  </div>

  <IngredientPicker
    type="fermentable"
    open={adding}
    onclose={() => adding = false}
    onadd={handlePickerAdd}
  />

  {#if recipe.fermentables.length > 0}
    <table class="w-full text-sm">
      <thead>
        <tr style="color: var(--color-text-muted);">
          <th class="text-left py-1 font-medium text-xs">Name</th>
          <th class="text-right py-1 font-medium text-xs">Lovibond</th>
          <th class="text-right py-1 font-medium text-xs">{weightLabel(units)}</th>
          <th class="w-6"></th>
        </tr>
      </thead>
      <tbody>
        {#each recipe.fermentables as f (f.id)}
          <tr class="border-t" style="border-color: var(--color-border);">
            <td class="py-1.5" style="color: var(--color-text-primary);">{f.name}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{f.color_lovibond}°L</td>
            <td class="text-right py-1.5">
              <input type="number" step={units === "imperial" ? 0.1 : 0.05}
                     value={(units === "imperial" ? kgToLb(f.amount_kg) : f.amount_kg).toFixed(2)}
                     onblur={(e) => handleAmountChange(f, (e.target as HTMLInputElement).value)}
                     class="w-16 text-right px-1 rounded text-xs"
                     style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid transparent;" />
            </td>
            <td class="pl-1">
              <button onclick={() => handleDelete(f.id)} class="text-xs opacity-40 hover:opacity-100"
                      style="color: var(--color-text-secondary);">×</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
