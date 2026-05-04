<script lang="ts">
  import type { Recipe } from "$lib/api";
  import FermentablesTable from "$lib/components/ingredients/FermentablesTable.svelte";
  import HopsTable from "$lib/components/ingredients/HopsTable.svelte";
  import YeastsTable from "$lib/components/ingredients/YeastsTable.svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();
</script>

<div class="flex flex-col gap-8">
  <FermentablesTable {recipe} {onchange} />
  <HopsTable {recipe} {onchange} />
  <YeastsTable {recipe} {onchange} />
</div>
