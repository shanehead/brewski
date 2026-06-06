#!/usr/bin/env node
/**
 * Capture README screenshots using Playwright against the dev-web server.
 *
 * Usage:
 *   just dev-web &          # start frontend server on :1420
 *   bun scripts/capture-screenshots.mjs
 *
 * Requires playwright: bun add -d playwright
 * On first run: bunx playwright install chromium
 *
 * Output: docs/screenshots/*.png
 */

import { chromium } from "playwright";
import { fileURLToPath } from "url";
import path from "path";

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const OUT = path.join(ROOT, "docs", "screenshots");
const DOCS_OUT = path.join(ROOT, "docs", "site", "public", "screenshots");
const BASE = "http://localhost:1420";

// Minimal Tauri IPC mock so invoke() succeeds with canned data.
// Add more commands here as new tools/pages are screenshot-worthy.
const TAURI_MOCK = `
  window.__TAURI_INTERNALS__ = {
    invoke: async (cmd, args) => {
      if (cmd === 'calculate_abv_calories')
        return { abvPct: 6.3, attenuationPct: 80.0, caloriesPer355ml: 193 };
      if (cmd === 'calculate_carbonation')
        return { primingSugarG: 142, kegPressureKpa: 103 };
      if (cmd === 'correct_hydrometer_temp') return 1.052;
      if (cmd === 'calculate_refractometer') return { sg: 1.012, fg: 1.010 };
      if (cmd === 'calculate_pitch_rate') return { cellsBillions: 200, starterVolL: 0 };
      if (cmd === 'list_recipes') return [
        { id: 'r1', name: 'Pliny the Elder Clone', style_name: 'Imperial IPA', type_: 'All Grain', batch_size_l: 19, created_at: 1716000000000, updated_at: 1716000000000, source: 'user' },
        { id: 'r2', name: 'Hefeweizen', style_name: 'Weizen/Weissbier', type_: 'All Grain', batch_size_l: 19, created_at: 1715000000000, updated_at: 1715000000000, source: 'user' },
        { id: 'r3', name: 'Irish Stout', style_name: 'Irish Stout', type_: 'All Grain', batch_size_l: 19, created_at: 1714000000000, updated_at: 1714000000000, source: 'user' },
      ];
      if (cmd === 'list_baseline_recipes') return [];
      if (cmd === 'list_batches') return [];
      if (cmd === 'list_batches_for_recipe') return [];
      if (cmd === 'list_recipe_versions') return [];
      if (cmd === 'list_equipment_profiles') return [];
      if (cmd === 'get_settings') return { database_path: '', unit_system: 'metric' };
      if (cmd === 'get_recipe') return {
        id: 'r1', name: 'Pliny the Elder Clone', type_: 'All Grain',
        batch_size_l: 19, boil_size_l: 23, boil_time_min: 90,
        efficiency_pct: 72, fermentation_stages: 2,
        forced_carbonation: false, source: 'user',
        created_at: 1716000000000, updated_at: 1716000000000,
        style: { id: 's1', name: 'Imperial IPA', category: 'India Pale Ale', category_number: '22', style_letter: 'C', style_guide: 'BJCP 2021', type_: 'Ale', og_min: 1.065, og_max: 1.085, fg_min: 1.008, fg_max: 1.017, ibu_min: 60, ibu_max: 120, color_min_srm: 6, color_max_srm: 14, abv_min: 7.5, abv_max: 10.0, notes: null },
        fermentables: [
          { id: 'f1', recipe_id: 'r1', name: 'Pale Malt (2-Row)', type_: 'Grain', yield_pct: 80, color_lovibond: 2, amount_kg: 7.26, add_after_boil: false, addition_order: 0 },
          { id: 'f2', recipe_id: 'r1', name: 'Crystal 45L', type_: 'Grain', yield_pct: 74, color_lovibond: 45, amount_kg: 0.34, add_after_boil: false, addition_order: 1 },
          { id: 'f3', recipe_id: 'r1', name: 'Corn Sugar (Dextrose)', type_: 'Sugar', yield_pct: 100, color_lovibond: 0, amount_kg: 0.45, add_after_boil: false, addition_order: 2 },
        ],
        hops: [
          { id: 'h1', recipe_id: 'r1', name: 'Columbus (CTZ)', alpha_pct: 15.0, form: 'Pellet', use_: 'Boil', time_min: 90, amount_kg: 0.057, addition_order: 0 },
          { id: 'h2', recipe_id: 'r1', name: 'Centennial', alpha_pct: 10.0, form: 'Pellet', use_: 'Boil', time_min: 45, amount_kg: 0.028, addition_order: 1 },
          { id: 'h3', recipe_id: 'r1', name: 'Simcoe', alpha_pct: 13.0, form: 'Pellet', use_: 'Boil', time_min: 30, amount_kg: 0.028, addition_order: 2 },
          { id: 'h4', recipe_id: 'r1', name: 'Centennial', alpha_pct: 10.0, form: 'Pellet', use_: 'Boil', time_min: 0, amount_kg: 0.028, addition_order: 3 },
          { id: 'h5', recipe_id: 'r1', name: 'Simcoe', alpha_pct: 13.0, form: 'Pellet', use_: 'Boil', time_min: 0, amount_kg: 0.028, addition_order: 4 },
          { id: 'h6', recipe_id: 'r1', name: 'Amarillo', alpha_pct: 9.5, form: 'Pellet', use_: 'Dry Hop', time_min: 4320, amount_kg: 0.057, addition_order: 5 },
          { id: 'h7', recipe_id: 'r1', name: 'Cascade', alpha_pct: 5.5, form: 'Pellet', use_: 'Dry Hop', time_min: 4320, amount_kg: 0.028, addition_order: 6 },
          { id: 'h8', recipe_id: 'r1', name: 'Simcoe', alpha_pct: 13.0, form: 'Pellet', use_: 'Dry Hop', time_min: 4320, amount_kg: 0.028, addition_order: 7 },
        ],
        yeasts: [
          { id: 'y1', recipe_id: 'r1', name: 'American Ale (WY1056)', type_: 'Ale', form: 'Liquid', laboratory: 'Wyeast', product_id: '1056', attenuation_pct: 75, amount: 1, amount_is_weight: false, add_to_secondary: false, times_cultured: 0 },
        ],
        miscs: [
          { id: 'm1', recipe_id: 'r1', name: 'Whirlfloc Tablet', type_: 'Fining', use_: 'Boil', amount: 1, amount_is_weight: false, time_min: 15, addition_order: 0 },
        ],
        waters: [], water_adjustments: [], mash: null,
      };
      if (cmd === 'get_recipe_stats') return {
        og: 1.072, fg: 1.009, abv_pct: 8.9, ibu: 100, srm: 6.2,
        calories_per_355ml: 240, bu_gu_ratio: 1.39,
        pre_boil_gravity: 1.062, pre_boil_volume_l: 23, post_boil_volume_l: 19,
        strike_temp_c: 72,
      };
      if (cmd === 'list_hop_library') return [
        { id: '1', name: 'Amarillo', alpha_pct: 9.5, form: 'Pellet', type_: 'Aroma', origin: 'US', source: 'seeded' },
        { id: '2', name: 'Cascade', alpha_pct: 5.5, form: 'Pellet', type_: 'Aroma', origin: 'US', source: 'seeded' },
        { id: '3', name: 'Centennial', alpha_pct: 10.0, form: 'Pellet', type_: 'Both', origin: 'US', source: 'seeded' },
        { id: '4', name: 'Chinook', alpha_pct: 13.0, form: 'Pellet', type_: 'Both', origin: 'US', source: 'seeded' },
        { id: '5', name: 'Columbus (CTZ)', alpha_pct: 15.0, form: 'Pellet', type_: 'Bittering', origin: 'US', source: 'seeded' },
        { id: '6', name: 'Hallertau Mittelfrueh', alpha_pct: 3.8, form: 'Pellet', type_: 'Aroma', origin: 'Germany', source: 'seeded' },
        { id: '7', name: 'Mosaic', alpha_pct: 12.5, form: 'Pellet', type_: 'Aroma', origin: 'US', source: 'seeded' },
        { id: '8', name: 'Saaz', alpha_pct: 3.5, form: 'Pellet', type_: 'Aroma', origin: 'Czech Republic', source: 'seeded' },
        { id: '9', name: 'Simcoe', alpha_pct: 13.0, form: 'Pellet', type_: 'Both', origin: 'US', source: 'seeded' },
        { id: '10', name: "Falconer's Flight", alpha_pct: 10.5, form: 'Pellet', type_: 'Aroma', origin: 'US', source: 'user' },
      ];
      if (cmd === 'list_fermentable_library') return [
        { id: '1', name: 'Pale Malt (2-Row)', type_: 'Grain', yield_pct: 80, color_lovibond: 2, origin: 'US', add_after_boil: false, source: 'seeded' },
        { id: '2', name: 'Crystal 45L', type_: 'Grain', yield_pct: 74, color_lovibond: 45, origin: 'US', add_after_boil: false, source: 'seeded' },
        { id: '3', name: 'Corn Sugar (Dextrose)', type_: 'Sugar', yield_pct: 100, color_lovibond: 0, add_after_boil: true, source: 'seeded' },
        { id: '4', name: 'Munich Malt', type_: 'Grain', yield_pct: 78, color_lovibond: 8, origin: 'Germany', add_after_boil: false, source: 'seeded' },
        { id: '5', name: 'Pilsner Malt', type_: 'Grain', yield_pct: 81, color_lovibond: 1.5, origin: 'Belgium', add_after_boil: false, source: 'seeded' },
        { id: '6', name: 'Roasted Barley', type_: 'Grain', yield_pct: 56, color_lovibond: 300, origin: 'US', add_after_boil: false, source: 'seeded' },
        { id: '7', name: 'Wheat Malt', type_: 'Grain', yield_pct: 83, color_lovibond: 2, origin: 'Germany', add_after_boil: false, source: 'seeded' },
      ];
      if (cmd === 'list_yeast_library') return [
        { id: '1', name: 'American Ale (WY1056)', type_: 'Ale', form: 'Liquid', laboratory: 'Wyeast', attenuation_pct: 75, add_to_secondary: false, source: 'seeded' },
        { id: '2', name: 'Belgian Witbier (WY3944)', type_: 'Ale', form: 'Liquid', laboratory: 'Wyeast', attenuation_pct: 74, add_to_secondary: false, source: 'seeded' },
        { id: '3', name: 'Bohemian Lager (WY2124)', type_: 'Lager', form: 'Liquid', laboratory: 'Wyeast', attenuation_pct: 73, add_to_secondary: false, source: 'seeded' },
        { id: '4', name: 'Safale US-05', type_: 'Ale', form: 'Dry', laboratory: 'Fermentis', attenuation_pct: 77, add_to_secondary: false, source: 'seeded' },
        { id: '5', name: 'Saflager W-34/70', type_: 'Lager', form: 'Dry', laboratory: 'Fermentis', attenuation_pct: 80, add_to_secondary: false, source: 'seeded' },
      ];
      if (cmd === 'list_misc_library') return [
        { id: '1', name: 'Irish Moss', type_: 'Fining', use_: 'Boil', time_min: 15, amount_is_weight: false, source: 'seeded' },
        { id: '2', name: 'Whirlfloc Tablet', type_: 'Fining', use_: 'Boil', time_min: 15, amount_is_weight: false, source: 'seeded' },
        { id: '3', name: 'Gypsum (CaSO4)', type_: 'Water Agent', use_: 'Mash', time_min: 0, amount_is_weight: true, source: 'seeded' },
      ];
      if (cmd === 'list_water_library') return [
        { id: '1', name: 'Burton-on-Trent', calcium_ppm: 275, magnesium_ppm: 40, sodium_ppm: 25, sulfate_ppm: 610, chloride_ppm: 35, bicarbonate_ppm: 270, source: 'seeded' },
        { id: '2', name: 'Dublin', calcium_ppm: 118, magnesium_ppm: 4, sodium_ppm: 12, sulfate_ppm: 54, chloride_ppm: 19, bicarbonate_ppm: 319, source: 'seeded' },
        { id: '3', name: 'Munich', calcium_ppm: 75, magnesium_ppm: 18, sodium_ppm: 10, sulfate_ppm: 10, chloride_ppm: 2, bicarbonate_ppm: 295, source: 'seeded' },
      ];
      return null;
    },
    convertFileSrc: (p) => p,
    transformCallback: (cb) => { const id = Math.random(); window[id] = cb; return id; },
  };
`;

