<script lang="ts">
  import { openUrl } from "@tauri-apps/plugin-opener";

  let { label, url }: { label: string; url: string } = $props();

  function handleClick(e: MouseEvent) {
    e.preventDefault();
    openUrl(url);
  }
</script>

<button
  type="button"
  onclick={handleClick}
  class="text-xs opacity-40 hover:opacity-70 transition-opacity cursor-pointer"
  style="color: var(--color-text-muted);"
>
  {label} ↗
</button>
