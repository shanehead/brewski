import { describe, expect, it } from "vitest";
import {
  kgToLb, lbToKg,
  kgToHopDisplay, hopDisplayToKg,
  lToGal, galToL,
  cToF, fToC,
  weightLabel, hopWeightLabel, volumeLabel, tempLabel,
  lPerKgToQtPerLb, qtPerLbToLPerKg, ratioLabel,
} from "$lib/units";

describe("fermentable weight: kg ↔ lb", () => {
  it("converts 1 kg to lb", () => { expect(kgToLb(1)).toBeCloseTo(2.20462, 4); });
  it("converts 1 lb to kg", () => { expect(lbToKg(1)).toBeCloseTo(0.453592, 4); });
  it("round-trips", () => { expect(lbToKg(kgToLb(5))).toBeCloseTo(5, 4); });
  it("zero stays zero", () => { expect(kgToLb(0)).toBe(0); expect(lbToKg(0)).toBe(0); });
});

describe("hop weight: kg ↔ g (metric) / oz (imperial)", () => {
  it("converts kg to grams in metric", () => {
    expect(kgToHopDisplay(0.028, "metric")).toBeCloseTo(28, 1);
    expect(kgToHopDisplay(0.1, "metric")).toBeCloseTo(100, 1);
  });
  it("converts kg to oz in imperial", () => {
    expect(kgToHopDisplay(1 / 35.274, "imperial")).toBeCloseTo(1, 4);
    expect(kgToHopDisplay(0.028, "imperial")).toBeCloseTo(0.988, 2);
  });
  it("round-trips in metric", () => {
    expect(hopDisplayToKg(kgToHopDisplay(0.057, "metric"), "metric")).toBeCloseTo(0.057, 4);
  });
  it("round-trips in imperial", () => {
    expect(hopDisplayToKg(kgToHopDisplay(0.057, "imperial"), "imperial")).toBeCloseTo(0.057, 4);
  });
});

describe("volume: L ↔ gal", () => {
  it("converts 1 gal to L", () => { expect(galToL(1)).toBeCloseTo(3.78541, 4); });
  it("converts 1 L to gal", () => { expect(lToGal(1)).toBeCloseTo(0.264172, 4); });
  it("converts a typical batch (23 L ≈ 6.06 gal)", () => { expect(lToGal(23)).toBeCloseTo(6.076, 2); });
  it("round-trips", () => { expect(galToL(lToGal(23))).toBeCloseTo(23, 4); });
});

describe("temperature: °C ↔ °F", () => {
  it("converts 0 °C → 32 °F", () => { expect(cToF(0)).toBeCloseTo(32, 4); });
  it("converts 100 °C → 212 °F", () => { expect(cToF(100)).toBeCloseTo(212, 4); });
  it("converts typical mash temp (67 °C → 152.6 °F)", () => { expect(cToF(67)).toBeCloseTo(152.6, 1); });
  it("converts typical sparge temp (75 °C → 167 °F)", () => { expect(cToF(75)).toBeCloseTo(167, 1); });
  it("converges at -40 (same in both scales)", () => { expect(cToF(-40)).toBeCloseTo(-40, 4); });
  it("round-trips", () => { expect(fToC(cToF(67))).toBeCloseTo(67, 4); });
});

describe("water:grain ratio: L/kg ↔ qt/lb", () => {
  it("converts 3.0 L/kg to qt/lb", () => { expect(lPerKgToQtPerLb(3.0)).toBeCloseTo(1.438, 2); });
  it("converts 1.5 qt/lb to L/kg", () => { expect(qtPerLbToLPerKg(1.5)).toBeCloseTo(3.130, 2); });
  it("round-trips", () => { expect(qtPerLbToLPerKg(lPerKgToQtPerLb(3.0))).toBeCloseTo(3.0, 4); });
});

describe("label helpers", () => {
  it("weightLabel", () => {
    expect(weightLabel("metric")).toBe("kg");
    expect(weightLabel("imperial")).toBe("lb");
  });
  it("hopWeightLabel", () => {
    expect(hopWeightLabel("metric")).toBe("g");
    expect(hopWeightLabel("imperial")).toBe("oz");
  });
  it("volumeLabel", () => {
    expect(volumeLabel("metric")).toBe("L");
    expect(volumeLabel("imperial")).toBe("gal");
  });
  it("tempLabel", () => {
    expect(tempLabel("metric")).toBe("°C");
    expect(tempLabel("imperial")).toBe("°F");
  });
  it("ratioLabel", () => {
    expect(ratioLabel("metric")).toBe("L/kg");
    expect(ratioLabel("imperial")).toBe("qt/lb");
  });
});
