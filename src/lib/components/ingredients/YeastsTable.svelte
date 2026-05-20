<script lang="ts">
  import type { Recipe } from "$lib/api";
  import { createRecipeYeast, deleteRecipeYeast } from "$lib/api";
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";
  import { ipc } from "$lib/stores/error";
  import IngredientPicker, { type AddPayload } from "$platform/IngredientPicker.svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let adding = $state(false);

  async function handlePickerAdd(payload: AddPayload) {
    if (payload.type !== "yeast") return;
    const result = await ipc(createRecipeYeast(recipe.id, {
      yeast_id: payload.item.id,
      name: payload.item.name,
      type_: payload.item.type_,
      form: payload.item.form,
      laboratory: payload.item.laboratory,
      product_id: payload.item.product_id,
      attenuation_pct: payload.item.attenuation_pct,
      amount: payload.amount,
    }));
    if (result === undefined) return;
    adding = false;
    onchange();
  }

  async function handleDelete(id: string) {
    await ipc(deleteRecipeYeast(id));
    onchange();
  }
</script>

<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold flex items-center gap-2" style="color: var(--color-text-primary);">
      <BrewingIcon name="yeast" />
      Yeast
    </h3>
    <button onclick={() => adding = true} class="text-xs px-2 py-1 rounded"
            style="background: var(--color-accent); color: #fff;">+ Add</button>
  </div>

  <IngredientPicker
    type="yeast"
    open={adding}
    onclose={() => adding = false}
    onadd={handlePickerAdd}
  />

  {#each recipe.yeasts as y (y.id)}
    <div class="flex items-center justify-between py-1.5 border-t" style="border-color: var(--color-border);">
      <div>
        <p class="text-sm" style="color: var(--color-text-primary);">{y.name}</p>
        <p class="text-xs" style="color: var(--color-text-secondary);">
          {y.laboratory ?? ""} {y.product_id ?? ""} · {y.attenuation_pct ?? "?"}% attenuation
        </p>
      </div>
      <button onclick={() => handleDelete(y.id)} class="text-xs opacity-40 hover:opacity-100"
              style="color: var(--color-text-secondary);">×</button>
    </div>
  {/each}
</div>
