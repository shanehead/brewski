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
      if (cmd === 'list_recipes') return [];
      if (cmd === 'list_batches') return [];
      if (cmd === 'list_hops') return [];
      if (cmd === 'list_fermentables') return [];
      if (cmd === 'list_yeasts') return [];
      if (cmd === 'list_equipment_profiles') return [];
      if (cmd === 'get_settings') return { database_path: '', unit_system: 'metric' };
      return null;
    },
    convertFileSrc: (p) => p,
    transformCallback: (cb) => { const id = Math.random(); window[id] = cb; return id; },
  };
`;

const browser = await chromium.launch({ headless: true });

async function capture(width, height, url, name, interact) {
  const pg = await browser.newPage();
  await pg.setViewportSize({ width, height });
  await pg.addInitScript({ content: TAURI_MOCK });
  await pg.goto(`${BASE}${url}`, { waitUntil: "networkidle", timeout: 15000 });
  await pg.waitForTimeout(2000);
  if (interact) {
    await interact(pg);
    await pg.waitForTimeout(600);
  }
  await pg.screenshot({ path: path.join(OUT, `${name}.png`) });
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
await capture(1280, 800, "/", "recipes");
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
