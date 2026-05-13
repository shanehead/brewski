<script lang="ts">
  import type { Recipe, MashStep, RecipeStats, UpdateMashInput } from "$lib/api";
  import { updateMash, createMashStep, deleteMashStep, updateMashStep } from "$lib/api";
  import { ipc } from "$lib/stores/error";
  import { settings } from "$lib/stores/settings";
  import { type Units, cToF, fToC, lToGal, galToL, tempLabel, volumeLabel, lPerKgToQtPerLb, qtPerLbToLPerKg, ratioLabel } from "$lib/units";

  let { recipe, stats, onchange }: { recipe: Recipe; stats: RecipeStats | null; onchange: () => void } = $props();

  let addingStep = $state(false);
  let stepName = $state("Mash In");
  let stepType = $state("infusion");
  let stepTemp = $state(67.0);
  let stepTime = $state(60);
  let stepInfuse = $state<number | null>(null);

  const mash = $derived(recipe.mash);
  const totalGrainKg = $derived(
    recipe.fermentables.reduce((sum, f) => sum + f.amount_kg, 0)
  );
  const firstInfuseAmount = $derived(
    recipe.mash?.steps.find(s => s.infuse_amount_l != null)?.infuse_amount_l ?? null
  );
  const canAutoDerive = $derived(totalGrainKg > 0 && firstInfuseAmount != null);

  async function ensureMash() {
    if (!mash) {
      await ipc(updateMash(recipe.id, { name: "Single Infusion", grain_temp_c: 21 }));
      onchange();
    }
  }

  async function handleAddStep() {
    await ensureMash();
    const currentMash = recipe.mash!;
    await ipc(createMashStep(currentMash.id, {
      name: stepName,
      type_: stepType,
      step_temp_c: stepTemp,
      step_time_min: stepTime,
      infuse_amount_l: stepInfuse ?? undefined,
    }));
    addingStep = false;
    onchange();
  }

  async function handleDeleteStep(id: string) {
    await ipc(deleteMashStep(id));
    onchange();
  }

  async function handleMashField(input: UpdateMashInput) {
    await ensureMash();
    await ipc(updateMash(recipe.id, input));
    onchange();
  }

  const STEP_TYPES = ["infusion", "temperature", "decoction"] as const;
  const units = $derived<Units>($settings.units === "imperial" ? "imperial" : "metric");

import { onDestroy } from 'svelte';
let editingStepId = $state<string | null>(null);
let hoveredStepId = $state<string | null>(null);
let _docClickHandler: ((e: MouseEvent) => void) | null = null;
  
async function handleUpdateStepField(id: string, field: string, value: any) {
  const payload: any = {};
  payload[field] = value;
  await ipc(updateMashStep(id, payload));
  onchange();
}
  
function _attachDocClick(id: string) {
  // detach any existing
  if (_docClickHandler) {
    document.removeEventListener('click', _docClickHandler);
    _docClickHandler = null;
  }
  const currentId = id;
  _docClickHandler = (e: MouseEvent) => {
    const target = e.target as HTMLElement | null;
    if (!target) return;
    // if click is outside the editing row, close the editor
    if (!target.closest(`[data-step-id="${currentId}"]`)) {
      closeEdit();
    }
  };
  // attach handler on next tick so the opening click doesn't immediately trigger it
  setTimeout(() => {
    document.addEventListener('click', _docClickHandler as EventListener);
  }, 0);
}

function _detachDocClick() {
  if (_docClickHandler) {
    document.removeEventListener('click', _docClickHandler);
    _docClickHandler = null;
  }
}

function toggleEditStep(id: string) {
  if (editingStepId === id) {
    closeEdit();
    return;
  }
  editingStepId = id;
  _attachDocClick(id);
}
  
function closeEdit() {
  editingStepId = null;
  _detachDocClick();
}
  
function onEditKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    closeEdit();
  }
}

onDestroy(() => {
  _detachDocClick();
});
</script>

