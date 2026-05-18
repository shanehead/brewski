<script lang="ts">
  import { page } from "$app/stores";

  const TABS = [
    {
      label: "Recipes",
      href: "/",
      activeWhen: (p: string) => p === "/" || p.startsWith("/recipe"),
      icon: `<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><polyline points="10 9 9 9 8 9"/>`,
    },
    {
      label: "Batches",
      href: "/batches",
      activeWhen: (p: string) => p.startsWith("/batches"),
      icon: `<path d="M9 3H5a2 2 0 0 0-2 2v4m6-6h10a2 2 0 1 2 2v4M9 3v18m0 0h10a2 2 0 0 0 2-2v-4M9 21H5a2 2 0 0 1-2-2v-4m0 0h18"/>`,
    },
    {
      label: "Tools",
      href: "/tools",
      activeWhen: (p: string) => p.startsWith("/tools"),
      icon: `<path d="M14.7 6.3a4 4 0 0 0-4.7 4.7L3 18a2 2 0 0 0 2.8 2.8l7-7a4 4 0 0 0 4.7-4.7l-2.1 2.1-1.4-1.4z"/>`,
    },
    {
      label: "More",
      href: "/settings",
      activeWhen: (p: string) => p.startsWith("/settings") || p.startsWith("/equipment"),
      icon: `<circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>`,
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
      class="flex flex-col items-center justify-center flex-1 py-2 gap-1 text-xs transition-colors"
      style={active ? "color: var(--color-accent);" : "color: var(--color-text-secondary);"}
    >
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        {@html tab.icon}
      </svg>
      {tab.label}
    </a>
  {/each}
</nav>
