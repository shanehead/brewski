<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    title,
    children,
  }: {
    title: string;
    children: Snippet;
  } = $props();
</script>

<div class="rounded-xl overflow-hidden bg-bg-surface border border-border"
    >
  <div class="px-4 py-2.5 border-b border-border"
      >
    <span class="text-xs font-semibold uppercase tracking-wider text-text-muted"
         >{title}</span>
  </div>
  <div class="p-4">
    {@render children()}
  </div>
</div>
