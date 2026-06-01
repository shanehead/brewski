<script lang="ts">
  import { onMount } from "svelte";
  import { afterNavigate, goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { loadSettings, settings, saveSetting } from "$lib/stores/settings";
  import { lastError, lastSuccess } from "$lib/stores/error";
  import BottomTabBar from "./BottomTabBar.svelte";

  let { children } = $props();

  onMount(async () => {
    await loadSettings();
    if ($settings.last_route && $settings.last_route !== $page.url.pathname) {
      goto($settings.last_route);
    }
  });

  afterNavigate(({ to }) => {
    if (to) saveSetting('last_route', to.url.pathname);
  });
</script>

<div
  class="flex flex-col overflow-hidden"
  style="
    height: 100dvh;
    background: var(--color-bg-base);
    color: var(--color-text-primary);
    padding-top: env(safe-area-inset-top, 0px);
  "
>
  <div class="flex-1 flex flex-col overflow-hidden min-h-0">
    {@render children()}
  </div>
  <BottomTabBar />
</div>

{#if $lastError}
  <div class="fixed bottom-20 left-4 right-4 z-50 flex items-center gap-3 px-4 py-2.5 rounded shadow-lg text-sm"
       style="background: #7f1d1d; color: #fecaca;">
    <span class="flex-1">{$lastError}</span>
    <button onclick={() => lastError.set(null)} class="opacity-70 flex-shrink-0">✕</button>
  </div>
{/if}

{#if $lastSuccess}
  <div role="status" aria-live="polite" aria-atomic="true"
       class="fixed bottom-32 left-4 right-4 z-50 flex items-center gap-3 px-4 py-2.5 rounded shadow-lg text-sm"
       style="background: #14532d; color: #bbf7d0;">
    <span class="flex-1">{$lastSuccess}</span>
    <button aria-label="Dismiss" onclick={() => lastSuccess.set(null)} class="opacity-70 flex-shrink-0">✕</button>
  </div>
{/if}
