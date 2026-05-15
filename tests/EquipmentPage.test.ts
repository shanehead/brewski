import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import EquipmentPage from "../src/routes/equipment/+page.svelte";

vi.mock("$lib/api", () => ({
  listEquipmentProfiles: vi.fn().mockResolvedValue([
    { id: "1", name: "My Kettle", batch_size_l: 23, boil_size_l: 27, efficiency_pct: 72 },
  ]),
  createEquipmentProfile: vi.fn().mockResolvedValue({}),
  deleteEquipmentProfile: vi.fn().mockResolvedValue({}),
}));

vi.mock("$lib/stores/settings", () => ({
  settings: { subscribe: vi.fn((fn) => { fn({ theme: "midnight", units: "metric", default_equipment_profile_id: "" }); return () => {}; }) },
  loadSettings: vi.fn().mockResolvedValue(undefined),
  saveSetting: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p) => p),
}));

describe("EquipmentPage", () => {
  it("renders the page heading", async () => {
    const { getByText } = render(EquipmentPage);
    expect(getByText("Equipment")).toBeTruthy();
  });

  it("renders the Default Profile label", async () => {
    const { getByText } = render(EquipmentPage);
    expect(getByText("Default Profile")).toBeTruthy();
  });

  it("renders the new profile name input", async () => {
    const { getByPlaceholderText } = render(EquipmentPage);
    expect(getByPlaceholderText("New profile name")).toBeTruthy();
  });

  it("renders the Add button", async () => {
    const { getByText } = render(EquipmentPage);
    expect(getByText("Add")).toBeTruthy();
  });
});
