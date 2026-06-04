import type { SugarType, GravityUnit, ColorUnit, AbvCaloriesResult, RefractometerResult, RefractometerFgResult, GravityConversionResult, PitchRateResult, ColorConversionResult } from "$lib/api";

function ogToPlato(sg: number): number {
  return -616.868 + 1111.14 * sg - 630.272 * sg * sg + 135.997 * sg * sg * sg;
}

function hydroCorrectionFactor(tempF: number): number {
  return (
    1.00130346 -
    0.000134722124 * tempF +
    0.00000204052596 * tempF ** 2 -
    0.00000000232820948 * tempF ** 3
  );
}

function residualCo2Vols(tempC: number): number {
  const tempF = (tempC * 9) / 5 + 32;
  return 3.0378 - 0.050062 * tempF + 0.00026555 * tempF ** 2;
}

const SUGAR_GRAMS_PER_LITER_PER_VOL: Record<SugarType, number> = {
  table_sugar: 4.01,
  corn_sugar: 4.22,
  dry_malt_extract: 5.07,
};

export function calculateAbvCalories(og: number, fg: number): AbvCaloriesResult {
  const abvPct = (og - fg) * 131.25;
  const attenuationPct = og > 1.0 ? ((og - fg) / (og - 1.0)) * 100 : 0;
  const alcoholByWeight = (og - fg) * 105;
  const realExtract = 0.1808 * ogToPlato(og) + 0.8192 * ogToPlato(fg);
  const caloriesPerMl = (6.9 * alcoholByWeight + 4.0 * (realExtract - 0.1)) * fg * 10 / 1000;
  return { abvPct, attenuationPct, caloriesPer355ml: caloriesPerMl * 355 };
}

export function correctHydrometerTemp(
  measuredSg: number,
  measuredTempC: number,
  calibrationTempC: number,
): number {
  const measuredTempF = (measuredTempC * 9) / 5 + 32;
  const calibrationTempF = (calibrationTempC * 9) / 5 + 32;
  return measuredSg * (hydroCorrectionFactor(measuredTempF) / hydroCorrectionFactor(calibrationTempF));
}

export function calculateRefractometer(brix: number, wortCorrectionFactor: number): RefractometerResult {
  const adjusted = brix * wortCorrectionFactor;
  return { sg: 1.0 + adjusted / (258.6 - (adjusted / 258.2) * 227.1) };
}

export function correctRefractometerFg(
  ogBrix: number,
  fgBrix: number,
  wortCorrectionFactor: number,
): RefractometerFgResult {
  const correctedOg = ogBrix * wortCorrectionFactor;
  const correctedFg = fgBrix * wortCorrectionFactor;
  return { fgSg: 1.0 - 0.00085683 * correctedOg + 0.0034941 * correctedFg };
}

export function calculatePrimingSugar(
  targetVols: number,
  batchSizeL: number,
  tempC: number,
  sugarType: SugarType,
): number {
  const additionalVols = Math.max(targetVols - residualCo2Vols(tempC), 0);
  return additionalVols * batchSizeL * SUGAR_GRAMS_PER_LITER_PER_VOL[sugarType];
}

export function calculateCo2Pressure(targetVols: number, tempC: number): number {
  const tempF = (tempC * 9) / 5 + 32;
  const psi =
    -16.6999 -
    0.0101059 * tempF +
    0.00116512 * tempF ** 2 +
    0.173354 * tempF * targetVols +
    4.24267 * targetVols -
    0.0684226 * targetVols ** 2;
  return Math.max(psi, 0) * 6.89476;
}

export function convertGravity(value: number, fromUnit: GravityUnit): GravityConversionResult {
  let sg: number;
  if (fromUnit === "sg") {
    sg = value;
  } else if (fromUnit === "plato") {
    sg = 1.0 + value / (258.6 - (value / 258.2) * 227.1);
  } else if (fromUnit === "brix") {
    sg = 1.0 + value / (258.6 - (value / 258.2) * 227.1);
  } else {
    throw new Error(`unknown gravity unit: ${fromUnit}`);
  }
  const plato = ogToPlato(sg);
  const brix = ((182.4601 * sg - 775.6821) * sg + 1262.7794) * sg - 669.5622;
  return { sg, plato, brix };
}

export function calculatePitchRate(
  og: number,
  batchSizeL: number,
  pitchRate: number,
  yeastPackCells: number,
  viabilityPct: number,
): PitchRateResult {
  const plato = ogToPlato(og);
  const requiredCells = (pitchRate * batchSizeL * 1000 * plato) / 1000;
  const availableCells = yeastPackCells * (viabilityPct / 100);
  const starterVolumeL = Math.max(requiredCells - availableCells, 0) / 100;
  return { requiredCells, starterVolumeL };
}

export function convertColor(value: number, fromUnit: ColorUnit): ColorConversionResult {
  let srm: number;
  if (fromUnit === "srm") {
    srm = value;
  } else if (fromUnit === "ebc") {
    srm = value / 1.97;
  } else if (fromUnit === "lovibond") {
    srm = 1.3546 * value - 0.76;
  } else {
    throw new Error(`unknown color unit: ${fromUnit}`);
  }
  return { srm, ebc: srm * 1.97, lovibond: (srm + 0.76) / 1.3546 };
}
