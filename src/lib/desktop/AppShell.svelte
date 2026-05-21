<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { loadSettings } from "$lib/stores/settings";
  import { lastError } from "$lib/stores/error";

  let { children } = $props();

  onMount(async () => {
    try {
      await loadSettings();
    } finally {
      const { getCurrentWindow } = await import('@tauri-apps/api/window');
      getCurrentWindow().show();
    }
  });

  const isRecipes = $derived($page.url.pathname === "/" || $page.url.pathname.startsWith("/recipe"));
  const isBatches = $derived($page.url.pathname.startsWith("/batches"));
  const isTools = $derived($page.url.pathname.startsWith("/tools"));
  const isEquipment = $derived($page.url.pathname.startsWith("/equipment"));
  const isLibrary = $derived($page.url.pathname.startsWith("/library"));
</script>

<div class="flex h-screen overflow-hidden" style="background: var(--color-bg-base); color: var(--color-text-primary);">
  <!-- Icon rail -->
  <nav class="flex flex-col items-center w-14 py-3 gap-2 border-r flex-shrink-0"
       style="background: var(--color-bg-surface); border-color: var(--color-border);">
    <!-- Recipes icon -->
    <a href="/" class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
       aria-label="Recipes"
       style={isRecipes ? "background: var(--color-accent); color: #fff; box-shadow: 0 0 10px color-mix(in srgb, var(--color-accent) 40%, transparent);" : "color: var(--color-text-secondary);"}>
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
        <polyline points="14 2 14 8 20 8"/>
        <line x1="16" y1="13" x2="8" y2="13"/>
        <line x1="16" y1="17" x2="8" y2="17"/>
        <polyline points="10 9 9 9 8 9"/>
      </svg>
    </a>

    <!-- Batches icon -->
    <a href="/batches"
       class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
       aria-label="Batches"
       style={isBatches ? "background: var(--color-accent); color: #fff; box-shadow: 0 0 10px color-mix(in srgb, var(--color-accent) 40%, transparent);" : "color: var(--color-text-secondary);"}>
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M9 3H5a2 2 0 0 0-2 2v4m6-6h10a2 2 0 0 1 2 2v4M9 3v18m0 0h10a2 2 0 0 0 2-2v-4M9 21H5a2 2 0 0 1-2-2v-4m0 0h18"/>
      </svg>
    </a>

    <!-- Tools icon -->
    <a href="/tools" class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
       aria-label="Tools"
       style={isTools ? "background: var(--color-accent); color: #fff; box-shadow: 0 0 10px color-mix(in srgb, var(--color-accent) 40%, transparent);" : "color: var(--color-text-secondary);"}>
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M14.7 6.3a4 4 0 0 0-4.7 4.7L3 18a2 2 0 0 0 2.8 2.8l7-7a4 4 0 0 0 4.7-4.7l-2.1 2.1-1.4-1.4z"/>
      </svg>
    </a>

    <!-- Equipment icon -->
    <a href="/equipment" class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
       aria-label="Equipment"
       style={isEquipment ? "background: var(--color-accent); color: #fff; box-shadow: 0 0 10px color-mix(in srgb, var(--color-accent) 40%, transparent);" : "color: var(--color-text-secondary);"}>
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M3 6h18"/>
        <path d="M4 6v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V6"/>
        <path d="M8 6V4a1 1 0 0 1 1-1h6a1 1 0 0 1 1 1v2"/>
        <path d="M10 11h4"/>
      </svg>
    </a>

    <!-- Library icon -->
    <a href="/library" class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
       aria-label="Library"
       style={isLibrary ? "background: var(--color-accent); color: #fff; box-shadow: 0 0 10px color-mix(in srgb, var(--color-accent) 40%, transparent);" : "color: var(--color-text-secondary);"}>
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/>
        <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
      </svg>
    </a>

    <div class="flex-1"></div>

    <!-- Settings icon -->
    <a href="/settings" class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors"
       aria-label="Settings"
       style={$page.url.pathname.startsWith('/settings') ? "background: var(--color-accent); color: #fff; box-shadow: 0 0 10px color-mix(in srgb, var(--color-accent) 40%, transparent);" : "color: var(--color-text-secondary);"}>
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="3"/>
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
      </svg>
    </a>
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
