<script lang="ts">
  import type { Recipe } from "$lib/api";
  import { createRecipeHop, deleteRecipeHop } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { type Units, kgToHopDisplay, hopWeightLabel } from "$lib/units";
  import IngredientPicker, { type AddPayload } from "./IngredientPicker.svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let adding = $state(false);
  const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");

  async function handlePickerAdd(payload: AddPayload) {
    if (payload.type !== "hop") return;
    const result = await ipc(createRecipeHop(recipe.id, {
      hop_id: payload.item.id,
      name: payload.item.name,
      alpha_pct: payload.item.alpha_pct,
      form: payload.item.form,
      amount_kg: payload.amount_kg,
      use_: payload.use_,
      time_min: payload.time_min,
    }));
    if (result === undefined) return;
    adding = false;
    onchange();
  }

  async function handleDelete(id: string) {
    await ipc(deleteRecipeHop(id));
    onchange();
  }
</script>

<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Hops</h3>
    <button onclick={() => adding = true} class="text-xs px-2 py-1 rounded"
            style="background: var(--color-accent); color: #fff;">+ Add</button>
  </div>

  <IngredientPicker
    type="hop"
    open={adding}
    onclose={() => adding = false}
    onadd={handlePickerAdd}
  />

  {#if recipe.hops.length > 0}
    <table class="w-full text-sm">
      <thead>
        <tr style="color: var(--color-text-muted);">
          <th class="text-left py-1 font-medium text-xs">Name</th>
          <th class="text-right py-1 font-medium text-xs">AA%</th>
          <th class="text-right py-1 font-medium text-xs">{hopWeightLabel(units)}</th>
          <th class="text-right py-1 font-medium text-xs">Use</th>
          <th class="text-right py-1 font-medium text-xs">Time</th>
          <th class="w-6"></th>
        </tr>
      </thead>
      <tbody>
        {#each recipe.hops as h (h.id)}
          <tr class="border-t" style="border-color: var(--color-border);">
            <td class="py-1.5" style="color: var(--color-text-primary);">{h.name}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{h.alpha_pct}%</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{kgToHopDisplay(h.amount_kg, units).toFixed(units === "imperial" ? 2 : 0)}{hopWeightLabel(units)}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{h.use_}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{h.time_min}min</td>
            <td class="pl-1">
              <button onclick={() => handleDelete(h.id)} class="text-xs opacity-40 hover:opacity-100"
                      style="color: var(--color-text-secondary);">×</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
