<script lang="ts">
  import type { Recipe, UpdateRecipeInput } from "$lib/api";
  import { updateRecipe } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import MarkdownEditor from "$lib/components/MarkdownEditor.svelte";
  import TabContent from "$lib/components/tabs/TabContent.svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  async function save<K extends keyof UpdateRecipeInput>(field: K, value: UpdateRecipeInput[K] | null) {
    const result = await ipc(updateRecipe(recipe.id, { [field]: value } as UpdateRecipeInput));
    if (!result) return;
    onchange();
  }
</script>

<TabContent>
  <div class="flex flex-col gap-1">
    <label for="notes-recipe" class="text-sm font-medium text-text-secondary">Recipe Notes</label>
    <MarkdownEditor
      id="notes-recipe"
      value={recipe.notes ?? null}
      onchange={(v) => save("notes", v)}
      rows={8}
      placeholder="Process notes, observations…"
    />
  </div>

  <div class="flex flex-col gap-1">
    <label for="notes-taste" class="text-sm font-medium text-text-secondary">Taste Notes</label>
    <MarkdownEditor
      id="notes-taste"
      value={recipe.taste_notes ?? null}
      onchange={(v) => save("taste_notes", v)}
      rows={4}
      placeholder="Aroma, flavor, appearance, mouthfeel…"
    />
  </div>

  <div class="flex flex-col gap-1">
    <label for="notes-rating" class="text-sm font-medium text-text-secondary">Taste Rating (0–50)</label>
    <input id="notes-rating" type="number" inputmode="decimal" step="1" min="0" max="50"
           value={recipe.taste_rating ?? ""}
           onblur={(e) => {
             const v = (e.target as HTMLInputElement).value;
             save("taste_rating", v ? parseFloat(v) : null);
           }}
           class="w-24 px-2 py-1.5 rounded text-sm bg-bg-elevated text-text-primary border border-border"
           />
  </div>
</TabContent>
