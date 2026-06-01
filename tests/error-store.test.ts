import { afterEach, describe, expect, it, vi } from "vitest";
import { get } from "svelte/store";
import { lastSuccess, setSuccess } from "$lib/stores/error";

describe("setSuccess", () => {
  afterEach(() => {
    vi.useRealTimers();
    lastSuccess.set(null);
  });

  it("sets lastSuccess to the provided message", () => {
    setSuccess("2 recipes imported");
    expect(get(lastSuccess)).toBe("2 recipes imported");
  });

  it("auto-clears lastSuccess after 3 seconds", () => {
    vi.useFakeTimers();
    setSuccess("1 recipe imported");
    expect(get(lastSuccess)).toBe("1 recipe imported");
    vi.advanceTimersByTime(3000);
    expect(get(lastSuccess)).toBeNull();
  });

  it("does not clear before 3 seconds", () => {
    vi.useFakeTimers();
    setSuccess("1 recipe imported");
    vi.advanceTimersByTime(2999);
    expect(get(lastSuccess)).toBe("1 recipe imported");
  });
});
