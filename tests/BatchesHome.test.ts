// tests/BatchesHome.test.ts
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/svelte";
import DesktopBatchesHome from "../src/lib/desktop/BatchesHome.svelte";
import MobileBatchesHome from "../src/lib/mobile/BatchesHome.svelte";
import type { BatchSummary } from "$lib/api";

vi.mock("$app/navigation", () => ({ goto: vi.fn() }));
vi.mock("$lib/stores/error", () => ({ ipc: vi.fn((p) => p) }));

const { mockRefreshBatchList, mockBatchList } = vi.hoisted(() => ({
  mockRefreshBatchList: vi.fn().mockResolvedValue(undefined),
  mockBatchList: { subscribe: vi.fn((fn) => { fn([]); return () => {}; }) },
}));

vi.mock("$lib/stores/batches", () => ({
  batchList: mockBatchList,
  refreshBatchList: mockRefreshBatchList,
}));

function makeBatch(overrides: Partial<BatchSummary> = {}): BatchSummary {
  return {
    id: "b1",
    recipe_id: "r1",
    recipe_name: "Pliny the Elder",
    name: "Batch 1",
    status: "brewing",
    brew_date: null,
    actual_og: null,
    actual_fg: null,
    ...overrides,
  } as BatchSummary;
}

beforeEach(() => {
  mockRefreshBatchList.mockClear();
  mockBatchList.subscribe.mockImplementation((fn: (v: BatchSummary[]) => void) => { fn([]); return () => {}; });
});

describe.each([
  { label: "desktop", Component: DesktopBatchesHome },
  { label: "mobile", Component: MobileBatchesHome },
])("BatchesHome ($label)", ({ Component }) => {
  it("shows the empty-state hint when there are no batches", () => {
    render(Component);
    expect(screen.getByText(/No batches yet/i)).toBeInTheDocument();
    expect(screen.getByText(/Open a recipe/i)).toBeInTheDocument();
  });

  it("does not show a New Batch button", () => {
    render(Component);
    expect(screen.queryByRole("button", { name: /New Batch/i })).not.toBeInTheDocument();
  });

  it("calls refreshBatchList on mount", () => {
    render(Component);
    expect(mockRefreshBatchList).toHaveBeenCalledTimes(1);
  });
});
