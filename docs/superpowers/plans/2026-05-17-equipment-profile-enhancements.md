# Equipment Profile Enhancements Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add 17 new fields to equipment profiles, wire 6 into brewing calculations, and build a full-screen modal edit UI with copy action.

**Architecture:** One SQLite migration adds all new columns; OpenAPI schemas are updated then codegen (`just gen`, `just gen-entities`) regenerates the Rust and TypeScript types; the Rust repository, commands, and brewing calculations are updated; a new `EquipmentProfileModal.svelte` handles both create and edit; the equipment list page gains Edit and Copy per-row actions.

**Tech Stack:** SQLite (SeaORM migrations), Rust (SeaORM, Tauri commands), OpenAPI + `just gen` codegen, SvelteKit 5 (runes), TypeScript, Vitest, `@testing-library/svelte`

**Spec:** `docs/superpowers/specs/2026-05-17-equipment-profile-enhancements-design.md`

---

## File Structure

| File | Action | Responsibility |
|---|---|---|
| `src-tauri/migrations/003_equipment_profile_enhancements.sql` | Create | Add 17 new columns with defaults |
| `src-tauri/src/entities/equipment_profiles.rs` | Regenerate (gen-entities) | SeaORM entity with new fields |
| `docs/openapi/components/schemas/EquipmentProfile.yaml` | Modify | Add 17 new fields to response schema |
| `docs/openapi/components/schemas/CreateEquipmentProfileInput.yaml` | Modify | Add new optional input fields |
| `docs/openapi/components/schemas/UpdateEquipmentProfileInput.yaml` | Modify | Add new optional patch fields |
| `docs/openapi/paths/commands/copy_equipment_profile.yaml` | Create | Copy command OpenAPI path |
| `docs/openapi/openapi.yaml` | Modify | Register copy path |
| `src/lib/api.gen.ts` | Regenerate (gen-ts) | TypeScript types from OpenAPI |
| `src-tauri/src/models.gen.rs` | Regenerate (gen-rust) | Rust types from OpenAPI |
| `src-tauri/src/models.rs` | Modify | TryFrom entity→model for new fields |
| `src-tauri/src/repositories/equipment.rs` | Modify | create/update new fields + copy fn |
| `src-tauri/src/commands/equipment.rs` | Modify | Add copy_equipment_profile command |
| `src-tauri/src/lib.rs` | Modify | Register copy command in invoke_handler |
| `src-tauri/src/brewing/volumes.rs` | Modify | mash_tun_loss, hlt_deadspace, cooling_shrinkage params |
| `src-tauri/src/brewing/ibu.rs` | Modify | aroma_hop_utilization_pct, whirlpool_time_min |
| `src-tauri/src/brewing/mod.rs` | Modify | Wire all new equipment fields to calcs |
| `src/lib/api.ts` | Modify | Add copyEquipmentProfile |
| `src/lib/components/EquipmentProfileModal.svelte` | Create | Full-screen modal for create/edit |
| `src/routes/equipment/+page.svelte` | Modify | Edit/Copy actions per row, wire modal |
| `tests/EquipmentPage.test.ts` | Modify | Update mocks + add edit/copy tests |

---

## Task 1: Migration

**Files:**
- Create: `src-tauri/migrations/003_equipment_profile_enhancements.sql`

- [ ] **Step 1: Create migration file**

```sql
-- src-tauri/migrations/003_equipment_profile_enhancements.sql
ALTER TABLE equipment_profiles ADD COLUMN batch_volume_target TEXT NOT NULL DEFAULT 'fermenter';
ALTER TABLE equipment_profiles ADD COLUMN mash_tun_loss_l REAL NOT NULL DEFAULT 0;
ALTER TABLE equipment_profiles ADD COLUMN hlt_deadspace_l REAL;
ALTER TABLE equipment_profiles ADD COLUMN cooling_shrinkage_pct REAL NOT NULL DEFAULT 4.0;
ALTER TABLE equipment_profiles ADD COLUMN calc_mash_efficiency INTEGER NOT NULL DEFAULT 1;
ALTER TABLE equipment_profiles ADD COLUMN mash_efficiency_pct REAL;
ALTER TABLE equipment_profiles ADD COLUMN calc_aroma_hop_utilization INTEGER NOT NULL DEFAULT 1;
ALTER TABLE equipment_profiles ADD COLUMN aroma_hop_utilization_pct REAL NOT NULL DEFAULT 23;
ALTER TABLE equipment_profiles ADD COLUMN whirlpool_time_min REAL;
ALTER TABLE equipment_profiles ADD COLUMN altitude_adjustment INTEGER NOT NULL DEFAULT 0;
ALTER TABLE equipment_profiles ADD COLUMN boil_temp_f REAL;
ALTER TABLE equipment_profiles ADD COLUMN sparge_method TEXT NOT NULL DEFAULT 'no_sparge';
ALTER TABLE equipment_profiles ADD COLUMN mash_volume_min_l REAL;
ALTER TABLE equipment_profiles ADD COLUMN mash_volume_max_l REAL;
ALTER TABLE equipment_profiles ADD COLUMN sparge_volume_min_l REAL;
ALTER TABLE equipment_profiles ADD COLUMN sparge_volume_max_l REAL;
ALTER TABLE equipment_profiles ADD COLUMN calc_strike_water_temp INTEGER NOT NULL DEFAULT 0;
```

- [ ] **Step 2: Apply migration to dev database**

```bash
just migrate
```

Expected: migration runs without error, `dev.db` updated.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/migrations/003_equipment_profile_enhancements.sql
git commit -m "feat(db): add 17 new columns to equipment_profiles"
```

---

## Task 2: Regenerate Entity

**Files:**
- Modify: `src-tauri/src/entities/equipment_profiles.rs` (regenerated)

- [ ] **Step 1: Regenerate SeaORM entities from updated dev.db**

```bash
just gen-entities
```

Expected: `src-tauri/src/entities/equipment_profiles.rs` now includes all 17 new fields. The `fix-entities.sh` post-processing runs automatically as part of `gen-entities`.

- [ ] **Step 2: Verify the new fields are present**

```bash
grep "batch_volume_target\|mash_tun_loss\|cooling_shrinkage\|sparge_method" src-tauri/src/entities/equipment_profiles.rs
```

Expected: all four names appear in the output.

- [ ] **Step 3: Verify Rust compiles**

```bash
cd src-tauri && cargo check
```

Expected: compile warnings only (unused fields), no errors.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/entities/equipment_profiles.rs
git commit -m "chore: regenerate equipment_profiles entity with new fields"
```

---

## Task 3: OpenAPI Schema Updates

**Files:**
- Modify: `docs/openapi/components/schemas/EquipmentProfile.yaml`
- Modify: `docs/openapi/components/schemas/CreateEquipmentProfileInput.yaml`
- Modify: `docs/openapi/components/schemas/UpdateEquipmentProfileInput.yaml`
- Create: `docs/openapi/paths/commands/copy_equipment_profile.yaml`
- Modify: `docs/openapi/openapi.yaml`

- [ ] **Step 1: Update EquipmentProfile.yaml — add new required fields to `required` list**

Add to the `required` array (alongside existing required fields):
```yaml
  - batch_volume_target
  - mash_tun_loss_l
  - cooling_shrinkage_pct
  - calc_mash_efficiency
  - calc_aroma_hop_utilization
  - aroma_hop_utilization_pct
  - altitude_adjustment
  - sparge_method
  - calc_strike_water_temp
```

- [ ] **Step 2: Update EquipmentProfile.yaml — add new properties**

