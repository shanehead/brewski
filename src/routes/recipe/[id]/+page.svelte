<script lang="ts">
  import type { PageData } from "./$types";
  import RecipeView from "$platform/RecipeView.svelte";

  let { data }: { data: PageData } = $props();
</script>

<RecipeView id={data.id} />
