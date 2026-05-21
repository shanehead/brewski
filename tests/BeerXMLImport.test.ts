import { describe, it, expect, vi } from "vitest";
import { render, fireEvent } from "@testing-library/svelte";
import RecipeList from "$lib/components/RecipeList.svelte";
import MobileRecipesHome from "$lib/mobile/RecipesHome.svelte";

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

  it("calls createRecipesFromBeerxml with file contents when a file is selected", async () => {
    const { createRecipesFromBeerxml } = await import("$lib/api");
    const { container } = render(RecipeList);

    const xml = "<RECIPES><RECIPE><NAME>Test</NAME></RECIPE></RECIPES>";
    const file = new File([xml], "recipe.xml", { type: "text/xml" });
    vi.spyOn(file, "text").mockResolvedValue(xml);

    const input = container.querySelector('input[type="file"]') as HTMLInputElement;
    Object.defineProperty(input, "files", { value: [file], configurable: true });

    await fireEvent.change(input);

    expect(createRecipesFromBeerxml).toHaveBeenCalledWith(xml);
  });
});

describe("Mobile RecipesHome", () => {
  it("renders the Import BeerXML button", () => {
    const { getByText } = render(MobileRecipesHome);
    expect(getByText("Import BeerXML")).toBeTruthy();
  });
});
