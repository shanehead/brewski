<script lang="ts">
  import { untrack } from "svelte";
  import type { EquipmentProfile } from "$lib/api";

  let {
    profile,
    onconfirm,
    oncancel,
  }: {
    profile: EquipmentProfile;
    onconfirm: (name: string) => void;
    oncancel: () => void;
  } = $props();

  let name = $state(untrack(() => profile.name + " (copy)"));
</script>

<div class="fixed inset-0 flex items-center justify-center" style="background: rgba(0,0,0,0.4);">
  <div class="rounded-lg p-6 w-full max-w-md shadow-xl bg-bg-elevated border border-border"
      >
    <h3 class="text-base font-semibold mb-1 text-text-primary">Copy Equipment Profile</h3>
    <p class="text-sm mb-4 text-text-secondary">Create a copy of "{profile.name}"</p>
    <input class="w-full px-2 py-1.5 rounded text-sm mb-4 bg-bg-base text-text-primary border border-border"
          
           bind:value={name} />
    <div class="flex justify-end gap-2">
      <button class="px-3 py-1.5 rounded text-sm bg-bg-base text-text-secondary border border-border"
             
              onclick={oncancel}>Cancel</button>
      <button class="px-3 py-1.5 rounded text-sm bg-accent"
              style="color: #fff;"
              onclick={() => onconfirm(name)}>Copy</button>
    </div>
  </div>
</div>
