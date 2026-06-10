<!-- src/lib/components/TabBar.svelte -->
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
  }: {
    tabs: readonly Tab[];
    active: string;
    onchange: (key: string) => void;
  } = $props();
</script>

<div class="flex gap-0 border-b border-border">
  {#each tabs as tab}
    <button
      onclick={() => onchange(tab.key)}
      class="px-4 py-2.5 text-sm transition-colors inline-flex items-center gap-1.5 border-b-2 -mb-px"
      style={active === tab.key
        ? "color: var(--color-accent); border-color: var(--color-accent); background: transparent;"
        : "color: var(--color-text-primary); border-color: transparent; background: transparent;"}
    >
      {#if tab.icon}
        <span style={active === tab.key ? "opacity: 1;" : "opacity: 0.6;"}>
          <BrewingIcon name={tab.icon} size={20} />
        </span>
      {/if}
      {tab.label}
    </button>
  {/each}
</div>
