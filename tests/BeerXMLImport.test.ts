import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, fireEvent } from "@testing-library/svelte";
import { tick } from "svelte";
import userEvent from "@testing-library/user-event";
import RecipeList from "$lib/components/RecipeList.svelte";
import MobileRecipesHome from "$lib/mobile/RecipesHome.svelte";

const { setSuccessMock } = vi.hoisted(() => ({ setSuccessMock: vi.fn() }));

vi.mock("$lib/stores/recipes", () => ({
  recipeList: {
    subscribe: (fn: (val: unknown[]) => void) => {
      fn([]);
      return () => {};
    },
  },
  refreshRecipeList: vi.fn().mockResolvedValue(undefined),
  baselineRecipeList: {
    subscribe: (fn: (val: unknown[]) => void) => {
      fn([]);
      return () => {};
    },
  },
  refreshBaselineRecipeList: vi.fn().mockResolvedValue(undefined),
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
  setSuccess: setSuccessMock,
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
  beforeEach(() => {
    setSuccessMock.mockClear();
  });

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

  it("disables the Import button and shows 'Importing…' while in-flight", async () => {
    const { createRecipesFromBeerxml } = await import("$lib/api");
    let resolve!: (val: unknown[]) => void;
    vi.mocked(createRecipesFromBeerxml).mockReturnValue(
      new Promise((r) => { resolve = r; }) as Promise<never>
    );

    const { container, getByText } = render(RecipeList);

    const xml = "<RECIPES></RECIPES>";
    const file = new File([xml], "recipe.xml", { type: "text/xml" });
    vi.spyOn(file, "text").mockResolvedValue(xml);

    const input = container.querySelector('input[type="file"]') as HTMLInputElement;
    Object.defineProperty(input, "files", { value: [file], configurable: true });
    await fireEvent.change(input);
    await tick();

    const btn = getByText("Importing…");
    expect(btn).toBeTruthy();
    expect((btn as HTMLButtonElement).disabled).toBe(true);

    resolve([]);
    await tick();
    await tick();
    expect(getByText("Import BeerXML")).toBeTruthy();
  });

  it("calls setSuccess with 'N recipes imported' after a successful import", async () => {
    const { createRecipesFromBeerxml } = await import("$lib/api");
    vi.mocked(createRecipesFromBeerxml).mockResolvedValue([
      { id: "r1" } as never,
      { id: "r2" } as never,
    ]);

    const { container } = render(RecipeList);

    const xml = "<RECIPES></RECIPES>";
    const file = new File([xml], "recipe.xml", { type: "text/xml" });
    vi.spyOn(file, "text").mockResolvedValue(xml);

    const input = container.querySelector('input[type="file"]') as HTMLInputElement;
    Object.defineProperty(input, "files", { value: [file], configurable: true });
    await fireEvent.change(input);
    await tick();
    await tick();

    expect(setSuccessMock).toHaveBeenCalledWith("2 recipes imported");
  });

  it("uses singular 'recipe' when exactly 1 is imported", async () => {
    const { createRecipesFromBeerxml } = await import("$lib/api");
    vi.mocked(createRecipesFromBeerxml).mockResolvedValue([{ id: "r1" } as never]);

    const { container } = render(RecipeList);

    const xml = "<RECIPES></RECIPES>";
    const file = new File([xml], "recipe.xml", { type: "text/xml" });
    vi.spyOn(file, "text").mockResolvedValue(xml);

    const input = container.querySelector('input[type="file"]') as HTMLInputElement;
    Object.defineProperty(input, "files", { value: [file], configurable: true });
    await fireEvent.change(input);
    await tick();
    await tick();

    expect(setSuccessMock).toHaveBeenCalledWith("1 recipe imported");
  });
});

describe("Mobile RecipesHome", () => {
  it("renders the Import BeerXML button", () => {
    const { getByText } = render(MobileRecipesHome);
    expect(getByText("Import BeerXML")).toBeTruthy();
  });
});
