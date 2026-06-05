# Batch View: Stage-Driven Redesign — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Collapse the two-tab-bar batch view into a single stage tab bar (Planned | Brewing | Fermenting | Packaged), embed the gravity log in the Fermenting stage, embed tasting in the Packaged stage, surface Attachments via a header button/modal, and remove the Conditioning status entirely.

**Architecture:** `BatchOverviewTab.svelte` absorbs `BatchGravityTab` and `BatchTastingTab` as inline sections that appear only for their relevant stage. `desktop/BatchView.svelte` removes the outer tab bar and adds an Attachments overlay. `mobile/BatchView.svelte` removes its separate gravity/tasting sections since they're now inside `BatchOverviewTab`. A SQL migration migrates any existing `conditioning` batches to `packaged`.

**Tech Stack:** Svelte 5 (runes), TypeScript, Vitest + @testing-library/svelte, SeaORM (SQLite), Tauri IPC

---

## File Map

| File | Action | What changes |
|------|--------|--------------|
| `src-tauri/migrations/012_migrate_conditioning_to_packaged.sql` | Create | SQL UPDATE migrating conditioning → packaged |
| `src-tauri/src/repositories/batches.rs` | Modify | Update one Rust test that asserts `status = "conditioning"` |
| `docs/openapi/components/schemas/Batch.yaml` | Modify | Remove "conditioning" from status description |
| `docs/openapi/components/schemas/BatchSummary.yaml` | Modify | Remove "conditioning" from status description |
| `src/lib/components/BatchList.svelte` | Modify | Fix STATUS_LABELS/STATUS_COLORS (remove "conditioning"/"complete", those values were wrong) |
| `tests/BatchCarbonationSection.test.ts` | Modify | Change `status: "conditioning"` to `status: "packaged"` in `makeBatch` |
| `src/lib/components/batch/BatchOverviewTab.svelte` | Modify | Major: remove conditioning, add onRefresh prop, embed gravity + tasting inline |
| `tests/BatchOverviewTab.test.ts` | Create | Tests verifying gravity shown for fermenting, tasting for packaged, neither for others |
| `src/lib/desktop/BatchView.svelte` | Modify | Remove outer tabs, pass onRefresh to BatchOverviewTab, add Attachments modal |
| `src/lib/mobile/BatchView.svelte` | Modify | Remove separate gravity/tasting sections, pass onRefresh to BatchOverviewTab |

---

## Task 1: DB migration — conditioning → packaged

**Files:**
- Create: `src-tauri/migrations/012_migrate_conditioning_to_packaged.sql`
- Modify: `src-tauri/src/repositories/batches.rs`

- [ ] **Step 1.1: Write the migration**

Create `src-tauri/migrations/012_migrate_conditioning_to_packaged.sql`:

```sql
-- Merge "conditioning" status into "packaged". The conditioning_date column
-- is kept so existing data is not lost, but the status value is retired.
UPDATE batches SET status = 'packaged' WHERE status = 'conditioning';
```

- [ ] **Step 1.2: Update the Rust test that asserts "conditioning" status**

In `src-tauri/src/repositories/batches.rs`, find `test_update_conditioning_date_and_notes` (around line 366). Rename it and change the status values:

```rust
// Rename the fn:
#[tokio::test]
async fn test_update_packaging_date_and_notes() {   // was test_update_conditioning_date_and_notes

// Then inside the test (~381-390):
let updated = repo
    .update_batch(
        &batch.id,
        UpdateBatchInput {
            status: Some("packaged".into()),        // was "conditioning"
            conditioning_date: Some(1_700_000_000),
            ..Default::default()
        },
    )
    .await
    .unwrap();

assert_eq!(updated.status, "packaged");            // was "conditioning"
assert_eq!(updated.conditioning_date, Some(1_700_000_000));
```

- [ ] **Step 1.3: Run Rust tests to verify they pass**

```bash
cd src-tauri && cargo test repositories::batches --features test -- --nocapture 2>&1 | tail -20
```

Expected: all batch repository tests pass.

- [ ] **Step 1.4: Commit**

```bash
git add src-tauri/migrations/012_migrate_conditioning_to_packaged.sql src-tauri/src/repositories/batches.rs
git commit -m "feat: migrate conditioning status to packaged, remove conditioning from status set"
```

