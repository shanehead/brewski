<script lang="ts">
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { settings } from "$lib/stores/settings";

  let { label, url }: { label: string; url: string } = $props();

  const showTooltips = $derived($settings.show_tooltips ?? true);

  function handleClick(e: MouseEvent) {
    e.preventDefault();
    openUrl(url);
  }
</script>

{#if showTooltips}
  <button
    type="button"
    onclick={handleClick}
    class="text-xs opacity-70 hover:opacity-100 transition-opacity cursor-pointer"
    style="color: var(--color-accent);"
  >
    {label} ↗
  </button>
{/if}