Append to the `properties` block:
```yaml
  batch_volume_target:
    type: string
    description: "Whether batch_size_l targets the fermenter or the kettle. Enum: 'fermenter' | 'kettle'"
  mash_tun_loss_l:
    type: number
    description: Volume left in the mash tun after lautering, in litres
  hlt_deadspace_l:
    type:
      - number
      - "null"
    description: Volume that remains in the HLT and cannot be transferred, in litres
  cooling_shrinkage_pct:
    type: number
    description: Wort volume reduction from boiling temperature to room temperature, as a percentage
  calc_mash_efficiency:
    type: boolean
    description: When true, mash efficiency is calculated from brewhouse efficiency and losses
  mash_efficiency_pct:
    type:
      - number
      - "null"
    description: Manual mash efficiency percentage, used when calc_mash_efficiency is false
  calc_aroma_hop_utilization:
    type: boolean
    description: When true, aroma hop utilization is calculated using the temperature model
  aroma_hop_utilization_pct:
    type: number
    description: Utilization percentage for whirlpool/aroma hop additions
  whirlpool_time_min:
    type:
      - number
      - "null"
    description: Time wort sits in the whirlpool before chilling, in minutes
  altitude_adjustment:
    type: boolean
    description: When true, boil temperature is calculated from altitude
  boil_temp_f:
    type:
      - number
      - "null"
    description: Manual boil temperature in Fahrenheit, used when altitude_adjustment is false
  sparge_method:
    type: string
    description: "Mash/sparge water calculation method. Enum: 'no_sparge' | 'batch_sparge' | 'fly_sparge'"
  mash_volume_min_l:
    type:
      - number
      - "null"
    description: Minimum mash tun volume in litres
  mash_volume_max_l:
    type:
      - number
      - "null"
    description: Maximum mash tun volume in litres
  sparge_volume_min_l:
    type:
      - number
      - "null"
    description: Minimum sparge water volume in litres
  sparge_volume_max_l:
    type:
      - number
      - "null"
    description: Maximum sparge water volume in litres
  calc_strike_water_temp:
    type: boolean
    description: When true, strike water temperature is calculated from tun thermal mass (calculation deferred)
```

- [ ] **Step 3: Update CreateEquipmentProfileInput.yaml — add new optional properties**

Append to the `properties` block (all optional on create — defaults apply):
```yaml
  batch_volume_target:
    type: string
  mash_tun_loss_l:
    type: number
  hlt_deadspace_l:
    type: number
  cooling_shrinkage_pct:
    type: number
  calc_mash_efficiency:
    type: boolean
  mash_efficiency_pct:
    type: number
  calc_aroma_hop_utilization:
    type: boolean
  aroma_hop_utilization_pct:
    type: number
  whirlpool_time_min:
    type: number
  altitude_adjustment:
    type: boolean
  boil_temp_f:
    type: number
  sparge_method:
    type: string
  mash_volume_min_l:
    type: number
  mash_volume_max_l:
    type: number
  sparge_volume_min_l:
    type: number
  sparge_volume_max_l:
    type: number
  calc_strike_water_temp:
    type: boolean
```

- [ ] **Step 4: Update UpdateEquipmentProfileInput.yaml — same properties as Create (all optional)**

Append the exact same block as Step 3 to `UpdateEquipmentProfileInput.yaml`.

- [ ] **Step 5: Create copy_equipment_profile.yaml**

```yaml
# docs/openapi/paths/commands/copy_equipment_profile.yaml
post:
  operationId: copy_equipment_profile
  summary: Duplicate an equipment profile
  requestBody:
    required: true
    content:
      application/json:
        schema:
          type: object
          required:
            - id
          properties:
            id:
              type: string
  responses:
    "200":
      description: The newly created copy
      content:
        application/json:
          schema:
            $ref: "../../components/schemas/EquipmentProfile.yaml"
```

- [ ] **Step 6: Register copy path in openapi.yaml**

In `docs/openapi/openapi.yaml`, add alongside the other equipment paths:
```yaml
  /commands/copy_equipment_profile:
    $ref: ./paths/commands/copy_equipment_profile.yaml
```

- [ ] **Step 7: Lint the OpenAPI spec**

```bash
just lint-openapi
```

Expected: no errors.

- [ ] **Step 8: Commit**

```bash
git add docs/openapi/
git commit -m "feat(openapi): add new equipment profile fields and copy command"
```

---

## Task 4: Regenerate TypeScript and Rust Types

**Files:**
- Modify: `src/lib/api.gen.ts` (regenerated)
- Modify: `src-tauri/src/models.gen.rs` (regenerated)

- [ ] **Step 1: Regenerate both**

```bash
just gen
```

Expected: `src/lib/api.gen.ts` and `src-tauri/src/models.gen.rs` updated with all 17 new fields.

- [ ] **Step 2: Verify new fields in TypeScript types**

```bash
grep "batch_volume_target\|mash_tun_loss_l\|cooling_shrinkage" src/lib/api.gen.ts
```

Expected: all three names appear.

- [ ] **Step 3: Verify Rust compiles**

```bash
cd src-tauri && cargo check
```