<div class="flex flex-col gap-4 w-full">
  <!-- Mash profile settings -->
  <div class="grid grid-cols-2 gap-3">
    <div class="flex flex-col gap-1">
      <label for="mash-name" class="text-xs font-medium" style="color: var(--color-text-secondary);">Profile Name</label>
      <input id="mash-name" type="text" value={mash?.name ?? "Single Infusion"}
             onblur={(e) => handleMashField({ name: (e.target as HTMLInputElement).value })}
             class="px-2 py-1.5 rounded text-sm"
             style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
    </div>
    <div class="flex flex-col gap-1">
      <label for="mash-grain-temp" class="text-xs font-medium" style="color: var(--color-text-secondary);">Grain Temp ({tempLabel(units)})</label>
      <input id="mash-grain-temp" type="number" step={units === "imperial" ? 1 : 0.5}
             value={(units === "imperial" ? cToF(mash?.grain_temp_c ?? 21) : mash?.grain_temp_c ?? 21).toFixed(1)}
             onblur={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); handleMashField({ grain_temp_c: units === "imperial" ? fToC(v) : v }); }}
             class="px-2 py-1.5 rounded text-sm"
             style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
    </div>
    <div class="flex flex-col gap-1">
      <label for="mash-sparge-temp" class="text-xs font-medium" style="color: var(--color-text-secondary);">Sparge Temp ({tempLabel(units)})</label>
      <input id="mash-sparge-temp" type="number" step={units === "imperial" ? 1 : 0.5}
             value={mash?.sparge_temp_c != null ? (units === "imperial" ? cToF(mash.sparge_temp_c) : mash.sparge_temp_c).toFixed(1) : ""}
             placeholder={units === "imperial" ? "167" : "75"}
             onblur={(e) => {
               const v = (e.target as HTMLInputElement).value;
               handleMashField({ sparge_temp_c: v ? (units === "imperial" ? fToC(parseFloat(v)) : parseFloat(v)) : undefined });
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
               handleMashField({ ph: v ? parseFloat(v) : undefined });
             }}
             class="px-2 py-1.5 rounded text-sm"
             style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
    </div>

    {#if stats?.strike_temp_c != null}
      <div class="flex flex-col gap-1">
        <span class="text-xs font-medium" style="color: var(--color-text-secondary);">Strike Temp ({tempLabel(units)})</span>
        <span class="px-2 py-1.5 text-sm" style="color: var(--color-text-primary);">
          {(units === "imperial" ? cToF(stats.strike_temp_c) : stats.strike_temp_c).toFixed(1)}{tempLabel(units)}
        </span>
      </div>
    {/if}

    {#if mash && !canAutoDerive}
      <div class="flex flex-col gap-1">
        <label for="mash-ratio" class="text-xs font-medium" style="color: var(--color-text-secondary);">Water:Grain Ratio ({ratioLabel(units)})</label>
        <input id="mash-ratio" type="number" step="0.1"
               value={mash.ratio_l_per_kg != null
                 ? (units === "imperial" ? lPerKgToQtPerLb(mash.ratio_l_per_kg) : mash.ratio_l_per_kg).toFixed(2)
                 : ""}
               placeholder={units === "imperial" ? "1.5" : "3.0"}
               onblur={(e) => {
                 const v = (e.target as HTMLInputElement).value;
                 if (v) {
                   const parsed = parseFloat(v);
                   handleMashField({ ratio_l_per_kg: units === "imperial" ? qtPerLbToLPerKg(parsed) : parsed });
                 }
               }}
               class="px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-elevated); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
      </div>
    {/if}
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
        <input type="number" step={units === "imperial" ? 1 : 0.5}
               value={(units === "imperial" ? cToF(stepTemp) : stepTemp).toFixed(1)}
               oninput={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); if (!isNaN(v)) stepTemp = units === "imperial" ? fToC(v) : v; }}
               placeholder="Temp {tempLabel(units)}"
               class="w-20 px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
        <input type="number" bind:value={stepTime} step="5" placeholder="Time min"
               class="w-20 px-2 py-1.5 rounded text-sm"
               style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
        {#if stepType === "infusion"}
          <input type="number" step="0.1"
                 value={stepInfuse != null ? (units === "imperial" ? lToGal(stepInfuse) : stepInfuse).toFixed(1) : ""}
                 oninput={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); stepInfuse = isNaN(v) ? null : (units === "imperial" ? galToL(v) : v); }}
                 placeholder="Infuse {volumeLabel(units)}"
                 class="w-24 px-2 py-1.5 rounded text-sm"
                 style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
        {/if}
        <button onclick={handleAddStep} class="text-xs px-3 py-1.5 rounded"
                style="background: var(--color-accent); color: #fff;">Add</button>
      </div>
    {/if}

    {#if mash && mash.steps.length > 0}
      <div class="flex flex-col gap-1">
        {#each mash.steps as step (step.id)}
          <div class="flex items-center gap-3 py-2 border-t" 
               style="border-color: var(--color-border); background: {hoveredStepId === step.id ? 'var(--color-bg-elevated)' : 'transparent'};"
               data-step-id={step.id}
               role="button"
               onclick={() => toggleEditStep(step.id)}
               onmouseenter={() => hoveredStepId = step.id}
               onmouseleave={() => hoveredStepId = null}
               tabindex="0"
               onkeydown={editingStepId === step.id ? onEditKeydown : undefined}>
            <div class="flex-1">
              {#if editingStepId === step.id}
                <div class="flex flex-wrap gap-2 p-2 rounded" style="background: var(--color-bg-elevated);">
                  <div class="flex flex-col min-w-24">
                    <label class="text-xs" style="color: var(--color-text-secondary);">Name</label>
                    <input type="text" value={step.name}
                           onclick={(e) => e.stopPropagation()}
                           onblur={(e) => handleUpdateStepField(step.id, 'name', (e.target as HTMLInputElement).value)}
                           class="flex-1 min-w-24 px-2 py-1.5 h-10 rounded text-sm"
                           style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
                  </div>
                  <div class="flex flex-col w-28">
                    <label class="text-xs" style="color: var(--color-text-secondary);">Type</label>
                    <select value={step.type_} onclick={(e) => e.stopPropagation()} onblur={(e) => handleUpdateStepField(step.id, 'type_', (e.target as HTMLSelectElement).value)}
                            class="w-28 px-2 py-1.5 h-10 rounded text-sm"
                            style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);">
                      {#each STEP_TYPES as t}
                        <option value={t}>{t}</option>
                      {/each}
                    </select>
                  </div>
                  <div class="flex flex-col w-20">
                    <label class="text-xs" style="color: var(--color-text-secondary);">Temp ({tempLabel(units)})</label>
                    <input type="number" step={units === "imperial" ? 1 : 0.5}
                           value={(units === "imperial" ? cToF(step.step_temp_c) : step.step_temp_c).toFixed(1)}
                           onclick={(e) => e.stopPropagation()}
                           onblur={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); handleUpdateStepField(step.id, 'step_temp_c', units === 'imperial' ? fToC(v) : v); }}
                           class="w-20 px-2 py-1.5 h-10 rounded text-sm"
                           style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
                  </div>
                  <div class="flex flex-col w-20">
                    <label class="text-xs" style="color: var(--color-text-secondary);">Time (min)</label>
                    <input type="number" step="5" value={step.step_time_min}
                           onclick={(e) => e.stopPropagation()}
                           onblur={(e) => handleUpdateStepField(step.id, 'step_time_min', parseFloat((e.target as HTMLInputElement).value))}
                           class="w-20 px-2 py-1.5 h-10 rounded text-sm"
                           style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
                  </div>
                  {#if step.type_ === 'infusion'}
                    <div class="flex flex-col w-24">
                      <label class="text-xs" style="color: var(--color-text-secondary);">Infuse ({volumeLabel(units)})</label>
                      <input type="number" step="0.1"
                             value={step.infuse_amount_l != null ? (units === 'imperial' ? lToGal(step.infuse_amount_l) : step.infuse_amount_l).toFixed(1) : ''}
                             onclick={(e) => e.stopPropagation()}
                             onblur={(e) => { const v = parseFloat((e.target as HTMLInputElement).value); handleUpdateStepField(step.id, 'infuse_amount_l', units === 'imperial' ? galToL(v) : v); }}
                             class="w-24 px-2 py-1.5 h-10 rounded text-sm"
                             style="background: var(--color-bg-base); color: var(--color-text-primary); border: 1px solid var(--color-border);" />
                    </div>
                  {/if}
                </div>
              {:else}
              <p class="text-lg font-medium" style="color: var(--color-text-primary);">{step.name}</p>
              <p class="text-sm" style="color: var(--color-text-secondary);">
                  {(units === "imperial" ? cToF(step.step_temp_c) : step.step_temp_c).toFixed(1)}{tempLabel(units)} · {step.step_time_min} min · {step.type_}
                  {#if step.infuse_amount_l} · {(units === "imperial" ? lToGal(step.infuse_amount_l) : step.infuse_amount_l).toFixed(1)}{volumeLabel(units)}{/if}
                </p>
              {/if}
            </div>
            <button onclick={(e) => { e.stopPropagation(); handleDeleteStep(step.id); }} class="text-xs opacity-40 hover:opacity-100"
                    style="color: var(--color-text-secondary);">×</button>
          </div>
        {/each}
      </div>
    {:else}
      <p class="text-xs py-2" style="color: var(--color-text-muted);">No mash steps yet</p>
    {/if}
  </div>
</div>
