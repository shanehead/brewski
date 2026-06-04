import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import DocLink from "$lib/components/DocLink.svelte";

vi.mock("@tauri-apps/plugin-opener", () => ({ openUrl: vi.fn() }));

let mockShowTooltips = true;
vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: vi.fn((fn) => {
      fn({ show_tooltips: mockShowTooltips });
      return () => {};
    }),
  },
}));

describe("DocLink", () => {
  it("renders the link with the correct label when tooltips are on", () => {
    mockShowTooltips = true;
    render(DocLink, { label: "Hops guide", url: "https://example.com/hops" });
    expect(screen.getByText("Hops guide ↗")).toBeInTheDocument();
  });

  it("calls openUrl with the correct URL on click", async () => {
    mockShowTooltips = true;
    const { openUrl } = await import("@tauri-apps/plugin-opener");
    const user = userEvent.setup();
    render(DocLink, { label: "Hops guide", url: "https://example.com/hops" });
    await user.click(screen.getByText("Hops guide ↗"));
    expect(openUrl).toHaveBeenCalledWith("https://example.com/hops");
  });

  it("does not render when tooltips are off", () => {
    mockShowTooltips = false;
    render(DocLink, { label: "Hops guide", url: "https://example.com/hops" });
    expect(screen.queryByText("Hops guide ↗")).not.toBeInTheDocument();
  });
});
