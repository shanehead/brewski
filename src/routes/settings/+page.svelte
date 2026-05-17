<script lang="ts">
  import { onMount } from "svelte";
  import { settings, loadSettings, saveSetting } from "$lib/stores/settings";
  import { ipc } from "$lib/stores/error";
  import DatabaseLocation from "$lib/components/DatabaseLocation.svelte";

  onMount(async () => {
    await ipc(loadSettings());
  });

  async function handleThemeChange(e: Event) {
    await ipc(saveSetting("theme", (e.target as HTMLSelectElement).value));
  }

  async function handleUnitsChange(e: Event) {
    await ipc(saveSetting("units", (e.target as HTMLSelectElement).value));
  }
</script>

<div class="flex-1 overflow-y-auto p-6" style="background: var(--color-bg-base);">
  <h1 class="text-lg font-semibold mb-6" style="color: var(--color-text-primary);">Settings</h1>

  <div class="flex flex-col gap-6 max-w-md">
    <!-- Appearance -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-semibold" style="color: var(--color-text-secondary);">Appearance</h2>
      <div class="flex items-center justify-between">
        <label for="select-theme" class="text-sm" style="color: var(--color-text-primary);">Theme</label>
        <select id="select-theme" value={$settings.theme ?? "midnight"} onchange={handleThemeChange}
                class="px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="midnight">Midnight</option>
          <option value="dracula">Dracula</option>
          <option value="tokyo-night">Tokyo Night</option>
          <option value="catppuccin">Catppuccin</option>
          <option value="nord">Nord</option>
          <option value="monokai">Monokai</option>
          <option value="catppuccin-latte">Catppuccin Latte</option>
          <option value="solarized-light">Solarized Light</option>
          <option value="ayu-light">Ayu Light</option>
          <option value="github-light">GitHub Light</option>
        </select>
      </div>
    </section>

    <!-- Units -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-semibold" style="color: var(--color-text-secondary);">Units</h2>
      <div class="flex items-center justify-between">
        <label for="select-units" class="text-sm" style="color: var(--color-text-primary);">Measurement System</label>
        <select id="select-units" value={$settings.units ?? "metric"} onchange={handleUnitsChange}
                class="px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="metric">Metric (L, kg, °C)</option>
          <option value="imperial">Imperial (gal, lb, °F)</option>
        </select>
      </div>
    </section>

    <DatabaseLocation />
  </div>
</div>