---

## Task 2: OpenAPI spec — remove conditioning from status description

**Files:**
- Modify: `docs/openapi/components/schemas/Batch.yaml` (line 24)
- Modify: `docs/openapi/components/schemas/BatchSummary.yaml`

- [ ] **Step 2.1: Update Batch.yaml**

In `docs/openapi/components/schemas/Batch.yaml`, change line 24:

```yaml
# Before:
    description: "planned | brewing | fermenting | conditioning | packaged"

# After:
    description: "planned | brewing | fermenting | packaged"
```

- [ ] **Step 2.2: Update BatchSummary.yaml**

Open `docs/openapi/components/schemas/BatchSummary.yaml` and make the same change to the status field description (remove `| conditioning`).

- [ ] **Step 2.3: Commit**

```bash
git add docs/openapi/components/schemas/Batch.yaml docs/openapi/components/schemas/BatchSummary.yaml
git commit -m "docs: remove conditioning from batch status description in OpenAPI spec"
```

---

## Task 3: Fix BatchList status label/color map

**Files:**
- Modify: `src/lib/components/BatchList.svelte` (lines 16–30)

- [ ] **Step 3.1: Fix STATUS_LABELS and STATUS_COLORS**

In `src/lib/components/BatchList.svelte`, replace the STATUS_LABELS and STATUS_COLORS objects:

```typescript
// Remove "complete" (wrong key — never a real status) and "conditioning".
// Result: exactly the four valid statuses.
const STATUS_LABELS: Record<string, string> = {
  planned: "Planned",
  brewing: "Brewing",
  fermenting: "Fermenting",
  packaged: "Packaged",
};

const STATUS_COLORS: Record<string, string> = {
  planned: "var(--color-text-muted)",
  brewing: "#f59e0b",
  fermenting: "#10b981",
  packaged: "#3b82f6",
};
```

- [ ] **Step 3.2: Run frontend tests**

```bash
npm run test -- --reporter=verbose 2>&1 | tail -30
```

Expected: all tests pass (no test directly covers STATUS_LABELS, but nothing should break).

- [ ] **Step 3.3: Commit**

```bash
git add src/lib/components/BatchList.svelte
git commit -m "fix: correct BatchList status labels — remove invalid 'complete' and 'conditioning' keys"
```

---

## Task 4: Fix BatchCarbonationSection test

**Files:**
- Modify: `tests/BatchCarbonationSection.test.ts` (line 30)

- [ ] **Step 4.1: Update the makeBatch default status**

In `tests/BatchCarbonationSection.test.ts`, change `status: "conditioning"` to `status: "packaged"` in the `makeBatch` helper (line 30):

```typescript
// Before:
    status: "conditioning",

// After:
    status: "packaged",
```

- [ ] **Step 4.2: Run the carbonation tests**

```bash
npm run test -- BatchCarbonationSection --reporter=verbose 2>&1 | tail -20
```

Expected: all tests in `BatchCarbonationSection.test.ts` pass.

- [ ] **Step 4.3: Commit**

```bash
git add tests/BatchCarbonationSection.test.ts
git commit -m "fix: update BatchCarbonationSection test to use packaged status (conditioning removed)"
```

---

## Task 5: Refactor BatchOverviewTab — core of the redesign

This task refactors `BatchOverviewTab.svelte` to:
- Add an `onRefresh` prop (needed to pass to embedded `BatchGravityTab`)
- Remove `"conditioning"` from STATUSES, HIGHLIGHTED, stageTargets, and onStatusChange
- Embed `BatchGravityTab` inline when `batch.status === "fermenting"`
- Embed `BatchTastingTab` inline when `batch.status === "packaged"`
- Remove `conditioning_date` from the Dates section
- Show carbonation only for `"packaged"` (was `"conditioning" || "packaged"`)

**Files:**
- Modify: `src/lib/components/batch/BatchOverviewTab.svelte`

- [ ] **Step 5.1: Write a failing test first**

Create `tests/BatchOverviewTab.test.ts`:

