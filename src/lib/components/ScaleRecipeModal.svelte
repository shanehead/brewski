<script lang="ts">
  import { untrack } from "svelte";
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

  let targetValue = $state(
    untrack(() => {
      const u: Units = $settings.units === "imperial" ? "imperial" : "metric";
      return parseFloat((u === "imperial" ? lToGal(currentBatchSizeL) : currentBatchSizeL).toFixed(2));
    })
  );
  let scaling = $state(false);
  let error = $state<string | null>(null);

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

<svelte:window onkeydown={(e) => e.key === "Escape" && onClose()} />

<div class="fixed inset-0 flex items-center justify-center" style="z-index: 1000;">
  <div
    class="absolute inset-0"
    style="background: rgba(0,0,0,0.4);"
    role="none"
    onclick={onClose}
  ></div>
  <div
    class="p-4 rounded relative flex flex-col gap-3 bg-bg-elevated border border-border"
    style="z-index: 1001; min-width: 280px;"
  >
    <div class="text-sm font-semibold text-text-primary">Scale Recipe</div>
    <div class="flex items-center gap-2">
      <label for="target-batch-size" class="text-sm text-text-secondary">Target Batch Size</label>
      <input
        id="target-batch-size"
        type="number"
        bind:value={targetValue}
        min="0.1"
        step="0.1"
        class="px-2 py-1 rounded text-sm w-24 outline-none bg-bg-surface text-text-primary border border-border"
       
      />
      <span class="text-sm text-text-secondary">{volumeLabel(units)}</span>
    </div>
    {#if error}
      <div class="text-xs text-text-danger">{error}</div>
    {/if}
    <div class="flex justify-end gap-2">
      <button
        onclick={onClose}
        class="px-3 py-1 rounded text-sm bg-bg-surface text-text-primary border border-border"
       
      >Cancel</button>
      <button
        onclick={handleConfirm}
        disabled={scaling || !targetValue || targetValue <= 0}
        class="px-3 py-1 rounded text-sm bg-accent"
        style="color: #fff;"
      >
        {scaling ? "Scaling…" : "Scale Recipe"}
      </button>
    </div>
  </div>
</div>
