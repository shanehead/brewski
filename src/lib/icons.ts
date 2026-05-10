export type BrewingIconName =
  | "fermentable"
  | "hop"
  | "yeast"
  | "overview"
  | "ingredients"
  | "mash"
  | "fermentation"
  | "notes";

export const ICONS: Record<BrewingIconName, string> = {
  fermentable: "🌾",
  hop: "🍃",
  yeast: "🧫",
  overview: "📋",
  ingredients: "🛒",
  mash: "🌡️",
  fermentation: "🍺",
  notes: "✏️",
};
