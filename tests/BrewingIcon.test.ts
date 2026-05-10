import { describe, expect, it } from "vitest";
import { render } from "@testing-library/svelte";
import BrewingIcon from "$lib/components/BrewingIcon.svelte";

describe("BrewingIcon", () => {
  it("renders the hop emoji for name='hop'", () => {
    const { container } = render(BrewingIcon, { name: "hop" });
    expect(container.textContent).toBe("🍃");
  });

  it("renders the fermentable emoji for name='fermentable'", () => {
    const { container } = render(BrewingIcon, { name: "fermentable" });
    expect(container.textContent).toBe("🌾");
  });

  it("renders a span with aria-hidden", () => {
    const { container } = render(BrewingIcon, { name: "yeast" });
    const span = container.querySelector("span");
    expect(span).not.toBeNull();
    expect(span?.getAttribute("aria-hidden")).toBe("true");
  });
});
