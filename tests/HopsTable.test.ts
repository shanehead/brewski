import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import type { Recipe, RecipeStats, RecipeAdditionHop } from "$lib/api";
import HopsTable from "$lib/components/ingredients/HopsTable.svelte";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

function makeHop(overrides: Partial<RecipeAdditionHop> = {}): RecipeAdditionHop {
  return {
    id: "h1",
    recipe_id: "r1",
    hop_id: null,
    name: "Cascade",
    alpha_pct: 5.5,
    form: "Pellet",
    amount_kg: 0.05,
    use_: "Boil",
    time_min: 60,
    addition_order: 0,
    hopstand_temp_c: null,
    ...overrides,
  };
}

function makeRecipe(hops: RecipeAdditionHop[]): Recipe {
  return {
    id: "r1",
    hops,
    fermentables: [],
    yeasts: [],
    miscs: [],
    waters: [],
    water_adjustments: [],
  } as unknown as Recipe;
}

function makeStats(hopStats: Array<{ hop_id: string; ibu: number }>): RecipeStats {
  return {
    og: 1.050,
    fg: 1.012,
    abv_pct: 5.0,
    ibu: hopStats.reduce((sum, s) => sum + s.ibu, 0),
    srm: 5,
    calories_per_355ml: 150,
    bu_gu_ratio: 0.4,
    pre_boil_gravity: 1.040,
    pre_boil_volume_l: 27,
    post_boil_volume_l: 23,
    strike_temp_c: null,
    hop_stats: hopStats,
  } as RecipeStats;
}

describe("HopsTable IBU column", () => {
  it("shows rounded IBU for a boil hop with a non-zero contribution", () => {
    const hop = makeHop({ id: "h1" });
    render(HopsTable, {
      recipe: makeRecipe([hop]),
      stats: makeStats([{ hop_id: "h1", ibu: 29.3 }]),
      onchange: vi.fn(),
    });
    expect(screen.getByText("29")).toBeInTheDocument();
  });

  it("shows — for a dry hop addition (zero IBU)", () => {
    const hop = makeHop({ id: "h1", use_: "Dry Hop" });
    render(HopsTable, {
      recipe: makeRecipe([hop]),
      stats: makeStats([{ hop_id: "h1", ibu: 0 }]),
      onchange: vi.fn(),
    });
    const dashes = screen.getAllByText("—");
    expect(dashes.length).toBeGreaterThan(0);
  });

  it("shows — for all hops when stats is null", () => {
    const hop = makeHop({ id: "h1" });
    render(HopsTable, {
      recipe: makeRecipe([hop]),
      stats: null,
      onchange: vi.fn(),
    });
    const dashes = screen.getAllByText("—");
    expect(dashes.length).toBeGreaterThan(0);
  });
});
