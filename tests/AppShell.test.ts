import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import AppShell from "../src/lib/desktop/AppShell.svelte";

// Capture the afterNavigate callback so we can trigger it manually
let afterNavigateCb: ((nav: { to: { url: { pathname: string; search: string } } | null }) => void) | null = null;

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
      fn({ url: { pathname: mockPathname, search: "" } });
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

let mockLastError: string | null = null;
let mockLastSuccess: string | null = null;

vi.mock("$lib/stores/error", () => ({
  lastError: {
    subscribe: vi.fn((fn) => { fn(mockLastError); return () => {}; }),
    set: vi.fn(),
  },
  lastSuccess: {
    subscribe: vi.fn((fn) => { fn(mockLastSuccess); return () => {}; }),
    set: vi.fn(),
  },
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
  mockLastError = null;
  mockLastSuccess = null;
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
    afterNavigateCb!({ to: { url: { pathname: "/library", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route", "/library");
  });

  it("does not call saveSetting when navigation.to is null", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: null });
    expect(saveSettingMock).not.toHaveBeenCalled();
  });
});

describe("AppShell success banner", () => {
  it("shows success banner when lastSuccess has a message", () => {
    mockLastSuccess = "2 recipes imported";
    const { getByText } = render(AppShell, { children: () => null });
    expect(getByText("2 recipes imported")).toBeTruthy();
  });

  it("does not show success banner when lastSuccess is null", () => {
    mockLastSuccess = null;
    const { queryByText } = render(AppShell, { children: () => null });
    expect(queryByText(/recipes imported/)).toBeNull();
  });
});

describe("AppShell section key saving", () => {
  it("saves last_route_recipes when navigating to /recipe/abc", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/recipe/abc", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_recipes", "/recipe/abc");
  });

  it("saves last_route_recipes when navigating to /", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_recipes", "/");
  });

  it("saves last_route_batches when navigating to /batches/xyz", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/batches/xyz", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_batches", "/batches/xyz");
  });

  it("saves last_route_tools when navigating to /tools/carbonation", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/tools/carbonation", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_tools", "/tools/carbonation");
  });

  it("saves last_route_equipment when navigating to /equipment", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/equipment", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_equipment", "/equipment");
  });

  it("saves last_route_library when navigating to /library", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/library", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_library", "/library");
  });

  it("saves last_route_settings when navigating to /settings", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/settings", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_settings", "/settings");
  });

  it("includes query string in saved URL", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/recipe/abc", search: "?tab=mash" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route", "/recipe/abc?tab=mash");
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_recipes", "/recipe/abc?tab=mash");
  });

  it("saves last_route_recipes when navigating to /baseline-recipe/abc", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/baseline-recipe/abc", search: "" } } });
    // baseline-recipe belongs to the recipes section
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_recipes", "/baseline-recipe/abc");
  });

  it("does not save a section key for unrecognised paths", () => {
    render(AppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/admin", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledTimes(1);
    expect(saveSettingMock).toHaveBeenCalledWith("last_route", "/admin");
  });
});
