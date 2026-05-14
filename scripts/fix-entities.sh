#!/usr/bin/env bash
# Post-process SeaORM-generated entities to match the project's type conventions:
#   - Use f64 instead of Decimal (sea-orm-cli maps SQLite REAL to Decimal,
#     but our Cargo.toml does not enable the with-rust_decimal feature)
#   - Use String instead of Option<String> for primary key fields
#     (SQLite TEXT PRIMARY KEY is implicitly nullable, but we always provide IDs)
#   - Remove Eq derive from models that contain f64 (f64 does not implement Eq)
set -euo pipefail

ENTITIES_DIR="${1:-src-tauri/src/entities}"

for file in "$ENTITIES_DIR"/*.rs; do
  # 1. Replace Decimal types with f64
  sed -i.bak 's/: Decimal,/: f64,/g' "$file"
  rm -f "${file}.bak"
  sed -i.bak 's/: Option<Decimal>/: Option<f64>/g' "$file"
  rm -f "${file}.bak"

  # 2. Fix primary key: remove "nullable" from the attribute and unwrap Option<String> to String.
  #    The pattern spans two lines: the #[sea_orm(..., nullable)] attribute and the pub field line.
  perl -0777 -i -pe '
    s/#\[sea_orm\(primary_key, auto_increment = false, column_type = "Text", nullable\)\]\n(\s*pub \w+): Option<String>/#[sea_orm(primary_key, auto_increment = false, column_type = "Text")]\n$1: String/g
  ' "$file"

  # 3. Remove Eq derive when the file contains f64 (f64 does not implement Eq)
  if grep -qE 'f64' "$file"; then
    sed -i.bak 's/DeriveEntityModel, Eq,/DeriveEntityModel,/g' "$file"
    rm -f "${file}.bak"
  fi
done

echo "Entity post-processing complete. Files processed: $(ls "$ENTITIES_DIR"/*.rs | wc -l | tr -d ' ')"
