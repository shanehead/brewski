import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/svelte";
import { tick } from "svelte";
import userEvent from "@testing-library/user-event";
import RecipeView from "../src/lib/desktop/RecipeView.svelte";
import type { Recipe, RecipeStats, RecipeVersionSummary, RecipeVersionStatus } from "$lib/api";

// --- Navigation / stores mocks ---
vi.mock("$app/navigation", () => ({
  goto: vi.fn(),
  afterNavigate: vi.fn(),
}));

vi.mock("$app/stores", () => ({
  page: {
    subscribe: vi.fn((fn) => {
      fn({ url: new URL("http://localhost/recipe/A") });
      return () => {};
    }),
  },
}));

// --- Tauri mocks ---
vi.mock("@tauri-apps/api/path", () => ({
  appDataDir: vi.fn().mockResolvedValue("/tmp"),
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
  convertFileSrc: (s: string) => s,
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({
  save: vi.fn(),
  open: vi.fn(),
}));

// --- Lib store mocks ---
vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn(async (p: Promise<unknown>) => {
    try { return await p; } catch { return undefined; }
  }),
  lastError: { set: vi.fn(), subscribe: vi.fn((fn) => { fn(null); return () => {}; }) },
}));

vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: vi.fn((fn) => {
      fn({ theme: "midnight", units: "metric", gravity_unit: "sg" });
      return () => {};
    }),
  },
  saveSetting: vi.fn(),
}));

vi.mock("$lib/stores/recipes", () => ({
  recipeList: { subscribe: vi.fn((fn) => { fn([]); return () => {}; }) },
  baselineRecipeList: { subscribe: vi.fn((fn) => { fn([]); return () => {}; }) },
  refreshRecipeList: vi.fn().mockResolvedValue(undefined),
  refreshBaselineRecipeList: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("$lib/brewFlow", () => ({
  startBrew: vi.fn(),
  brewCurrent: vi.fn(),
  brewVersion: vi.fn(),
}));

// --- Fixtures ---

function makeRecipe(id: string): Recipe {
  return {
    id,
    name: `Recipe ${id}`,
    type_: "all_grain",
    batch_size_l: 20,
    boil_size_l: 25,
    boil_time_min: 60,
    efficiency_pct: 72,
    fermentation_stages: 1,
    forced_carbonation: false,
    hopstand_temp_c: 80,
    created_at: 0,
    updated_at: 0,
    source: "user",
    fermentables: [],
    hops: [],
    yeasts: [],
    miscs: [],
    waters: [],
    water_adjustments: [],
    mash: null,
    brewer: null,
    asst_brewer: null,
    style_id: null,
    style: null,
    equipment_profile_id: null,
    equipment_profile: null,
    notes: null,
    taste_notes: null,
    taste_rating: null,
    og: null,
    fg: null,
    primary_age_days: null,
    primary_temp_c: null,
    secondary_age_days: null,
    secondary_temp_c: null,
    tertiary_age_days: null,
    tertiary_temp_c: null,
    age_days: null,
    age_temp_c: null,
    carbonation_vols: null,
    priming_sugar_name: null,
    carbonation_temp_c: null,
    priming_sugar_equiv: null,
    keg_priming_factor: null,
    date: null,
    mash_water_id: null,
    sparge_water_id: null,
    image_path: null,
  };
}

const versionSnapshot: Recipe = {
  ...makeRecipe("A"),
  name: "OLD VERSION SNAPSHOT",
};

const versionSummary: RecipeVersionSummary = {
  id: "ver1",
  recipe_id: "A",
  version_number: 1,
  name: null,
  created_at: 1000,
  parent_version_id: null,
};

const versionStatus: RecipeVersionStatus = {
  version_count: 1,
  latest_version_id: "ver1",
  has_unversioned_changes: false,
};

const mockStats: RecipeStats = {
  og: 1.054,
  fg: 1.010,
  abv_pct: 5.8,
  ibu: 45,
  srm: 8,
  bu_gu_ratio: 0.83,
  calories_per_355ml: 180,
  pre_boil_gravity: 1.040,
  pre_boil_volume_l: 26,
  post_boil_volume_l: 23,
  strike_temp_c: null,
  hop_stats: [],
  mash_water_l: 10,
  sparge_water_l: 0,
  top_up_water_l: 0,
  total_water_l: 10,
  mash_volume_l: 12,
  mash_volume_excess_l: null,
};

// --- Set up invoke mock ---

async function setupInvoke() {
  const { invoke } = await import("@tauri-apps/api/core");
  const mockInvoke = vi.mocked(invoke);

  mockInvoke.mockImplementation(async (cmd: string, args?: unknown) => {
    const a = args as Record<string, unknown> | undefined;
    switch (cmd) {
      case "get_recipe":
        return makeRecipe(a?.id as string);
      case "get_recipe_stats":
        return mockStats;
      case "list_recipe_versions":
        return [versionSummary];
      case "get_recipe_version":
        return versionSnapshot;
      case "recipe_version_status":
        return versionStatus;
      case "list_styles":
        return [];
      case "list_equipment_profiles":
        return [];
      default:
        return null;
    }
  });

  return mockInvoke;
}

beforeEach(async () => {
  vi.clearAllMocks();
  await setupInvoke();
});

describe("RecipeView — version-view state cleared on recipe navigation", () => {
  it("clears viewingVersion and viewingRecipe when id prop changes", async () => {
    const user = userEvent.setup();
    const { rerender } = render(RecipeView, { props: { id: "A" } });

    // Wait for Recipe A to load (name input shows recipe name)
    await waitFor(() => expect(screen.getByDisplayValue("Recipe A")).toBeInTheDocument(), { timeout: 3000 });

    // Open the version history panel by clicking the History button
    const historyBtn = screen.getByRole("button", { name: /History/i });
    await user.click(historyBtn);

    // The version panel should now be open; click on the version row to trigger onview
    // The version rows are role="button" divs that call onview
    await waitFor(() => expect(screen.getByText(/v1/)).toBeInTheDocument());

    const versionRow = screen.getByRole("button", { name: /v1/i });
    await user.click(versionRow);

    // Now we should be viewing v1 — banner should appear
    await waitFor(() => expect(screen.getByText(/Viewing v1/i)).toBeInTheDocument(), { timeout: 3000 });

    // BUG REPRODUCTION: navigate to Recipe B by changing the id prop
    await rerender({ id: "B" });

    // After navigation, version-view state must be cleared:
    // 1. Recipe B's name must appear in the header input
    // 2. The banner "Viewing v1" must be gone
    await waitFor(() => expect(screen.getByDisplayValue("Recipe B")).toBeInTheDocument(), { timeout: 3000 });
    expect(screen.queryByText(/Viewing v1/i)).not.toBeInTheDocument();
  });
});
