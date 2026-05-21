import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { tick } from "svelte";
import userEvent from "@testing-library/user-event";
import type { Batch, UpdateBatchInput } from "$lib/api";
import BatchCarbonationSection from "$lib/components/batch/BatchCarbonationSection.svelte";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: vi.fn((fn) => {
      fn({ theme: "midnight", units: "metric", default_equipment_profile_id: "" });
      return () => {};
    }),
  },
}));
vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p) => p),
}));

const mockInvoke = vi.mocked((await import("@tauri-apps/api/core")).invoke);

function makeBatch(overrides: Partial<Batch> = {}): Batch {
  return {
    id: "b1",
    recipe_id: "r1",
    recipe_name: "Test IPA",
    recipe_version_id: "v1",
    name: null,
    status: "conditioning",
    brew_date: null,
    fermenter_date: null,
    conditioning_date: null,
    packaging_date: null,
    actual_pre_boil_volume_l: null,
    actual_post_boil_volume_l: null,
    actual_batch_size_l: 19,
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

describe("BatchCarbonationSection", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
    mockInvoke.mockResolvedValue(134.5); // default: priming sugar grams
  });

  it("pre-fills temp from recipePrimaryTempC when batch.packaging_temp_c is null", () => {
    render(BatchCarbonationSection, {
      batch: makeBatch({ packaging_temp_c: null }),
      recipePrimaryTempC: 18,
      recipeCarbonationVols: 2.4,
      onUpdate: vi.fn(),
    });
    const input = screen.getByLabelText(/Packaging Temp/i) as HTMLInputElement;
    expect(input.value).toBe("18.0");
  });

  it("uses batch.packaging_temp_c when already set", () => {
    render(BatchCarbonationSection, {
      batch: makeBatch({ packaging_temp_c: 22 }),
      recipePrimaryTempC: 18,
      recipeCarbonationVols: 2.4,
      onUpdate: vi.fn(),
    });
    const input = screen.getByLabelText(/Packaging Temp/i) as HTMLInputElement;
    expect(input.value).toBe("22.0");
  });

  it("shows prompt when no batch size is available", () => {
    render(BatchCarbonationSection, {
      batch: makeBatch({ actual_batch_size_l: null, planned_batch_size_l: null }),
      recipePrimaryTempC: 20,
      recipeCarbonationVols: 2.4,
      onUpdate: vi.fn(),
    });
    expect(screen.getByText(/Enter batch size/i)).toBeInTheDocument();
  });

  it("calls onUpdate with all four carbonation fields on sugar type change", async () => {
    mockInvoke
      .mockResolvedValueOnce(134.5)  // calculatePrimingSugar (initial effect)
      .mockResolvedValueOnce(97.2)   // calculateCo2Pressure (initial effect)
      .mockResolvedValueOnce(128.0)  // calculatePrimingSugar (after sugar change)
      .mockResolvedValueOnce(97.2);  // calculateCo2Pressure (after sugar change)
    const onUpdate = vi.fn();
    const user = userEvent.setup();
    render(BatchCarbonationSection, {
      batch: makeBatch(),
      recipePrimaryTempC: 20,
      recipeCarbonationVols: 2.4,
      onUpdate,
    });
    // Flush the initial $effect async calculation
    await tick();
    await new Promise((r) => setTimeout(r, 0));
    await tick();

    const select = screen.getByLabelText(/Sugar Type/i);
    await user.selectOptions(select, "table_sugar");

    // Flush the recalculation triggered by sugar type change
    await tick();
    await new Promise((r) => setTimeout(r, 0));
    await tick();

    expect(onUpdate).toHaveBeenCalledWith(
      expect.objectContaining({
        carbonation_sugar_type: "table_sugar",
        packaging_temp_c: expect.any(Number),
        priming_sugar_g: expect.any(Number),
        serving_pressure_kpa: expect.any(Number),
      })
    );
  });

  it("calls onUpdate after temperature change", async () => {
    mockInvoke
      .mockResolvedValueOnce(134.5)  // calculatePrimingSugar (initial effect)
      .mockResolvedValueOnce(97.2)   // calculateCo2Pressure (initial effect)
      .mockResolvedValueOnce(130.0)  // calculatePrimingSugar (after temp change)
      .mockResolvedValueOnce(95.0);  // calculateCo2Pressure (after temp change)
    const onUpdate = vi.fn();
    const user = userEvent.setup();
    render(BatchCarbonationSection, {
      batch: makeBatch(),
      recipePrimaryTempC: 20,
      recipeCarbonationVols: 2.4,
      onUpdate,
    });

    // Flush the initial $effect async calculation
    await tick();
    await new Promise((r) => setTimeout(r, 0));
    await tick();

    const input = screen.getByLabelText(/Packaging Temp/i) as HTMLInputElement;
    await user.clear(input);
    await user.type(input, "18");

    // Flush the recalculation triggered by temp change
    await tick();
    await new Promise((r) => setTimeout(r, 0));
    await tick();

    expect(onUpdate).toHaveBeenCalledWith(
      expect.objectContaining({
        packaging_temp_c: expect.any(Number),
        priming_sugar_g: expect.any(Number),
        serving_pressure_kpa: expect.any(Number),
      })
    );
  });
});
