import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { tick } from "svelte";
import Tooltip from "$lib/components/Tooltip.svelte";

vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: vi.fn((fn) => {
      fn({ show_tooltips: true });
      return () => {};
    }),
  },
}));

describe("Tooltip", () => {
  it("renders the ? icon", () => {
    render(Tooltip, { text: "Alpha % is the bitterness potential of the hop." });
    expect(screen.getByRole("button", { name: "?" })).toBeInTheDocument();
  });

  it("does not show tooltip text by default", () => {
    render(Tooltip, { text: "Alpha % is the bitterness potential of the hop." });
    expect(screen.queryByText("Alpha % is the bitterness potential of the hop.")).not.toBeInTheDocument();
  });

  it("shows tooltip text after clicking the ? icon", async () => {
    const user = userEvent.setup();
    render(Tooltip, { text: "Alpha % is the bitterness potential of the hop." });
    await user.click(screen.getByRole("button", { name: "?" }));
    await tick();
    expect(screen.getByText("Alpha % is the bitterness potential of the hop.")).toBeInTheDocument();
  });

  it("hides after clicking again", async () => {
    const user = userEvent.setup();
    render(Tooltip, { text: "Some tooltip text." });
    await user.click(screen.getByRole("button", { name: "?" }));
    await tick();
    await user.click(screen.getByRole("button", { name: "?" }));
    await tick();
    expect(screen.queryByText("Some tooltip text.")).not.toBeInTheDocument();
  });
});
