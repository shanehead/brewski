import type { GravityConversionResult, GravityUnit } from "$lib/api";

// SG -> Plato (Brewer's cubic, same polynomial as the Rust `og_to_plato`).
function sgToPlato(sg: number): number {
  return -616.868 + 1111.14 * sg - 630.272 * sg * sg + 135.997 * sg * sg * sg;
}

// SG -> Brix (same polynomial as the Rust `sg_to_brix`).
function sgToBrix(sg: number): number {
  return ((182.4601 * sg - 775.6821) * sg + 1262.7794) * sg - 669.5622;
}

// Convert a specific gravity into all three display units. Pure math, so display
// can be a synchronous $derived rather than an async IPC round-trip. The Rust
// `convert_gravity` command remains the source of truth for parsing user input
// back to SG for storage.
export function convertSg(sg: number): GravityConversionResult {
  return { sg, plato: sgToPlato(sg), brix: sgToBrix(sg) };
}

export function formatGravity(result: GravityConversionResult, unit: GravityUnit): string {
  switch (unit) {
    case "plato": return result.plato.toFixed(1) + "°P";
    case "brix":  return result.brix.toFixed(1) + "°Bx";
    default:      return result.sg.toFixed(3);
  }
}

// Convenience for display call sites: convert an SG value and format it in one step.
export function formatSg(sg: number, unit: GravityUnit): string {
  return formatGravity(convertSg(sg), unit);
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
