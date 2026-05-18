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
  <div class="rounded-lg p-6 w-full max-w-md shadow-xl"
       style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);">
    <h3 class="text-base font-semibold mb-1" style="color: var(--color-text-primary);">Copy Equipment Profile</h3>
    <p class="text-sm mb-4" style="color: var(--color-text-secondary);">Create a copy of "{profile.name}"</p>
    <input class="w-full px-2 py-1.5 rounded text-sm mb-4"
           style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);"
           bind:value={name} />
    <div class="flex justify-end gap-2">
      <button class="px-3 py-1.5 rounded text-sm"
              style="background: var(--color-bg-base); color: var(--color-text-secondary); border: 1px solid var(--color-border);"
              onclick={oncancel}>Cancel</button>
      <button class="px-3 py-1.5 rounded text-sm"
              style="background: var(--color-accent); color: #fff;"
              onclick={() => onconfirm(name)}>Copy</button>
    </div>
  </div>
</div>
