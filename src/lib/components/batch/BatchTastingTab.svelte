<!-- src/lib/components/batch/BatchTastingTab.svelte -->
<script lang="ts">
  import { untrack } from "svelte";
  import type { Batch, UpdateBatchInput } from "$lib/api";
  import { escRevert } from "$lib/actions/escRevert";

  let { batch, onUpdate }: { batch: Batch; onUpdate: (input: UpdateBatchInput) => void } = $props();

  let ratingInput = $state(untrack(() => batch.rating?.toString() ?? ""));
</script>

<div class="p-4 flex flex-col gap-4 overflow-y-auto">
  <div class="flex flex-col gap-1">
    <label for="batch-rating" class="text-xs text-text-muted">RATING (1–10)</label>
    <input
      id="batch-rating"
      type="number" inputmode="decimal"
      min="1"
      max="10"
      step="1"
      bind:value={ratingInput}
      use:escRevert
      onblur={() => {
        const v = parseInt(ratingInput);
        onUpdate({ rating: isNaN(v) ? null : Math.min(10, Math.max(1, v)) });
      }}
      placeholder="—"
      class="w-24 px-2 py-1.5 rounded text-sm outline-none bg-bg-elevated text-text-primary border border-border"

    />
  </div>
</div>
