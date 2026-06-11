import { render, fireEvent } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
import BrewVersionModal from "$lib/components/BrewVersionModal.svelte";

const versions = [
  { id: "v2", version_number: 2, name: null },
  { id: "v1", version_number: 1, name: "first" },
] as any;

describe("BrewVersionModal", () => {
  it("shows the current-changes option only when dirty", () => {
    const clean = render(BrewVersionModal, {
      props: {
        status: { version_count: 2, latest_version_id: "v2", has_unversioned_changes: false },
        versions, onBrewCurrent: vi.fn(), onBrewVersion: vi.fn(), onCancel: vi.fn(),
      },
    });
    expect(clean.queryByText(/un-versioned changes/i)).toBeNull();
  });

  it("emits onBrewCurrent with the typed name", async () => {
    const onBrewCurrent = vi.fn();
    const { getByPlaceholderText, getByText } = render(BrewVersionModal, {
      props: {
        status: { version_count: 1, latest_version_id: "v2", has_unversioned_changes: true },
        versions, onBrewCurrent, onBrewVersion: vi.fn(), onCancel: vi.fn(),
      },
    });
    await fireEvent.input(getByPlaceholderText("Name (optional)"), { target: { value: "hop bump" } });
    await fireEvent.click(getByText("Brew with current changes"));
    expect(onBrewCurrent).toHaveBeenCalledWith("hop bump");
  });

  it("emits onBrewVersion with the selected id", async () => {
    const onBrewVersion = vi.fn();
    const { getByText } = render(BrewVersionModal, {
      props: {
        status: { version_count: 2, latest_version_id: "v2", has_unversioned_changes: false },
        versions, onBrewCurrent: vi.fn(), onBrewVersion, onCancel: vi.fn(),
      },
    });
    await fireEvent.click(getByText("Brew a saved version"));
    expect(onBrewVersion).toHaveBeenCalledWith("v2");
  });
});
