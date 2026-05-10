export interface ToolDefinition {
  slug: string;
  name: string;
  description: string;
}

export const TOOLS: ToolDefinition[] = [
  {
    slug: "abv-calories",
    name: "ABV / Attenuation / Calories",
    description: "Estimate strength, attenuation, and calories from OG and FG.",
  },
  {
    slug: "hydrometer-temp",
    name: "Hydrometer Temperature Correction",
    description: "Correct a gravity reading for sample temperature and calibration.",
  },
  {
    slug: "refractometer",
    name: "Refractometer / Brix",
    description: "Convert Brix to SG and correct FG readings after fermentation.",
  },
  {
    slug: "carbonation",
    name: "Carbonation",
    description: "Calculate priming sugar or serving pressure for a target CO2 level.",
  },
  {
    slug: "gravity-conversions",
    name: "Gravity Conversions",
    description: "Convert between specific gravity, Plato, and Brix.",
  },
  {
    slug: "unit-conversions",
    name: "Unit Conversions",
    description: "Convert common brewing volumes, weights, and temperatures.",
  },
  {
    slug: "pitch-rate",
    name: "Yeast Pitch Rate",
    description: "Estimate required cells and starter size from wort gravity and batch size.",
  },
  {
    slug: "color-conversion",
    name: "Color Conversion",
    description: "Convert between SRM, EBC, and Lovibond.",
  },
];
