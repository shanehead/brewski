<script lang="ts">
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";
  import type { BrewingIconName } from "$lib/icons";

  interface Tab {
    key: string;
    label: string;
    icon?: BrewingIconName;
  }

  let {
    tabs,
    active,
    onchange,
    flush = false,
  }: {
    tabs: readonly Tab[];
    active: string;
    onchange: (key: string) => void;
    flush?: boolean;
  } = $props();
</script>

<div
  class="flex gap-1 w-fit"
  class:p-1={!flush}
  class:px-1={flush}
  class:pt-1={flush}
  class:rounded-lg={!flush}
  class:rounded-t-lg={flush}
  style="background: var(--color-bg-elevated); border: 1px solid var(--color-border); {flush ? 'border-bottom: none;' : ''}"
>
  {#each tabs as tab}
    <button
      onclick={() => onchange(tab.key)}
      class="px-3 py-1 text-sm transition-colors inline-flex items-center gap-1.5"
      class:rounded-md={!flush}
      class:rounded-t-md={flush}
      style={active === tab.key
        ? "background: var(--color-accent); color: #fff;"
        : "color: var(--color-text-secondary); background: transparent;"}
    >
      {#if tab.icon}
        <BrewingIcon name={tab.icon} />
      {/if}
      {tab.label}
    </button>
  {/each}
</div>
