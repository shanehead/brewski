<script lang="ts">
  import type { PageData } from "./$types";
  import BatchView from "$platform/BatchView.svelte";

  let { data }: { data: PageData } = $props();
</script>

<BatchView id={data.id} />
