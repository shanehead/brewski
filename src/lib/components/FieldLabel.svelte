<!-- src/lib/components/FieldLabel.svelte -->
<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    for: htmlFor,
    class: extraClass = "",
    children,
  }: {
    for?: string;
    class?: string;
    children: Snippet;
  } = $props();
</script>

<label
  for={htmlFor}
  class="text-[13px] font-medium {extraClass}"
  style="color: var(--color-text-secondary);"
>
  {@render children()}
</label>
