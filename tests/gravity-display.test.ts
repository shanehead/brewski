import { describe, it, expect } from "vitest";
import { formatGravity, formatSg, convertSg, gravityStep, gravityPlaceholder } from "$lib/gravity-display";

const result = { sg: 1.054, plato: 13.3, brix: 13.5 };

describe("convertSg", () => {
  it("passes the SG value through unchanged", () => {
    expect(convertSg(1.05).sg).toBe(1.05);
  });
  it("derives Plato from SG (matches the Rust polynomial)", () => {
    expect(convertSg(1.05).plato).toBeCloseTo(12.4, 1);
  });
  it("derives Brix from SG (matches the Rust polynomial)", () => {
    expect(convertSg(1.05).brix).toBeCloseTo(12.4, 1);
  });
});

describe("formatSg", () => {
  it("formats SG to 3 decimal places with no suffix", () => {
    expect(formatSg(1.054, "sg")).toBe("1.054");
  });
  it("formats SG as Plato to 1 decimal place with °P suffix", () => {
    expect(formatSg(1.05, "plato")).toBe("12.4°P");
  });
  it("formats SG as Brix to 1 decimal place with °Bx suffix", () => {
    expect(formatSg(1.05, "brix")).toBe("12.4°Bx");
  });
});

describe("formatGravity", () => {
  it("formats SG to 3 decimal places with no suffix", () => {
    expect(formatGravity(result, "sg")).toBe("1.054");
  });
  it("formats Plato to 1 decimal place with °P suffix", () => {
    expect(formatGravity(result, "plato")).toBe("13.3°P");
  });
  it("formats Brix to 1 decimal place with °Bx suffix", () => {
    expect(formatGravity(result, "brix")).toBe("13.5°Bx");
  });
});

describe("gravityStep", () => {
  it("returns 0.001 for sg", () => { expect(gravityStep("sg")).toBe("0.001"); });
  it("returns 0.1 for plato", () => { expect(gravityStep("plato")).toBe("0.1"); });
  it("returns 0.1 for brix", () => { expect(gravityStep("brix")).toBe("0.1"); });
});

describe("gravityPlaceholder", () => {
  it("returns SG example for sg", () => {
    expect(gravityPlaceholder("sg")).toBe("Gravity (e.g. 1.058)");
  });
  it("returns °P for plato", () => {
    expect(gravityPlaceholder("plato")).toBe("Gravity (°P)");
  });
  it("returns °Bx for brix", () => {
    expect(gravityPlaceholder("brix")).toBe("Gravity (°Bx)");
  });
});
