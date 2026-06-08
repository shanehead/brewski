<script lang="ts">
  import type { Recipe, RecipeStats } from "$lib/api";
  import { createRecipeHop, deleteRecipeHop } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { type Units, kgToHopDisplay, hopWeightLabel, cToF, tempLabel } from "$lib/units";
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";
  import IngredientPicker, { type AddPayload } from "$platform/IngredientPicker.svelte";
  import Tooltip from "$lib/components/Tooltip.svelte";
  import DocLink from "$lib/components/DocLink.svelte";
  import { DOCS } from "$lib/docs-urls";

  let { recipe, stats, onchange }: { recipe: Recipe; stats: RecipeStats | null; onchange: () => void } = $props();

  let adding = $state(false);
  const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");

  const hopIbus = $derived(
    new Map(stats?.hop_stats?.map(s => [s.hop_id, s.ibu]) ?? [])
  );

  async function handlePickerAdd(payload: AddPayload) {
    if (payload.type !== "hop") return;
    const result = await ipc(createRecipeHop(recipe.id, {
      hop_id: payload.item.id,
      name: payload.item.name,
      alpha_pct: payload.item.alpha_pct,
      form: payload.form,
      amount_kg: payload.amount_kg,
      use_: payload.use_,
      time_min: payload.time_min,
      hopstand_temp_c: payload.hopstand_temp_c ?? undefined,
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
    <h3 class="text-sm font-semibold flex items-center gap-2" style="color: var(--color-text-primary);">
      <BrewingIcon name="hop" />
      Hops
    </h3>
    <div class="flex items-center gap-2">
      <DocLink label="Hops guide" url={DOCS.hops} />
      <button onclick={() => adding = true} class="text-xs px-2 py-1 rounded"
              style="background: var(--color-accent); color: #fff;">+ Add</button>
    </div>
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
        <tr style="color: var(--color-text-secondary);">
          <th class="text-left py-1 font-medium text-sm">Name</th>
          <th class="text-right py-1 font-medium text-sm">
            <span class="inline-flex items-center gap-1">AA% <Tooltip text="Alpha acid percentage. This drives bitterness. Higher alpha means fewer grams to hit your IBU target." /></span>
          </th>
          <th class="text-right py-1 font-medium text-sm">{hopWeightLabel(units)}</th>
          <th class="text-right py-1 font-medium text-sm">
            <span class="inline-flex items-center gap-1">Use <Tooltip text="When the hop is added. Boil adds bitterness. Whirlpool and Hopstand add flavor and aroma. Dry Hop adds aroma only." /></span>
          </th>
          <th class="text-right py-1 font-medium text-sm">Time</th>
          <th class="text-right py-1 font-medium text-sm">IBU</th>
          <th class="w-6"></th>
        </tr>
      </thead>
      <tbody>
        {#each recipe.hops as h (h.id)}
          {@const ibu = hopIbus.get(h.id)}
          <tr class="border-t" style="border-color: var(--color-border);">
            <td class="py-1.5" style="color: var(--color-text-primary);">
              {h.name}
              {#if h.form !== 'Pellet'}
                {@const badgeColor =
                  h.form === 'Cryo' ? 'background: #d1fae5; color: #065f46;' :
                  h.form === 'CO2 Extract' ? 'background: #ede9fe; color: #5b21b6;' :
                  'background: var(--color-bg-elevated); color: var(--color-text-secondary);'}
                <span style="font-size: 10px; padding: 1px 5px; border-radius: 4px; font-weight: 600; margin-left: 4px; {badgeColor}">{h.form}</span>
              {/if}
            </td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{h.alpha_pct}%</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{kgToHopDisplay(h.amount_kg, units).toFixed(units === "imperial" ? 2 : 0)}{hopWeightLabel(units)}</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">
              {#if h.use_ === 'hopstand' && h.hopstand_temp_c != null}
                {h.use_} ({units === 'imperial' ? cToF(h.hopstand_temp_c).toFixed(0) : h.hopstand_temp_c.toFixed(0)}{tempLabel(units)})
              {:else}
                {h.use_}
              {/if}
            </td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">{h.time_min}min</td>
            <td class="text-right py-1.5" style="color: var(--color-text-secondary);">
              {ibu != null && ibu > 0 ? ibu.toFixed(0) : '—'}
            </td>
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
