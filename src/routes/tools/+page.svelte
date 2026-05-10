<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";

  onMount(() => {
    void goto("/tools/abv-calories", { replaceState: true });
  });
</script>
