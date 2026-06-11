import { render, fireEvent } from "@testing-library/svelte";
import { describe, it, expect, vi } from "vitest";
import BrewVersionModal from "$lib/components/BrewVersionModal.svelte";

const versions = [
  { id: "v2", version_number: 2, name: null },
  { id: "v1", version_number: 1, name: "first" },
] as any;

describe("BrewVersionModal", () => {
  it("hides the current-changes block when clean and versions exist", () => {
    const { queryByText } = render(BrewVersionModal, {
      props: {
        status: { version_count: 2, latest_version_id: "v2", has_unversioned_changes: false },
        versions, onBrewCurrent: vi.fn(), onBrewVersion: vi.fn(), onCancel: vi.fn(),
      },
    });
    expect(queryByText(/un-versioned changes/i)).toBeNull();
    expect(queryByText(/isn't saved as a version yet/i)).toBeNull();
  });

  it("shows the un-versioned-changes warning and hides it when clean", () => {
    const { queryByText } = render(BrewVersionModal, {
      props: {
        status: { version_count: 1, latest_version_id: "v1", has_unversioned_changes: true },
        versions, onBrewCurrent: vi.fn(), onBrewVersion: vi.fn(), onCancel: vi.fn(),
      },
    });
    expect(queryByText(/un-versioned changes/i)).not.toBeNull();
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

  it("shows 'not saved as version yet' warning and no saved-version picker when version_count is 0", () => {
    const { queryByText, queryByRole } = render(BrewVersionModal, {
      props: {
        status: { version_count: 0, latest_version_id: null, has_unversioned_changes: false },
        versions: [],
        onBrewCurrent: vi.fn(), onBrewVersion: vi.fn(), onCancel: vi.fn(),
      },
    });
    expect(queryByText(/isn't saved as a version yet/i)).not.toBeNull();
    expect(queryByText("Brew with current changes")).not.toBeNull();
    expect(queryByRole("button", { name: /Brew a saved version/i })).toBeNull();
  });

  it("emits onCancel when the Cancel button is clicked", async () => {
    const onCancel = vi.fn();
    const { getByText } = render(BrewVersionModal, {
      props: {
        status: { version_count: 2, latest_version_id: "v2", has_unversioned_changes: false },
        versions, onBrewCurrent: vi.fn(), onBrewVersion: vi.fn(), onCancel,
      },
    });
    await fireEvent.click(getByText("Cancel"));
    expect(onCancel).toHaveBeenCalled();
  });

  it("emits onCancel when Escape is pressed", async () => {
    const onCancel = vi.fn();
    render(BrewVersionModal, {
      props: {
        status: { version_count: 2, latest_version_id: "v2", has_unversioned_changes: false },
        versions, onBrewCurrent: vi.fn(), onBrewVersion: vi.fn(), onCancel,
      },
    });
    await fireEvent.keyDown(window, { key: "Escape" });
    expect(onCancel).toHaveBeenCalled();
  });

  it("emits onCancel when the backdrop is clicked", async () => {
    const onCancel = vi.fn();
    const { getByRole } = render(BrewVersionModal, {
      props: {
        status: { version_count: 2, latest_version_id: "v2", has_unversioned_changes: false },
        versions, onBrewCurrent: vi.fn(), onBrewVersion: vi.fn(), onCancel,
      },
    });
    // The dialog card is inside the backdrop; clicking the dialog must NOT close,
    // clicking the backdrop (its parent) must.
    const backdrop = getByRole("dialog").parentElement as HTMLElement;
    await fireEvent.click(backdrop);
    expect(onCancel).toHaveBeenCalled();
  });
});