```typescript
// tests/BatchOverviewTab.test.ts
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { tick } from "svelte";
import BatchOverviewTab from "$lib/components/batch/BatchOverviewTab.svelte";
import type { Batch, UpdateBatchInput } from "$lib/api";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("$lib/stores/error", () => ({ ipc: vi.fn((p) => p) }));
vi.mock("$lib/stores/settings", () => ({
  settings: {
    subscribe: vi.fn((fn) => { fn({ gravity_unit: "sg" }); return () => {}; }),
  },
}));

function makeBatch(status: string): Batch {
  return {
    id: "b1",
    recipe_id: "r1",
    recipe_name: "Test IPA",
    recipe_version_id: "v1",
    name: null,
    status,
    brew_date: null,
    fermenter_date: null,
    conditioning_date: null,
    packaging_date: null,
    actual_pre_boil_volume_l: null,
    actual_post_boil_volume_l: null,
    actual_batch_size_l: null,
    actual_pre_boil_gravity: null,
    actual_og: null,
    actual_fg: null,
    notes: null,
    rating: null,
    planned_og: null,
    planned_fg: null,
    planned_pre_boil_gravity: null,
    planned_post_boil_volume_l: null,
    planned_batch_size_l: null,
    packaging_temp_c: null,
    carbonation_sugar_type: null,
    priming_sugar_g: null,
    serving_pressure_kpa: null,
    gravity_readings: [],
    created_at: 0,
    updated_at: 0,
  } as unknown as Batch;
}

const noop = () => {};

describe("BatchOverviewTab", () => {
  it("shows gravity log section when fermenting", async () => {
    render(BatchOverviewTab, {
      batch: makeBatch("fermenting"),
      onUpdate: noop,
      onRefresh: noop,
    });
    await tick();
    expect(screen.getByText(/GRAVITY LOG/i)).toBeInTheDocument();
  });

  it("does not show gravity log section when planned", async () => {
    render(BatchOverviewTab, {
      batch: makeBatch("planned"),
      onUpdate: noop,
      onRefresh: noop,
    });
    await tick();
    expect(screen.queryByText(/GRAVITY LOG/i)).toBeNull();
  });

  it("shows tasting section when packaged", async () => {
    render(BatchOverviewTab, {
      batch: makeBatch("packaged"),
      onUpdate: noop,
      onRefresh: noop,
    });
    await tick();
    expect(screen.getByText(/TASTING/i)).toBeInTheDocument();
  });

  it("does not show tasting section when fermenting", async () => {
    render(BatchOverviewTab, {
      batch: makeBatch("fermenting"),
      onUpdate: noop,
      onRefresh: noop,
    });
    await tick();
    expect(screen.queryByText(/TASTING/i)).toBeNull();
  });

  it("does not include conditioning as a status option", () => {
    render(BatchOverviewTab, {
      batch: makeBatch("planned"),
      onUpdate: noop,
      onRefresh: noop,
    });
    // Mobile select shows the status options
    const options = screen.queryAllByRole("option");
    const values = options.map((o) => (o as HTMLOptionElement).value);
    expect(values).not.toContain("conditioning");
    expect(values).toEqual(["planned", "brewing", "fermenting", "packaged"]);
  });
});
```

- [ ] **Step 5.2: Run the test to confirm it fails**

```bash
npm run test -- BatchOverviewTab --reporter=verbose 2>&1 | tail -20
```

Expected: tests fail — `onRefresh` prop missing or gravity/tasting sections don't appear.

- [ ] **Step 5.3: Rewrite BatchOverviewTab.svelte**