const browser = await chromium.launch({ headless: true });

async function capture(width, height, url, name, interact, outDir = OUT) {
  const pg = await browser.newPage();
  await pg.setViewportSize({ width, height });
  await pg.addInitScript({ content: TAURI_MOCK });
  await pg.goto(`${BASE}${url}`, { waitUntil: "networkidle", timeout: 15000 });
  await pg.waitForTimeout(2000);
  if (interact) {
    await interact(pg);
    await pg.waitForTimeout(600);
  }
  await pg.screenshot({ path: path.join(outDir, `${name}.png`) });
  console.log(`✓ ${name}`);
  await pg.close();
}

async function fillInputs(pg, values) {
  const inputs = pg.locator('input[type="number"]');
  for (let i = 0; i < values.length; i++) {
    await inputs.nth(i).fill(String(values[i]));
  }
  await inputs.last().press("Tab");
}

// ── Desktop (1280×800) ───────────────────────────────────────────────────────
await capture(1280, 800, "/recipe/r1", "recipes");
await capture(1280, 800, "/tools", "tools");
await capture(1280, 800, "/tools/abv-calories", "tools-abv", (pg) =>
  fillInputs(pg, [1.06, 1.012]),
);
await capture(1280, 800, "/tools/carbonation", "tools-carbonation");
await capture(1280, 800, "/library", "library");

// ── Mobile (390×844) ─────────────────────────────────────────────────────────
await capture(390, 844, "/", "recipes-mobile");
await capture(390, 844, "/tools", "tools-mobile");
await capture(390, 844, "/tools/abv-calories", "tools-abv-mobile", (pg) =>
  fillInputs(pg, [1.06, 1.012]),
);

await browser.close();
console.log(`\nScreenshots written to docs/screenshots/`);
