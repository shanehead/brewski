import { describe, expect, it } from "vitest";
import { render } from "@testing-library/svelte";
import BrewingIcon from "$lib/components/BrewingIcon.svelte";

describe("BrewingIcon", () => {
  it("renders an svg for name='hop'", () => {
    const { container } = render(BrewingIcon, { name: "hop" });
    const svg = container.querySelector("svg");
    expect(svg).not.toBeNull();
    expect(svg?.getAttribute("data-icon")).toBe("hop");
  });

  it("renders shape markup for name='fermentable'", () => {
    const { container } = render(BrewingIcon, { name: "fermentable" });
    const svg = container.querySelector("svg");
    expect(svg?.innerHTML.trim().length).toBeGreaterThan(0);
  });

  it("renders an svg with aria-hidden", () => {
    const { container } = render(BrewingIcon, { name: "yeast" });
    const svg = container.querySelector("svg");
    expect(svg).not.toBeNull();
    expect(svg?.getAttribute("aria-hidden")).toBe("true");
  });
});