Replace `src/lib/components/batch/BatchOverviewTab.svelte` with:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import TabBar from "$lib/components/TabBar.svelte";
  import type { Batch, UpdateBatchInput, RecipeVersionSummary, Recipe } from "$lib/api";
  import { listRecipeVersions, getRecipe, convertGravity } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { formatSg, gravityStep } from "$lib/gravity-display";
  import BatchCarbonationSection from "$lib/components/batch/BatchCarbonationSection.svelte";
  import BatchGravityTab from "$lib/components/batch/BatchGravityTab.svelte";
  import BatchTastingTab from "$lib/components/batch/BatchTastingTab.svelte";
  import MarkdownEditor from "$lib/components/MarkdownEditor.svelte";
  import DocLink from "$lib/components/DocLink.svelte";
  import { DOCS } from "$lib/docs-urls";

  let { batch, onUpdate, onRefresh }: {
    batch: Batch;
    onUpdate: (input: UpdateBatchInput) => void;
    onRefresh: () => void;
  } = $props();

  const STATUSES = ["planned", "brewing", "fermenting", "packaged"] as const;

  let batchVersion = $state<RecipeVersionSummary | null>(null);
  let recipe = $state<Recipe | null>(null);

  const gravityUnit = $derived($settings.gravity_unit ?? "sg");

  const gravityDisplays = $derived.by(() => {
    const gravityFields = [
      "planned_og", "planned_fg", "planned_pre_boil_gravity",
      "actual_og", "actual_fg", "actual_pre_boil_gravity",
    ] as const;
    const out: Record<string, string> = {};
    for (const f of gravityFields) {
      const v = (batch as Record<string, unknown>)[f];
      out[f] = v != null ? formatSg(v as number, gravityUnit) : "";
    }
    return out;
  });

  onMount(async () => {
    const [versions, fetchedRecipe] = await Promise.all([
      ipc(listRecipeVersions(batch.recipe_id)),
      ipc(getRecipe(batch.recipe_id)),
    ]);
    if (versions) {
      batchVersion = versions.find((v) => v.id === batch.recipe_version_id) ?? null;
    }
    if (fetchedRecipe) {
      recipe = fetchedRecipe;
    }
  });

  function toDateInput(ts: number | null | undefined): string {
    if (ts == null) return "";
    return new Date(ts * 1000).toISOString().slice(0, 10);
  }

  function fromDateInput(val: string): number | null {
    if (!val) return null;
    return Math.floor(new Date(val).getTime() / 1000);
  }

  function onStatusChange(newStatus: string) {
    const update: UpdateBatchInput = { status: newStatus };
    const todayTs = Math.floor(Date.now() / 1000);
    if (newStatus === "brewing" && !batch.brew_date) update.brew_date = todayTs;
    if (newStatus === "fermenting" && !batch.fermenter_date) update.fermenter_date = todayTs;
    if (newStatus === "packaged" && !batch.packaging_date) update.packaging_date = todayTs;
    onUpdate(update);
  }

  const HIGHLIGHTED: Record<string, string[]> = {
    planned: [],
    brewing: ["actual_pre_boil_gravity", "actual_og", "actual_post_boil_volume_l"],
    fermenting: ["actual_og", "actual_fg"],
    packaged: ["actual_og", "actual_fg"],
  };

  const highlightedFields = $derived(new Set(HIGHLIGHTED[batch.status] ?? []));

  const stageTargets = $derived.by(() => {
    const { planned_og: og, planned_fg: fg, planned_pre_boil_gravity: pbg,
            planned_post_boil_volume_l: pbv, planned_batch_size_l: bs,
            actual_og, actual_fg } = batch;
    const targetAbv = og && fg ? ((og - fg) * 131.25).toFixed(1) : null;
    const actualAbv = actual_og && actual_fg ? ((actual_og - actual_fg) * 131.25).toFixed(1) : null;
    const items: { label: string; value: string }[] = [];
    switch (batch.status) {
      case "planned":
        if (og && gravityDisplays.planned_og) items.push({ label: "OG", value: gravityDisplays.planned_og });
        if (fg && gravityDisplays.planned_fg) items.push({ label: "FG", value: gravityDisplays.planned_fg });
        if (bs) items.push({ label: "Batch", value: `${bs.toFixed(1)} L` });
        break;
      case "brewing":
        if (pbg && gravityDisplays.planned_pre_boil_gravity) items.push({ label: "Pre-boil", value: gravityDisplays.planned_pre_boil_gravity });
        if (og && gravityDisplays.planned_og) items.push({ label: "OG", value: gravityDisplays.planned_og });
        if (pbv) items.push({ label: "Post-boil", value: `${pbv.toFixed(1)} L` });
        break;
      case "fermenting":
        if (actual_og && gravityDisplays.actual_og) items.push({ label: "Actual OG", value: gravityDisplays.actual_og });
        if (fg && gravityDisplays.planned_fg) items.push({ label: "Target FG", value: gravityDisplays.planned_fg });
        if (targetAbv) items.push({ label: "Target ABV", value: `${targetAbv}%` });
        break;
      case "packaged":
        if (actual_og && gravityDisplays.actual_og) items.push({ label: "OG", value: gravityDisplays.actual_og });
        if (actual_fg && gravityDisplays.actual_fg) items.push({ label: "FG", value: gravityDisplays.actual_fg });
        if (actualAbv) items.push({ label: "ABV", value: `${actualAbv}%` });
        break;
    }
    return items;
  });
