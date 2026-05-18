<script lang="ts">
  import "../app.css";
  import "../themes/midnight.css";
  import "../themes/dracula.css";
  import "../themes/tokyo-night.css";
  import "../themes/catppuccin.css";
  import "../themes/nord.css";
  import "../themes/monokai.css";
  import "../themes/catppuccin-latte.css";
  import "../themes/solarized-light.css";
  import "../themes/ayu-light.css";
  import "../themes/github-light.css";
  import { settings } from "$lib/stores/settings";
  import AppShell from "$platform/AppShell.svelte";

  let { children } = $props();

  $effect(() => {
    document.documentElement.dataset.theme = $settings.theme ?? "midnight";
  });
</script>

<AppShell>
  {@render children()}
</AppShell>