Expected: compiles (may have new unused field warnings, that's fine).

- [ ] **Step 4: Commit**

```bash
git add src/lib/api.gen.ts src-tauri/src/models.gen.rs
git commit -m "chore: regenerate TypeScript and Rust types with new equipment profile fields"
```

---

## Task 5: models.rs — TryFrom for New Fields

**Files:**
- Modify: `src-tauri/src/models.rs`

The `TryFrom<entities::equipment_profiles::Model> for EquipmentProfile` impl needs all 17 new fields mapped. The `CreateEquipmentProfileInput` and `UpdateEquipmentProfileInput` types are generated directly from OpenAPI and do not need a manual TryFrom — they're passed directly to the repository.

- [ ] **Step 1: Update the EquipmentProfile TryFrom in models.rs**

Locate the `impl TryFrom<entities::equipment_profiles::Model> for EquipmentProfile` block and add the new field mappings:

```rust
// After the existing fields (efficiency_pct, created_at, updated_at), add:
batch_volume_target: m.batch_volume_target,
mash_tun_loss_l: m.mash_tun_loss_l,
hlt_deadspace_l: m.hlt_deadspace_l,
cooling_shrinkage_pct: m.cooling_shrinkage_pct,
calc_mash_efficiency: m.calc_mash_efficiency != 0,
mash_efficiency_pct: m.mash_efficiency_pct,
calc_aroma_hop_utilization: m.calc_aroma_hop_utilization != 0,
aroma_hop_utilization_pct: m.aroma_hop_utilization_pct,
whirlpool_time_min: m.whirlpool_time_min,
altitude_adjustment: m.altitude_adjustment != 0,
boil_temp_f: m.boil_temp_f,
sparge_method: m.sparge_method,
mash_volume_min_l: m.mash_volume_min_l,
mash_volume_max_l: m.mash_volume_max_l,
sparge_volume_min_l: m.sparge_volume_min_l,
sparge_volume_max_l: m.sparge_volume_max_l,
calc_strike_water_temp: m.calc_strike_water_temp != 0,
```

- [ ] **Step 2: Run Rust tests to verify nothing broke**

```bash
cd src-tauri && cargo test
```

Expected: all existing tests pass.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/models.rs
git commit -m "feat(models): map new equipment profile fields in TryFrom"
```

---

## Task 6: Repository — New Fields + Copy

**Files:**
- Modify: `src-tauri/src/repositories/equipment.rs`

- [ ] **Step 1: Write a failing test for copy**

Add to the `#[cfg(test)]` block in `src-tauri/src/repositories/equipment.rs`:

```rust
#[tokio::test]
async fn test_copy() {
    let db = setup_test_db().await;
    let repo = EquipmentRepository::new(&db);
    let created = repo.create(input()).await.unwrap();
    let copy = repo.copy(&created.id).await.unwrap();
    assert_eq!(copy.name, "10 Gallon Kettle (copy)");
    assert_eq!(copy.batch_size_l, created.batch_size_l);
    assert_eq!(copy.efficiency_pct, created.efficiency_pct);
    assert_ne!(copy.id, created.id);
    let all = repo.list().await.unwrap();
    assert_eq!(all.len(), 2);
}
```

- [ ] **Step 2: Run test to confirm it fails**

```bash
cd src-tauri && cargo test test_copy -- --nocapture
```

Expected: compile error — `copy` method does not exist.

- [ ] **Step 3: Update the `create` method to include new fields**

Replace the `equipment_profiles::ActiveModel` block in `create` with:

```rust
equipment_profiles::ActiveModel {
    id: Set(id.clone()),
    name: Set(input.name),
    notes: Set(input.notes),
    boil_size_l: Set(input.boil_size_l),
    batch_size_l: Set(input.batch_size_l),
    boil_time_min: Set(input.boil_time_min.unwrap_or(60.0)),
    evap_rate_pct_hr: Set(input.evap_rate_pct_hr),
    trub_chiller_loss_l: Set(input.trub_chiller_loss_l),
    fermenter_loss_l: Set(input.fermenter_loss_l),
    efficiency_pct: Set(input.efficiency_pct),
    calc_boil_volume: Set(input.calc_boil_volume.map(|b| b as i32).unwrap_or(1)),
    batch_volume_target: Set(input.batch_volume_target.unwrap_or_else(|| "fermenter".into())),
    mash_tun_loss_l: Set(input.mash_tun_loss_l.unwrap_or(0.0)),
    hlt_deadspace_l: Set(input.hlt_deadspace_l),
    cooling_shrinkage_pct: Set(input.cooling_shrinkage_pct.unwrap_or(4.0)),
    calc_mash_efficiency: Set(input.calc_mash_efficiency.map(|b| b as i32).unwrap_or(1)),
    mash_efficiency_pct: Set(input.mash_efficiency_pct),
    calc_aroma_hop_utilization: Set(input.calc_aroma_hop_utilization.map(|b| b as i32).unwrap_or(1)),
    aroma_hop_utilization_pct: Set(input.aroma_hop_utilization_pct.unwrap_or(23.0)),
    whirlpool_time_min: Set(input.whirlpool_time_min),
    altitude_adjustment: Set(input.altitude_adjustment.map(|b| b as i32).unwrap_or(0)),
    boil_temp_f: Set(input.boil_temp_f),
    sparge_method: Set(input.sparge_method.unwrap_or_else(|| "no_sparge".into())),
    mash_volume_min_l: Set(input.mash_volume_min_l),
    mash_volume_max_l: Set(input.mash_volume_max_l),
    sparge_volume_min_l: Set(input.sparge_volume_min_l),
    sparge_volume_max_l: Set(input.sparge_volume_max_l),
    calc_strike_water_temp: Set(input.calc_strike_water_temp.map(|b| b as i32).unwrap_or(0)),
    created_at: Set(now),
    updated_at: Set(now),
    ..Default::default()
}
```

- [ ] **Step 4: Update the `update` method with new fields**

Add these `if let` blocks inside the `update` method, after the existing ones:

```rust
if let Some(v) = input.calc_boil_volume {
    active.calc_boil_volume = Set(v as i32);
}
if let Some(v) = input.batch_volume_target {
    active.batch_volume_target = Set(v);
}
if let Some(v) = input.mash_tun_loss_l {
    active.mash_tun_loss_l = Set(v);
}
active.hlt_deadspace_l = Set(input.hlt_deadspace_l);
if let Some(v) = input.cooling_shrinkage_pct {
    active.cooling_shrinkage_pct = Set(v);
}
if let Some(v) = input.calc_mash_efficiency {
    active.calc_mash_efficiency = Set(v as i32);
}
active.mash_efficiency_pct = Set(input.mash_efficiency_pct);
if let Some(v) = input.calc_aroma_hop_utilization {
    active.calc_aroma_hop_utilization = Set(v as i32);
}
if let Some(v) = input.aroma_hop_utilization_pct {
    active.aroma_hop_utilization_pct = Set(v);
}
active.whirlpool_time_min = Set(input.whirlpool_time_min);
if let Some(v) = input.altitude_adjustment {
    active.altitude_adjustment = Set(v as i32);
}
active.boil_temp_f = Set(input.boil_temp_f);
if let Some(v) = input.sparge_method {
    active.sparge_method = Set(v);
}
active.mash_volume_min_l = Set(input.mash_volume_min_l);
active.mash_volume_max_l = Set(input.mash_volume_max_l);
active.sparge_volume_min_l = Set(input.sparge_volume_min_l);
active.sparge_volume_max_l = Set(input.sparge_volume_max_l);
if let Some(v) = input.calc_strike_water_temp {
    active.calc_strike_water_temp = Set(v as i32);
}
```

- [ ] **Step 5: Add the `copy` method**

Add after the `update` method, before `delete`:

```rust
pub async fn copy(&self, id: &str) -> Result<EquipmentProfile, AppError> {
    let source = equipment_profiles::Entity::find_by_id(id)
        .one(self.db)
        .await?
        .ok_or(AppError::NotFound)?;
    let new_id = new_id();
    let now = now_secs() as i32;
    let mut active: equipment_profiles::ActiveModel = source.into();
    active.id = Set(new_id.clone());
    active.name = Set({
        let current = active.name.take().unwrap_or_default();
        format!("{current} (copy)")
    });
    active.created_at = Set(now);
    active.updated_at = Set(now);
    active.insert(self.db).await?;
    self.get(&new_id).await
}
```

- [ ] **Step 6: Run all repository tests**

```bash
cd src-tauri && cargo test repositories::equipment
```

Expected: all tests pass including `test_copy`.

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/repositories/equipment.rs
git commit -m "feat(repository): add new fields to equipment create/update and add copy method"
```

---

## Task 7: Commands + Registration

**Files:**
- Modify: `src-tauri/src/commands/equipment.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Add copy command to commands/equipment.rs**

```rust
#[tauri::command]
pub async fn copy_equipment_profile(
    state: State<'_, AppState>,
    id: String,
) -> Result<EquipmentProfile, AppError> {
    EquipmentRepository::new(&state.db).copy(&id).await
}
```

- [ ] **Step 2: Register in lib.rs invoke_handler**

In `src-tauri/src/lib.rs`, add to the `tauri::generate_handler!` macro list:
```rust
commands::equipment::copy_equipment_profile,
```

- [ ] **Step 3: Verify Rust builds**

```bash
cd src-tauri && cargo build
```

Expected: builds without error.

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/commands/equipment.rs src-tauri/src/lib.rs
git commit -m "feat(commands): add copy_equipment_profile Tauri command"
```

---

## Task 8: Volume Calculations

**Files:**
- Modify: `src-tauri/src/brewing/volumes.rs`

The volume calculation needs three new parameters: `mash_tun_loss_l` (water that stays in the tun, increasing pre-boil requirement), `hlt_deadspace_l` (returned as total water needed), and `cooling_shrinkage_pct` (to express hot volumes).

- [ ] **Step 1: Write failing tests**

Add to the `#[cfg(test)]` block in `volumes.rs`:

```rust
#[test]
fn test_mash_tun_loss_increases_pre_boil() {
    // Without mash tun loss: pre ≈ 27.8L (from existing test)
    let (pre_no_loss, _) = calculate_boil_volumes(23.0, 60.0, 10.0, 1.0, 1.0, 0.0, 0.0, 0.0);
    // With 2L mash tun loss: pre should be ~2L higher
    let (pre_with_loss, _) = calculate_boil_volumes(23.0, 60.0, 10.0, 1.0, 1.0, 0.0, 2.0, 0.0);
    assert!(
        (pre_with_loss - pre_no_loss - 2.0).abs() < 0.1,
        "mash_tun_loss should add ~2L to pre_boil: no_loss={pre_no_loss:.2}, with_loss={pre_with_loss:.2}"
    );
}

#[test]
fn test_hlt_deadspace_returned_in_total_water() {
    let (pre, _, total) = calculate_boil_volumes(23.0, 60.0, 10.0, 1.0, 1.0, 0.0, 0.0, 2.0);
    assert!(
        (total - pre - 2.0).abs() < 0.1,
        "total_water should be pre_boil + hlt_deadspace: pre={pre:.2}, total={total:.2}"
    );
}

#[test]
fn test_hot_volume_conversion() {
    let cold = 25.0_f64;
    let hot = hot_volume(cold, 4.0);
    assert!((hot - 26.0).abs() < 0.1, "hot={hot:.2}, expected ~26.0");
}
```

- [ ] **Step 2: Run to confirm they fail**

```bash
cd src-tauri && cargo test brewing::volumes
```

Expected: compile errors — wrong function signature.

- [ ] **Step 3: Update calculate_boil_volumes signature and implementation**

Replace the entire function (keep existing doc comment):

```rust
/// Returns (pre_boil_volume_l, post_boil_volume_l, total_water_needed_l)
/// - pre_boil: cold volume needed in the kettle at start of boil
/// - post_boil: cold volume in kettle after boil
/// - total_water: pre_boil + hlt_deadspace (all water you need to start with)
pub fn calculate_boil_volumes(
    batch_size_l: f64,
    boil_time_min: f64,
    evap_rate_pct_hr: f64,
    trub_chiller_loss_l: f64,
    fermenter_loss_l: f64,
    top_up_water_l: f64,
    mash_tun_loss_l: f64,
    hlt_deadspace_l: f64,
) -> (f64, f64, f64) {
    let post_boil_volume = batch_size_l + trub_chiller_loss_l + fermenter_loss_l - top_up_water_l;
    let boil_hours = boil_time_min / 60.0;
    let evaporation_fraction = evap_rate_pct_hr / 100.0 * boil_hours;
    let pre_boil_volume = post_boil_volume / (1.0 - evaporation_fraction) + mash_tun_loss_l;
    let total_water_needed = pre_boil_volume + hlt_deadspace_l;
    (pre_boil_volume, post_boil_volume, total_water_needed)
}

/// Convert a cold (room temperature) volume to the equivalent hot (boiling) volume.
/// Wort expands by cooling_shrinkage_pct when heated to boiling.
pub fn hot_volume(cold_volume_l: f64, cooling_shrinkage_pct: f64) -> f64 {
    cold_volume_l * (1.0 + cooling_shrinkage_pct / 100.0)
}
```

- [ ] **Step 4: Update the existing test to match new signature**

The `test_boil_volumes_standard` test passes `0.0, 0.0` for the two new params:

```rust
#[test]
fn test_boil_volumes_standard() {
    let (pre, post, _total) = calculate_boil_volumes(23.0, 60.0, 10.0, 1.0, 1.0, 0.0, 0.0, 0.0);
    assert!((post - 25.0).abs() < 0.5, "post_boil was {post:.2}L, expected ~25L");
    assert!((pre - 27.8).abs() < 0.5, "pre_boil was {pre:.2}L, expected ~27.8L");
}
```

- [ ] **Step 5: Run volume tests**

```bash
cd src-tauri && cargo test brewing::volumes
```

Expected: all 4 tests pass.

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/brewing/volumes.rs
git commit -m "feat(brewing): add mash_tun_loss, hlt_deadspace, and hot_volume to volume calculations"
```

---

## Task 9: IBU Calculations

**Files:**
- Modify: `src-tauri/src/brewing/ibu.rs`

Two changes: (1) `whirlpool_time_min` adds extra effective hopstand time to "Hopstand" additions; (2) when `calc_aroma_hop_utilization` is false, use the flat `aroma_hop_utilization_pct` instead of the Malowicki temperature model.

- [ ] **Step 1: Write failing tests**

Add to `ibu.rs` tests:

```rust
#[test]
fn test_whirlpool_time_adds_to_hopstand_ibu() {
    let hops_no_whirlpool = vec![HopIbuInput {
        alpha_pct: &10.0f64,
        amount_kg: &0.028f64,
        time_min: &20.0f64,
        use_type: "Hopstand",
        hopstand_temp_c: 80.0,
        whirlpool_time_min: 0.0,
        aroma_utilization_override: None,
    }];
    let hops_with_whirlpool = vec![HopIbuInput {
        alpha_pct: &10.0f64,
        amount_kg: &0.028f64,
        time_min: &20.0f64,
        use_type: "Hopstand",
        hopstand_temp_c: 80.0,
        whirlpool_time_min: 15.0,
        aroma_utilization_override: None,
    }];
    let ibu_no = tinseth_ibu(&hops_no_whirlpool, 1.047, 23.0, 60.0);
    let ibu_with = tinseth_ibu(&hops_with_whirlpool, 1.047, 23.0, 60.0);
    assert!(ibu_with > ibu_no, "whirlpool_time should increase hopstand IBU: {ibu_with} > {ibu_no}");
}

#[test]
fn test_aroma_utilization_override_flat_pct() {
    // With override at 23%, result should differ from Malowicki at 80°C for same time
    let hops_malowicki = vec![HopIbuInput {
        alpha_pct: &10.0f64,
        amount_kg: &0.028f64,
        time_min: &20.0f64,
        use_type: "Hopstand",
        hopstand_temp_c: 80.0,
        whirlpool_time_min: 0.0,
        aroma_utilization_override: None,
    }];
    let hops_flat = vec![HopIbuInput {
        alpha_pct: &10.0f64,
        amount_kg: &0.028f64,
        time_min: &20.0f64,
        use_type: "Hopstand",
        hopstand_temp_c: 80.0,
        whirlpool_time_min: 0.0,
        aroma_utilization_override: Some(0.50), // 50% flat — obviously different from Malowicki
    }];
    let ibu_m = tinseth_ibu(&hops_malowicki, 1.047, 23.0, 60.0);
    let ibu_flat = tinseth_ibu(&hops_flat, 1.047, 23.0, 60.0);
    assert!((ibu_m - ibu_flat).abs() > 0.5, "flat override should produce different result: malowicki={ibu_m:.2}, flat={ibu_flat:.2}");
}
```

- [ ] **Step 2: Run to confirm they fail**

```bash
cd src-tauri && cargo test brewing::ibu
```

Expected: compile errors — `HopIbuInput` missing new fields.

- [ ] **Step 3: Update HopIbuInput struct**

```rust
pub struct HopIbuInput<'a> {
    pub alpha_pct: &'a f64,
    pub amount_kg: &'a f64,
    pub time_min: &'a f64,
    pub use_type: &'a str,
    /// Pre-resolved: per-hop override → recipe default → 80.0
    pub hopstand_temp_c: f64,
    /// Extra whirlpool time added to hopstand additions, in minutes
    pub whirlpool_time_min: f64,
    /// When Some, use this flat utilization fraction instead of the Malowicki model
    pub aroma_utilization_override: Option<f64>,
}
```

- [ ] **Step 4: Update tinseth_ibu to use new fields**

Replace the `"hopstand"` arm in the match and the IBU calculation:

```rust
"hopstand" => {
    if let Some(flat_util) = h.aroma_utilization_override {
        let ounces = *h.amount_kg * 35.274;
        let alpha_fraction = *h.alpha_pct / 100.0;
        let volume_gallons = post_boil_volume_l * 0.264172;
        return (flat_util * alpha_fraction * ounces * 7490.0) / volume_gallons;
    }
    malowicki_effective_time(*h.time_min + h.whirlpool_time_min, h.hopstand_temp_c)
},
```

- [ ] **Step 5: Update all existing test structs to include new fields**

Every `HopIbuInput { ... }` in the test block needs:
```rust
whirlpool_time_min: 0.0,
aroma_utilization_override: None,
```

- [ ] **Step 6: Run IBU tests**

```bash
cd src-tauri && cargo test brewing::ibu
```

Expected: all tests pass.

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/brewing/ibu.rs
git commit -m "feat(brewing): add whirlpool_time and aroma_utilization_override to IBU calculation"
```

