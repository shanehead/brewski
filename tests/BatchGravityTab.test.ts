// tests/BatchGravityTab.test.ts
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { tick } from "svelte";
import userEvent from "@testing-library/user-event";
import BatchGravityTab from "$lib/components/batch/BatchGravityTab.svelte";
import type { Batch } from "$lib/api";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("$lib/stores/error", () => ({ ipc: vi.fn((p) => p) }));

let currentGravityUnit = "sg";

vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: vi.fn((fn) => {
      fn({ gravity_unit: currentGravityUnit });
      return () => {};
    }),
  },
}));

const mockInvoke = vi.mocked((await import("@tauri-apps/api/core")).invoke);

function makeBatch(overrides: Partial<Batch> = {}): Batch {
  return {
    id: "b1",
    recipe_id: "r1",
    recipe_name: "Test IPA",
    recipe_version_id: "v1",
    name: null,
    status: "fermenting",
    brew_date: null,
    fermenter_date: null,
    conditioning_date: null,
    packaging_date: null,
    actual_pre_boil_volume_l: null,
    actual_post_boil_volume_l: null,
    actual_batch_size_l: null,
    actual_pre_boil_gravity: null,
    actual_og: null,
    actual_fg: null,
    notes: null,
    rating: null,
    planned_og: null,
    planned_fg: null,
    planned_pre_boil_gravity: null,
    planned_post_boil_volume_l: null,
    planned_batch_size_l: null,
    packaging_temp_c: null,
    carbonation_sugar_type: null,
    priming_sugar_g: null,
    serving_pressure_kpa: null,
    gravity_readings: [],
    created_at: 0,
    updated_at: 0,
    ...overrides,
  } as unknown as Batch;
}

describe("BatchGravityTab", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
    currentGravityUnit = "sg";
  });

  it("shows 'No readings yet' when batch has no readings", () => {
    render(BatchGravityTab, { batch: makeBatch(), onRefresh: vi.fn() });
    expect(screen.getByText(/No readings yet/)).toBeInTheDocument();
  });

  it("displays existing reading in Plato when gravity_unit is plato", async () => {
    currentGravityUnit = "plato";
    mockInvoke.mockResolvedValue({ sg: 1.050, plato: 12.4, brix: 12.6 });
    const batch = makeBatch({
      gravity_readings: [{ id: "gr1", gravity: 1.050, temp_c: null, recorded_at: 1700000000, notes: null }] as any,
    });
    render(BatchGravityTab, { batch, onRefresh: vi.fn() });
    await tick();
    await new Promise(r => setTimeout(r, 0));
    await tick();
    expect(screen.getByText("12.4°P")).toBeInTheDocument();
  });

  it("converts Brix input to SG before saving", async () => {
    currentGravityUnit = "brix";
    mockInvoke
      .mockResolvedValueOnce({ sg: 1.054, plato: 13.3, brix: 13.5 })  // convertGravity for input
      .mockResolvedValueOnce(undefined);                                  // addGravityReading

    const onRefresh = vi.fn();
    const user = userEvent.setup();
    render(BatchGravityTab, { batch: makeBatch(), onRefresh });

    const gravityInput = screen.getByPlaceholderText(/Gravity.*°Bx/i);
    await user.type(gravityInput, "13.5");

    const addBtn = screen.getByRole("button", { name: /Add/i });
    await user.click(addBtn);
    await tick();
    await new Promise(r => setTimeout(r, 0));
    await tick();

    expect(mockInvoke).toHaveBeenCalledWith("convert_gravity", { value: 13.5, fromUnit: "brix" });
    expect(mockInvoke).toHaveBeenCalledWith("add_gravity_reading",
      expect.objectContaining({ input: expect.objectContaining({ gravity: 1.054 }) })
    );
  });
});
