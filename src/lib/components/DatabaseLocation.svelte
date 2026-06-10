<script lang="ts">
  import { onMount } from "svelte";
  import { detectSyncFolders, moveDatabase, getDbPath } from "$lib/api";
  import type { SyncFolder } from "$lib/api";
  import { ipc } from "$lib/stores/error";

  let currentPath: string = "";
  let syncFolders: SyncFolder[] = [];
  let customPath: string = "";
  let moving = false;

  onMount(async () => {
    const [path, folders] = await Promise.all([ipc(getDbPath()), ipc(detectSyncFolders())]);
    currentPath = path ?? "";
    syncFolders = folders ?? [];
  });

  async function handleMove(path: string) {
    if (!path || !path.trim()) return;
    moving = true;
    await ipc(moveDatabase(path));
    moving = false;
  }
</script>

<section class="flex flex-col gap-3">
  <h2 class="text-sm font-semibold text-text-secondary">
    Database Location
  </h2>

  {#if currentPath}
    <p class="text-xs font-mono break-all text-text-muted">
      {currentPath}
    </p>
  {/if}

  {#if syncFolders.length > 0}
    <div class="flex flex-col gap-2">
      {#each syncFolders as folder (folder.path)}
        <div class="flex items-center justify-between gap-3">
          <div class="flex flex-col">
            <span class="text-sm text-text-primary">{folder.name}</span>
            <span class="text-xs font-mono text-text-muted">{folder.path}</span>
          </div>
          <button
            disabled={moving}
            on:click={() => handleMove(folder.path)}
            class="px-3 py-1 text-sm rounded bg-bg-elevated text-text-primary border border-border"
           
          >
            Move here
          </button>
        </div>
      {/each}
    </div>
  {/if}

  <div class="flex items-center gap-2">
    <input
      type="text"
      placeholder="Custom path..."
      bind:value={customPath}
      class="flex-1 px-2 py-1.5 rounded text-sm bg-bg-elevated text-text-primary border border-border"
     
    />
    <button
      disabled={moving || !customPath.trim()}
      on:click={() => handleMove(customPath)}
      class="px-3 py-1.5 text-sm rounded bg-bg-elevated text-text-primary border border-border"
     
    >
      Move here
    </button>
  </div>

  <p class="text-xs text-text-muted">
    <strong>Last write wins</strong> — if you edit on two devices without syncing in between,
    the device that syncs last will overwrite the other.
  </p>
</section>