---

## Task 10: Wire New Fields in brewing/mod.rs

**Files:**
- Modify: `src-tauri/src/brewing/mod.rs`

- [ ] **Step 1: Write a failing test for mash_tun_loss wiring**

Add to the test block in `brewing/mod.rs`:

```rust
#[test]
fn test_mash_tun_loss_increases_pre_boil_via_equipment() {
    let mut recipe = minimal_recipe();
    recipe.fermentables = vec![pale_malt()];
    let mut eq = EquipmentProfile {
        id: "eq1".into(),
        name: "Test".into(),
        notes: None,
        boil_size_l: 27.0,
        batch_size_l: 23.0,
        calc_boil_volume: false,
        tun_volume_l: None,
        tun_weight_kg: None,
        tun_specific_heat: None,
        lauter_deadspace_l: 0.0,
        top_up_kettle_l: 0.0,
        trub_chiller_loss_l: 1.0,
        evap_rate_pct_hr: 10.0,
        boil_time_min: 60.0,
        top_up_water_l: 0.0,
        fermenter_loss_l: 1.0,
        hop_utilization_pct: 100.0,
        efficiency_pct: 72.0,
        batch_volume_target: "fermenter".into(),
        mash_tun_loss_l: 0.0,
        hlt_deadspace_l: None,
        cooling_shrinkage_pct: 4.0,
        calc_mash_efficiency: true,
        mash_efficiency_pct: None,
        calc_aroma_hop_utilization: true,
        aroma_hop_utilization_pct: 23.0,
        whirlpool_time_min: None,
        altitude_adjustment: false,
        boil_temp_f: None,
        sparge_method: "no_sparge".into(),
        mash_volume_min_l: None,
        mash_volume_max_l: None,
        sparge_volume_min_l: None,
        sparge_volume_max_l: None,
        calc_strike_water_temp: false,
        created_at: 0,
        updated_at: 0,
    };
    recipe.equipment_profile = Some(eq.clone());
    let stats_no_loss = calculate_stats(&recipe);

    eq.mash_tun_loss_l = 2.0;
    recipe.equipment_profile = Some(eq);
    let stats_with_loss = calculate_stats(&recipe);

    assert!(
        stats_with_loss.pre_boil_volume_l > stats_no_loss.pre_boil_volume_l + 1.5,
        "mash_tun_loss=2L should raise pre_boil by ~2L: no_loss={:.2}, with_loss={:.2}",
        stats_no_loss.pre_boil_volume_l,
        stats_with_loss.pre_boil_volume_l
    );
}
```

- [ ] **Step 2: Run to confirm the test fails**

```bash
cd src-tauri && cargo test test_mash_tun_loss_increases_pre_boil_via_equipment -- --nocapture
```

Expected: compile errors — `EquipmentProfile` is missing new fields.

- [ ] **Step 3: Add new constants**

