<script lang="ts">
  import { page } from "$app/stores";
  import BrewingIcon from "$lib/components/BrewingIcon.svelte";

  const TABS = [
    {
      label: "Recipes",
      href: "/",
      activeWhen: (p: string) => p === "/" || p.startsWith("/recipe"),
      icon: "recipes",
    },
    {
      label: "Batches",
      href: "/batches",
      activeWhen: (p: string) => p.startsWith("/batches"),
      icon: "batches",
    },
    {
      label: "Tools",
      href: "/tools",
      activeWhen: (p: string) => p.startsWith("/tools"),
      icon: "tools",
    },
    {
      label: "More",
      href: "/settings",
      activeWhen: (p: string) => p.startsWith("/settings") || p.startsWith("/equipment") || p.startsWith("/library"),
      icon: "settings",
    },
  ] as const;
</script>

<nav
  class="flex border-t flex-shrink-0"
  style="background: var(--color-bg-surface); border-color: var(--color-border); padding-bottom: env(safe-area-inset-bottom, 0px);"
>
  {#each TABS as tab}
    {@const active = tab.activeWhen($page.url.pathname)}
    <a
      href={tab.href}
      aria-current={active ? "page" : undefined}
      class="flex flex-col items-center justify-center flex-1 py-2 gap-1 text-xs transition-colors"
      style={active ? "color: var(--color-accent);" : "color: var(--color-text-secondary);"}
    >
      <span style={active ? "opacity: 1; transition: opacity 0.15s;" : "opacity: 0.45; transition: opacity 0.15s;"}>
        <BrewingIcon name={tab.icon} size={22} />
      </span>
      {tab.label}
    </a>
  {/each}
</nav>
