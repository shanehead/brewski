<script lang="ts">
  import { onMount } from "svelte";
  import { settings, loadSettings, saveSetting } from "$lib/stores/settings";
  import { lToGal, volumeLabel } from "$lib/units";
  import { listEquipmentProfiles, createEquipmentProfile, deleteEquipmentProfile, copyEquipmentProfile } from "$lib/api";
  import type { EquipmentProfile } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import ConfirmModal from "$lib/components/ConfirmModal.svelte";
  import EquipmentProfileModal from "$lib/components/EquipmentProfileModal.svelte";
  import CopyNameModal from "./EquipmentProfileModal.svelte";

  let profiles = $state<EquipmentProfile[]>([]);
  let newProfileName = $state("");
  let showDeleteModal = $state(false);
  let deleteCandidate = $state<EquipmentProfile | null>(null);
  let editingProfile = $state<EquipmentProfile | null>(null);
  let showCopyModal = $state(false);
  let copyCandidate = $state<EquipmentProfile | null>(null);

  onMount(async () => {
    await ipc(loadSettings());
    profiles = await ipc(listEquipmentProfiles()) ?? [];
  });

  async function refreshProfiles() {
    profiles = await ipc(listEquipmentProfiles()) ?? profiles;
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
    await refreshProfiles();
    newProfileName = "";
  }

  function handleEditProfile(profile: EquipmentProfile) {
    editingProfile = profile;
  }

  function handleDeleteProfile(profile: EquipmentProfile) {
    deleteCandidate = profile;
    showDeleteModal = true;
  }

  async function confirmDelete() {
    if (!deleteCandidate) return;
    showDeleteModal = false;
    await ipc(deleteEquipmentProfile(deleteCandidate.id));
    await refreshProfiles();
    deleteCandidate = null;
  }

  function cancelDelete() {
    showDeleteModal = false;
    deleteCandidate = null;
  }

  async function handleModalSave(saved: EquipmentProfile) {
    editingProfile = null;
    await refreshProfiles();
  }

  function handleModalCancel() {
    editingProfile = null;
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

{#if editingProfile}
  <EquipmentProfileModal
    profile={editingProfile}
    onsave={handleModalSave}
    oncancel={handleModalCancel}
  />
{/if}

{#if showCopyModal && copyCandidate}
  <CopyNameModal
    profile={copyCandidate}
    onconfirm={async (newName) => {
      showCopyModal = false;
      const candidate = copyCandidate!;
      await ipc(copyEquipmentProfile(candidate.id, newName));
      await refreshProfiles();
      copyCandidate = null;
    }}
    oncancel={() => { showCopyModal = false; copyCandidate = null; }}
  />
{/if}

<div class="flex-1 overflow-y-auto p-6 bg-bg-base">
  <h1 class="text-lg font-semibold mb-6 text-text-primary">Equipment</h1>

  <div class="flex flex-col gap-6 max-w-md">
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-semibold text-text-secondary">Equipment Profiles</h2>
      <div class="flex items-center justify-between">
        <label for="select-default-profile" class="text-sm text-text-primary">Default Profile</label>
        <select id="select-default-profile" value={$settings.default_equipment_profile_id ?? ""}
                onchange={handleDefaultEquipChange}
                class="px-2 py-1.5 rounded text-sm bg-bg-elevated text-text-primary border border-border"
               >
          <option value="">None</option>
          {#each profiles as p}
            <option value={p.id}>{p.name}</option>
          {/each}
        </select>
      </div>

      {#each profiles as p (p.id)}
        <div class="flex items-center justify-between py-1 border-t border-border">
          <div>
            <p class="text-sm text-text-primary">{p.name}</p>
            <p class="text-xs text-text-secondary">
              {($settings.units === "imperial" ? lToGal(p.batch_size_l) : p.batch_size_l).toFixed(1)}{volumeLabel($settings.units ?? "metric")} batch · {p.efficiency_pct}% efficiency
            </p>
          </div>
          <div class="flex gap-2">
            <button onclick={() => handleEditProfile(p)} class="text-xs px-2 py-1 rounded text-text-secondary bg-bg-elevated"
                   >Edit</button>
            <button onclick={() => { copyCandidate = p; showCopyModal = true; }} class="text-xs px-2 py-1 rounded text-text-secondary bg-bg-elevated"
                   >Copy</button>
            <button onclick={() => handleDeleteProfile(p)} class="text-xs px-2 py-1 rounded text-text-secondary bg-bg-elevated"
                   >Delete</button>
          </div>
        </div>
      {/each}

      <div class="flex gap-2 pt-1">
        <input type="text" bind:value={newProfileName} placeholder="New profile name"
               class="flex-1 px-2 py-1.5 rounded text-sm bg-bg-elevated text-text-primary border border-border"
               />
        <button onclick={handleAddProfile} class="text-xs px-3 py-1.5 rounded bg-accent"
                style="color: #fff;">Add</button>
      </div>
    </section>
  </div>
</div>
