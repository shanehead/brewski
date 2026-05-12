#!/usr/bin/env node
import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const bjcpPath = path.join(repoRoot, 'data/bjcp-2021-styles.json');
const sqlPath = path.join(repoRoot, 'src-tauri/src/migration/sql/001_initial.sql');

const raw = JSON.parse(readFileSync(bjcpPath, 'utf8'));
if (!Array.isArray(raw)) {
  throw new Error('Expected bjcp-2021-styles.json to be an array');
}

const sqlEscape = (value) => String(value).replace(/'/g, "''");
const asNumberOr = (value, fallback) => (typeof value === 'number' ? value : fallback);
const asNullableNumber = (value) => (typeof value === 'number' ? value : null);

const parseCategoryNumber = (title) => {
  const match = String(title).match(/^(\d+)\./);
  return match ? match[1] : '';
};

const parseStyleCodeAndName = (title) => {
  const match = String(title).match(/^(\d+)([A-Z]+)\.\s+(.+)$/);
  if (!match) {
    return null;
  }
  return {
    categoryNumber: match[1],
    styleLetter: match[2],
    name: match[3],
  };
};

const inferType = (tags) => {
  const t = String(tags ?? '').toLowerCase();
  if (t.includes('bottom-fermented')) return 'Lager';
  if (t.includes('top-fermented')) return 'Ale';
  if (t.includes('spontaneous') || t.includes('wild-fermented') || t.includes('mixed-fermentation')) return 'Beer';
  return 'Beer';
};

const rows = [];
for (const category of raw) {
  const categoryNumber = parseCategoryNumber(category.title);
  const categoryName = category.title?.replace(/^\d+\.\s*/, '') ?? '';

  for (const style of category.styles ?? []) {
    const parsed = parseStyleCodeAndName(style.title);
    if (!parsed) continue;

    const stats = style.properties?.vitalStatistics ?? {};
    const og = Array.isArray(stats.OG) ? stats.OG : [];
    const fg = Array.isArray(stats.FG) ? stats.FG : [];
    const ibu = Array.isArray(stats.IBUs) ? stats.IBUs : [];
    const srm = Array.isArray(stats.SRM) ? stats.SRM : [];
    const abv = Array.isArray(stats.ABV) ? stats.ABV : [];

    const profileParts = [];
    if (style.properties?.aroma) profileParts.push(`Aroma: ${style.properties.aroma}`);
    if (style.properties?.appearance) profileParts.push(`Appearance: ${style.properties.appearance}`);
    if (style.properties?.flavor) profileParts.push(`Flavor: ${style.properties.flavor}`);
    const profile = profileParts.length > 0 ? profileParts.join('\n') : null;

    rows.push({
      id: `${parsed.categoryNumber}${parsed.styleLetter}`,
      name: parsed.name,
      category: categoryName,
      category_number: categoryNumber || parsed.categoryNumber,
      style_letter: parsed.styleLetter,
      style_guide: 'BJCP 2021',
      type: inferType(style.properties?.tags),
      og_min: asNumberOr(og[0], 0),
      og_max: asNumberOr(og[1], 0),
      fg_min: asNumberOr(fg[0], 0),
      fg_max: asNumberOr(fg[1], 0),
      ibu_min: asNumberOr(ibu[0], 0),
      ibu_max: asNumberOr(ibu[1], 0),
      color_min_srm: asNumberOr(srm[0], 0),
      color_max_srm: asNumberOr(srm[1], 0),
      abv_min_pct: asNullableNumber(abv[0]),
      abv_max_pct: asNullableNumber(abv[1]),
      carb_min_vols: null,
      carb_max_vols: null,
      notes: style.properties?.overallImpression ?? null,
      profile,
      ingredients: style.properties?.characteristicIngredients ?? null,
      examples: style.properties?.commercialExamples ?? null,
    });
  }
}

rows.sort((a, b) => a.id.localeCompare(b.id, 'en', { numeric: true }));

const columns = [
  'id',
  'name',
  'category',
  'category_number',
  'style_letter',
  'style_guide',
  'type',
  'og_min',
  'og_max',
  'fg_min',
  'fg_max',
  'ibu_min',
  'ibu_max',
  'color_min_srm',
  'color_max_srm',
  'abv_min_pct',
  'abv_max_pct',
  'carb_min_vols',
  'carb_max_vols',
  'notes',
  'profile',
  'ingredients',
  'examples',
];

const sqlValue = (v) => {
  if (v === null || v === undefined) return 'NULL';
  if (typeof v === 'number') return String(v);
  return `'${sqlEscape(v)}'`;
};

const valuesSql = rows
  .map((row) => `(${columns.map((col) => sqlValue(row[col])).join(', ')})`)
  .join(',\n');

const newBlock = [
  '-- BJCP 2021 style seed',
  `INSERT OR IGNORE INTO styles (${columns.join(', ')}) VALUES`,
  `${valuesSql};`,
].join('\n');

const sql = readFileSync(sqlPath, 'utf8');
const sectionStart = sql.indexOf('INSERT OR IGNORE INTO styles');
const sectionEnd = sql.indexOf('-- Default equipment profile');
if (sectionStart === -1 || sectionEnd === -1 || sectionEnd <= sectionStart) {
  throw new Error('Could not find styles seed section boundaries');
}
const commentStart = sql.lastIndexOf('--', sectionStart);
if (commentStart === -1) {
  throw new Error('Could not find styles seed comment start');
}
const replaced = `${sql.slice(0, commentStart)}${newBlock}\n\n${sql.slice(sectionEnd)}`;

writeFileSync(sqlPath, replaced, 'utf8');
console.log(`Seeded ${rows.length} BJCP styles into 001_initial.sql`);
