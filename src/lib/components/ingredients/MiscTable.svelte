<script lang="ts">
  import type { Recipe } from "$lib/api";
  import { createRecipeMisc, deleteRecipeMisc } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";
  import IngredientPicker, { type AddPayload } from "$platform/IngredientPicker.svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let adding = $state(false);

  async function handlePickerAdd(payload: AddPayload) {
    if (payload.type !== "misc") return;
    const result = await ipc(
      createRecipeMisc(recipe.id, {
        misc_id: payload.item.id,
        name: payload.item.name,
        type_: payload.item.type_,
        use_: payload.use_,
        amount: payload.amount,
        unit: payload.unit,
        time_min: payload.time_min,
      })
    );
    if (result === undefined) return;
    adding = false;
    onchange();
  }

  async function handleDelete(id: string) {
    await ipc(deleteRecipeMisc(id));
    onchange();
  }
</script>

<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-semibold flex items-center gap-2" style="color: var(--color-text-primary);">
      <BrewingIcon name="misc" />
      Misc
    </h3>
    <button
      onclick={() => (adding = true)}
      class="text-xs px-2 py-1 rounded"
      style="background: var(--color-accent); color: #fff;"
    >+ Add</button>
  </div>

  <IngredientPicker
    type="misc"
    open={adding}
    onclose={() => (adding = false)}
    onadd={handlePickerAdd}
  />

  {#if recipe.miscs.length > 0}
    <table class="ingredient-table">
      <thead>
        <tr style="color: var(--color-text-secondary);">
          <th class="text-left py-1">Name</th>
          <th class="text-left py-1">Type</th>
          <th class="text-right py-1">Amount</th>
          <th class="text-right py-1">Use</th>
          <th class="text-right py-1">Time</th>
          <th class="w-6"></th>
        </tr>
      </thead>
      <tbody>
        {#each recipe.miscs as m (m.id)}
          <tr>
            <td class="py-1.5" style="color: var(--color-text-primary);">{m.name}</td>
            <td class="py-1.5" style="color: var(--color-text-secondary);">{m.type_}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);"
              >{m.amount} {m.unit}</td
            >
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{m.use_}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);"
              >{m.time_min}min</td
            >
            <td class="pl-1">
              <button
                onclick={() => handleDelete(m.id)}
                class="opacity-40 hover:opacity-100"
                style="color: var(--color-text-secondary);">×</button
              >
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
