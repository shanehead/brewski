import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, fireEvent } from "@testing-library/svelte";
import SettingsPage from "../src/routes/settings/+page.svelte";

const { saveSettingMock } = vi.hoisted(() => ({
  saveSettingMock: vi.fn(),
}));

let mockSettings: Record<string, unknown> = {};

vi.mock("$lib/stores/settings", () => ({
  loadSettings: vi.fn().mockResolvedValue(undefined),
  saveSetting: saveSettingMock,
  settings: {
    subscribe: vi.fn((fn) => {
      fn(mockSettings);
      return () => {};
    }),
  },
}));

vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p) => p),
}));

vi.mock("$lib/api", () => ({
  getDbPath: vi.fn().mockResolvedValue("/data/brewski.db"),
  detectSyncFolders: vi.fn().mockResolvedValue([]),
  moveDatabase: vi.fn().mockResolvedValue(undefined),
}));

beforeEach(() => {
  saveSettingMock.mockClear();
  mockSettings = {};
});

describe("Settings page - hide example recipes", () => {
  it("renders a Recipes section heading", () => {
    const { getByText } = render(SettingsPage);
    expect(getByText("Recipes")).toBeInTheDocument();
  });

  it("renders a Hide Example Recipes label", () => {
    const { getByText } = render(SettingsPage);
    expect(getByText("Hide Example Recipes")).toBeInTheDocument();
  });

  it("checkbox is unchecked when setting is false", () => {
    mockSettings = { hide_example_recipes: false };
    const { getByLabelText } = render(SettingsPage);
    expect((getByLabelText("Hide Example Recipes") as HTMLInputElement).checked).toBe(false);
  });

  it("checkbox is checked when setting is true", () => {
    mockSettings = { hide_example_recipes: true };
    const { getByLabelText } = render(SettingsPage);
    expect((getByLabelText("Hide Example Recipes") as HTMLInputElement).checked).toBe(true);
  });

  it("calls saveSetting with 'true' when checkbox is clicked while unchecked", async () => {
    mockSettings = { hide_example_recipes: false };
    const { getByLabelText } = render(SettingsPage);
    await fireEvent.click(getByLabelText("Hide Example Recipes"));
    expect(saveSettingMock).toHaveBeenCalledWith("hide_example_recipes", "true");
  });

  it("calls saveSetting with 'false' when checkbox is clicked while checked", async () => {
    mockSettings = { hide_example_recipes: true };
    const { getByLabelText } = render(SettingsPage);
    await fireEvent.click(getByLabelText("Hide Example Recipes"));
    expect(saveSettingMock).toHaveBeenCalledWith("hide_example_recipes", "false");
  });
});
