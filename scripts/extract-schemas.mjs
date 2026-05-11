import { readFile, writeFile } from "node:fs/promises";
import yaml from "js-yaml";

const [, , inputPath, outputPath] = process.argv;

if (!inputPath || !outputPath) {
  console.error("Usage: bun scripts/extract-schemas.mjs <bundled-openapi.yaml> <schemas.json>");
  process.exit(1);
}

const doc = yaml.load(await readFile(inputPath, "utf8"));
const schemas = doc?.components?.schemas;

if (!schemas || typeof schemas !== "object") {
  throw new Error("No components.schemas found in bundled OpenAPI document");
}

await writeFile(outputPath, `${JSON.stringify({ $schema: "https://json-schema.org/draft/2020-12/schema", $defs: schemas }, null, 2)}\n`);
