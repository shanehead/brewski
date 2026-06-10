<script lang="ts">
  import { TOOLS } from "$lib/tools";
</script>

<!-- Mobile: full tool list -->
<div class="md:hidden flex flex-col">
  {#each TOOLS as tool}
    <a
      href="/tools/{tool.slug}"
      class="flex items-center justify-between px-4 py-3 border-b text-sm border-border text-text-primary"
     
    >
      <div>
        <div class="font-medium">{tool.name}</div>
        <div class="mt-0.5 text-xs text-text-secondary">{tool.description}</div>
      </div>
      <span class="text-text-muted">›</span>
    </a>
  {/each}
</div>

<!-- Desktop: prompt to select a tool -->
<div class="hidden md:flex flex-1 items-center justify-center text-text-muted">
  <p class="text-sm">Select a tool from the list</p>
</div>
