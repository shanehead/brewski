<script lang="ts">
  import { ICONS, type BrewingIconName } from "$lib/icons";

  let { name, size = 18 }: { name: BrewingIconName; size?: number } = $props();
</script>

<svg
  aria-hidden="true"
  data-icon={name}
  width={size}
  height={size}
  viewBox="0 0 24 24"
>
  {@html ICONS[name]}
</svg>
