<script lang="ts">
  import type { PageData } from "./$types";
  import BaselineRecipeView from "$platform/BaselineRecipeView.svelte";

  let { data }: { data: PageData } = $props();
</script>

<BaselineRecipeView id={data.id} />
