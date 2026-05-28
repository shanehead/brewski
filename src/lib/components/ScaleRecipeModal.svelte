<script lang="ts">
  import { goto } from "$app/navigation";
  import { scaleRecipe } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { lToGal, galToL, volumeLabel, type Units } from "$lib/units";

  let {
    recipeId,
    currentBatchSizeL,
    onClose,
  }: {
    recipeId: string;
    currentBatchSizeL: number;
    onClose: () => void;
  } = $props();

  const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");
  const initialValue = $derived(units === "imperial" ? lToGal(currentBatchSizeL) : currentBatchSizeL);

  let targetValue = $state(0);
  let scaling = $state(false);
  let error = $state<string | null>(null);

  $effect(() => {
    targetValue = parseFloat(initialValue.toFixed(2));
  });

  async function handleConfirm() {
    if (!targetValue || targetValue <= 0) return;
    scaling = true;
    error = null;
    const batchSizeL = units === "imperial" ? galToL(targetValue) : targetValue;
    const result = await ipc(scaleRecipe(recipeId, batchSizeL));
    scaling = false;
    if (result) {
      goto(`/recipe/${result.id}`);
    } else {
      error = "Scaling failed. Please try again.";
    }
  }
</script>

<div class="fixed inset-0 flex items-center justify-center" style="z-index: 1000;">
  <div
    class="absolute inset-0"
    style="background: rgba(0,0,0,0.4);"
    role="none"
    onclick={onClose}
    onkeydown={onClose}
  ></div>
  <div
    class="p-4 rounded relative flex flex-col gap-3"
    style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); z-index: 1001; min-width: 280px;"
  >
    <div class="text-sm font-semibold" style="color: var(--color-text-primary);">Scale Recipe</div>
    <div class="flex items-center gap-2">
      <label for="target-batch-size" class="text-sm" style="color: var(--color-text-secondary);">Target Batch Size</label>
      <input
        id="target-batch-size"
        type="number"
        bind:value={targetValue}
        min="0.1"
        step="0.1"
        class="px-2 py-1 rounded text-sm w-24 outline-none"
        style="background: var(--color-bg-surface); color: var(--color-text-primary); border: 1px solid var(--color-border);"
      />
      <span class="text-sm" style="color: var(--color-text-secondary);">{volumeLabel(units)}</span>
    </div>
    {#if error}
      <div class="text-xs" style="color: var(--color-text-danger, #e55);">{error}</div>
    {/if}
    <div class="flex justify-end gap-2">
      <button
        onclick={onClose}
        class="px-3 py-1 rounded text-sm"
        style="background: var(--color-bg-surface); color: var(--color-text-primary); border: 1px solid var(--color-border);"
      >Cancel</button>
      <button
        onclick={handleConfirm}
        disabled={scaling || !targetValue || targetValue <= 0}
        class="px-3 py-1 rounded text-sm"
        style="background: var(--color-accent); color: #fff;"
      >
        {scaling ? "Scaling…" : "Scale Recipe"}
      </button>
    </div>
  </div>
</div>
