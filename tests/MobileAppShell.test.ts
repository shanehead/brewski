import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import MobileAppShell from "../src/lib/mobile/AppShell.svelte";

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

  it("calls saveSetting with the navigated-to pathname", () => {
    render(MobileAppShell, { children: () => null });
    afterNavigateCb!({ to: { url: { pathname: "/tools/carbonation" } } });
    expect(saveSettingMock).toHaveBeenCalledWith("last_route", "/tools/carbonation");
  });

  it("does not call saveSetting when navigation.to is null", () => {
    render(MobileAppShell, { children: () => null });
    afterNavigateCb!({ to: null });
    expect(saveSettingMock).not.toHaveBeenCalled();
  });
});
