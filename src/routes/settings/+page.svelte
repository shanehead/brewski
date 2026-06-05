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

  async function handleGravityUnitChange(e: Event) {
    await ipc(saveSetting("gravity_unit", (e.target as HTMLSelectElement).value));
  }

  async function handleHideExamplesChange(e: Event) {
    await ipc(saveSetting("hide_example_recipes", (e.target as HTMLInputElement).checked ? "true" : "false"));
  }

  async function handleTooltipsChange(e: Event) {
    await ipc(saveSetting("show_tooltips", String((e.target as HTMLInputElement).checked)));
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
      <div class="flex items-center justify-between">
        <label for="select-gravity-unit" class="text-sm" style="color: var(--color-text-primary);">Gravity Unit</label>
        <select id="select-gravity-unit" value={$settings.gravity_unit ?? "sg"} onchange={handleGravityUnitChange}
                class="px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="sg">SG (1.050)</option>
          <option value="plato">Plato (°P)</option>
          <option value="brix">Brix (°Bx)</option>
        </select>
      </div>
    </section>

    <!-- Recipes -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-semibold" style="color: var(--color-text-secondary);">Recipes</h2>
      <div class="flex items-center justify-between">
        <label for="toggle-hide-examples" class="text-sm" style="color: var(--color-text-primary);">Hide Example Recipes</label>
        <input
          id="toggle-hide-examples"
          type="checkbox"
          checked={$settings.hide_example_recipes ?? false}
          onchange={handleHideExamplesChange}
          class="w-4 h-4 rounded cursor-pointer"
          style="accent-color: var(--color-accent);"
        />
      </div>
    </section>

    <!-- Help -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-semibold" style="color: var(--color-text-secondary);">Help</h2>
      <div class="flex items-center justify-between">
        <label for="toggle-tooltips" class="text-sm" style="color: var(--color-text-primary);">Show tooltips</label>
        <input
          id="toggle-tooltips"
          type="checkbox"
          checked={$settings.show_tooltips ?? true}
          onchange={handleTooltipsChange}
          class="w-4 h-4 rounded cursor-pointer"
          style="accent-color: var(--color-accent);"
        />
      </div>
      <p class="text-xs" style="color: var(--color-text-muted);">Turn off once you know your way around.</p>
    </section>

    <DatabaseLocation />

  </div>
</div>
