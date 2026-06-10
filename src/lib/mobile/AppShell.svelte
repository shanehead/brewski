<script lang="ts">
  import { onMount } from "svelte";
  import { afterNavigate, goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { loadSettings, settings, saveSetting } from "$lib/stores/settings";
  import type { AppSettings } from "$lib/stores/settings";
  import { lastError, lastSuccess } from "$lib/stores/error";
  import BottomTabBar from "./BottomTabBar.svelte";

  let { children } = $props();

  function sectionKeyFor(pathname: string): keyof AppSettings | null {
    if (pathname === "/" || pathname.startsWith("/recipe") || pathname.startsWith("/baseline-recipe"))
      return "last_route_recipes";
    if (pathname.startsWith("/batches"))   return "last_route_batches";
    if (pathname.startsWith("/tools"))     return "last_route_tools";
    if (pathname.startsWith("/equipment")) return "last_route_equipment";
    if (pathname.startsWith("/library"))   return "last_route_library";
    if (pathname.startsWith("/settings"))  return "last_route_settings";
    return null;
  }

  onMount(async () => {
    await loadSettings();
    const lastRoute = $settings.last_route;
    const currentUrl = $page.url.pathname + $page.url.search;
    if (lastRoute && lastRoute !== currentUrl) {
      goto(lastRoute);
    }
  });

  afterNavigate(({ to }) => {
    if (to) {
      const url = to.url.pathname + to.url.search;
      saveSetting("last_route", url);
      const key = sectionKeyFor(to.url.pathname);
      if (key) saveSetting(key, url);
    }
  });
</script>

<div
  class="flex flex-col overflow-hidden bg-bg-base text-text-primary"
  style="height: 100dvh; padding-top: env(safe-area-inset-top, 0px);"
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
