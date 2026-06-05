// tests/BatchesHome.test.ts
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import DesktopBatchesHome from "../src/lib/desktop/BatchesHome.svelte";
import MobileBatchesHome from "../src/lib/mobile/BatchesHome.svelte";
import type { RecipeSummary, RecipeVersionSummary } from "$lib/api";

vi.mock("$app/navigation", () => ({ goto: vi.fn() }));
vi.mock("$lib/stores/error", () => ({ ipc: vi.fn((p) => p) }));

const { mockRefreshBatchList } = vi.hoisted(() => ({
  mockRefreshBatchList: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("$lib/stores/batches", () => ({
  batchList: { subscribe: vi.fn((fn) => { fn([]); return () => {}; }) },
  refreshBatchList: mockRefreshBatchList,
}));

const { mockListRecipes, mockListRecipeVersions, mockCreateBatch } = vi.hoisted(() => ({
  mockListRecipes: vi.fn(),
  mockListRecipeVersions: vi.fn(),
  mockCreateBatch: vi.fn(),
}));

vi.mock("$lib/api", () => ({
  listRecipes: mockListRecipes,
  listRecipeVersions: mockListRecipeVersions,
  createBatch: mockCreateBatch,
}));

function makeRecipe(overrides: Partial<RecipeSummary> = {}): RecipeSummary {
  return {
    id: "r1",
    name: "Pliny the Elder",
    type_: "All Grain",
    batch_size_l: 19,
    style_name: null,
    image_path: null,
    created_at: 0,
    updated_at: 0,
    source: "user",
    ...overrides,
  } as RecipeSummary;
}

function makeVersion(overrides: Partial<RecipeVersionSummary> = {}): RecipeVersionSummary {
  return {
    id: "ver1",
    recipe_id: "r1",
    version_number: 1,
    name: null,
    parent_version_id: null,
    created_at: 1700000000,
    ...overrides,
  };
}

beforeEach(() => {
  mockListRecipes.mockReset();
  mockListRecipeVersions.mockReset();
  mockCreateBatch.mockReset();
  mockRefreshBatchList.mockClear();
});

describe.each([
  { label: "desktop", Component: DesktopBatchesHome },
  { label: "mobile", Component: MobileBatchesHome },
])("BatchesHome ($label) — version picker", ({ Component }) => {
  it("creates batch immediately when recipe has only one version", async () => {
    const user = userEvent.setup();
    const recipe = makeRecipe();
    mockListRecipes.mockResolvedValue([recipe]);
    mockListRecipeVersions.mockResolvedValue([makeVersion()]);
    mockCreateBatch.mockResolvedValue({ id: "b1" });

    render(Component);

    await user.click(screen.getByRole("button", { name: /\+ New Batch/i }));
    await waitFor(() => screen.getByText("Pliny the Elder"));
    await user.click(screen.getByText("Pliny the Elder"));

    await waitFor(() => expect(mockCreateBatch).toHaveBeenCalledWith(
      expect.objectContaining({ recipe_id: "r1" })
    ));
    expect(mockCreateBatch).toHaveBeenCalledWith(
      expect.not.objectContaining({ version_id: expect.anything() })
    );
    expect(screen.queryByText(/Choose a version/i)).not.toBeInTheDocument();
  });

  it("shows version picker when recipe has two or more versions", async () => {
    const user = userEvent.setup();
    const recipe = makeRecipe();
    mockListRecipes.mockResolvedValue([recipe]);
    mockListRecipeVersions.mockResolvedValue([
      makeVersion({ id: "ver2", version_number: 2, name: "Added Citra dry hop", created_at: 1710000000 }),
      makeVersion({ id: "ver1", version_number: 1, name: null, created_at: 1700000000 }),
    ]);

    render(Component);

    await user.click(screen.getByRole("button", { name: /\+ New Batch/i }));
    await waitFor(() => screen.getByText("Pliny the Elder"));
    await user.click(screen.getByText("Pliny the Elder"));

    await waitFor(() => expect(screen.getByText(/v2/i)).toBeInTheDocument());
    expect(screen.getByText(/v1/i)).toBeInTheDocument();
    expect(mockCreateBatch).not.toHaveBeenCalled();
  });

  it("creates batch with explicit version_id when version is selected", async () => {
    const user = userEvent.setup();
    const recipe = makeRecipe();
    mockListRecipes.mockResolvedValue([recipe]);
    mockListRecipeVersions.mockResolvedValue([
      makeVersion({ id: "ver2", version_number: 2, name: "Added Citra dry hop", created_at: 1710000000 }),
      makeVersion({ id: "ver1", version_number: 1, name: null, created_at: 1700000000 }),
    ]);
    mockCreateBatch.mockResolvedValue({ id: "b1" });

    render(Component);

    await user.click(screen.getByRole("button", { name: /\+ New Batch/i }));
    await waitFor(() => screen.getByText("Pliny the Elder"));
    await user.click(screen.getByText("Pliny the Elder"));

    await waitFor(() => screen.getByText(/v2/i));
    await user.click(screen.getByText(/v1/i));

    await waitFor(() => expect(mockCreateBatch).toHaveBeenCalledWith(
      expect.objectContaining({ recipe_id: "r1", version_id: "ver1", name: null })
    ));
  });

  it("back link in version picker returns to recipe list", async () => {
    const user = userEvent.setup();
    const recipe = makeRecipe();
    mockListRecipes.mockResolvedValue([recipe]);
    mockListRecipeVersions.mockResolvedValue([
      makeVersion({ id: "ver2", version_number: 2, created_at: 1710000000 }),
      makeVersion({ id: "ver1", version_number: 1, created_at: 1700000000 }),
    ]);

    render(Component);

    await user.click(screen.getByRole("button", { name: /\+ New Batch/i }));
    await waitFor(() => screen.getByText("Pliny the Elder"));
    await user.click(screen.getByText("Pliny the Elder"));

    await waitFor(() => screen.getByText(/v2/i));
    await user.click(screen.getByRole("button", { name: /Pliny the Elder/i }));

    await waitFor(() => screen.getByText("Pliny the Elder"));
    expect(screen.queryByText(/v2/i)).not.toBeInTheDocument();
  });
});
