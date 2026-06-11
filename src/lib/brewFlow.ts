import {
  recipeVersionStatus,
  listRecipeVersions,
  saveRecipeVersion,
  createBatch,
  type Batch,
  type RecipeVersionStatus,
  type RecipeVersionSummary,
} from "$lib/api";
import { ipc } from "$lib/stores/error";

/**
 * Fetches version status and the versions list for `recipeId`. Always returns
 * both so the caller can show BrewVersionModal in every case.
 */
export async function startBrew(
  recipeId: string
): Promise<{ status: RecipeVersionStatus; versions: RecipeVersionSummary[] } | null> {
  const status = await ipc(recipeVersionStatus(recipeId));
  if (!status) return null;
  const versions = (await ipc(listRecipeVersions(recipeId))) ?? [];
  return { status, versions };
}

/** "Brew with current changes": snapshot (optional name) then create the batch. */
export async function brewCurrent(recipeId: string, name: string | null): Promise<Batch | null> {
  const v = await ipc(saveRecipeVersion({ recipe_id: recipeId, name }));
  if (!v) return null;
  return (await ipc(createBatch({ recipe_id: recipeId, version_id: v.id, name: null }))) ?? null;
}

/** "Brew a saved version". */
export async function brewVersion(recipeId: string, versionId: string): Promise<Batch | null> {
  return (await ipc(createBatch({ recipe_id: recipeId, version_id: versionId, name: null }))) ?? null;
}