At the top of `brewing/mod.rs`, alongside the existing constants:

```rust
const DEFAULT_MASH_TUN_LOSS_L: f64 = 0.0;
const DEFAULT_HLT_DEADSPACE_L: f64 = 0.0;
const DEFAULT_COOLING_SHRINKAGE_PCT: f64 = 4.0;
const DEFAULT_AROMA_HOP_UTILIZATION_PCT: f64 = 23.0;
```

- [ ] **Step 4: Update calculate_stats to wire new equipment fields**

Replace the equipment field extraction and volume calculation block (lines ~51–71):

```rust
let evaporation_rate = equipment
    .map(|e| e.evap_rate_pct_hr)
    .unwrap_or(DEFAULT_EVAP_RATE_PCT_HR);
let trub_chiller_loss = equipment
    .map(|e| e.trub_chiller_loss_l)
    .unwrap_or(DEFAULT_TRUB_CHILLER_LOSS_L);
let fermenter_loss = equipment
    .map(|e| e.fermenter_loss_l)
    .unwrap_or(DEFAULT_FERMENTER_LOSS_L);
let top_up_water = equipment
    .map(|e| e.top_up_water_l)
    .unwrap_or(DEFAULT_TOP_UP_WATER_L);
let mash_tun_loss = equipment
    .map(|e| e.mash_tun_loss_l)
    .unwrap_or(DEFAULT_MASH_TUN_LOSS_L);
let hlt_deadspace = equipment
    .and_then(|e| e.hlt_deadspace_l)
    .unwrap_or(DEFAULT_HLT_DEADSPACE_L);
let cooling_shrinkage = equipment
    .map(|e| e.cooling_shrinkage_pct)
    .unwrap_or(DEFAULT_COOLING_SHRINKAGE_PCT);
let aroma_hop_utilization_override: Option<f64> = equipment.and_then(|e| {
    if e.calc_aroma_hop_utilization {
        None
    } else {
        Some(e.aroma_hop_utilization_pct / 100.0)
    }
});
let whirlpool_time = equipment
    .and_then(|e| e.whirlpool_time_min)
    .unwrap_or(0.0);

let (pre_boil_volume_l, post_boil_volume_l, _total_water_l) = volumes::calculate_boil_volumes(
    recipe.batch_size_l,
    recipe.boil_time_min,
    evaporation_rate,
    trub_chiller_loss,
    fermenter_loss,
    top_up_water,
    mash_tun_loss,
    hlt_deadspace,
);
```

- [ ] **Step 5: Update the hop_inputs to pass new HopIbuInput fields**

Replace the `hop_inputs` construction:

```rust
let hop_inputs: Vec<ibu::HopIbuInput> = recipe
    .hops
    .iter()
    .map(|h| ibu::HopIbuInput {
        alpha_pct: &h.alpha_pct,
        amount_kg: &h.amount_kg,
        time_min: &h.time_min,
        use_type: &h.use_,
        hopstand_temp_c: h.hopstand_temp_c.unwrap_or(hopstand_default),
        whirlpool_time_min: whirlpool_time,
        aroma_utilization_override,
    })
    .collect();
```

- [ ] **Step 6: Update the existing test_stats_equipment_profile_used test struct**

The `EquipmentProfile` struct literal in `test_stats_equipment_profile_used` needs the new fields added (use the same values as the test in Step 1 above).

- [ ] **Step 7: Run all brewing tests**

```bash
cd src-tauri && cargo test brewing::
```

Expected: all tests pass.

- [ ] **Step 8: Commit**

```bash
git add src-tauri/src/brewing/mod.rs
git commit -m "feat(brewing): wire new equipment profile fields into stats calculation"
```

---

## Task 11: Frontend API

**Files:**
- Modify: `src/lib/api.ts`

- [ ] **Step 1: Add copyEquipmentProfile**

In `src/lib/api.ts`, alongside the other equipment API functions:

```typescript
export const copyEquipmentProfile = (id: string) =>
  invoke<EquipmentProfile>("copy_equipment_profile", { id });
```

- [ ] **Step 2: Verify TypeScript check passes**

```bash
bun run check
```

Expected: no type errors.

- [ ] **Step 3: Commit**

```bash
git add src/lib/api.ts
git commit -m "feat(api): add copyEquipmentProfile"
```

---

## Task 12: EquipmentProfileModal Component

**Files:**
- Create: `src/lib/components/EquipmentProfileModal.svelte`

This is a full-screen scrollable modal used for both create and edit. It receives an optional `profile` prop — when set, it pre-populates the form and calls `updateEquipmentProfile`; when null, it calls `createEquipmentProfile`.

- [ ] **Step 1: Create the component**

