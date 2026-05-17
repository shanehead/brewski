<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { EquipmentProfile } from "$lib/api";

  export let profile: EquipmentProfile;

  const dispatch = createEventDispatcher();
  let name: string = profile.name + " (copy)";

  function confirm() {
    dispatch('confirm', name);
  }
  function cancel() {
    dispatch('cancel');
  }
</script>

<div class="fixed inset-0 flex items-center justify-center bg-black/40">
  <div class="bg-white dark:bg-gray-900 rounded p-4 w-full max-w-md">
    <h3 class="text-lg font-semibold mb-2">Copy Equipment Profile</h3>
    <p class="text-sm text-gray-600 mb-3">Create a copy of "{profile.name}"</p>
    <input class="w-full p-2 border rounded mb-3" bind:value={name} />
    <div class="flex justify-end gap-2">
      <button class="px-3 py-1 rounded" on:click={cancel}>Cancel</button>
      <button class="px-3 py-1 rounded bg-blue-600 text-white" on:click={confirm}>Copy</button>
    </div>
  </div>
</div>