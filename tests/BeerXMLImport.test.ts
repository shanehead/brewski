import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import RecipeList from "$lib/components/RecipeList.svelte";

vi.mock("$lib/stores/recipes", () => ({
  recipeList: {
    subscribe: (fn: (val: unknown[]) => void) => {
      fn([]);
      return () => {};
    },
  },
  refreshRecipeList: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: (fn: (val: { units: string }) => void) => {
      fn({ units: "metric" });
      return () => {};
    },
  },
}));

vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p: Promise<unknown>) => p),
}));

vi.mock("$lib/api", () => ({
  createRecipe: vi.fn(),
  deleteRecipe: vi.fn(),
  createRecipesFromBeerxml: vi.fn().mockResolvedValue([]),
}));

vi.mock("$app/navigation", () => ({
  goto: vi.fn(),
}));

describe("RecipeList", () => {
  it("renders the Import BeerXML button", () => {
    const { getByText } = render(RecipeList);
    expect(getByText("Import BeerXML")).toBeTruthy();
  });
});