```svelte
<!-- src/lib/components/EquipmentProfileModal.svelte -->
<script lang="ts">
  import { createEquipmentProfile, updateEquipmentProfile } from "$lib/api";
  import type { EquipmentProfile, CreateEquipmentProfileInput, UpdateEquipmentProfileInput } from "$lib/api";
  import { ipc } from "$lib/stores/error";

  let {
    profile = null,
    onsave,
    oncancel,
  }: {
    profile?: EquipmentProfile | null;
    onsave: (saved: EquipmentProfile) => void;
    oncancel: () => void;
  } = $props();

  // ── form state ──────────────────────────────────────────────────────────
  let name = $state(profile?.name ?? "");
  let notes = $state(profile?.notes ?? "");
  let boilTimeMin = $state(profile?.boil_time_min ?? 60);

  // Volumes
  let batchVolumeTarget = $state(profile?.batch_volume_target ?? "fermenter");
  let batchSizeL = $state(profile?.batch_size_l ?? 23);
  let calcBoilVolume = $state(profile?.calc_boil_volume ?? true);
  let boilSizeL = $state(profile?.boil_size_l ?? 27);
  let evapRatePctHr = $state(profile?.evap_rate_pct_hr ?? 10);
  let trubChillerLossL = $state(profile?.trub_chiller_loss_l ?? 0);
  let lauterDeadspaceL = $state(profile?.lauter_deadspace_l ?? 0);
  let mashTunLossL = $state(profile?.mash_tun_loss_l ?? 0);
  let hltDeadspaceL = $state(profile?.hlt_deadspace_l ?? null);
  let fermenterLossL = $state(profile?.fermenter_loss_l ?? 0);
  let topUpWaterL = $state(profile?.top_up_water_l ?? 0);
  let coolingShrinakgePct = $state(profile?.cooling_shrinkage_pct ?? 4);

  // Efficiency
  let efficiencyPct = $state(profile?.efficiency_pct ?? 72);
  let calcMashEfficiency = $state(profile?.calc_mash_efficiency ?? true);
  let mashEfficiencyPct = $state(profile?.mash_efficiency_pct ?? null);

  // Hops
  let hopUtilizationPct = $state(profile?.hop_utilization_pct ?? 100);
  let calcAromaHopUtilization = $state(profile?.calc_aroma_hop_utilization ?? true);
  let aromaHopUtilizationPct = $state(profile?.aroma_hop_utilization_pct ?? 23);
  let whirlpoolTimeMin = $state(profile?.whirlpool_time_min ?? null);

  // Boil temperature
  let altitudeAdjustment = $state(profile?.altitude_adjustment ?? false);
  let boilTempF = $state(profile?.boil_temp_f ?? null);

  // Mash / Sparge
  let tunVolumeL = $state(profile?.tun_volume_l ?? null);
  let tunWeightKg = $state(profile?.tun_weight_kg ?? null);
  let tunSpecificHeat = $state(profile?.tun_specific_heat ?? null);
  let spargeMethod = $state(profile?.sparge_method ?? "no_sparge");
  let mashVolumeMinL = $state(profile?.mash_volume_min_l ?? null);
  let mashVolumeMaxL = $state(profile?.mash_volume_max_l ?? null);
  let spargeVolumeMinL = $state(profile?.sparge_volume_min_l ?? null);
  let spargeVolumeMaxL = $state(profile?.sparge_volume_max_l ?? null);
  let calcStrikeWaterTemp = $state(profile?.calc_strike_water_temp ?? false);

  let saving = $state(false);

  // ── derived display values ───────────────────────────────────────────────
  let postBoilColdL = $derived(batchSizeL + trubChillerLossL + fermenterLossL - topUpWaterL);
  let boilHours = $derived(boilTimeMin / 60);
  let evapFraction = $derived(evapRatePctHr / 100 * boilHours);
  let preBoilColdL = $derived(postBoilColdL / (1 - evapFraction) + mashTunLossL);
  let preBoilHotL = $derived(preBoilColdL * (1 + coolingShrinakgePct / 100));
  let postBoilHotL = $derived(postBoilColdL * (1 + coolingShrinakgePct / 100));
  let evapPct = $derived(evapRatePctHr * boilHours);
  let batchLabel = $derived(batchVolumeTarget === "kettle" ? "Batch Volume (Kettle)" : "Batch Volume (Fermenter)");

  async function handleSave() {
    saving = true;
    const base = {
      name,
      notes: notes || undefined,
      boil_time_min: boilTimeMin,
      batch_volume_target: batchVolumeTarget,
      batch_size_l: batchSizeL,
      calc_boil_volume: calcBoilVolume,
      boil_size_l: calcBoilVolume ? preBoilColdL : boilSizeL,
      evap_rate_pct_hr: evapRatePctHr,
      trub_chiller_loss_l: trubChillerLossL,
      lauter_deadspace_l: lauterDeadspaceL,
      mash_tun_loss_l: mashTunLossL,
      hlt_deadspace_l: hltDeadspaceL ?? undefined,
      fermenter_loss_l: fermenterLossL,
      top_up_water_l: topUpWaterL,
      cooling_shrinkage_pct: coolingShrinakgePct,
      efficiency_pct: efficiencyPct,
      calc_mash_efficiency: calcMashEfficiency,
      mash_efficiency_pct: mashEfficiencyPct ?? undefined,
      hop_utilization_pct: hopUtilizationPct,
      calc_aroma_hop_utilization: calcAromaHopUtilization,
      aroma_hop_utilization_pct: aromaHopUtilizationPct,
      whirlpool_time_min: whirlpoolTimeMin ?? undefined,
      altitude_adjustment: altitudeAdjustment,
      boil_temp_f: boilTempF ?? undefined,
      tun_volume_l: tunVolumeL ?? undefined,
      tun_weight_kg: tunWeightKg ?? undefined,
      tun_specific_heat: tunSpecificHeat ?? undefined,
      sparge_method: spargeMethod,
      mash_volume_min_l: mashVolumeMinL ?? undefined,
      mash_volume_max_l: mashVolumeMaxL ?? undefined,
      sparge_volume_min_l: spargeVolumeMinL ?? undefined,
      sparge_volume_max_l: spargeVolumeMaxL ?? undefined,
      calc_strike_water_temp: calcStrikeWaterTemp,
    };

    const saved = profile
      ? await ipc(updateEquipmentProfile(profile.id, base as UpdateEquipmentProfileInput))
      : await ipc(createEquipmentProfile(base as CreateEquipmentProfileInput));

    saving = false;
    if (saved) onsave(saved);
  }

  function numInput(e: Event) {
    return parseFloat((e.target as HTMLInputElement).value) || 0;
  }
  function nullableNumInput(e: Event) {
    const v = parseFloat((e.target as HTMLInputElement).value);
    return isNaN(v) ? null : v;
  }
</script>

<!-- Backdrop -->
<div class="fixed inset-0 z-50 flex items-start justify-center overflow-y-auto py-8"
     style="background: rgba(0,0,0,0.6);"
     onclick={(e) => e.target === e.currentTarget && oncancel()}>

  <div class="w-full max-w-2xl rounded-lg shadow-xl flex flex-col"
       style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">

    <!-- Header row -->
    <div class="flex items-center justify-between px-6 py-4 border-b" style="border-color: var(--color-border);">
      <h2 class="text-base font-semibold" style="color: var(--color-text-primary);">
        {profile ? "Edit Equipment Profile" : "New Equipment Profile"}
      </h2>
      <button onclick={oncancel} class="text-lg leading-none" style="color: var(--color-text-secondary);">✕</button>
    </div>

    <!-- Body -->
    <div class="px-6 py-4 flex flex-col gap-6 overflow-y-auto">

      <!-- Name / Boil Time / Description -->
      <div class="grid grid-cols-2 gap-4">
        <div class="flex flex-col gap-1">
          <label class="text-xs" style="color: var(--color-text-secondary);">Name</label>
          <input type="text" bind:value={name} class="field-input" />
        </div>
        <div class="flex flex-col gap-1">
          <label class="text-xs" style="color: var(--color-text-secondary);">Boil Time <span style="color: var(--color-text-tertiary);">min</span></label>
          <input type="number" value={boilTimeMin} oninput={(e) => boilTimeMin = numInput(e)} class="field-input" />
        </div>
        <div class="col-span-2 flex flex-col gap-1">
          <label class="text-xs" style="color: var(--color-text-secondary);">Description</label>
          <input type="text" bind:value={notes} class="field-input" />
        </div>
      </div>

      <!-- Volumes -->
      <section>
        <h3 class="section-label">Volumes</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Batch Volume Target</label>
            <select bind:value={batchVolumeTarget} class="field-input">
              <option value="fermenter">Fermenter</option>
              <option value="kettle">Kettle</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">{batchLabel} <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.1" value={batchSizeL} oninput={(e) => batchSizeL = numInput(e)} class="field-input" />
          </div>

          <div class="flex items-center gap-2 col-span-1">
            <input type="checkbox" id="calc-boil" bind:checked={calcBoilVolume} />
            <label for="calc-boil" class="text-sm" style="color: var(--color-text-primary);">Calc boil volume</label>
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Pre-Boil Volume* <span style="color: var(--color-text-tertiary);">L</span></label>
            {#if calcBoilVolume}
              <div class="field-display">{preBoilHotL.toFixed(2)} <span style="color: var(--color-text-tertiary);">(hot)</span></div>
            {:else}
              <input type="number" step="0.1" value={boilSizeL} oninput={(e) => boilSizeL = numInput(e)} class="field-input" />
            {/if}
          </div>

          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Boil Off <span style="color: var(--color-text-tertiary);">({evapPct.toFixed(1)}%)</span> <span style="color: var(--color-text-tertiary);">L/hr</span></label>
            <input type="number" step="0.1" value={evapRatePctHr} oninput={(e) => evapRatePctHr = numInput(e)} class="field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Trub/Chiller Loss <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.01" value={trubChillerLossL} oninput={(e) => trubChillerLossL = numInput(e)} class="field-input" />
          </div>

          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Mash-Tun Deadspace <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.01" value={lauterDeadspaceL} oninput={(e) => lauterDeadspaceL = numInput(e)} class="field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Mash-Tun Loss <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.01" value={mashTunLossL} oninput={(e) => mashTunLossL = numInput(e)} class="field-input" />
          </div>

          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">HLT Deadspace <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.01" placeholder="optional"
                   value={hltDeadspaceL ?? ""} oninput={(e) => hltDeadspaceL = nullableNumInput(e)} class="field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Fermenter Loss <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.01" value={fermenterLossL} oninput={(e) => fermenterLossL = numInput(e)} class="field-input" />
          </div>

          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Fermenter Top-Up <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.01" placeholder="optional"
                   value={topUpWaterL || ""} oninput={(e) => topUpWaterL = numInput(e)} class="field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Cooling Shrinkage <span style="color: var(--color-text-tertiary);">%</span></label>
            <input type="number" step="0.1" value={coolingShrinakgePct} oninput={(e) => coolingShrinakgePct = numInput(e)} class="field-input" />
          </div>
        </div>
        <p class="text-xs mt-2 text-right" style="color: var(--color-text-tertiary);">
          Post-Boil Kettle: {postBoilHotL.toFixed(2)} L &nbsp;·&nbsp; *Pre-Boil is <span style="color: #e07b54;">hot</span> (incl. {coolingShrinakgePct}% expansion)
        </p>
      </section>

      <!-- Efficiency -->
      <section>
        <h3 class="section-label">Efficiency</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Brewhouse Efficiency <span style="color: var(--color-text-tertiary);">%</span></label>
            <input type="number" step="0.1" value={efficiencyPct} oninput={(e) => efficiencyPct = numInput(e)} class="field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Mash Efficiency <span style="color: var(--color-text-tertiary);">%</span></label>
            {#if calcMashEfficiency}
              <div class="field-display" style="color: var(--color-text-tertiary);">calculated</div>
            {:else}
              <input type="number" step="0.1" placeholder="optional"
                     value={mashEfficiencyPct ?? ""} oninput={(e) => mashEfficiencyPct = nullableNumInput(e)} class="field-input" />
            {/if}
          </div>
          <div class="flex items-center gap-2 col-span-2">
            <input type="checkbox" id="calc-mash-eff" bind:checked={calcMashEfficiency} />
            <label for="calc-mash-eff" class="text-sm" style="color: var(--color-text-primary);">Calc mash efficiency</label>
          </div>
        </div>
      </section>

      <!-- Hops -->
      <section>
        <h3 class="section-label">Hops</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Hop Utilization Multiplier <span style="color: var(--color-text-tertiary);">%</span></label>
            <input type="number" step="1" value={hopUtilizationPct} oninput={(e) => hopUtilizationPct = numInput(e)} class="field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Aroma Hop Utilization <span style="color: var(--color-text-tertiary);">%</span></label>
            {#if calcAromaHopUtilization}
              <div class="field-display" style="color: var(--color-text-tertiary);">calculated</div>
            {:else}
              <input type="number" step="0.1" value={aromaHopUtilizationPct} oninput={(e) => aromaHopUtilizationPct = numInput(e)} class="field-input" />
            {/if}
          </div>
          <div class="flex items-center gap-2 col-span-2">
            <input type="checkbox" id="calc-aroma" bind:checked={calcAromaHopUtilization} />
            <label for="calc-aroma" class="text-sm" style="color: var(--color-text-primary);">Calc aroma hop utilization</label>
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Whirlpool / No-Chill Time <span style="color: var(--color-text-tertiary);">min</span></label>
            <input type="number" step="1" placeholder="optional"
                   value={whirlpoolTimeMin ?? ""} oninput={(e) => whirlpoolTimeMin = nullableNumInput(e)} class="field-input" />
          </div>
        </div>
      </section>

      <!-- Boil Temperature -->
      <section>
        <h3 class="section-label">Boil Temperature</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex items-center gap-2 col-span-2">
            <input type="checkbox" id="altitude-adj" bind:checked={altitudeAdjustment} />
            <label for="altitude-adj" class="text-sm" style="color: var(--color-text-primary);">Altitude adjustment</label>
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Boil Temperature <span style="color: var(--color-text-tertiary);">°F</span></label>
            {#if altitudeAdjustment}
              <div class="field-display" style="color: var(--color-text-tertiary);">calculated from altitude</div>
            {:else}
              <input type="number" step="1" placeholder="212"
                     value={boilTempF ?? ""} oninput={(e) => boilTempF = nullableNumInput(e)} class="field-input" />
            {/if}
          </div>
        </div>
      </section>

      <!-- Mash / Sparge Water -->
      <section>
        <h3 class="section-label">Mash / Sparge Water</h3>
        <div class="grid grid-cols-2 gap-4">
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Tun Volume <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.1" placeholder="optional"
                   value={tunVolumeL ?? ""} oninput={(e) => tunVolumeL = nullableNumInput(e)} class="field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Tun Weight <span style="color: var(--color-text-tertiary);">kg</span></label>
            <input type="number" step="0.1" placeholder="optional"
                   value={tunWeightKg ?? ""} oninput={(e) => tunWeightKg = nullableNumInput(e)} class="field-input" />
          </div>
          <div class="col-span-2 flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Sparge Method</label>
            <select bind:value={spargeMethod} class="field-input">
              <option value="no_sparge">No Sparge</option>
              <option value="batch_sparge">Batch Sparge</option>
              <option value="fly_sparge">Fly Sparge</option>
            </select>
          </div>

          <div class="col-span-2 text-xs font-medium mt-1" style="color: var(--color-text-secondary);">Mash Volume Limits</div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Min <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.1" placeholder="optional"
                   value={mashVolumeMinL ?? ""} oninput={(e) => mashVolumeMinL = nullableNumInput(e)} class="field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Max <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.1" placeholder="optional"
                   value={mashVolumeMaxL ?? ""} oninput={(e) => mashVolumeMaxL = nullableNumInput(e)} class="field-input" />
          </div>

          <div class="col-span-2 text-xs font-medium mt-1" style="color: var(--color-text-secondary);">Sparge Volume Limits</div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Min <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.1" placeholder="optional"
                   value={spargeVolumeMinL ?? ""} oninput={(e) => spargeVolumeMinL = nullableNumInput(e)} class="field-input" />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs" style="color: var(--color-text-secondary);">Max <span style="color: var(--color-text-tertiary);">L</span></label>
            <input type="number" step="0.1" placeholder="optional"
                   value={spargeVolumeMaxL ?? ""} oninput={(e) => spargeVolumeMaxL = nullableNumInput(e)} class="field-input" />
          </div>

          <div class="flex items-center gap-2 col-span-2 mt-1">
            <input type="checkbox" id="calc-strike" bind:checked={calcStrikeWaterTemp} />
            <label for="calc-strike" class="text-sm" style="color: var(--color-text-primary);">Calc strike water temperature</label>
          </div>
        </div>
      </section>

    </div>

    <!-- Footer -->
    <div class="flex justify-end gap-3 px-6 py-4 border-t" style="border-color: var(--color-border);">
      <button onclick={oncancel} class="px-4 py-2 rounded text-sm"
              style="background: var(--color-bg-base); color: var(--color-text-secondary); border: 1px solid var(--color-border);">
        Cancel
      </button>
      <button onclick={handleSave} disabled={saving || !name.trim()} class="px-4 py-2 rounded text-sm"
              style="background: var(--color-accent); color: #fff; opacity: {saving || !name.trim() ? 0.5 : 1};">
        {saving ? "Saving…" : "Save"}
      </button>
    </div>

  </div>
</div>

<style>
  :global(.field-input) {
    width: 100%;
    padding: 0.375rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.875rem;
    background: var(--color-bg-base);
    color: var(--color-text-primary);
    border: 1px solid var(--color-border);
  }
  :global(.field-display) {
    padding: 0.375rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.875rem;
    background: var(--color-bg-base);
    border: 1px solid var(--color-border);
    opacity: 0.6;
  }
  .section-label {
    font-size: 0.7rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--color-text-secondary);
    margin-bottom: 0.75rem;
  }
</style>
```

