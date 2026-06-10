<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { settings } from "$lib/stores/settings";
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";
</script>

<nav
  class="flex border-t flex-shrink-0 bg-bg-surface border-border"
  style="padding-bottom: env(safe-area-inset-bottom, 0px);"
>
  <button
    onclick={() => goto($settings.last_route_recipes ?? "/")}
    aria-current={($page.url.pathname === "/" || $page.url.pathname.startsWith("/recipe") || $page.url.pathname.startsWith("/baseline-recipe")) ? "page" : undefined}
    class="flex flex-col items-center justify-center flex-1 py-2 gap-1 text-xs transition-colors"
    style={($page.url.pathname === "/" || $page.url.pathname.startsWith("/recipe") || $page.url.pathname.startsWith("/baseline-recipe")) ? "color: var(--color-accent);" : "color: var(--color-text-secondary);"}
  >
    <span style={($page.url.pathname === "/" || $page.url.pathname.startsWith("/recipe") || $page.url.pathname.startsWith("/baseline-recipe")) ? "opacity: 1; transition: opacity 0.15s;" : "opacity: 0.45; transition: opacity 0.15s;"}>
      <BrewingIcon name="recipes" size={22} />
    </span>
    Recipes
  </button>

  <button
    onclick={() => goto($settings.last_route_batches ?? "/batches")}
    aria-current={$page.url.pathname.startsWith("/batches") ? "page" : undefined}
    class="flex flex-col items-center justify-center flex-1 py-2 gap-1 text-xs transition-colors"
    style={$page.url.pathname.startsWith("/batches") ? "color: var(--color-accent);" : "color: var(--color-text-secondary);"}
  >
    <span style={$page.url.pathname.startsWith("/batches") ? "opacity: 1; transition: opacity 0.15s;" : "opacity: 0.45; transition: opacity 0.15s;"}>
      <BrewingIcon name="batches" size={22} />
    </span>
    Batches
  </button>

  <button
    onclick={() => goto($settings.last_route_tools ?? "/tools")}
    aria-current={$page.url.pathname.startsWith("/tools") ? "page" : undefined}
    class="flex flex-col items-center justify-center flex-1 py-2 gap-1 text-xs transition-colors"
    style={$page.url.pathname.startsWith("/tools") ? "color: var(--color-accent);" : "color: var(--color-text-secondary);"}
  >
    <span style={$page.url.pathname.startsWith("/tools") ? "opacity: 1; transition: opacity 0.15s;" : "opacity: 0.45; transition: opacity 0.15s;"}>
      <BrewingIcon name="tools" size={22} />
    </span>
    Tools
  </button>

  <button
    onclick={() => goto($settings.last_route_settings ?? $settings.last_route_equipment ?? $settings.last_route_library ?? "/settings")}
    aria-current={($page.url.pathname.startsWith("/settings") || $page.url.pathname.startsWith("/equipment") || $page.url.pathname.startsWith("/library")) ? "page" : undefined}
    class="flex flex-col items-center justify-center flex-1 py-2 gap-1 text-xs transition-colors"
    style={($page.url.pathname.startsWith("/settings") || $page.url.pathname.startsWith("/equipment") || $page.url.pathname.startsWith("/library")) ? "color: var(--color-accent);" : "color: var(--color-text-secondary);"}
  >
    <span style={($page.url.pathname.startsWith("/settings") || $page.url.pathname.startsWith("/equipment") || $page.url.pathname.startsWith("/library")) ? "opacity: 1; transition: opacity 0.15s;" : "opacity: 0.45; transition: opacity 0.15s;"}>
      <BrewingIcon name="settings" size={22} />
    </span>
    More
  </button>
</nav>
