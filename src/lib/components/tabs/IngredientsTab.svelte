<script lang="ts">
  import type { Recipe } from "$lib/api";
  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();
</script>
<p style="color: var(--color-text-muted)">Ingredients — coming soon</p>
