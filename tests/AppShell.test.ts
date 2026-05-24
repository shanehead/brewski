import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import AppShell from "../src/lib/desktop/AppShell.svelte";

// Capture the afterNavigate callback so we can trigger it manually
let afterNavigateCb: ((nav: { to: { url: { pathname: string } } | null }) => void) | null = null;

const { gotoMock, saveSettingMock } = vi.hoisted(() => ({
  gotoMock: vi.fn(),
  saveSettingMock: vi.fn(),
}));

vi.mock("$app/navigation", () => ({
  afterNavigate: vi.fn((cb) => { afterNavigateCb = cb; }),
  goto: gotoMock,
}));

let mockPathname = "/";
vi.mock("$app/stores", () => ({
  page: {
    subscribe: vi.fn((fn) => {
      fn({ url: { pathname: mockPathname } });
      return () => {};
    }),
  },
}));

let mockSettings: Record<string, string> = {};

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
  lastError: { subscribe: vi.fn((fn) => { fn(null); return () => {}; }) },
}));

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: vi.fn(() => ({ show: vi.fn() })),
}));

beforeEach(() => {
  gotoMock.mockClear();
  saveSettingMock.mockClear();
  afterNavigateCb = null;
  mockPathname = "/";
  mockSettings = {};
});

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

describe("AppShell last route", () => {
  it("calls goto with last_route when it differs from current path", async () => {
    mockSettings = { last_route: "/tools" };
    mockPathname = "/";
    render(AppShell, { children: () => null });
    // Wait for onMount async logic to settle
    await new Promise((r) => setTimeout(r, 0));
    expect(gotoMock).toHaveBeenCalledWith("/tools");
  });

  it("does not call goto when last_route matches current path", async () => {
    mockSettings = { last_route: "/tools" };
    mockPathname = "/tools";
    render(AppShell, { children: () => null });
    await new Promise((r) => setTimeout(r, 0));
    expect(gotoMock).not.toHaveBeenCalled();
  });

  it("does not call goto when last_route is absent", async () => {
    mockSettings = {};
    mockPathname = "/";
    render(AppShell, { children: () => null });
    await new Promise((r) => setTimeout(r, 0));
    expect(gotoMock).not.toHaveBeenCalled();
  });

  it("calls saveSetting with the navigated-to pathname", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/library" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route", "/library");
  });

  it("does not call saveSetting when navigation.to is null", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: null });
    expect(saveSettingMock).not.toHaveBeenCalled();
  });
});
