<script lang="ts">
  import { settings } from "$lib/stores/settings";

  let { text }: { text: string } = $props();

  let open = $state(false);
  const showTooltips = $derived($settings.show_tooltips ?? true);

  function toggle() {
    open = !open;
  }
</script>

{#if showTooltips}
  <span class="relative inline-flex items-center">
    <button
      type="button"
      onclick={toggle}
      aria-label="?"
      class="inline-flex items-center justify-center w-4 h-4 rounded-full text-[10px] opacity-40 hover:opacity-80 transition-opacity cursor-pointer flex-shrink-0"
      style="border: 1px solid var(--color-border); color: var(--color-text-muted);"
    >?</button>

    {#if open}
      <div
        role="tooltip"
        class="absolute left-6 top-0 z-50 w-56 rounded-md px-3 py-2 text-xs leading-relaxed shadow-lg"
        style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
      >
        {text}
        <button
          type="button"
          onclick={toggle}
          class="block mt-1 text-[10px] opacity-40 hover:opacity-70"
          style="color: var(--color-text-muted);"
        >tap to dismiss</button>
      </div>
    {/if}
  </span>
{/if}
