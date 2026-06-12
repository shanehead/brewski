import { describe, it, expect, vi } from "vitest";
import { render } from "@testing-library/svelte";
import LibraryPage from "../src/routes/library/+page.svelte";

vi.mock("$lib/api", () => ({
  listHopLibrary: vi.fn().mockResolvedValue([]),
  listFermentableLibrary: vi.fn().mockResolvedValue([]),
  listYeastLibrary: vi.fn().mockResolvedValue([]),
  listMiscLibrary: vi.fn().mockResolvedValue([]),
  listWaterLibrary: vi.fn().mockResolvedValue([]),
  deleteHop: vi.fn().mockResolvedValue(undefined),
  deleteFermentable: vi.fn().mockResolvedValue(undefined),
  deleteYeast: vi.fn().mockResolvedValue(undefined),
  deleteMisc: vi.fn().mockResolvedValue(undefined),
  deleteWater: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p) => p),
}));

describe("LibraryPage", () => {
  it("renders the search input", () => {
    const { getByPlaceholderText } = render(LibraryPage);
    expect(getByPlaceholderText("Search hops…")).toBeInTheDocument();
  });
});
