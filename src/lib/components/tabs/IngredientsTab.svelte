<!-- src/lib/components/tabs/IngredientsTab.svelte -->
<script lang="ts">
  import type { Recipe, RecipeStats } from "$lib/api";
  import FermentablesTable from "$lib/components/ingredients/FermentablesTable.svelte";
  import HopsTable from "$lib/components/ingredients/HopsTable.svelte";
  import YeastsTable from "$lib/components/ingredients/YeastsTable.svelte";
  import MiscTable from "$lib/components/ingredients/MiscTable.svelte";
  import Card from "$lib/components/Card.svelte";
  import TabContent from "$lib/components/tabs/TabContent.svelte";

  let { recipe, stats, onchange }: { recipe: Recipe; stats: RecipeStats | null; onchange: () => void } = $props();
</script>

<TabContent>
  <Card title="Fermentables">
    <FermentablesTable {recipe} {onchange} />
  </Card>

  <Card title="Hops">
    <HopsTable {recipe} {stats} {onchange} />
  </Card>

  <Card title="Yeast">
    <YeastsTable {recipe} {onchange} />
  </Card>

  <Card title="Misc">
    <MiscTable {recipe} {onchange} />
  </Card>
</TabContent>
