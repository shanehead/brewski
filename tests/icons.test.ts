import { describe, expect, it } from "vitest";
import { ICONS, type BrewingIconName } from "$lib/icons";

const ALL_NAMES: BrewingIconName[] = [
  "fermentable",
  "hop",
  "yeast",
  "overview",
  "ingredients",
  "mash",
  "fermentation",
  "notes",
];

describe("ICONS registry", () => {
  it("has an entry for every BrewingIconName", () => {
    for (const name of ALL_NAMES) {
      expect(ICONS[name], `missing icon for "${name}"`).toBeDefined();
    }
  });

  it("all entries are non-empty strings", () => {
    for (const name of ALL_NAMES) {
      expect(typeof ICONS[name]).toBe("string");
      expect(ICONS[name].length).toBeGreaterThan(0);
    }
  });
});
