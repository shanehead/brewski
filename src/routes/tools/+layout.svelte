<script lang="ts">
  import { page } from "$app/stores";
  import { TOOLS } from "$lib/tools";

  let { children } = $props();
</script>

<div class="flex flex-1 flex-col overflow-hidden md:flex-row bg-bg-base">
  <!-- Desktop: vertical sidebar -->
  <aside class="hidden md:flex md:flex-col w-80 flex-shrink-0 border-r bg-bg-surface border-border"
        >
    <div class="px-5 py-4">
      <h1 class="text-lg font-semibold text-text-primary">Tools</h1>
      <p class="mt-1 text-sm text-text-secondary">
        Standalone brewing calculators.
      </p>
    </div>

    <nav class="flex flex-col pb-3">
      {#each TOOLS as tool}
        <a
          href={`/tools/${tool.slug}`}
          class="border-l px-5 py-3 transition-colors"
          style={
            $page.url.pathname === `/tools/${tool.slug}`
              ? "border-left-width: 3px; border-color: var(--color-accent); background: var(--color-bg-elevated);"
              : "border-left-width: 3px; border-color: transparent;"
          }
        >
          <div class="text-sm font-medium text-text-primary">{tool.name}</div>
          <div class="mt-1 text-xs text-text-secondary">{tool.description}</div>
        </a>
      {/each}
    </nav>
  </aside>

  <!-- Mobile: back button when inside a specific tool -->
  {#if $page.url.pathname !== '/tools'}
    <div class="md:hidden flex items-center gap-3 px-4 border-b flex-shrink-0 bg-bg-surface border-border"
         style="min-height: 44px;">
      <a href="/tools" class="flex items-center gap-1 text-sm text-accent"
        >
        ‹ Tools
      </a>
      <span class="text-sm font-medium truncate text-text-primary">
        {TOOLS.find(t => $page.url.pathname === `/tools/${t.slug}`)?.name ?? ""}
      </span>
    </div>
  {/if}

  <div class="flex-1 overflow-y-auto">
    {@render children()}
  </div>
</div>
