import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { tick } from "svelte";
import StatsSidebar from "$lib/components/StatsSidebar.svelte";
import type { RecipeStats } from "$lib/api";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("$lib/stores/error", () => ({ ipc: vi.fn((p) => p) }));

// Mutable — tests set this before rendering
let currentGravityUnit = "sg";

vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: vi.fn((fn) => {
      fn({ theme: "midnight", units: "metric", gravity_unit: currentGravityUnit });
      return () => {};
    }),
  },
}));

const mockInvoke = vi.mocked((await import("@tauri-apps/api/core")).invoke);

function makeStats(og = 1.054, fg = 1.010): RecipeStats {
  return {
    og, fg,
    abv_pct: 5.8,
    ibu: 45,
    srm: 8,
    bu_gu_ratio: 0.83,
    calories_per_355ml: 180,
    pre_boil_gravity: 1.040,
    pre_boil_volume_l: 26,
    post_boil_volume_l: 23,
  } as RecipeStats;
}

describe("StatsSidebar gravity display", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
    currentGravityUnit = "sg"; // reset to default
  });

  it("shows SG values when gravity_unit is sg", async () => {
    mockInvoke.mockResolvedValue({ sg: 1.054, plato: 13.3, brix: 13.5 });
    render(StatsSidebar, { stats: makeStats() });
    await tick();
    await new Promise(r => setTimeout(r, 0));
    await tick();
    expect(screen.getAllByText("1.054").length).toBeGreaterThan(0);
  });

  it("shows Plato values when gravity_unit is plato", async () => {
    currentGravityUnit = "plato";
    mockInvoke.mockResolvedValue({ sg: 1.054, plato: 13.3, brix: 13.5 });
    render(StatsSidebar, { stats: makeStats() });
    await tick();
    await new Promise(r => setTimeout(r, 0));
    await tick();
    expect(screen.getAllByText("13.3°P").length).toBeGreaterThan(0);
  });

  it("shows — when stats is null", () => {
    render(StatsSidebar, { stats: null });
    expect(screen.getByText(/Add ingredients/)).toBeInTheDocument();
  });
});
