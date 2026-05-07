export type Units = "metric" | "imperial";

export function kgToLb(kg: number): number { return kg * 2.20462; }
export function lbToKg(lb: number): number { return lb * 0.453592; }

export function kgToHopDisplay(kg: number, units: Units): number {
  return units === "imperial" ? kg * 35.274 : kg * 1000;
}
export function hopDisplayToKg(value: number, units: Units): number {
  return units === "imperial" ? value / 35.274 : value / 1000;
}

export function lToGal(l: number): number { return l * 0.264172; }
export function galToL(gal: number): number { return gal * 3.78541; }

export function cToF(c: number): number { return (c * 9) / 5 + 32; }
export function fToC(f: number): number { return ((f - 32) * 5) / 9; }

export function weightLabel(units: Units): string { return units === "imperial" ? "lb" : "kg"; }
export function hopWeightLabel(units: Units): string { return units === "imperial" ? "oz" : "g"; }
export function volumeLabel(units: Units): string { return units === "imperial" ? "gal" : "L"; }
export function tempLabel(units: Units): string { return units === "imperial" ? "°F" : "°C"; }
