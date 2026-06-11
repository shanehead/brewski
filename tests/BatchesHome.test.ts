// tests/BatchesHome.test.ts
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { goto } from "$app/navigation";
import DesktopBatchesHome from "../src/lib/desktop/BatchesHome.svelte";
import MobileBatchesHome from "../src/lib/mobile/BatchesHome.svelte";
import type { RecipeSummary, RecipeVersionSummary, RecipeVersionStatus } from "$lib/api";

vi.mock("$app/navigation", () => ({ goto: vi.fn() }));
vi.mock("$lib/stores/error", () => ({ ipc: vi.fn((p) => p) }));

const { mockRefreshBatchList } = vi.hoisted(() => ({
  mockRefreshBatchList: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("$lib/stores/batches", () => ({
  batchList: { subscribe: vi.fn((fn) => { fn([]); return () => {}; }) },
  refreshBatchList: mockRefreshBatchList,
}));

const { mockListRecipes, mockStartBrew, mockBrewCurrent, mockBrewVersion } = vi.hoisted(() => ({
  mockListRecipes: vi.fn(),
  mockStartBrew: vi.fn(),
  mockBrewCurrent: vi.fn(),
  mockBrewVersion: vi.fn(),
}));

vi.mock("$lib/api", () => ({
  listRecipes: mockListRecipes,
}));

vi.mock("$lib/brewFlow", () => ({
  startBrew: mockStartBrew,
  brewCurrent: mockBrewCurrent,
  brewVersion: mockBrewVersion,
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

function makeStatus(overrides: Partial<RecipeVersionStatus> = {}): RecipeVersionStatus {
  return {
    version_count: 2,
    has_unversioned_changes: false,
    latest_version_id: "ver1",
    ...overrides,
  } as RecipeVersionStatus;
}

beforeEach(() => {
  mockListRecipes.mockReset();
  mockStartBrew.mockReset();
  mockBrewCurrent.mockReset();
  mockBrewVersion.mockReset();
  mockRefreshBatchList.mockClear();
  vi.mocked(goto).mockClear();
});

describe.each([
  { label: "desktop", Component: DesktopBatchesHome },
  { label: "mobile", Component: MobileBatchesHome },
])("BatchesHome ($label) — version picker", ({ Component }) => {
  it("always shows the version modal for a clean recipe (no auto path)", async () => {
    const user = userEvent.setup();
    const recipe = makeRecipe();
    mockListRecipes.mockResolvedValue([recipe]);
    const versions = [makeVersion({ id: "ver1", version_number: 1 })];
    mockStartBrew.mockResolvedValue({
      status: makeStatus({ has_unversioned_changes: false, latest_version_id: "ver1" }),
      versions,
    });

    render(Component);

    await user.click(screen.getByRole("button", { name: /\+ New Batch/i }));
    await waitFor(() => screen.getByText("Pliny the Elder"));
    await user.click(screen.getByText("Pliny the Elder"));

    await waitFor(() => expect(mockStartBrew).toHaveBeenCalledWith("r1"));
    // Modal must appear even for a clean recipe — no auto navigation.
    await waitFor(() => expect(screen.getByText(/Choose a version to brew/i)).toBeInTheDocument());
    expect(goto).not.toHaveBeenCalled();
  });

  it("always shows the version modal for a dirty recipe", async () => {
    const user = userEvent.setup();
    const recipe = makeRecipe();
    mockListRecipes.mockResolvedValue([recipe]);
    const versions = [
      makeVersion({ id: "ver2", version_number: 2, name: "Added Citra dry hop", created_at: 1710000000 }),
      makeVersion({ id: "ver1", version_number: 1, name: null, created_at: 1700000000 }),
    ];
    mockStartBrew.mockResolvedValue({
      status: makeStatus({ has_unversioned_changes: true, latest_version_id: "ver2" }),
      versions,
    });

    render(Component);

    await user.click(screen.getByRole("button", { name: /\+ New Batch/i }));
    await waitFor(() => screen.getByText("Pliny the Elder"));
    await user.click(screen.getByText("Pliny the Elder"));

    await waitFor(() => expect(screen.getByText(/Choose a version to brew/i)).toBeInTheDocument());
    // The recipe picker overlay should be gone once the version modal is shown.
    expect(screen.queryByText(/Choose a recipe to brew/i)).not.toBeInTheDocument();
  });

  it("calls brewVersion when a saved version is selected in the modal", async () => {
    const user = userEvent.setup();
    const recipe = makeRecipe();
    mockListRecipes.mockResolvedValue([recipe]);
    const versions = [
      makeVersion({ id: "ver2", version_number: 2, name: "Added Citra dry hop", created_at: 1710000000 }),
      makeVersion({ id: "ver1", version_number: 1, name: null, created_at: 1700000000 }),
    ];
    mockStartBrew.mockResolvedValue({
      status: makeStatus({ has_unversioned_changes: false, latest_version_id: "ver2" }),
      versions,
    });
    mockBrewVersion.mockResolvedValue({ id: "b1" });

    render(Component);

    await user.click(screen.getByRole("button", { name: /\+ New Batch/i }));
    await waitFor(() => screen.getByText("Pliny the Elder"));
    await user.click(screen.getByText("Pliny the Elder"));

    await waitFor(() => screen.getByText(/Brew a saved version/i));
    await user.click(screen.getByRole("button", { name: /Brew a saved version/i }));

    // Modal defaults `selected` to status.latest_version_id ("ver2").
    await waitFor(() => expect(mockBrewVersion).toHaveBeenCalledWith("r1", "ver2"));
  });

  it("dismisses the modal when Cancel is clicked", async () => {
    const user = userEvent.setup();
    const recipe = makeRecipe();
    mockListRecipes.mockResolvedValue([recipe]);
    const versions = [
      makeVersion({ id: "ver2", version_number: 2, created_at: 1710000000 }),
      makeVersion({ id: "ver1", version_number: 1, created_at: 1700000000 }),
    ];
    mockStartBrew.mockResolvedValue({
      status: makeStatus({ has_unversioned_changes: false, latest_version_id: "ver2" }),
      versions,
    });

    render(Component);

    await user.click(screen.getByRole("button", { name: /\+ New Batch/i }));
    await waitFor(() => screen.getByText("Pliny the Elder"));
    await user.click(screen.getByText("Pliny the Elder"));

    await waitFor(() => screen.getByText(/Choose a version to brew/i));
    await user.click(screen.getByRole("button", { name: /Cancel/i }));

    await waitFor(() => expect(screen.queryByText(/Choose a version to brew/i)).not.toBeInTheDocument());
  });
});
