import { describe, it, expect, vi, beforeEach } from "vitest";

vi.mock("$lib/api", () => ({
  recipeVersionStatus: vi.fn(),
  listRecipeVersions: vi.fn(),
  saveRecipeVersion: vi.fn(),
  createBatch: vi.fn(),
}));
vi.mock("$lib/stores/error", () => ({ ipc: (p: any) => p }));

import * as api from "$lib/api";
import { startBrew } from "$lib/brewFlow";

beforeEach(() => vi.clearAllMocks());

describe("startBrew", () => {
  it("auto-snapshots v1 when there are no versions", async () => {
    (api.recipeVersionStatus as any).mockResolvedValue({ version_count: 0, latest_version_id: null, has_unversioned_changes: false });
    (api.saveRecipeVersion as any).mockResolvedValue({ id: "v1" });
    (api.createBatch as any).mockResolvedValue({ id: "b1" });
    const r = await startBrew("r1");
    expect(api.saveRecipeVersion).toHaveBeenCalledWith({ recipe_id: "r1", name: null });
    expect(r).toEqual({ kind: "auto", batch: { id: "b1" } });
  });

  it("auto-creates on latest when clean", async () => {
    (api.recipeVersionStatus as any).mockResolvedValue({ version_count: 2, latest_version_id: "v2", has_unversioned_changes: false });
    (api.createBatch as any).mockResolvedValue({ id: "b2" });
    const r = await startBrew("r1");
    expect(api.createBatch).toHaveBeenCalledWith({ recipe_id: "r1", version_id: "v2", name: null });
    expect(r).toEqual({ kind: "auto", batch: { id: "b2" } });
  });

  it("prompts when dirty", async () => {
    (api.recipeVersionStatus as any).mockResolvedValue({ version_count: 1, latest_version_id: "v1", has_unversioned_changes: true });
    (api.listRecipeVersions as any).mockResolvedValue([{ id: "v1", version_number: 1, name: null }]);
    const r = await startBrew("r1");
    expect(r?.kind).toBe("prompt");
  });
});
