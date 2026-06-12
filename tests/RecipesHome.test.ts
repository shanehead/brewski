import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import RecipesHome from "../src/lib/mobile/RecipesHome.svelte";

vi.mock("$app/navigation", () => ({ goto: vi.fn() }));

let mockSettings: Record<string, unknown> = {};
let mockBaselineRecipes: {
  id: string; name: string; type_: string; batch_size_l: number;
  style_name: string | null; image_path: string | null;
  created_at: number; updated_at: number; source: "user" | "seeded";
}[] = [];
let mockRecipes: {
  id: string; name: string; type_: string; batch_size_l: number;
  style_name: string | null; image_path: string | null;
  created_at: number; updated_at: number; source: "user" | "seeded";
}[] = [];

vi.mock("$lib/stores/recipes", () => ({
  recipeList: { subscribe: vi.fn((fn) => { fn(mockRecipes); return () => {}; }) },
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
  mockRecipes = [];
});

const exampleRecipe = {
  id: "ex1", name: "Heady Topper", type_: "All Grain",
  batch_size_l: 18.9, style_name: null, image_path: null,
  created_at: 0, updated_at: 0, source: "seeded" as const,
};

describe("RecipesHome (mobile) - example recipes visibility", () => {
  it("shows Example Recipes section when setting is absent", () => {
    mockBaselineRecipes = [exampleRecipe];
    mockSettings = {};
    const { getByText } = render(RecipesHome);
    expect(getByText("Example Recipes")).toBeInTheDocument();
  });

  it("shows Example Recipes section when setting is false", () => {
    mockBaselineRecipes = [exampleRecipe];
    mockSettings = { hide_example_recipes: false };
    const { getByText } = render(RecipesHome);
    expect(getByText("Example Recipes")).toBeInTheDocument();
  });

  it("hides Example Recipes section when setting is true", () => {
    mockBaselineRecipes = [exampleRecipe];
    mockSettings = { hide_example_recipes: true };
    const { queryByText } = render(RecipesHome);
    expect(queryByText("Example Recipes")).toBeNull();
  });

  it("hides individual example recipe names when setting is true", () => {
    mockBaselineRecipes = [exampleRecipe];
    mockSettings = { hide_example_recipes: true };
    const { queryByText } = render(RecipesHome);
    expect(queryByText("Heady Topper")).toBeNull();
  });
});

const userRecipe = {
  id: "u1", name: "Citra Pale Ale", type_: "All Grain",
  batch_size_l: 23, style_name: null, image_path: null,
  created_at: 0, updated_at: 0, source: "user" as const,
};

describe("RecipesHome (mobile) - search", () => {
  it("renders the search input", () => {
    const { getByPlaceholderText } = render(RecipesHome);
    expect(getByPlaceholderText("Search recipes…")).toBeInTheDocument();
  });

  it("shows a matching recipe", async () => {
    mockRecipes = [userRecipe];
    const { getByPlaceholderText, getByText } = render(RecipesHome);
    const input = getByPlaceholderText("Search recipes…");
    await userEvent.type(input, "Citra");
    expect(getByText("Citra Pale Ale")).toBeInTheDocument();
  });

  it("hides a non-matching recipe", async () => {
    mockRecipes = [userRecipe];
    const { getByPlaceholderText, queryByText } = render(RecipesHome);
    const input = getByPlaceholderText("Search recipes…");
    await userEvent.type(input, "Zzz");
    expect(queryByText("Citra Pale Ale")).toBeNull();
  });

  it("example recipes are not filtered by search", async () => {
    mockBaselineRecipes = [exampleRecipe];
    mockSettings = {};
    const { getByPlaceholderText, getByText } = render(RecipesHome);
    const input = getByPlaceholderText("Search recipes…");
    await userEvent.type(input, "Zzz");
    expect(getByText("Example Recipes")).toBeInTheDocument();
  });
});
