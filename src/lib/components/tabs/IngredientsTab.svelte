<!-- src/lib/components/tabs/IngredientsTab.svelte -->
<script lang="ts">
  import type { Recipe } from "$lib/api";
  import FermentablesTable from "$lib/components/ingredients/FermentablesTable.svelte";
  import HopsTable from "$lib/components/ingredients/HopsTable.svelte";
  import YeastsTable from "$lib/components/ingredients/YeastsTable.svelte";
  import MiscTable from "$lib/components/ingredients/MiscTable.svelte";
  import Card from "$lib/components/Card.svelte";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();
</script>

<div class="flex flex-col gap-4">
  <Card title="Fermentables">
    <FermentablesTable {recipe} {onchange} />
  </Card>

  <Card title="Hops">
    <HopsTable {recipe} {onchange} />
  </Card>

  <Card title="Yeast">
    <YeastsTable {recipe} {onchange} />
  </Card>

  <Card title="Misc">
    <MiscTable {recipe} {onchange} />
  </Card>
</div>
