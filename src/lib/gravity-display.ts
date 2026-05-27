import type { GravityConversionResult, GravityUnit } from "$lib/api";

export function formatGravity(result: GravityConversionResult, unit: GravityUnit): string {
  switch (unit) {
    case "plato": return result.plato.toFixed(1) + "°P";
    case "brix":  return result.brix.toFixed(1) + "°Bx";
    default:      return result.sg.toFixed(3);
  }
}

export function gravityStep(unit: GravityUnit): string {
  return unit === "sg" ? "0.001" : "0.1";
}

export function gravityPlaceholder(unit: GravityUnit): string {
  switch (unit) {
    case "plato": return "Gravity (°P)";
    case "brix":  return "Gravity (°Bx)";
    default:      return "Gravity (e.g. 1.058)";
  }
}
