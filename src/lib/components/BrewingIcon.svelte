<script lang="ts">
  import { ICONS, type BrewingIconName } from "$lib/icons";

  let { name }: { name: BrewingIconName } = $props();
</script>

<span aria-hidden="true">{ICONS[name]}</span>
