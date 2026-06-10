<!-- src/lib/components/tabs/TabContent.svelte -->
<script lang="ts">
  import type { Snippet } from "svelte";

  let { children }: { children: Snippet } = $props();
</script>

<div class="flex flex-col gap-4 w-full">
  {@render children()}
</div>
