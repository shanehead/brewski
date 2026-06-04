import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import DocLink from "$lib/components/DocLink.svelte";

vi.mock("@tauri-apps/plugin-opener", () => ({ openUrl: vi.fn() }));

describe("DocLink", () => {
  it("renders the link with the correct label", () => {
    render(DocLink, { label: "Hops guide", url: "https://example.com/hops" });
    expect(screen.getByText("Hops guide ↗")).toBeInTheDocument();
  });

  it("calls openUrl with the correct URL on click", async () => {
    const { openUrl } = await import("@tauri-apps/plugin-opener");
    const user = userEvent.setup();
    render(DocLink, { label: "Hops guide", url: "https://example.com/hops" });
    await user.click(screen.getByText("Hops guide ↗"));
    expect(openUrl).toHaveBeenCalledWith("https://example.com/hops");
  });
});
