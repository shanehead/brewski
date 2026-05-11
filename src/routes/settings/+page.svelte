<script lang="ts">
  import { onMount } from "svelte";
  import { settings, loadSettings, saveSetting } from "$lib/stores/settings";
  import { listEquipmentProfiles, createEquipmentProfile, deleteEquipmentProfile } from "$lib/api";
  import type { EquipmentProfile } from "$lib/api";
  import { ipc } from "$lib/stores/error";

  let profiles = $state<EquipmentProfile[]>([]);
  let newProfileName = $state("");

  onMount(async () => {
    await ipc(loadSettings());
    profiles = await ipc(listEquipmentProfiles()) ?? [];
  });

  async function handleThemeChange(e: Event) {
    await ipc(saveSetting("theme", (e.target as HTMLSelectElement).value));
  }

  async function handleUnitsChange(e: Event) {
    await ipc(saveSetting("units", (e.target as HTMLSelectElement).value));
  }

  async function handleDefaultEquipChange(e: Event) {
    await ipc(saveSetting("default_equipment_profile_id", (e.target as HTMLSelectElement).value));
  }

  async function handleAddProfile() {
    if (!newProfileName.trim()) return;
    await ipc(createEquipmentProfile({
      name: newProfileName,
      boil_size_l: 27.0,
      batch_size_l: 23.0,
      efficiency_pct: 72.0,
    }));
    profiles = await ipc(listEquipmentProfiles()) ?? profiles;
    newProfileName = "";
  }

  async function handleDeleteProfile(id: string) {
    if (!confirm("Delete this equipment profile?")) return;
    await ipc(deleteEquipmentProfile(id));
    profiles = await ipc(listEquipmentProfiles()) ?? profiles;
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

    <!-- Equipment profiles -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-semibold" style="color: var(--color-text-secondary);">Equipment Profiles</h2>
      <div class="flex items-center justify-between">
        <label for="select-default-profile" class="text-sm" style="color: var(--color-text-primary);">Default Profile</label>
        <select id="select-default-profile" value={$settings.default_equipment_profile_id ?? ""}
                onchange={handleDefaultEquipChange}
                class="px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          <option value="">None</option>
          {#each profiles as p}
            <option value={p.id}>{p.name}</option>
          {/each}
        </select>
      </div>

      {#each profiles as p (p.id)}
        <div class="flex items-center justify-between py-1 border-t" style="border-color: var(--color-border);">
          <div>
            <p class="text-sm" style="color: var(--color-text-primary);">{p.name}</p>
            <p class="text-xs" style="color: var(--color-text-secondary);">
              {p.batch_size_l}L batch · {p.efficiency_pct}% efficiency
            </p>
          </div>
          <button onclick={() => handleDeleteProfile(p.id)} class="text-xs px-2 py-1 rounded"
                  style="color: var(--color-text-secondary); background: var(--color-bg-elevated);">Delete</button>
        </div>
      {/each}

      <div class="flex gap-2 pt-1">
        <input type="text" bind:value={newProfileName} placeholder="New profile name"
               class="flex-1 px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
        <button onclick={handleAddProfile} class="text-xs px-3 py-1.5 rounded"
                style="background: var(--color-accent); color: #fff;">Add</button>
      </div>
    </section>
  </div>
</div>
