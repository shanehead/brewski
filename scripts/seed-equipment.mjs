#!/usr/bin/env node
import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const dataPath = path.join(repoRoot, 'data/equipment.json');
const sqlPath = path.join(repoRoot, 'src-tauri/src/migration/sql/001_initial.sql');

const raw = JSON.parse(readFileSync(dataPath, 'utf8'));
if (!Array.isArray(raw)) {
  throw new Error('Expected equipment.json to be an array');
}

const sqlEscape = (value) => String(value).replace(/'/g, "''");
const sqlValue = (v) => {
  if (v === null || v === undefined) return 'NULL';
  if (typeof v === 'number') return String(v);
  return `'${sqlEscape(v)}'`;
};

const columns = [
  'id',
  'name',
  'boil_size_l',
  'batch_size_l',
  'boil_time_min',
  'evap_rate_pct_hr',
  'trub_chiller_loss_l',
  'fermenter_loss_l',
  'hop_utilization_pct',
  'efficiency_pct',
  'created_at',
  'updated_at',
];

const rows = raw.map((r) => ({ ...r }));
rows.sort((a, b) => a.id.localeCompare(b.id, 'en', { numeric: true }));

const valuesSql = rows
  .map((row) => `(${columns.map((col) => sqlValue(row[col])).join(', ')})`)
  .join(',\n');

const newBlock = [
  '-- Default equipment profile',
  `INSERT OR IGNORE INTO equipment_profiles (${columns.join(', ')}) VALUES`,
  `${valuesSql};`,
].join('\n');

const sql = readFileSync(sqlPath, 'utf8');
const sectionStart = sql.indexOf('INSERT OR IGNORE INTO equipment_profiles');
if (sectionStart === -1) {
  throw new Error('Could not find equipment_profiles seed section');
}
// include the preceding comment line (if present)
const commentStart = sql.lastIndexOf('--', sectionStart);
const endIndex = sql.indexOf(');', sectionStart);
if (endIndex === -1) {
  throw new Error('Could not find end of equipment_profiles insert');
}

const replaced = `${sql.slice(0, commentStart)}${newBlock}\n\n${sql.slice(endIndex + 2)}`;
writeFileSync(sqlPath, replaced, 'utf8');
console.log(`Seeded ${rows.length} equipment profiles into 001_initial.sql`);
