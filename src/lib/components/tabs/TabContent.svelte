<!-- src/lib/components/tabs/TabContent.svelte -->
<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    width = "2xl",
    children,
  }: {
    width?: "xl" | "2xl" | "3xl" | "4xl";
    children: Snippet;
  } = $props();

  const widthClass: Record<string, string> = {
    xl:  "max-w-xl",
    "2xl": "max-w-2xl",
    "3xl": "max-w-3xl",
    "4xl": "max-w-4xl",
  };
</script>

<div class="flex flex-col gap-4 {widthClass[width]}">
  {@render children()}
</div>
