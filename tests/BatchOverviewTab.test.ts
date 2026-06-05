// tests/BatchOverviewTab.test.ts
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { tick } from "svelte";
import BatchOverviewTab from "$lib/components/batch/BatchOverviewTab.svelte";
import type { Batch, UpdateBatchInput } from "$lib/api";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("$lib/stores/error", () => ({ ipc: vi.fn((p) => p) }));
vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: vi.fn((fn) => { fn({ gravity_unit: "sg" }); return () => {}; }),
  },
}));

function makeBatch(status: string): Batch {
  return {
    id: "b1",
    recipe_id: "r1",
    recipe_name: "Test IPA",
    recipe_version_id: "v1",
    name: null,
    status,
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
  } as unknown as Batch;
}

const noop = () => {};

describe("BatchOverviewTab", () => {
  it("shows gravity log section when fermenting", async () => {
    render(BatchOverviewTab, {
      batch: makeBatch("fermenting"),
      onUpdate: noop,
      onRefresh: noop,
    });
    await tick();
    expect(screen.getByText(/GRAVITY LOG/i)).toBeInTheDocument();
  });

  it("does not show gravity log section when planned", async () => {
    render(BatchOverviewTab, {
      batch: makeBatch("planned"),
      onUpdate: noop,
      onRefresh: noop,
    });
    await tick();
    expect(screen.queryByText(/GRAVITY LOG/i)).toBeNull();
  });

  it("shows tasting section when packaged", async () => {
    render(BatchOverviewTab, {
      batch: makeBatch("packaged"),
      onUpdate: noop,
      onRefresh: noop,
    });
    await tick();
    expect(screen.getByText(/TASTING/i)).toBeInTheDocument();
  });

  it("does not show tasting section when fermenting", async () => {
    render(BatchOverviewTab, {
      batch: makeBatch("fermenting"),
      onUpdate: noop,
      onRefresh: noop,
    });
    await tick();
    expect(screen.queryByText(/TASTING/i)).toBeNull();
  });

  it("does not include conditioning as a status option", () => {
    render(BatchOverviewTab, {
      batch: makeBatch("planned"),
      onUpdate: noop,
      onRefresh: noop,
    });
    // Mobile select shows the status options
    const options = screen.queryAllByRole("option");
    const values = options.map((o) => (o as HTMLOptionElement).value);
    expect(values).not.toContain("conditioning");
    expect(values).toEqual(["planned", "brewing", "fermenting", "packaged"]);
  });
});
