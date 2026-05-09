import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import { tick } from "svelte";
import userEvent from "@testing-library/user-event";
import type { Recipe, Mash, MashStep, RecipeStats, RecipeAdditionFermentable } from "$lib/api";
import MashTab from "$lib/components/tabs/MashTab.svelte";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

// --- Fixtures ---
// Use `as unknown as T` casts: these are test doubles that only populate
// the fields MashTab actually reads, not the full interface.

function makeStep(overrides: Partial<MashStep> = {}): MashStep {
  return {
    id: "s1",
    mash_id: "m1",
    name: "Mash In",
    type_: "infusion",
    infuse_amount_l: null,
    step_temp_c: 67,
    step_time_min: 60,
    ramp_time_min: null,
    end_temp_c: null,
    step_order: 0,
    ...overrides,
  };
}

function makeMash(overrides: Partial<Mash> = {}): Mash {
  return {
    id: "m1",
    recipe_id: "r1",
    name: "Single Infusion",
    grain_temp_c: 21,
    tun_temp_c: null,
    sparge_temp_c: null,
    ph: null,
    tun_weight_kg: null,
    tun_specific_heat: null,
    equip_adjust: false,
    ratio_l_per_kg: null,
    notes: null,
    steps: [],
    ...overrides,
  };
}

function makeRecipe(overrides: Partial<{ mash: Mash | null; fermentables: RecipeAdditionFermentable[] }> = {}): Recipe {
  return {
    id: "r1",
    fermentables: [],
    hops: [],
    yeasts: [],
    miscs: [],
    waters: [],
    mash: null,
    ...overrides,
  } as unknown as Recipe;
}

function makeStats(overrides: Partial<RecipeStats> = {}): RecipeStats {
  return {
    og: 1.050,
    fg: 1.012,
    abv_pct: 5.0,
    ibu: 20,
    srm: 5,
    calories_per_355ml: 150,
    bu_gu_ratio: 0.4,
    pre_boil_gravity: 1.040,
    pre_boil_volume_l: 27,
    post_boil_volume_l: 23,
    strike_temp_c: null,
    ...overrides,
  };
}

// --- Tests ---

describe("MashTab: strike temp display", () => {
  it("renders strike temp when stats.strike_temp_c is set", () => {
    const recipe = makeRecipe({
      mash: makeMash({ steps: [makeStep({ infuse_amount_l: 15 })] }),
      fermentables: [{ amount_kg: 4.5 } as RecipeAdditionFermentable],
    });
    const stats = makeStats({ strike_temp_c: 69.82 });

    render(MashTab, { recipe, stats, onchange: vi.fn() });

    // units defaults to "metric" (settings store initialises to {} in tests)
    // 69.82.toFixed(1) + tempLabel("metric") → "69.8°C"
    expect(screen.getByText(/69\.8°C/)).toBeInTheDocument();
  });

  it("does not render strike temp when stats.strike_temp_c is null", () => {
    render(MashTab, {
      recipe: makeRecipe(),
      stats: makeStats({ strike_temp_c: null }),
      onchange: vi.fn(),
    });

    expect(screen.queryByText(/Strike Temp/)).not.toBeInTheDocument();
  });
});

describe("MashTab: infuse amount input in Add Step form", () => {
  it("shows infuse amount input when step type is infusion", async () => {
    const user = userEvent.setup();
    render(MashTab, { recipe: makeRecipe(), stats: makeStats(), onchange: vi.fn() });

    await user.click(screen.getByRole("button", { name: /\+ Add Step/i }));

    // stepType defaults to "infusion" — infuse input should appear
    // placeholder is "Infuse L" with metric units
    expect(screen.getByPlaceholderText("Infuse L")).toBeInTheDocument();
  });

  it("hides infuse amount input when step type is not infusion", async () => {
    const user = userEvent.setup();
    render(MashTab, { recipe: makeRecipe(), stats: makeStats(), onchange: vi.fn() });

    await user.click(screen.getByRole("button", { name: /\+ Add Step/i }));
    const select = screen.getByRole("combobox") as HTMLSelectElement;
    // happy-dom doesn't implement :checked for <option> elements, so Svelte's
    // bind:value change-event path can't detect the new value. Work around this
    // by using the [selected] attribute path: set the attribute, then call the
    // Svelte-internal __on_r reset handler (is_reset=true uses '[selected]').
    for (const option of Array.from(select.options)) {
      if (option.value === "temperature") {
        option.setAttribute("selected", "");
      } else {
        option.removeAttribute("selected");
      }
    }
    // Trigger Svelte's binding reset handler directly.
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (select as any).__on_r?.();
    await tick();

    expect(screen.queryByPlaceholderText("Infuse L")).not.toBeInTheDocument();
  });
});

describe("MashTab: water:grain ratio fallback input", () => {
  it("shows ratio input when mash has no step with an infuse amount", () => {
    const recipe = makeRecipe({
      // step exists but infuse_amount_l is null → canAutoDerive = false
      mash: makeMash({ steps: [makeStep({ infuse_amount_l: null })] }),
      fermentables: [{ amount_kg: 4.5 } as RecipeAdditionFermentable],
    });

    render(MashTab, { recipe, stats: makeStats(), onchange: vi.fn() });

    // label is "Water:Grain Ratio (L/kg)" with metric units
    expect(screen.getByLabelText(/Water:Grain Ratio/)).toBeInTheDocument();
  });

  it("hides ratio input when a step has an infuse amount and fermentables exist", () => {
    const recipe = makeRecipe({
      // infuse_amount_l set + fermentables → canAutoDerive = true
      mash: makeMash({ steps: [makeStep({ infuse_amount_l: 15 })] }),
      fermentables: [{ amount_kg: 4.5 } as RecipeAdditionFermentable],
    });

    render(MashTab, { recipe, stats: makeStats(), onchange: vi.fn() });

    expect(screen.queryByLabelText(/Water:Grain Ratio/)).not.toBeInTheDocument();
  });
});