- [ ] **Step 2: Verify TypeScript check passes**

```bash
bun run check
```

Expected: no type errors.

- [ ] **Step 3: Commit**

```bash
git add src/lib/components/EquipmentProfileModal.svelte
git commit -m "feat(ui): add EquipmentProfileModal component"
```

---

## Task 13: Equipment Page — Edit/Copy Actions + Tests

**Files:**
- Modify: `src/routes/equipment/+page.svelte`
- Modify: `tests/EquipmentPage.test.ts`

- [ ] **Step 1: Update the failing test first**

Replace the contents of `tests/EquipmentPage.test.ts`:

```typescript
import { describe, it, expect, vi } from "vitest";
import { render, fireEvent } from "@testing-library/svelte";
import { tick } from "svelte";
import EquipmentPage from "../src/routes/equipment/+page.svelte";

const mockProfile = {
  id: "1",
  name: "My Kettle",
  batch_size_l: 23,
  boil_size_l: 27,
  efficiency_pct: 72,
  boil_time_min: 60,
  calc_boil_volume: false,
  evap_rate_pct_hr: 10,
  trub_chiller_loss_l: 1,
  fermenter_loss_l: 1,
  top_up_water_l: 0,
  lauter_deadspace_l: 0,
  top_up_kettle_l: 0,
  hop_utilization_pct: 100,
  batch_volume_target: "fermenter",
  mash_tun_loss_l: 0,
  hlt_deadspace_l: null,
  cooling_shrinkage_pct: 4,
  calc_mash_efficiency: true,
  mash_efficiency_pct: null,
  calc_aroma_hop_utilization: true,
  aroma_hop_utilization_pct: 23,
  whirlpool_time_min: null,
  altitude_adjustment: false,
  boil_temp_f: null,
  sparge_method: "no_sparge",
  mash_volume_min_l: null,
  mash_volume_max_l: null,
  sparge_volume_min_l: null,
  sparge_volume_max_l: null,
  calc_strike_water_temp: false,
  tun_volume_l: null,
  tun_weight_kg: null,
  tun_specific_heat: null,
  notes: null,
  created_at: 0,
  updated_at: 0,
};

vi.mock("$lib/api", () => ({
  listEquipmentProfiles: vi.fn().mockResolvedValue([mockProfile]),
  createEquipmentProfile: vi.fn().mockResolvedValue(mockProfile),
  updateEquipmentProfile: vi.fn().mockResolvedValue(mockProfile),
  deleteEquipmentProfile: vi.fn().mockResolvedValue({}),
  copyEquipmentProfile: vi.fn().mockResolvedValue({ ...mockProfile, id: "2", name: "My Kettle (copy)" }),
}));

vi.mock("$lib/stores/settings", () => ({
  settings: { subscribe: vi.fn((fn) => { fn({ theme: "midnight", units: "metric", default_equipment_profile_id: "" }); return () => {}; }) },
  loadSettings: vi.fn().mockResolvedValue(undefined),
  saveSetting: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("$lib/stores/error", () => ({
  ipc: vi.fn((p) => p),
}));

async function mountAndWait() {
  const result = render(EquipmentPage);
  await new Promise((r) => setTimeout(r, 10));
  await tick();
  await tick();
  return result;
}

describe("EquipmentPage", () => {
  it("renders the page heading", async () => {
    const { getByText } = render(EquipmentPage);
    expect(getByText("Equipment")).toBeInTheDocument();
  });

  it("renders the Default Profile label", async () => {
    const { getByText } = render(EquipmentPage);
    expect(getByText("Default Profile")).toBeInTheDocument();
  });

  it("renders the new profile name input", async () => {
    const { getByPlaceholderText } = render(EquipmentPage);
    expect(getByPlaceholderText("New profile name")).toBeInTheDocument();
  });

  it("renders the Add button", async () => {
    const { getByText } = render(EquipmentPage);
    expect(getByText("Add")).toBeInTheDocument();
  });

  it("renders loaded profile name and details after onMount", async () => {
    const { getByText } = await mountAndWait();
    expect(getByText(/23L batch · 72% efficiency/)).toBeInTheDocument();
  });

  it("renders an Edit button for each profile", async () => {
    const { getAllByText } = await mountAndWait();
    expect(getAllByText("Edit").length).toBeGreaterThan(0);
  });

  it("renders a Copy button for each profile", async () => {
    const { getAllByText } = await mountAndWait();
    expect(getAllByText("Copy").length).toBeGreaterThan(0);
  });

  it("clicking Copy calls copyEquipmentProfile", async () => {
    const api = await import("$lib/api");
    const { getAllByText } = await mountAndWait();
    const copyBtn = getAllByText("Copy")[0];
    await fireEvent.click(copyBtn);
    await tick();
    expect(api.copyEquipmentProfile).toHaveBeenCalledWith("1");
  });
});
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
bun run test -- EquipmentPage
```

