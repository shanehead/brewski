import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import path from 'node:path';
import YAML from 'yaml';

const repo = '/Users/shead/Documents/code/brewski';
const input = path.join(repo, 'docs/openapi.yaml');
const outRoot = path.join(repo, 'docs/openapi');
const pathsDir = path.join(outRoot, 'paths/commands');
const schemasDir = path.join(outRoot, 'components/schemas');
const responsesDir = path.join(outRoot, 'components/responses');

const toBlock = (v) => YAML.stringify(v, { indent: 2, lineWidth: 0 }).trimEnd() + '\n';

const src = YAML.parse(await readFile(input, 'utf8'));

await rm(outRoot, { recursive: true, force: true });
await mkdir(pathsDir, { recursive: true });
await mkdir(schemasDir, { recursive: true });
await mkdir(responsesDir, { recursive: true });

const root = {
  openapi: src.openapi,
  info: src.info,
  servers: src.servers,
  tags: src.tags,
  paths: {},
  components: {
    schemas: {},
    responses: {
      Error: { $ref: './components/responses/Error.yaml' },
    },
  },
};

for (const [route, methods] of Object.entries(src.paths ?? {})) {
  const command = route.replace('/commands/', '');
  root.paths[route] = { $ref: `./paths/commands/${command}.yaml` };
  const out = {};
  for (const [method, op] of Object.entries(methods)) {
    out[method] = op;
  }
  await writeFile(path.join(pathsDir, `${command}.yaml`), toBlock(out));
}

for (const [name, schema] of Object.entries(src.components?.schemas ?? {})) {
  root.components.schemas[name] = { $ref: `./components/schemas/${name}.yaml` };
  await writeFile(path.join(schemasDir, `${name}.yaml`), toBlock(schema));
}

if (!src.components?.responses?.Error) {
  throw new Error('components.responses.Error not found in source spec');
}
await writeFile(path.join(responsesDir, 'Error.yaml'), toBlock(src.components.responses.Error));
await writeFile(path.join(outRoot, 'openapi.yaml'), toBlock(root));
