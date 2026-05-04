<script lang="ts">
  import type { Recipe, MashStep } from "$lib/api";
  import { updateMash, createMashStep, deleteMashStep } from "$lib/api";

  let { recipe, onchange }: { recipe: Recipe; onchange: () => void } = $props();

  let addingStep = $state(false);
  let stepName = $state("Mash In");
  let stepType = $state("infusion");
  let stepTemp = $state(67.0);
  let stepTime = $state(60);
  let stepInfuse = $state<number | null>(null);

  const mash = $derived(recipe.mash);

  async function ensureMash() {
    if (!mash) {
      await updateMash(recipe.id, { name: "Single Infusion", grain_temp_c: 21 });
      onchange();
    }
  }

  async function handleAddStep() {
    await ensureMash();
    const currentMash = recipe.mash!;
    await createMashStep(currentMash.id, {
      name: stepName,
      type_: stepType,
      step_temp_c: stepTemp,
      step_time_min: stepTime,
      infuse_amount_l: stepInfuse,
    });
    addingStep = false;
    onchange();
  }

  async function handleDeleteStep(id: string) {
    await deleteMashStep(id);
    onchange();
  }

  async function handleMashField(field: string, value: unknown) {
    await ensureMash();
    await updateMash(recipe.id, { [field]: value });
    onchange();
  }

  const STEP_TYPES = ["infusion", "temperature", "decoction"] as const;
</script>

<div class="flex flex-col gap-4 max-w-xl">
  <!-- Mash profile settings -->
  <div class="grid grid-cols-2 gap-3">
    <div class="flex flex-col gap-1">
      <label for="mash-name" class="text-xs font-medium" style="color: var(--color-text-secondary);">Profile Name</label>
      <input id="mash-name" type="text" value={mash?.name ?? "Single Infusion"}
             onblur={(e) => handleMashField("name", (e.target as HTMLInputElement).value)}
             class="px-2 py-1.5 rounded text-sm"
             style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
    </div>
    <div class="flex flex-col gap-1">
      <label for="mash-grain-temp" class="text-xs font-medium" style="color: var(--color-text-secondary);">Grain Temp (°C)</label>
      <input id="mash-grain-temp" type="number" step="1" value={mash?.grain_temp_c ?? 21}
             onblur={(e) => handleMashField("grain_temp_c", parseFloat((e.target as HTMLInputElement).value))}
             class="px-2 py-1.5 rounded text-sm"
             style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
    </div>
    <div class="flex flex-col gap-1">
      <label for="mash-sparge-temp" class="text-xs font-medium" style="color: var(--color-text-secondary);">Sparge Temp (°C)</label>
      <input id="mash-sparge-temp" type="number" step="1" value={mash?.sparge_temp_c ?? ""}
             placeholder="75"
             onblur={(e) => {
               const v = (e.target as HTMLInputElement).value;
               handleMashField("sparge_temp_c", v ? parseFloat(v) : null);
             }}
             class="px-2 py-1.5 rounded text-sm"
             style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
    </div>
    <div class="flex flex-col gap-1">
      <label for="mash-ph" class="text-xs font-medium" style="color: var(--color-text-secondary);">Mash pH</label>
      <input id="mash-ph" type="number" step="0.1" value={mash?.ph ?? ""}
             placeholder="5.4"
             onblur={(e) => {
               const v = (e.target as HTMLInputElement).value;
               handleMashField("ph", v ? parseFloat(v) : null);
             }}
             class="px-2 py-1.5 rounded text-sm"
             style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
    </div>
  </div>

  <!-- Mash steps -->
  <div class="flex flex-col gap-2">
    <div class="flex items-center justify-between">
      <h3 class="text-sm font-semibold" style="color: var(--color-text-primary);">Steps</h3>
      <button onclick={() => addingStep = !addingStep} class="text-xs px-2 py-1 rounded"
              style="background: var(--color-accent); color: #fff;">+ Add Step</button>
    </div>

    {#if addingStep}
      <div class="flex flex-wrap gap-2 p-2 rounded" style="background: var(--color-bg-elevated);">
        <input type="text" bind:value={stepName} placeholder="Step name"
               class="flex-1 min-w-24 px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
        <select bind:value={stepType} class="w-28 px-2 py-1.5 rounded text-sm"
                style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);">
          {#each STEP_TYPES as t}
            <option value={t}>{t}</option>
          {/each}
        </select>
        <input type="number" bind:value={stepTemp} step="0.5" placeholder="Temp °C"
               class="w-20 px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
        <input type="number" bind:value={stepTime} step="5" placeholder="Time min"
               class="w-20 px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
        <button onclick={handleAddStep} class="text-xs px-3 py-1.5 rounded"
                style="background: var(--color-accent); color: #fff;">Add</button>
      </div>
    {/if}

    {#if mash && mash.steps.length > 0}
      <div class="flex flex-col gap-1">
        {#each mash.steps as step (step.id)}
          <div class="flex items-center gap-3 py-2 border-t" style="border-color: var(--color-border);">
            <div class="flex-1">
              <p class="text-sm" style="color: var(--color-text-primary);">{step.name}</p>
              <p class="text-xs" style="color: var(--color-text-secondary);">
                {step.step_temp_c}°C · {step.step_time_min} min · {step.type_}
                {#if step.infuse_amount_l} · {step.infuse_amount_l}L{/if}
              </p>
            </div>
            <button onclick={() => handleDeleteStep(step.id)} class="text-xs opacity-40 hover:opacity-100"
                    style="color: var(--color-text-secondary);">×</button>
          </div>
        {/each}
      </div>
    {:else}
      <p class="text-xs py-2" style="color: var(--color-text-muted);">No mash steps yet</p>
    {/if}
  </div>
</div>
