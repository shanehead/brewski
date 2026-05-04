import { writable } from "svelte/store";
import type { RecipeSummary } from "$lib/api";
import { listRecipes } from "$lib/api";

export const recipeList = writable<RecipeSummary[]>([]);

export async function refreshRecipeList() {
  const list = await listRecipes();
  recipeList.set(list);
}
