import { describe, it, expect, vi } from "vitest";
import { render } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { tick } from "svelte";
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
    expect(getByText("Equipment")).toBeInTheDocument();
  });

  it("renders the Default Profile label", async () => {
    const { getByText } = render(EquipmentPage);
    expect(getByText("Default Profile")).toBeInTheDocument();
  });

  it("renders the new profile name input", async () => {
    const { getByPlaceholderText } = render(EquipmentPage);
    expect(getByPlaceholderText("New profile name")).toBeInTheDocument();
  });

  it("renders the Add button", async () => {
    const { getByText } = render(EquipmentPage);
    expect(getByText("Add")).toBeInTheDocument();
  });

  it("renders loaded profile name and details after onMount", async () => {
    const { getByText } = render(EquipmentPage);
    // Wait for onMount to complete and state to update
    await new Promise((r) => setTimeout(r, 10));
    await tick();
    await tick();
    // Check that the profile details are rendered (not just in dropdown)
    expect(getByText(/23.0L batch · 72% efficiency/)).toBeInTheDocument();
  });
});

describe("EquipmentPage - search", () => {
  it("renders the search input", async () => {
    const { getByPlaceholderText } = render(EquipmentPage);
    expect(getByPlaceholderText("Search profiles…")).toBeInTheDocument();
  });

  it("shows a matching profile", async () => {
    const { getByPlaceholderText, getByText } = render(EquipmentPage);
    await new Promise((r) => setTimeout(r, 10));
    await tick();
    const input = getByPlaceholderText("Search profiles…");
    await userEvent.type(input, "Kettle");
    expect(getByText("My Kettle")).toBeInTheDocument();
  });

  it("hides a non-matching profile", async () => {
    const { getByPlaceholderText, queryByText } = render(EquipmentPage);
    await new Promise((r) => setTimeout(r, 10));
    await tick();
    const input = getByPlaceholderText("Search profiles…");
    await userEvent.type(input, "Zzz");
    expect(queryByText("My Kettle")).toBeNull();
  });

  it("shows empty state when no profiles match", async () => {
    const { getByPlaceholderText, getByText } = render(EquipmentPage);
    await new Promise((r) => setTimeout(r, 10));
    await tick();
    const input = getByPlaceholderText("Search profiles…");
    await userEvent.type(input, "Zzz");
    expect(getByText(/No profiles match/)).toBeInTheDocument();
  });
});
