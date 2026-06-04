import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import RecipeList from "../src/lib/components/RecipeList.svelte";

vi.mock("$app/navigation", () => ({ goto: vi.fn() }));

let mockSettings: Record<string, unknown> = {};
let mockBaselineRecipes: {
  id: string; name: string; type_: string; batch_size_l: number;
  style_name: string | null; image_path: string | null;
  created_at: number; updated_at: number; source: "user" | "seeded";
}[] = [];

vi.mock("$lib/stores/recipes", () => ({
  recipeList: { subscribe: vi.fn((fn) => { fn([]); return () => {}; }) },
  baselineRecipeList: { subscribe: vi.fn((fn) => { fn(mockBaselineRecipes); return () => {}; }) },
  refreshRecipeList: vi.fn().mockResolvedValue(undefined),
  refreshBaselineRecipeList: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: vi.fn((fn) => { fn(mockSettings); return () => {}; }),
  },
  saveSetting: vi.fn(),
}));

vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p) => p),
  setSuccess: vi.fn(),
}));

vi.mock("$lib/api", () => ({
  createRecipe: vi.fn(),
  deleteRecipe: vi.fn(),
  createRecipesFromBeerxml: vi.fn(),
}));

vi.mock("@tauri-apps/api/core", () => ({
  convertFileSrc: vi.fn((p: string) => p),
}));

vi.mock("@tauri-apps/api/path", () => ({
  appDataDir: vi.fn().mockResolvedValue("/data"),
}));

beforeEach(() => {
  mockSettings = {};
  mockBaselineRecipes = [];
});

const exampleRecipe = {
  id: "ex1", name: "Pliny the Elder", type_: "All Grain",
  batch_size_l: 20.8, style_name: null, image_path: null,
  created_at: 0, updated_at: 0, source: "seeded" as const,
};

describe("RecipeList - example recipes visibility", () => {
  it("shows Example Recipes section when setting is absent", () => {
    mockBaselineRecipes = [exampleRecipe];
    mockSettings = {};
    const { getByText } = render(RecipeList, { selectedId: null });
    expect(getByText("Example Recipes")).toBeInTheDocument();
  });

  it("shows Example Recipes section when setting is false", () => {
    mockBaselineRecipes = [exampleRecipe];
    mockSettings = { hide_example_recipes: false };
    const { getByText } = render(RecipeList, { selectedId: null });
    expect(getByText("Example Recipes")).toBeInTheDocument();
  });

  it("hides Example Recipes section when setting is true", () => {
    mockBaselineRecipes = [exampleRecipe];
    mockSettings = { hide_example_recipes: true };
    const { queryByText } = render(RecipeList, { selectedId: null });
    expect(queryByText("Example Recipes")).toBeNull();
  });

  it("hides individual example recipe names when setting is true", () => {
    mockBaselineRecipes = [exampleRecipe];
    mockSettings = { hide_example_recipes: true };
    const { queryByText } = render(RecipeList, { selectedId: null });
    expect(queryByText("Pliny the Elder")).toBeNull();
  });
});
