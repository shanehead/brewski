// Brewski sample data — recipes, batches, brewing tools.
// Used by the UI kit to make the click-thru feel populated.

window.BREWSKI_RECIPES = [
  {
    id: "r1",
    name: "West Coast IPA",
    style: "American IPA",
    type: "all_grain",
    brewer: "Shane",
    date: "2025-11-12",
    batch_size_gal: 5.5,
    boil_size_gal: 7.0,
    boil_time_min: 60,
    efficiency_pct: 75,
    notes: "Bright and bitter — Citra + Simcoe + Mosaic, late-boil and dry-hop.",
    og: 1.062, fg: 1.012, abv: 6.6, ibu: 64, srm: 5.8, bu_gu: 1.03,
    cal_per_12oz: 198,
    pre_boil_vol: 6.5, post_boil_vol: 5.8, pre_boil_g: 1.054,
  },
  {
    id: "r2",
    name: "Maris Pale",
    style: "English IPA",
    type: "all_grain",
    brewer: "Shane",
    date: "2025-10-04",
    batch_size_gal: 5.0,
    boil_size_gal: 6.5,
    boil_time_min: 75,
    efficiency_pct: 78,
    notes: "Maris Otter + East Kent Goldings, classic English session strength.",
    og: 1.048, fg: 1.011, abv: 4.9, ibu: 38, srm: 7.2, bu_gu: 0.79,
    cal_per_12oz: 148,
    pre_boil_vol: 6.1, post_boil_vol: 5.2, pre_boil_g: 1.042,
  },
  {
    id: "r3",
    name: "Saison Belge",
    style: "Saison",
    type: "all_grain",
    brewer: "Shane",
    date: "2025-09-22",
    batch_size_gal: 5.0,
    boil_size_gal: 6.5,
    boil_time_min: 90,
    efficiency_pct: 72,
    notes: "Open-fermented with Wyeast 3711, peppery and dry.",
    og: 1.054, fg: 1.004, abv: 6.6, ibu: 26, srm: 4.2, bu_gu: 0.48,
    cal_per_12oz: 178,
    pre_boil_vol: 6.2, post_boil_vol: 5.2, pre_boil_g: 1.046,
  },
  {
    id: "r4",
    name: "Oatmeal Stout",
    style: "Sweet Stout",
    type: "all_grain",
    brewer: "Shane",
    date: "2025-08-15",
    batch_size_gal: 5.0,
    boil_size_gal: 6.5,
    boil_time_min: 60,
    efficiency_pct: 74,
    notes: "Flaked oats + chocolate malt, dark and round.",
    og: 1.054, fg: 1.014, abv: 5.2, ibu: 32, srm: 32.5, bu_gu: 0.59,
    cal_per_12oz: 188,
    pre_boil_vol: 6.3, post_boil_vol: 5.3, pre_boil_g: 1.046,
  },
  {
    id: "r5",
    name: "Helles Lager",
    style: "Munich Helles",
    type: "all_grain",
    brewer: "Shane",
    date: null,
    batch_size_gal: 5.0,
    boil_size_gal: 6.7,
    boil_time_min: 90,
    efficiency_pct: 76,
    notes: "Pilsner malt single, decoction-style mash. Lager for 6 weeks.",
    og: 1.048, fg: 1.010, abv: 5.0, ibu: 20, srm: 3.5, bu_gu: 0.42,
    cal_per_12oz: 156,
    pre_boil_vol: 6.3, post_boil_vol: 5.3, pre_boil_g: 1.041,
  },
];

window.BREWSKI_BATCHES = [
  { id: "b1", recipe_name: "West Coast IPA", name: "Batch #12", brew_date: "2025-11-14", status: "fermenting", actual_og: 1.061 },
  { id: "b2", recipe_name: "Maris Pale",      name: "Batch #11", brew_date: "2025-10-06", status: "packaged",   actual_og: 1.048 },
  { id: "b3", recipe_name: "Oatmeal Stout",   name: "Batch #10", brew_date: "2025-09-22", status: "complete",   actual_og: 1.054 },
  { id: "b4", recipe_name: "Saison Belge",    name: "Batch #9",  brew_date: "2025-09-12", status: "complete",   actual_og: 1.052 },
];

window.BREWSKI_TOOLS = [
  { slug: "abv",           name: "ABV / Calories",            desc: "Estimate beer strength and calories from gravity readings." },
  { slug: "carbonation",   name: "Carbonation Calculator",     desc: "Priming sugar or CO₂ pressure for a target carbonation level." },
  { slug: "color",         name: "Color Conversion",           desc: "Convert between SRM, EBC and Lovibond color units." },
  { slug: "gravity",       name: "Gravity Conversions",        desc: "Between Plato, Brix, and Specific Gravity." },
  { slug: "hydrometer",    name: "Hydrometer Temp Correction", desc: "Correct a hydrometer reading for temperature." },
  { slug: "pitch",         name: "Pitch Rate Calculator",      desc: "Yeast cells needed for a target pitch rate." },
  { slug: "refractometer", name: "Refractometer Correction",   desc: "Final-gravity correction from refractometer readings." },
  { slug: "units",         name: "Unit Conversions",           desc: "Volume, mass, and temperature." },
];

window.STATUS_LABELS = {
  planned: "Planned",
  brewing: "Brewing",
  fermenting: "Fermenting",
  packaged: "Packaged",
  complete: "Complete",
};

window.STATUS_COLORS = {
  planned:    "#8a8aa0",
  brewing:    "#f59e0b",
  fermenting: "#10b981",
  packaged:   "#3b82f6",
  complete:   "var(--color-accent)",
};

window.THEMES = [
  { id: "midnight",         name: "Midnight",         scheme: "dark",  accent: "#5c5cff" },
  { id: "tokyo-night",      name: "Tokyo Night",      scheme: "dark",  accent: "#7aa2f7" },
  { id: "dracula",          name: "Dracula",          scheme: "dark",  accent: "#bd93f9" },
  { id: "catppuccin",       name: "Catppuccin",       scheme: "dark",  accent: "#cba6f7" },
  { id: "nord",             name: "Nord",             scheme: "dark",  accent: "#88c0d0" },
  { id: "monokai",          name: "Monokai",          scheme: "dark",  accent: "#a6e22e" },
  { id: "catppuccin-latte", name: "Catppuccin Latte", scheme: "light", accent: "#8839ef" },
  { id: "solarized-light",  name: "Solarized Light",  scheme: "light", accent: "#268bd2" },
  { id: "ayu-light",        name: "Ayu Light",        scheme: "light", accent: "#ff9940" },
  { id: "github-light",     name: "GitHub Light",     scheme: "light", accent: "#0969da" },
];

// SRM stops (Brewski uses these inline in StatsSidebar)
window.SRM_STOPS = [
  [1,"#FFE699"],[2,"#FFD878"],[3,"#FFCA5A"],[4,"#FFBF42"],
  [6,"#FBB123"],[8,"#F8A600"],[10,"#F39C00"],[13,"#EA8F00"],
  [17,"#D77200"],[20,"#CF6900"],[24,"#BB5100"],[29,"#A13600"],
  [35,"#8D1D00"],[40,"#611200"],
];
window.srmToHex = function(srm) {
  const stops = window.SRM_STOPS;
  const v = Math.min(Math.max(srm, 1), 40);
  for (let i = stops.length - 1; i >= 0; i--) {
    if (v >= stops[i][0]) return stops[i][1];
  }
  return stops[0][1];
};
