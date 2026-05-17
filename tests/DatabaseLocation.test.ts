import { describe, it, expect, vi } from "vitest";
import { render } from "@testing-library/svelte";
import { tick } from "svelte";
import DatabaseLocation from "../src/lib/components/DatabaseLocation.svelte";

vi.mock("$lib/api", () => ({
  getDbPath: vi.fn().mockResolvedValue("/home/user/.local/share/brewski/brewski.db"),
  detectSyncFolders: vi.fn().mockResolvedValue([
    { name: "Dropbox", path: "/home/user/Dropbox/Brewski" },
  ]),
  moveDatabase: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p) => p),
}));

describe("DatabaseLocation", () => {
  it("renders the section heading", () => {
    const { getByText } = render(DatabaseLocation);
    expect(getByText("Database Location")).toBeInTheDocument();
  });

  it("renders the last-write-wins callout", () => {
    const { getByText } = render(DatabaseLocation);
    expect(getByText(/Last write wins/)).toBeInTheDocument();
  });

  it("renders the current db path after mount", async () => {
    const { getByText } = render(DatabaseLocation);
    await new Promise((r) => setTimeout(r, 10));
    await tick();
    expect(getByText(/brewski\.db/)).toBeInTheDocument();
  });

  it("renders detected sync folder names after mount", async () => {
    const { getByText } = render(DatabaseLocation);
    await new Promise((r) => setTimeout(r, 10));
    await tick();
    expect(getByText("Dropbox")).toBeInTheDocument();
  });

  it("renders a Move here button for each detected folder", async () => {
    const { getAllByText } = render(DatabaseLocation);
    await new Promise((r) => setTimeout(r, 10));
    await tick();
    expect(getAllByText("Move here").length).toBeGreaterThan(0);
  });
});
