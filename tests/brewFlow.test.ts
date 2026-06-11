import { describe, it, expect, vi, beforeEach } from "vitest";

vi.mock("$lib/api", () => ({
  recipeVersionStatus: vi.fn(),
  listRecipeVersions: vi.fn(),
  saveRecipeVersion: vi.fn(),
  createBatch: vi.fn(),
}));
vi.mock("$lib/stores/error", () => ({ ipc: (p: any) => p }));

import * as api from "$lib/api";
import { startBrew, brewCurrent, brewVersion } from "$lib/brewFlow";

beforeEach(() => vi.clearAllMocks());

describe("startBrew", () => {
  it("returns status and empty versions for a brand-new recipe (0 versions)", async () => {
    const status = { version_count: 0, latest_version_id: null, has_unversioned_changes: false };
    (api.recipeVersionStatus as any).mockResolvedValue(status);
    (api.listRecipeVersions as any).mockResolvedValue([]);
    const r = await startBrew("r1");
    expect(api.recipeVersionStatus).toHaveBeenCalledWith("r1");
    expect(api.listRecipeVersions).toHaveBeenCalledWith("r1");
    expect(r).toEqual({ status, versions: [] });
  });

  it("returns status and versions for a clean recipe", async () => {
    const status = { version_count: 2, latest_version_id: "v2", has_unversioned_changes: false };
    const versions = [{ id: "v2", version_number: 2, name: null }, { id: "v1", version_number: 1, name: null }];
    (api.recipeVersionStatus as any).mockResolvedValue(status);
    (api.listRecipeVersions as any).mockResolvedValue(versions);
    const r = await startBrew("r1");
    expect(r).toEqual({ status, versions });
  });

  it("returns status and versions for a dirty recipe", async () => {
    const status = { version_count: 1, latest_version_id: "v1", has_unversioned_changes: true };
    const versions = [{ id: "v1", version_number: 1, name: null }];
    (api.recipeVersionStatus as any).mockResolvedValue(status);
    (api.listRecipeVersions as any).mockResolvedValue(versions);
    const r = await startBrew("r1");
    expect(r).toEqual({ status, versions });
  });

  it("returns null when status fetch fails", async () => {
    (api.recipeVersionStatus as any).mockResolvedValue(null);
    const r = await startBrew("r1");
    expect(r).toBeNull();
  });

  it("does NOT call saveRecipeVersion or createBatch", async () => {
    (api.recipeVersionStatus as any).mockResolvedValue({ version_count: 0, latest_version_id: null, has_unversioned_changes: false });
    (api.listRecipeVersions as any).mockResolvedValue([]);
    await startBrew("r1");
    expect(api.saveRecipeVersion).not.toHaveBeenCalled();
    expect(api.createBatch).not.toHaveBeenCalled();
  });
});

describe("brewCurrent", () => {
  it("snapshots then creates a batch", async () => {
    (api.saveRecipeVersion as any).mockResolvedValue({ id: "v1" });
    (api.createBatch as any).mockResolvedValue({ id: "b1" });
    const result = await brewCurrent("r1", "hop bump");
    expect(api.saveRecipeVersion).toHaveBeenCalledWith({ recipe_id: "r1", name: "hop bump" });
    expect(api.createBatch).toHaveBeenCalledWith({ recipe_id: "r1", version_id: "v1", name: null });
    expect(result).toEqual({ id: "b1" });
  });
});

describe("brewVersion", () => {
  it("creates a batch on the given version", async () => {
    (api.createBatch as any).mockResolvedValue({ id: "b2" });
    const result = await brewVersion("r1", "v2");
    expect(api.createBatch).toHaveBeenCalledWith({ recipe_id: "r1", version_id: "v2", name: null });
    expect(result).toEqual({ id: "b2" });
  });
});
