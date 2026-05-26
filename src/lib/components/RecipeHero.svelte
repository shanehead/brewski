<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { srmToHex } from "$lib/utils/srm";
  import type { RecipeSummary } from "$lib/api";

  let {
    recipe,
    appDataDir,
    height = "120px",
    onUploadClick,
    onRemoveClick,
  }: {
    recipe: Pick<RecipeSummary, "id" | "name" | "image_path"> & { srm?: number | null };
    appDataDir: string;
    height?: string;
    onUploadClick: () => void;
    onRemoveClick: () => void;
  } = $props();

  const srm = $derived(recipe.srm ?? 4);
  const color1 = $derived(srmToHex(srm));
  const color2 = $derived(srmToHex(Math.min(srm + 12, 40)));

  const imageSrc = $derived(
    recipe.image_path
      ? convertFileSrc(`${appDataDir}/images/${recipe.image_path}`)
      : null
  );
</script>

<div
  class="relative w-full flex-shrink-0 overflow-hidden"
  style="height: {height};"
>
  {#if imageSrc}
    <img
      src={imageSrc}
      alt={recipe.name}
      class="absolute inset-0 w-full h-full object-cover"
    />
  {:else}
    <div
      class="absolute inset-0"
      style="background: linear-gradient(135deg, {color1} 0%, {color2} 100%);"
    ></div>
  {/if}

  <!-- Gradient overlay so name text is always readable -->
  <div
    class="absolute inset-0"
    style="background: linear-gradient(to top, rgba(0,0,0,0.55) 0%, rgba(0,0,0,0.05) 60%, transparent 100%);"
  ></div>

  <!-- Buttons -->
  <div class="absolute top-2 right-2 flex gap-1.5 z-10">
    {#if recipe.image_path}
      <button
        onclick={onRemoveClick}
        aria-label="Remove photo"
        class="w-7 h-7 rounded-full flex items-center justify-center text-xs"
        style="background: rgba(0,0,0,0.45); border: 1px solid rgba(255,255,255,0.25); color: rgba(255,255,255,0.85); backdrop-filter: blur(4px);"
      >✕</button>
    {/if}
    <button
      onclick={onUploadClick}
      aria-label="Upload photo"
      class="w-7 h-7 rounded-full flex items-center justify-center"
      style="background: rgba(0,0,0,0.45); border: 1px solid rgba(255,255,255,0.25); backdrop-filter: blur(4px);"
    >
      <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"/>
        <circle cx="12" cy="13" r="4"/>
      </svg>
    </button>
  </div>
</div>
