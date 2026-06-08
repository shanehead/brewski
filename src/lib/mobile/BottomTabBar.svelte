<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { settings } from "$lib/stores/settings";
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";
</script>

<nav
  class="flex border-t flex-shrink-0"
  style="background: var(--color-bg-surface); border-color: var(--color-border); padding-bottom: env(safe-area-inset-bottom, 0px);"
>
  {@const recipesActive = $page.url.pathname === "/" || $page.url.pathname.startsWith("/recipe")}
  {@const batchesActive = $page.url.pathname.startsWith("/batches")}
  {@const toolsActive   = $page.url.pathname.startsWith("/tools")}
  {@const moreActive    = $page.url.pathname.startsWith("/settings") || $page.url.pathname.startsWith("/equipment") || $page.url.pathname.startsWith("/library")}

  <button
    onclick={() => goto($settings.last_route_recipes ?? "/")}
    aria-current={recipesActive ? "page" : undefined}
    class="flex flex-col items-center justify-center flex-1 py-2 gap-1 text-xs transition-colors"
    style={recipesActive ? "color: var(--color-accent);" : "color: var(--color-text-secondary);"}
  >
    <span style={recipesActive ? "opacity: 1; transition: opacity 0.15s;" : "opacity: 0.45; transition: opacity 0.15s;"}>
      <BrewingIcon name="recipes" size={22} />
    </span>
    Recipes
  </button>

  <button
    onclick={() => goto($settings.last_route_batches ?? "/batches")}
    aria-current={batchesActive ? "page" : undefined}
    class="flex flex-col items-center justify-center flex-1 py-2 gap-1 text-xs transition-colors"
    style={batchesActive ? "color: var(--color-accent);" : "color: var(--color-text-secondary);"}
  >
    <span style={batchesActive ? "opacity: 1; transition: opacity 0.15s;" : "opacity: 0.45; transition: opacity 0.15s;"}>
      <BrewingIcon name="batches" size={22} />
    </span>
    Batches
  </button>

  <button
    onclick={() => goto($settings.last_route_tools ?? "/tools")}
    aria-current={toolsActive ? "page" : undefined}
    class="flex flex-col items-center justify-center flex-1 py-2 gap-1 text-xs transition-colors"
    style={toolsActive ? "color: var(--color-accent);" : "color: var(--color-text-secondary);"}
  >
    <span style={toolsActive ? "opacity: 1; transition: opacity 0.15s;" : "opacity: 0.45; transition: opacity 0.15s;"}>
      <BrewingIcon name="tools" size={22} />
    </span>
    Tools
  </button>

  <button
    onclick={() => goto($settings.last_route_settings ?? $settings.last_route_equipment ?? $settings.last_route_library ?? "/settings")}
    aria-current={moreActive ? "page" : undefined}
    class="flex flex-col items-center justify-center flex-1 py-2 gap-1 text-xs transition-colors"
    style={moreActive ? "color: var(--color-accent);" : "color: var(--color-text-secondary);"}
  >
    <span style={moreActive ? "opacity: 1; transition: opacity 0.15s;" : "opacity: 0.45; transition: opacity 0.15s;"}>
      <BrewingIcon name="settings" size={22} />
    </span>
    More
  </button>
</nav>
