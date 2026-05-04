<script lang="ts">
  import "../app.css";
  import "../themes/midnight.css";
  import AppShell from "$lib/components/AppShell.svelte";

  let { children } = $props();
</script>

<AppShell>
  {@render children()}
</AppShell>
