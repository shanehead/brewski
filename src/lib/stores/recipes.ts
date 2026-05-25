import { writable } from "svelte/store";
import type { RecipeSummary } from "$lib/api";
import { listRecipes, listBaselineRecipes } from "$lib/api";

export const recipeList = writable<RecipeSummary[]>([]);

export async function refreshRecipeList() {
  const list = await listRecipes();
  recipeList.set(list);
}

export const baselineRecipeList = writable<RecipeSummary[]>([]);

export async function refreshBaselineRecipeList() {
  const list = await listBaselineRecipes();
  baselineRecipeList.set(list);
}
