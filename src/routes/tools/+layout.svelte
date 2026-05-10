<script lang="ts">
  import { page } from "$app/stores";
  import { TOOLS } from "$lib/tools";

  let { children } = $props();
</script>

<div class="flex flex-1 flex-col overflow-hidden md:flex-row" style="background: var(--color-bg-base);">
  <aside class="w-full flex-shrink-0 border-b md:w-80 md:border-b-0 md:border-r"
         style="background: var(--color-bg-surface); border-color: var(--color-border);">
    <div class="px-4 py-4 md:px-5">
      <h1 class="text-lg font-semibold" style="color: var(--color-text-primary);">Tools</h1>
      <p class="mt-1 text-sm" style="color: var(--color-text-secondary);">
        Standalone brewing calculators.
      </p>
    </div>

    <nav class="flex flex-col pb-3">
      {#each TOOLS as tool}
        <a
          href={`/tools/${tool.slug}`}
          class="border-l px-4 py-3 transition-colors md:px-5"
          style={
            $page.url.pathname === `/tools/${tool.slug}`
              ? "border-left-width: 3px; border-color: var(--color-accent); background: var(--color-bg-elevated);"
              : "border-left-width: 3px; border-color: transparent;"
          }
        >
          <div class="text-sm font-medium" style="color: var(--color-text-primary);">{tool.name}</div>
          <div class="mt-1 text-xs" style="color: var(--color-text-secondary);">{tool.description}</div>
        </a>
      {/each}
    </nav>
  </aside>

  <div class="flex-1 overflow-y-auto">
    {@render children()}
  </div>
</div>
