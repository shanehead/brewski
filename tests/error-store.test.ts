import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { get } from "svelte/store";
import { lastSuccess, setSuccess } from "$lib/stores/error";

describe("setSuccess", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
    lastSuccess.set(null);
  });

  it("sets lastSuccess to the provided message", () => {
    setSuccess("2 recipes imported");
    expect(get(lastSuccess)).toBe("2 recipes imported");
  });

  it("auto-clears lastSuccess after 3 seconds", () => {
    setSuccess("1 recipe imported");
    expect(get(lastSuccess)).toBe("1 recipe imported");
    vi.advanceTimersByTime(3000);
    expect(get(lastSuccess)).toBeNull();
  });

  it("does not clear before 3 seconds", () => {
    setSuccess("1 recipe imported");
    vi.advanceTimersByTime(2999);
    expect(get(lastSuccess)).toBe("1 recipe imported");
  });

  it("cancels previous timer when called again before 3 seconds", () => {
    setSuccess("first");
    vi.advanceTimersByTime(1000);
    setSuccess("second");
    vi.advanceTimersByTime(2999);
    // first timer would have fired at t=3000 (1000ms after "second"), but clearTimeout stops it
    expect(get(lastSuccess)).toBe("second");
    vi.advanceTimersByTime(1);
    expect(get(lastSuccess)).toBeNull();
  });
});
