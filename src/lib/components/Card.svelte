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

<div class="rounded-xl overflow-hidden"
     style="background: var(--color-bg-surface); border: 1px solid var(--color-border);">
  <div class="px-4 py-2.5 border-b"
       style="border-color: var(--color-border);">
    <span class="text-xs font-semibold uppercase tracking-wider"
          style="color: var(--color-text-muted);">{title}</span>
  </div>
  <div class="p-4">
    {@render children()}
  </div>
</div>
