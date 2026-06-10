<script lang="ts">
  import { settings } from "$lib/stores/settings";

  let { text }: { text: string } = $props();

  let open = $state(false);
  let buttonEl = $state<HTMLButtonElement | null>(null);
  let popupTop = $state(0);
  let popupLeft = $state(0);
  const showTooltips = $derived($settings.show_tooltips ?? true);

  function toggle() {
    if (!open && buttonEl) {
      const rect = buttonEl.getBoundingClientRect();
      popupTop = rect.bottom + 4;
      popupLeft = Math.min(rect.left, window.innerWidth - 240);
    }
    open = !open;
  }
</script>

{#if showTooltips}
  <span class="inline-flex items-center">
    <button
      bind:this={buttonEl}
      type="button"
      onclick={toggle}
      aria-label="?"
      class="inline-flex items-center justify-center w-4 h-4 rounded-full text-[10px] opacity-60 hover:opacity-100 transition-opacity cursor-pointer flex-shrink-0 border border-accent text-accent"
     
    >?</button>

    {#if open}
      <div
        role="tooltip"
        class="w-56 rounded-md px-3 py-2 text-xs leading-relaxed shadow-lg"
        style="position: fixed; top: {popupTop}px; left: {popupLeft}px; z-index: 9999; background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);"
      >
        {text}
        <button
          type="button"
          onclick={toggle}
          class="block mt-1 text-[10px] opacity-40 hover:opacity-70 text-text-muted"
         
        >tap to dismiss</button>
      </div>
    {/if}
  </span>
{/if}
