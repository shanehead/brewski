<script lang="ts">
  import { onMount } from "svelte";
  import { afterNavigate, goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { loadSettings, settings, saveSetting } from "$lib/stores/settings";
  import type { AppSettings } from "$lib/stores/settings";
  import { lastError, lastSuccess } from "$lib/stores/error";
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";

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
    try {
      await loadSettings();
      const lastRoute = $settings.last_route;
      const currentUrl = $page.url.pathname + $page.url.search;
      if (lastRoute && lastRoute !== currentUrl) {
        goto(lastRoute);
      }
    } finally {
      const { getCurrentWindow } = await import('@tauri-apps/api/window');
      getCurrentWindow().show();
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

  const isRecipes   = $derived($page.url.pathname === "/" || $page.url.pathname.startsWith("/recipe"));
  const isBatches   = $derived($page.url.pathname.startsWith("/batches"));
  const isTools     = $derived($page.url.pathname.startsWith("/tools"));
  const isEquipment = $derived($page.url.pathname.startsWith("/equipment"));
  const isLibrary   = $derived($page.url.pathname.startsWith("/library"));
</script>

<div class="flex h-screen overflow-hidden bg-bg-base text-text-primary">
  <!-- Icon rail -->
  <nav class="flex flex-col items-center w-14 py-3 gap-2 border-r flex-shrink-0 bg-bg-surface border-border"
      >

    <button onclick={() => goto($settings.last_route_recipes ?? "/")}
            class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
            aria-label="Recipes"
            style={isRecipes ? "background: color-mix(in srgb, var(--color-accent) 18%, transparent);" : ""}>
      <BrewingIcon name="recipes" size={22} />
    </button>

    <button onclick={() => goto($settings.last_route_batches ?? "/batches")}
            class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
            aria-label="Batches"
            style={isBatches ? "background: color-mix(in srgb, var(--color-accent) 18%, transparent);" : ""}>
      <BrewingIcon name="batches" size={22} />
    </button>

    <button onclick={() => goto($settings.last_route_tools ?? "/tools")}
            class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
            aria-label="Tools"
            style={isTools ? "background: color-mix(in srgb, var(--color-accent) 18%, transparent);" : ""}>
      <BrewingIcon name="tools" size={22} />
    </button>

    <button onclick={() => goto($settings.last_route_equipment ?? "/equipment")}
            class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
            aria-label="Equipment"
            style={isEquipment ? "background: color-mix(in srgb, var(--color-accent) 18%, transparent);" : ""}>
      <BrewingIcon name="equipment" size={22} />
    </button>

    <button onclick={() => goto($settings.last_route_library ?? "/library")}
            class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
            aria-label="Library"
            style={isLibrary ? "background: color-mix(in srgb, var(--color-accent) 18%, transparent);" : ""}>
      <BrewingIcon name="library" size={22} />
    </button>

    <div class="flex-1"></div>

    <button onclick={() => goto($settings.last_route_settings ?? "/settings")}
            class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
            aria-label="Settings"
            style={$page.url.pathname.startsWith("/settings") ? "background: color-mix(in srgb, var(--color-accent) 18%, transparent);" : ""}>
      <BrewingIcon name="settings" size={22} />
    </button>
  </nav>

  <!-- Main content area -->
  <div class="flex flex-1 overflow-hidden">
    {@render children()}
  </div>
</div>

{#if $lastError}
  <div class="fixed bottom-4 left-1/2 -translate-x-1/2 z-50 flex items-center gap-3 px-4 py-2.5 rounded shadow-lg text-sm"
       style="background: #7f1d1d; color: #fecaca; max-width: 480px;">
    <span class="flex-1 truncate">{$lastError}</span>
    <button onclick={() => lastError.set(null)} class="opacity-70 hover:opacity-100 flex-shrink-0">✕</button>
  </div>
{/if}

{#if $lastSuccess}
  <div role="status" aria-live="polite" aria-atomic="true"
       class="fixed bottom-16 left-1/2 -translate-x-1/2 z-50 flex items-center gap-3 px-4 py-2.5 rounded shadow-lg text-sm"
       style="background: #14532d; color: #bbf7d0; max-width: 480px;">
    <span class="flex-1 truncate">{$lastSuccess}</span>
    <button aria-label="Dismiss" onclick={() => lastSuccess.set(null)} class="opacity-70 hover:opacity-100 flex-shrink-0">✕</button>
  </div>
{/if}
