<script lang="ts">
  import type { Recipe } from "$lib/api";
  import { updateRecipe } from "$lib/api";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  async function save(field: string, value: unknown) {
    await updateRecipe(recipe.id, { [field]: value } as any);
    onchange();
  }
</script>

<div class="flex flex-col gap-4 max-w-2xl">
  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Recipe Notes</label>
    <textarea value={recipe.notes ?? ""}
              onblur={(e) => save("notes", (e.target as HTMLTextAreaElement).value || null)}
              rows="8"
              placeholder="Process notes, observations…"
              class="px-3 py-2 rounded text-sm resize-none outline-none"
              style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"></textarea>
  </div>

  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Taste Notes</label>
    <textarea value={recipe.taste_notes ?? ""}
              onblur={(e) => save("taste_notes", (e.target as HTMLTextAreaElement).value || null)}
              rows="4"
              placeholder="Aroma, flavor, appearance, mouthfeel…"
              class="px-3 py-2 rounded text-sm resize-none outline-none"
              style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"></textarea>
  </div>

  <div class="flex flex-col gap-1">
    <label class="text-xs font-medium" style="color: var(--color-text-secondary);">Taste Rating (0–50)</label>
    <input type="number" step="1" min="0" max="50"
           value={recipe.taste_rating ?? ""}
           onblur={(e) => {
             const v = (e.target as HTMLInputElement).value;
             save("taste_rating", v ? parseFloat(v) : null);
           }}
           class="w-24 px-2 py-1.5 rounded text-sm"
           style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
  </div>
</div>
