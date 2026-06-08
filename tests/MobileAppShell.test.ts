import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import MobileAppShell from "../src/lib/mobile/AppShell.svelte";

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

// BottomTabBar uses page store and BrewingIcon — stub it out
vi.mock("../src/lib/mobile/BottomTabBar.svelte", () => ({
  default: vi.fn(),
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

describe("MobileAppShell last route", () => {
  it("calls goto with last_route when it differs from current path", async () => {
    mockSettings = { last_route: "/library" };
    mockPathname = "/";
    render(MobileAppShell, { children: () => null });
    await new Promise((r) => setTimeout(r, 0));
    expect(gotoMock).toHaveBeenCalledWith("/library");
  });

  it("does not call goto when last_route matches current path", async () => {
    mockSettings = { last_route: "/library" };
    mockPathname = "/library";
    render(MobileAppShell, { children: () => null });
    await new Promise((r) => setTimeout(r, 0));
    expect(gotoMock).not.toHaveBeenCalled();
  });

  it("does not call goto when last_route is absent", async () => {
    mockSettings = {};
    render(MobileAppShell, { children: () => null });
    await new Promise((r) => setTimeout(r, 0));
    expect(gotoMock).not.toHaveBeenCalled();
  });

  it("calls saveSetting with the navigated-to pathname and search", () => {
    render(MobileAppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/tools/carbonation", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route", "/tools/carbonation");
  });

  it("does not call saveSetting when navigation.to is null", () => {
    render(MobileAppShell, { children: () => null });
    afterNavigateCb!({ to: null });
    expect(saveSettingMock).not.toHaveBeenCalled();
  });
});

describe("MobileAppShell section key saving", () => {
  it("saves last_route_recipes when navigating to /recipe/abc", () => {
    render(MobileAppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/recipe/abc", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_recipes", "/recipe/abc");
  });

  it("saves last_route_recipes when navigating to /", () => {
    render(MobileAppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_recipes", "/");
  });

  it("saves last_route_batches when navigating to /batches/xyz", () => {
    render(MobileAppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/batches/xyz", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_batches", "/batches/xyz");
  });

  it("saves last_route_tools when navigating to /tools/carbonation", () => {
    render(MobileAppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/tools/carbonation", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_tools", "/tools/carbonation");
  });

  it("saves last_route_equipment when navigating to /equipment", () => {
    render(MobileAppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/equipment", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_equipment", "/equipment");
  });

  it("saves last_route_library when navigating to /library", () => {
    render(MobileAppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/library", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_library", "/library");
  });

  it("saves last_route_settings when navigating to /settings", () => {
    render(MobileAppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/settings", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_settings", "/settings");
  });

  it("includes query string in saved URL", () => {
    render(MobileAppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/recipe/abc", search: "?tab=mash" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route", "/recipe/abc?tab=mash");
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_recipes", "/recipe/abc?tab=mash");
  });

  it("saves last_route_recipes when navigating to /baseline-recipe/abc", () => {
    render(MobileAppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/baseline-recipe/abc", search: "" } } });
    // baseline-recipe belongs to the recipes section
    expect(saveSettingMock).toHaveBeenCalledWith("last_route_recipes", "/baseline-recipe/abc");
  });

  it("does not save a section key for unrecognised paths", () => {
    render(MobileAppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/admin", search: "" } } });
    expect(saveSettingMock).toHaveBeenCalledTimes(1);
    expect(saveSettingMock).toHaveBeenCalledWith("last_route", "/admin");
  });
});

describe("MobileAppShell success banner", () => {
  it("shows success banner when lastSuccess has a message", () => {
    mockLastSuccess = "1 recipe imported";
    const { getByText } = render(MobileAppShell, { children: () => null });
    expect(getByText("1 recipe imported")).toBeTruthy();
  });

  it("does not show success banner when lastSuccess is null", () => {
    mockLastSuccess = null;
    const { queryByText } = render(MobileAppShell, { children: () => null });
    expect(queryByText(/recipe imported/)).toBeNull();
  });
});
