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

export type BrewDecision =
  | { kind: "auto"; batch: Batch }                                   // 0 versions or clean: created directly
  | { kind: "prompt"; status: RecipeVersionStatus; versions: RecipeVersionSummary[] };

/**
 * Decides how to brew `recipeId`:
 * - 0 versions  → snapshot v1 + create batch (kind: "auto")
 * - clean       → create batch on latest version (kind: "auto")
 * - dirty       → return status + versions for the caller to show BrewVersionModal (kind: "prompt")
 */
export async function startBrew(recipeId: string): Promise<BrewDecision | null> {
  const status = await ipc(recipeVersionStatus(recipeId));
  if (!status) return null;

  if (status.version_count === 0) {
    const v = await ipc(saveRecipeVersion({ recipe_id: recipeId, name: null }));
    if (!v) return null;
    const batch = await ipc(createBatch({ recipe_id: recipeId, version_id: v.id, name: null }));
    return batch ? { kind: "auto", batch } : null;
  }

  if (!status.has_unversioned_changes && status.latest_version_id) {
    const batch = await ipc(
      createBatch({ recipe_id: recipeId, version_id: status.latest_version_id, name: null })
    );
    return batch ? { kind: "auto", batch } : null;
  }

  const versions = (await ipc(listRecipeVersions(recipeId))) ?? [];
  return { kind: "prompt", status, versions };
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
