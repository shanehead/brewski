<script lang="ts">
  import { onMount } from "svelte";
  import { settings, loadSettings, saveSetting } from "$lib/stores/settings";
  import { listEquipmentProfiles, createEquipmentProfile, deleteEquipmentProfile } from "$lib/api";
  import type { EquipmentProfile } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import ConfirmModal from "$lib/components/ConfirmModal.svelte";

  let profiles = $state<EquipmentProfile[]>([]);
  let newProfileName = $state("");
  let showDeleteModal = $state(false);
  let deleteCandidate = $state<EquipmentProfile | null>(null);

  onMount(async () => {
    await ipc(loadSettings());
    profiles = await ipc(listEquipmentProfiles()) ?? [];
  });

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

  async function handleDeleteProfile(profile: EquipmentProfile) {
    deleteCandidate = profile;
    showDeleteModal = true;
  }

  async function confirmDelete() {
    if (!deleteCandidate) return;
    showDeleteModal = false;
    await ipc(deleteEquipmentProfile(deleteCandidate.id));
    profiles = await ipc(listEquipmentProfiles()) ?? profiles;
    deleteCandidate = null;
  }

  function cancelDelete() {
    showDeleteModal = false;
    deleteCandidate = null;
  }
</script>

{#if showDeleteModal && deleteCandidate}
  <ConfirmModal
    message="Delete this equipment profile? This cannot be undone."
    confirmLabel="Delete"
    dangerous={true}
    onconfirm={confirmDelete}
    oncancel={cancelDelete}
  />
{/if}

<div class="flex-1 overflow-y-auto p-6" style="background: var(--color-bg-base);">
  <h1 class="text-lg font-semibold mb-6" style="color: var(--color-text-primary);">Equipment</h1>

  <div class="flex flex-col gap-6 max-w-md">
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
          <button onclick={() => handleDeleteProfile(p)} class="text-xs px-2 py-1 rounded"
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
