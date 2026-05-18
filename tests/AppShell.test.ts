import { describe, it, expect, vi } from "vitest";
import { render } from "@testing-library/svelte";
import AppShell from "../src/lib/desktop/AppShell.svelte";

vi.mock("$app/stores", () => ({
  page: {
    subscribe: vi.fn((fn) => {
      fn({ url: { pathname: "/" } });
      return () => {};
    }),
  },
}));

vi.mock("$lib/stores/settings", () => ({
  loadSettings: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("$lib/stores/error", () => ({
  lastError: { subscribe: vi.fn((fn) => { fn(null); return () => {}; }) },
}));

describe("AppShell rail", () => {
  it("renders an Equipment nav link", () => {
    const { container } = render(AppShell, { children: () => null });
    const links = container.querySelectorAll("nav a");
    const hrefs = Array.from(links).map((a) => a.getAttribute("href"));
    expect(hrefs).toContain("/equipment");
  });

  it("Equipment link has aria-label Equipment", () => {
    const { container } = render(AppShell, { children: () => null });
    const equipLink = container.querySelector('a[href="/equipment"]');
    expect(equipLink?.getAttribute("aria-label")).toBe("Equipment");
  });

  it("Equipment link appears before the spacer and after the Tools link", () => {
    const { container } = render(AppShell, { children: () => null });
    const links = Array.from(container.querySelectorAll("nav a"));
    const toolsIdx = links.findIndex((a) => a.getAttribute("href") === "/tools");
    const equipIdx = links.findIndex((a) => a.getAttribute("href") === "/equipment");
    const settingsIdx = links.findIndex((a) => a.getAttribute("href") === "/settings");
    expect(equipIdx).toBeGreaterThan(toolsIdx);
    expect(equipIdx).toBeLessThan(settingsIdx);
  });
});
