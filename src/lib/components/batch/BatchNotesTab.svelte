<!-- src/lib/components/batch/BatchNotesTab.svelte -->
<script lang="ts">
  import type { Batch, UpdateBatchInput } from "$lib/api";

  let { batch, onUpdate }: { batch: Batch; onUpdate: (input: UpdateBatchInput) => void } = $props();
</script>

<div class="p-4 flex flex-col gap-4 h-full overflow-y-auto">
  <div class="flex flex-col gap-1 flex-1">
    <label class="text-xs" style="color: var(--color-text-muted);">BREW DAY NOTES</label>
    <textarea
      value={batch.brew_day_notes ?? ""}
      onblur={(e) => onUpdate({ brew_day_notes: e.currentTarget.value || null })}
      placeholder="What happened on brew day…"
      class="flex-1 p-2 rounded text-sm outline-none resize-none min-h-32"
      style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
    ></textarea>
  </div>
  <div class="flex flex-col gap-1 flex-1">
    <label class="text-xs" style="color: var(--color-text-muted);">FERMENTATION NOTES</label>
    <textarea
      value={batch.fermentation_notes ?? ""}
      onblur={(e) => onUpdate({ fermentation_notes: e.currentTarget.value || null })}
      placeholder="Fermentation observations…"
      class="flex-1 p-2 rounded text-sm outline-none resize-none min-h-32"
      style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
    ></textarea>
  </div>
</div>