Expected: "Edit" and "Copy" button tests fail — buttons don't exist yet.

- [ ] **Step 3: Update the equipment page**

Replace the contents of `src/routes/equipment/+page.svelte`:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { settings, loadSettings, saveSetting } from "$lib/stores/settings";
  import {
    listEquipmentProfiles,
    createEquipmentProfile,
    copyEquipmentProfile,
    deleteEquipmentProfile,
  } from "$lib/api";
  import type { EquipmentProfile } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import ConfirmModal from "$lib/components/ConfirmModal.svelte";
  import EquipmentProfileModal from "$lib/components/EquipmentProfileModal.svelte";

  let profiles = $state<EquipmentProfile[]>([]);
  let newProfileName = $state("");
  let showDeleteModal = $state(false);
  let deleteCandidate = $state<EquipmentProfile | null>(null);
  let editingProfile = $state<EquipmentProfile | null>(null);
  let showNewModal = $state(false);

  onMount(async () => {
    await ipc(loadSettings());
    profiles = await ipc(listEquipmentProfiles()) ?? [];
  });

  async function refreshProfiles() {
    profiles = await ipc(listEquipmentProfiles()) ?? profiles;
  }

  async function handleDefaultEquipChange(e: Event) {
    await ipc(saveSetting("default_equipment_profile_id", (e.target as HTMLSelectElement).value));
  }

  async function handleAddProfile() {
    if (!newProfileName.trim()) return;
    await ipc(createEquipmentProfile({
      name: newProfileName,
      boil_size_l: 27.0,
      batch_size_l: 23.0,
      efficiency_pct: 72.0,
    }));
    await refreshProfiles();
    newProfileName = "";
  }

  async function handleCopyProfile(profile: EquipmentProfile) {
    await ipc(copyEquipmentProfile(profile.id));
    await refreshProfiles();
  }

  function handleEditProfile(profile: EquipmentProfile) {
    editingProfile = profile;
  }

  function handleDeleteProfile(profile: EquipmentProfile) {
    deleteCandidate = profile;
    showDeleteModal = true;
  }

  async function confirmDelete() {
    if (!deleteCandidate) return;
    showDeleteModal = false;
    await ipc(deleteEquipmentProfile(deleteCandidate.id));
    await refreshProfiles();
    deleteCandidate = null;
  }

  function cancelDelete() {
    showDeleteModal = false;
    deleteCandidate = null;
  }

  async function handleModalSave(saved: EquipmentProfile) {
    editingProfile = null;
    showNewModal = false;
    await refreshProfiles();
  }

  function handleModalCancel() {
    editingProfile = null;
    showNewModal = false;
  }
</script>

{#if showDeleteModal && deleteCandidate}
  <ConfirmModal
    message="Delete this equipment profile? This cannot be undone."
    confirmLabel="Delete"
    dangerous={true}
    onconfirm={confirmDelete}
    oncancel={cancelDelete}
  />
{/if}

{#if editingProfile}
  <EquipmentProfileModal
    profile={editingProfile}
    onsave={handleModalSave}
    oncancel={handleModalCancel}
  />
{/if}

{#if showNewModal}
  <EquipmentProfileModal
    onsave={handleModalSave}
    oncancel={handleModalCancel}
  />
{/if}

<div class="flex-1 overflow-y-auto p-6" style="background: var(--color-bg-base);">
  <h1 class="text-lg font-semibold mb-6" style="color: var(--color-text-primary);">Equipment</h1>

  <div class="flex flex-col gap-6 max-w-md">
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-semibold" style="color: var(--color-text-secondary);">Equipment Profiles</h2>
      <div class="flex items-center justify-between">
        <label for="select-default-profile" class="text-sm" style="color: var(--color-text-primary);">Default Profile</label>
        <select id="select-default-profile" value={$settings.default_equipment_profile_id ?? ""}
                onchange={handleDefaultEquipChange}
                class="px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="">None</option>
          {#each profiles as p}
            <option value={p.id}>{p.name}</option>
          {/each}
        </select>
      </div>

      {#each profiles as p (p.id)}
        <div class="flex items-center justify-between py-1 border-t" style="border-color: var(--color-border);">
          <div>
            <p class="text-sm" style="color: var(--color-text-primary);">{p.name}</p>
            <p class="text-xs" style="color: var(--color-text-secondary);">
              {p.batch_size_l}L batch · {p.efficiency_pct}% efficiency
            </p>
          </div>
          <div class="flex gap-2">
            <button onclick={() => handleEditProfile(p)} class="text-xs px-2 py-1 rounded"
                    style="color: var(--color-text-secondary); background: var(--color-bg-elevated);">Edit</button>
            <button onclick={() => handleCopyProfile(p)} class="text-xs px-2 py-1 rounded"
                    style="color: var(--color-text-secondary); background: var(--color-bg-elevated);">Copy</button>
            <button onclick={() => handleDeleteProfile(p)} class="text-xs px-2 py-1 rounded"
                    style="color: var(--color-text-secondary); background: var(--color-bg-elevated);">Delete</button>
          </div>
        </div>
      {/each}

      <div class="flex gap-2 pt-1">
        <input type="text" bind:value={newProfileName} placeholder="New profile name"
               class="flex-1 px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
        <button onclick={handleAddProfile} class="text-xs px-3 py-1.5 rounded"
                style="background: var(--color-accent); color: #fff;">Add</button>
      </div>
    </section>
  </div>
</div>
```

- [ ] **Step 4: Run the frontend tests**

```bash
bun run test -- EquipmentPage
```

Expected: all 8 tests pass.

- [ ] **Step 5: Run the full test suite**

```bash
just test
```

Expected: all Rust and frontend tests pass.

- [ ] **Step 6: Commit**

```bash
git add src/routes/equipment/+page.svelte tests/EquipmentPage.test.ts
git commit -m "feat(ui): add Edit and Copy actions to equipment profile list"
```

---

## Done

At this point:
- 17 new fields are in the database (migration 003)
- All Rust and TypeScript types are regenerated from the updated OpenAPI spec
- The repository handles create, update (all fields), and copy
- `copy_equipment_profile` is a registered Tauri command
- Volume calculations use `mash_tun_loss_l` and `hlt_deadspace_l`; `hot_volume()` helper replaces the implicit 4% constant
- IBU calculations respect `whirlpool_time_min` and `aroma_utilization_override`
- `brewing/mod.rs` wires all new equipment fields into `calculate_stats`
- The equipment list page has Edit, Copy, and Delete per profile
- `EquipmentProfileModal` handles create and full edit with all fields
- All tests pass (`just test`)