</script>

<div class="p-4 flex flex-col gap-6">
  <div class="flex justify-end">
    <DocLink label="Brew day guide" url={DOCS.brewDay} />
  </div>

  {#if batchVersion}
    <div class="text-xs" style="color: var(--color-text-muted);">
      Brewed with
      <button
        onclick={() => goto(`/recipe/${batch.recipe_id}`)}
        class="underline"
        style="color: var(--color-accent);"
      >
        Recipe v{batchVersion.version_number}{batchVersion.name ? ` · ${batchVersion.name}` : ""}
      </button>
    </div>
  {/if}

  <!-- Status -->
  <div>
    <div class="text-xs mb-2" style="color: var(--color-text-secondary);">STATUS</div>
    <select
      class="md:hidden w-full px-3 py-2 rounded text-sm outline-none"
      style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
      value={batch.status}
      onchange={(e) => onStatusChange(e.currentTarget.value)}
    >
      {#each STATUSES as s}
        <option value={s}>{s.charAt(0).toUpperCase() + s.slice(1)}</option>
      {/each}
    </select>
    <div class="hidden md:block">
      <TabBar
        tabs={STATUSES.map(s => ({ key: s, label: s.charAt(0).toUpperCase() + s.slice(1) }))}
        active={batch.status}
        onchange={(key) => onStatusChange(key)}
      />
    </div>
  </div>

  <!-- Stage callout banner -->
  {#if stageTargets.length > 0}
    <div
      class="flex items-center gap-4 flex-wrap px-3 py-2 rounded-lg text-sm"
      style="background: rgba(99,102,241,0.12); border: 1px solid rgba(99,102,241,0.25);"
    >
      <span class="text-xs font-bold uppercase tracking-wide" style="color: var(--color-text-secondary); min-width: 48px;">
        {batch.status === "packaged" ? "Actuals" : batch.status === "fermenting" ? "Progress" : "Targets"}
      </span>
      {#each stageTargets as t}
        <span style="color: var(--color-text-secondary);">
          {t.label} <strong style="color: var(--color-text-primary);">{t.value}</strong>
        </span>
      {/each}
    </div>
  {/if}

  <!-- Measurements -->
  <div>
    <div class="text-xs mb-2" style="color: var(--color-text-secondary);">MEASUREMENTS</div>
    <div class="grid grid-cols-2 md:grid-cols-3 gap-3">
      {#each [
        { label: "Pre-Boil Gravity", field: "actual_pre_boil_gravity", isGravity: true },
        { label: "Original Gravity (OG)", field: "actual_og", isGravity: true },
        { label: "Final Gravity (FG)", field: "actual_fg", isGravity: true },
        { label: "Pre-Boil Volume (L)", field: "actual_pre_boil_volume_l" },
        { label: "Post-Boil Volume (L)", field: "actual_post_boil_volume_l" },
        { label: "Batch Size (L)", field: "actual_batch_size_l" },
      ] as row}
        {@const rawValue = (batch as unknown as Record<string, number | null>)[row.field]}
        {@const highlighted = highlightedFields.has(row.field)}
        <div
          class="p-3 rounded"
          style="background: var(--color-bg-elevated);
                 border: 1px solid {highlighted ? 'rgba(99,102,241,0.4)' : 'var(--color-border)'};
                 opacity: {highlighted || rawValue != null ? '1' : '0.55'};"
        >
          <label for="batch-{row.field}" class="text-xs block mb-1" style="color: var(--color-text-secondary);">{row.label}</label>
          {#if row.isGravity}
            <input
              id="batch-{row.field}"
              type="number" inputmode="decimal"
              step={gravityStep(gravityUnit)}
              value={gravityDisplays[row.field] ?? ""}
              onblur={async (e) => {
                const v = e.currentTarget.value;
                if (!v) { onUpdate({ [row.field]: null } as UpdateBatchInput); return; }
                const converted = await ipc(convertGravity(parseFloat(v), gravityUnit));
                if (converted) onUpdate({ [row.field]: converted.sg } as UpdateBatchInput);
              }}
              placeholder="—"
              class="w-full bg-transparent text-sm outline-none"
              style="color: {highlighted ? 'var(--color-accent)' : 'var(--color-text-primary)'}; font-weight: {highlighted ? '600' : '400'};"
            />
          {:else}
            <input
              id="batch-{row.field}"
              type="number" inputmode="decimal"
              step="0.1"
              value={rawValue != null ? rawValue.toFixed(1) : ""}
              onblur={(e) => {
                const v = e.currentTarget.value;
                onUpdate({ [row.field]: v ? parseFloat(v) : null } as UpdateBatchInput);
              }}
              placeholder="—"
              class="w-full bg-transparent text-sm outline-none"
              style="color: {highlighted ? 'var(--color-accent)' : 'var(--color-text-primary)'}; font-weight: {highlighted ? '600' : '400'};"
            />
          {/if}
        </div>
      {/each}
    </div>
    {#if batch.actual_og && batch.actual_fg}
      <div class="mt-3 text-sm" style="color: var(--color-text-muted);">
        Actual ABV: {((batch.actual_og - batch.actual_fg) * 131.25).toFixed(1)}%
      </div>
    {/if}
  </div>

  <!-- Dates (conditioning_date removed) -->
  <div>
    <div class="text-xs mb-2" style="color: var(--color-text-secondary);">DATES</div>
    <div class="grid grid-cols-2 md:grid-cols-3 gap-3">
      {#each [
        { label: "Brew Date", field: "brew_date", value: batch.brew_date },
        { label: "Into Fermenter", field: "fermenter_date", value: batch.fermenter_date },
        { label: "Packaging", field: "packaging_date", value: batch.packaging_date },
      ] as item}
        <div>
          <label for="batch-{item.field}" class="text-xs block mb-1" style="color: var(--color-text-secondary);">{item.label}</label>
          <input
            id="batch-{item.field}"
            type="date"
            value={toDateInput(item.value)}
            onchange={(e) => onUpdate({ [item.field]: fromDateInput(e.currentTarget.value) } as UpdateBatchInput)}
            class="w-full px-2 py-1.5 rounded text-sm outline-none"
            style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border); opacity: {item.value ? '1' : '0.55'};"
          />
        </div>
      {/each}
    </div>
  </div>

  <!-- Gravity Log — fermenting stage only -->
  {#if batch.status === "fermenting"}
    <div>
      <div class="text-xs mb-2" style="color: var(--color-text-secondary);">GRAVITY LOG</div>
      <div class="-mx-4">
        <BatchGravityTab {batch} {onRefresh} />
      </div>
    </div>
  {/if}

  <!-- Carbonation — packaged stage only -->
  {#if batch.status === "packaged"}
    <BatchCarbonationSection
      {batch}
      recipePrimaryTempC={recipe?.primary_temp_c ?? null}
      recipeCarbonationVols={recipe?.carbonation_vols ?? null}
      {onUpdate}
    />
  {/if}

  <!-- Tasting — packaged stage only -->
  {#if batch.status === "packaged"}
    <div>
      <div class="text-xs mb-2" style="color: var(--color-text-secondary);">TASTING</div>
      <BatchTastingTab {batch} {onUpdate} />
    </div>
  {/if}

  <!-- Notes -->
  <div>
    <div class="text-xs mb-2" style="color: var(--color-text-secondary);">NOTES</div>
    <MarkdownEditor
      value={batch.notes ?? null}
      onchange={(v) => onUpdate({ notes: v })}
      rows={4}
      placeholder="Brew day observations, gravity readings, anything worth remembering…"
    />
  </div>
</div>
```

- [ ] **Step 5.4: Run the new tests**

```bash
npm run test -- BatchOverviewTab --reporter=verbose 2>&1 | tail -30
```

Expected: all 5 tests pass.

- [ ] **Step 5.5: Run the full test suite**

```bash
npm run test 2>&1 | tail -20
```

Expected: all tests pass.

- [ ] **Step 5.6: Commit**

```bash
git add src/lib/components/batch/BatchOverviewTab.svelte tests/BatchOverviewTab.test.ts
git commit -m "feat: stage-driven BatchOverviewTab — gravity in fermenting, tasting in packaged, conditioning removed"
```

---

## Task 6: Refactor desktop/BatchView.svelte — remove outer tabs, add Attachments modal

**Files:**
- Modify: `src/lib/desktop/BatchView.svelte`

- [ ] **Step 6.1: Rewrite desktop/BatchView.svelte**

Replace `src/lib/desktop/BatchView.svelte` with:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { Batch, UpdateBatchInput } from "$lib/api";
  import { getBatch, updateBatch } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { batchList, refreshBatchList } from "$lib/stores/batches";
  import BatchList from "$lib/components/BatchList.svelte";
  import BatchOverviewTab from "$lib/components/batch/BatchOverviewTab.svelte";
  import BatchAttachmentsTab from "$lib/components/batch/BatchAttachmentsTab.svelte";

  let { id }: { id: string } = $props();

  let batch = $state<Batch | null>(null);
  let showAttachments = $state(false);

  async function loadBatch() {
    batch = await ipc(getBatch(id)) ?? null;
  }

  onMount(async () => {
    await refreshBatchList();
    await loadBatch();
  });

  $effect(() => {
    if (id) loadBatch();
  });

  async function handleUpdate(input: UpdateBatchInput) {
    if (!batch) return;
    batch = await ipc(updateBatch(batch.id, input)) ?? batch;
  }
</script>

<aside class="w-56 flex flex-col flex-shrink-0 border-r overflow-hidden"
       style="background: var(--color-bg-surface); border-color: var(--color-border);">
  <div class="p-2 border-b" style="border-color: var(--color-border);">
    <button
      onclick={() => goto("/batches")}
      class="w-full px-2 py-1.5 rounded text-sm text-left"
      style="background: var(--color-accent); color: #fff;"
    >+ New Batch</button>
  </div>
  <div class="flex-1 overflow-y-auto">
    <BatchList batches={$batchList} onRefresh={async () => { await ipc(refreshBatchList()); }} />
  </div>
</aside>

<div class="flex flex-1 flex-col overflow-hidden">
  {#if batch}
    <!-- Header -->
    <div class="px-4 pt-3 pb-2 flex-shrink-0 flex items-start justify-between gap-2"
         style="border-bottom: 1px solid var(--color-border);">
      <div class="min-w-0">
        <div class="font-semibold text-base truncate">{batch.recipe_name}</div>
        <div class="text-xs" style="color: var(--color-text-muted);">
          {batch.name ?? "Batch"} · v{batch.recipe_version_id.slice(0, 6)}
        </div>
      </div>
      <button
        onclick={() => showAttachments = true}
        class="flex-shrink-0 px-2 py-1 rounded text-xs mt-0.5"
        style="background: var(--color-bg-elevated); color: var(--color-text-muted); border: 1px solid var(--color-border);"
      >📎 Attachments</button>
    </div>

    <!-- Stage content -->
    <div class="flex-1 overflow-y-auto">
      <BatchOverviewTab {batch} onUpdate={handleUpdate} onRefresh={loadBatch} />
    </div>

    <!-- Attachments modal -->
    {#if showAttachments}
      <div
        class="fixed inset-0 z-50 flex items-center justify-center"
        style="background: rgba(0,0,0,0.5);"
        role="dialog"
        aria-modal="true"
      >
        <div class="rounded-lg w-full max-w-2xl max-h-[80vh] flex flex-col overflow-hidden"
             style="background: var(--color-bg-surface); border: 1px solid var(--color-border);">
          <div class="flex items-center justify-between px-4 py-3 border-b flex-shrink-0"
               style="border-color: var(--color-border);">
            <div class="font-medium text-sm">Attachments</div>
            <button
              onclick={() => showAttachments = false}
              class="text-xs px-2 py-1 rounded"
              style="color: var(--color-text-muted); background: var(--color-bg-elevated);"
            >Close</button>
          </div>
          <div class="flex-1 overflow-y-auto">
            <BatchAttachmentsTab {batch} />
          </div>
        </div>
      </div>
    {/if}
  {:else}
    <div class="flex-1 flex items-center justify-center" style="color: var(--color-text-muted);">
      <p class="text-sm">Loading…</p>
    </div>
  {/if}
</div>
```

- [ ] **Step 6.2: Run the full test suite**

```bash
npm run test 2>&1 | tail -20
```

Expected: all tests pass.

- [ ] **Step 6.3: Commit**

```bash
git add src/lib/desktop/BatchView.svelte
git commit -m "feat: remove outer tab bar from desktop batch view, add Attachments modal"
```

---

## Task 7: Update mobile/BatchView.svelte

The mobile view is a single scrollable page. Now that `BatchOverviewTab` handles gravity (in fermenting) and tasting (in packaged), we remove those as standalone sections. Attachments stays as a persistent bottom section since the modal approach is desktop-centric.

**Files:**
- Modify: `src/lib/mobile/BatchView.svelte`

- [ ] **Step 7.1: Rewrite mobile/BatchView.svelte**

Replace `src/lib/mobile/BatchView.svelte` with:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import type { Batch, UpdateBatchInput } from "$lib/api";
  import { getBatch, updateBatch } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { refreshBatchList } from "$lib/stores/batches";
  import BatchOverviewTab from "$lib/components/batch/BatchOverviewTab.svelte";
  import BatchAttachmentsTab from "$lib/components/batch/BatchAttachmentsTab.svelte";

  let { id }: { id: string } = $props();

  let batch = $state<Batch | null>(null);

  async function loadBatch() {
    batch = await ipc(getBatch(id)) ?? null;
  }

  onMount(async () => {
    await refreshBatchList();
    await loadBatch();
  });

  $effect(() => { if (id) loadBatch(); });

  async function handleUpdate(input: UpdateBatchInput) {
    if (!batch) return;
    batch = await ipc(updateBatch(batch.id, input)) ?? batch;
  }
</script>

{#if batch}
  <div class="flex flex-col h-full overflow-hidden" style="background: var(--color-bg-base);">
    <div class="flex items-center gap-3 px-4 py-3 border-b flex-shrink-0"
         style="background: var(--color-bg-surface); border-color: var(--color-border);">
      <button
        onclick={() => goto("/batches")}
        class="text-sm"
        style="color: var(--color-accent);"
      >‹ Batches</button>
      <span class="flex-1 font-semibold text-base truncate"
            style="color: var(--color-text-primary);">{batch.recipe_name}</span>
    </div>

    <div class="flex-1 overflow-y-auto">
      <div class="flex flex-col gap-6 pb-6">
        <!-- Overview + stage content (includes gravity for fermenting, tasting for packaged) -->
        <BatchOverviewTab {batch} onUpdate={handleUpdate} onRefresh={loadBatch} />

        <!-- Attachments always accessible at bottom on mobile -->
        <section class="px-4">
          <div class="text-xs font-semibold uppercase tracking-wider mb-3"
               style="color: var(--color-text-secondary);">Attachments</div>
          <BatchAttachmentsTab {batch} />
        </section>
      </div>
    </div>
  </div>
{:else}
  <div class="flex-1 flex items-center justify-center">
    <p class="text-sm" style="color: var(--color-text-muted);">Loading…</p>
  </div>
{/if}
```

- [ ] **Step 7.2: Run the full test suite**

```bash
npm run test 2>&1 | tail -20
```

Expected: all tests pass.

- [ ] **Step 7.3: Commit**

```bash
git add src/lib/mobile/BatchView.svelte
git commit -m "feat: simplify mobile batch view — gravity and tasting now live inside BatchOverviewTab stages"
```

---

## Final verification

- [ ] **Step 8.1: Run the full test suite one more time**

```bash
npm run test 2>&1 | tail -30
```

Expected: all tests pass, no failures.

- [ ] **Step 8.2: Build to catch any TypeScript/Svelte errors**

```bash
npm run build 2>&1 | tail -30
```

Expected: build succeeds with no errors.
