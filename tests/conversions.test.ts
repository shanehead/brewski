import { describe, expect, it } from "vitest";
import {
  calculateAbvCalories,
  correctHydrometerTemp,
  calculateRefractometer,
  correctRefractometerFg,
  calculatePrimingSugar,
  calculateCo2Pressure,
  convertGravity,
  calculatePitchRate,
  convertColor,
} from "$lib/conversions";

describe("calculateAbvCalories", () => {
  it("calculates ~5.1% ABV for OG 1.052 / FG 1.013", () => {
    const result = calculateAbvCalories(1.052, 1.013);
    expect(result.abvPct).toBeCloseTo(5.1, 1);
  });

  it("calculates attenuation as percentage of fermentable extract", () => {
    const result = calculateAbvCalories(1.052, 1.013);
    expect(result.attenuationPct).toBeCloseTo(75.0, 0);
  });

  it("calculates ~150 kcal per 355ml for a standard ale", () => {
    const result = calculateAbvCalories(1.052, 1.013);
    expect(result.caloriesPer355ml).toBeGreaterThan(130);
    expect(result.caloriesPer355ml).toBeLessThan(175);
  });

  it("returns zero attenuation when OG is 1.000", () => {
    const result = calculateAbvCalories(1.0, 1.0);
    expect(result.attenuationPct).toBe(0);
  });
});

describe("correctHydrometerTemp", () => {
  it("corrects upward when sample is warmer than calibration", () => {
    const corrected = correctHydrometerTemp(1.050, 30.0, 20.0);
    expect(corrected).toBeGreaterThan(1.050);
    expect(corrected).toBeCloseTo(1.052, 2);
  });

  it("returns identity when measured temp equals calibration temp", () => {
    const corrected = correctHydrometerTemp(1.050, 20.0, 20.0);
    expect(corrected).toBeCloseTo(1.050, 4);
  });
});

describe("calculateRefractometer", () => {
  it("converts 12 Brix with WCF 1.04 to a reasonable SG", () => {
    const result = calculateRefractometer(12.0, 1.04);
    expect(result.sg).toBeGreaterThan(1.045);
    expect(result.sg).toBeLessThan(1.055);
  });
});

describe("correctRefractometerFg", () => {
  it("corrects post-fermentation FG to below the OG", () => {
    const result = correctRefractometerFg(12.0, 6.0, 1.04);
    expect(result.fgSg).toBeGreaterThan(1.000);
    expect(result.fgSg).toBeLessThan(1.020);
  });
});

describe("calculatePrimingSugar", () => {
  it("returns more grams for DME than table sugar", () => {
    const tableSugar = calculatePrimingSugar(2.4, 19.0, 20.0, "table_sugar");
    const dme = calculatePrimingSugar(2.4, 19.0, 20.0, "dry_malt_extract");
    expect(tableSugar).toBeGreaterThan(0);
    expect(dme).toBeGreaterThan(tableSugar);
  });

  it("returns more grams for corn sugar than table sugar", () => {
    const tableSugar = calculatePrimingSugar(2.4, 19.0, 20.0, "table_sugar");
    const cornSugar = calculatePrimingSugar(2.4, 19.0, 20.0, "corn_sugar");
    expect(cornSugar).toBeGreaterThan(tableSugar);
  });
});

describe("calculateCo2Pressure", () => {
  it("requires more pressure at warmer temperatures", () => {
    const cold = calculateCo2Pressure(2.4, 4.0);
    const warm = calculateCo2Pressure(2.4, 20.0);
    expect(warm).toBeGreaterThan(cold);
  });
});

describe("convertGravity", () => {
  it("converts from SG and round-trips back", () => {
    const result = convertGravity(1.050, "sg");
    expect(result.sg).toBeCloseTo(1.050, 4);
    const back = convertGravity(result.plato, "plato");
    expect(back.sg).toBeCloseTo(1.050, 2);
  });

  it("converts from Brix", () => {
    const result = convertGravity(12.0, "brix");
    expect(result.sg).toBeGreaterThan(1.040);
    expect(result.sg).toBeLessThan(1.060);
  });

  it("converts from Plato", () => {
    const result = convertGravity(12.0, "plato");
    expect(result.sg).toBeGreaterThan(1.040);
    expect(result.sg).toBeLessThan(1.060);
  });

  it("throws on unknown unit", () => {
    expect(() => convertGravity(1.050, "oz" as any)).toThrow();
  });
});

describe("calculatePitchRate", () => {
  it("returns a reasonable cell count for a standard ale", () => {
    const result = calculatePitchRate(1.050, 20.0, 0.75, 100.0, 100.0);
    expect(result.requiredCells).toBeGreaterThan(150);
    expect(result.requiredCells).toBeLessThan(250);
  });

  it("requires no starter when pack has enough cells", () => {
    const result = calculatePitchRate(1.050, 20.0, 0.75, 300.0, 100.0);
    expect(result.starterVolumeL).toBe(0);
  });

  it("requires a starter when pack is short", () => {
    const result = calculatePitchRate(1.050, 20.0, 0.75, 100.0, 75.0);
    expect(result.starterVolumeL).toBeGreaterThan(0);
  });
});

describe("convertColor", () => {
  it("round-trips SRM → EBC → SRM", () => {
    const result = convertColor(10.0, "srm");
    const back = convertColor(result.ebc, "ebc");
    expect(back.srm).toBeCloseTo(10.0, 3);
  });

  it("round-trips SRM → Lovibond → SRM", () => {
    const result = convertColor(12.0, "srm");
    const back = convertColor(result.lovibond, "lovibond");
    expect(back.srm).toBeCloseTo(12.0, 3);
  });

  it("throws on unknown unit", () => {
    expect(() => convertColor(10.0, "kelvin" as any)).toThrow();
  });
});
