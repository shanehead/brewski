<script lang="ts">
  import type {
    Hop, Fermentable, Yeast, Misc, Water,
    CreateHopInput, UpdateHopInput,
    CreateFermentableInput, UpdateFermentableInput,
    CreateYeastInput, UpdateYeastInput,
    CreateMiscInput, UpdateMiscInput,
    CreateWaterInput, UpdateWaterInput,
  } from '$lib/api';
  import {
    createHop, updateHop,
    createFermentable, updateFermentable,
    createYeast, updateYeast,
    createMisc, updateMisc,
    createWater, updateWater,
  } from '$lib/api';
  import { untrack } from 'svelte';
  import { ipc } from '$lib/stores/error';

  type IngredientType = 'hop' | 'fermentable' | 'yeast' | 'misc' | 'water';
  type AnyIngredient = Hop | Fermentable | Yeast | Misc | Water;

  let {
    type,
    ingredient = null,
    existingNames = [],
    onsave,
    onduplicate = onsave,
    oncancel,
  }: {
    type: IngredientType;
    ingredient?: AnyIngredient | null;
    existingNames?: string[];
    onsave: (saved: AnyIngredient) => void;
    onduplicate?: (saved: AnyIngredient) => void;
    oncancel: () => void;
  } = $props();

  const isEdit = $derived(ingredient !== null);
  const isSeeded = $derived((ingredient as { source?: string } | null)?.source === 'seeded');
  const typeName = $derived(type.charAt(0).toUpperCase() + type.slice(1));
  const title = $derived(
    isSeeded ? `Built-in ${typeName}` :
    isEdit ? `Edit ${typeName}` :
    `New ${typeName}`
  );

  // Snapshot all initial values once (component is always remounted fresh via {#if editModalOpen})
  const _init = untrack(() => {
    const hop = ingredient as Hop;
    const ferm = ingredient as Fermentable;
    const yeast = ingredient as Yeast;
    const misc = ingredient as Misc;
    const water = ingredient as Water;
    const e = ingredient !== null;
    return {
      hopName: e && type === 'hop' ? hop.name : '',
      hopAlpha: e && type === 'hop' ? hop.alpha_pct : 0,
      hopBeta: e && type === 'hop' ? (hop.beta_pct ?? '') : '',
      hopForm: e && type === 'hop' ? hop.form : 'Pellet',
      hopType: e && type === 'hop' ? (hop.type_ ?? '') : '',
      hopOrigin: e && type === 'hop' ? (hop.origin ?? '') : '',
      hopNotes: e && type === 'hop' ? (hop.notes ?? '') : '',
      hopSubstitutes: e && type === 'hop' ? (hop.substitutes ?? '') : '',
      fermName: e && type === 'fermentable' ? ferm.name : '',
      fermType: e && type === 'fermentable' ? ferm.type_ : 'Grain',
      fermYield: e && type === 'fermentable' ? ferm.yield_pct : 75,
      fermColor: e && type === 'fermentable' ? ferm.color_lovibond : 2,
      fermOrigin: e && type === 'fermentable' ? (ferm.origin ?? '') : '',
      fermNotes: e && type === 'fermentable' ? (ferm.notes ?? '') : '',
      fermAddAfterBoil: e && type === 'fermentable' ? ferm.add_after_boil : false,
      yeastName: e && type === 'yeast' ? yeast.name : '',
      yeastType: e && type === 'yeast' ? yeast.type_ : 'Ale',
      yeastForm: e && type === 'yeast' ? yeast.form : 'Dry',
      yeastLab: e && type === 'yeast' ? (yeast.laboratory ?? '') : '',
      yeastProductId: e && type === 'yeast' ? (yeast.product_id ?? '') : '',
      yeastAttenuation: e && type === 'yeast' ? (yeast.attenuation_pct ?? '') : '',
      yeastFlocculation: e && type === 'yeast' ? (yeast.flocculation ?? '') : '',
      yeastNotes: e && type === 'yeast' ? (yeast.notes ?? '') : '',
      yeastAddToSecondary: e && type === 'yeast' ? yeast.add_to_secondary : false,
      miscName: e && type === 'misc' ? misc.name : '',
      miscType: e && type === 'misc' ? misc.type_ : 'Spice',
      miscUse: e && type === 'misc' ? misc.use_ : 'Boil',
      miscTime: e && type === 'misc' ? misc.time_min : 15,
      miscAmountIsWeight: e && type === 'misc' ? misc.amount_is_weight : true,
      miscNotes: e && type === 'misc' ? (misc.notes ?? '') : '',
      miscUseFor: e && type === 'misc' ? (misc.use_for ?? '') : '',
      waterName: e && type === 'water' ? water.name : '',
      waterCa: e && type === 'water' ? water.calcium_ppm : 0,
      waterBicarb: e && type === 'water' ? water.bicarbonate_ppm : 0,
      waterSulfate: e && type === 'water' ? water.sulfate_ppm : 0,
      waterChloride: e && type === 'water' ? water.chloride_ppm : 0,
      waterSodium: e && type === 'water' ? water.sodium_ppm : 0,
      waterMg: e && type === 'water' ? water.magnesium_ppm : 0,
      waterNotes: e && type === 'water' ? (water.notes ?? '') : '',
    };
  });

  // --- Hop fields ---
  let hopName = $state(_init.hopName);
  let hopAlpha = $state(_init.hopAlpha);
  let hopBeta = $state(_init.hopBeta);
  let hopForm = $state(_init.hopForm);
  let hopType = $state(_init.hopType);
  let hopOrigin = $state(_init.hopOrigin);
  let hopNotes = $state(_init.hopNotes);
  let hopSubstitutes = $state(_init.hopSubstitutes);

  // --- Fermentable fields ---
  let fermName = $state(_init.fermName);
  let fermType = $state(_init.fermType);
  let fermYield = $state(_init.fermYield);
  let fermColor = $state(_init.fermColor);
  let fermOrigin = $state(_init.fermOrigin);
  let fermNotes = $state(_init.fermNotes);
  let fermAddAfterBoil = $state(_init.fermAddAfterBoil);

  // --- Yeast fields ---
  let yeastName = $state(_init.yeastName);
  let yeastType = $state(_init.yeastType);
  let yeastForm = $state(_init.yeastForm);
  let yeastLab = $state(_init.yeastLab);
  let yeastProductId = $state(_init.yeastProductId);
  let yeastAttenuation = $state(_init.yeastAttenuation);
  let yeastFlocculation = $state(_init.yeastFlocculation);
  let yeastNotes = $state(_init.yeastNotes);
  let yeastAddToSecondary = $state(_init.yeastAddToSecondary);

  // --- Misc fields ---
  let miscName = $state(_init.miscName);
  let miscType = $state(_init.miscType);
  let miscUse = $state(_init.miscUse);
  let miscTime = $state(_init.miscTime);
  let miscAmountIsWeight = $state(_init.miscAmountIsWeight);
  let miscNotes = $state(_init.miscNotes);
  let miscUseFor = $state(_init.miscUseFor);

  // --- Water fields ---
  let waterName = $state(_init.waterName);
  let waterCa = $state(_init.waterCa);
  let waterBicarb = $state(_init.waterBicarb);
  let waterSulfate = $state(_init.waterSulfate);
  let waterChloride = $state(_init.waterChloride);
  let waterSodium = $state(_init.waterSodium);
  let waterMg = $state(_init.waterMg);
  let waterNotes = $state(_init.waterNotes);

  let saving = $state(false);

  const currentName = $derived(
    type === 'hop' ? hopName :
    type === 'fermentable' ? fermName :
    type === 'yeast' ? yeastName :
    type === 'misc' ? miscName :
    waterName
  );
  const nameCollision = $derived(
    existingNames
      .filter(n => !isEdit || n.toLowerCase() !== (ingredient as AnyIngredient).name.toLowerCase())
      .some(n => n.toLowerCase() === currentName.trim().toLowerCase())
  );

  async function handleSave() {
    if (saving || nameCollision || !currentName.trim()) return;
    saving = true;
    let saved: AnyIngredient | null | undefined = null;
    if (type === 'hop') {
      const input: CreateHopInput | UpdateHopInput = {
        name: hopName.trim(),
        alpha_pct: hopAlpha,
        form: hopForm,
        beta_pct: hopBeta === '' ? null : Number(hopBeta),
        type_: hopType.trim() || null,
        origin: hopOrigin.trim() || null,
        notes: hopNotes.trim() || null,
        substitutes: hopSubstitutes.trim() || null,
      };
      if (isEdit) {
        saved = await ipc(updateHop((ingredient as Hop).id, input as UpdateHopInput));
      } else {
        saved = await ipc(createHop(input as CreateHopInput));
      }
    } else if (type === 'fermentable') {
      const input: CreateFermentableInput | UpdateFermentableInput = {
        name: fermName.trim(),
        type_: fermType,
        yield_pct: fermYield,
        color_lovibond: fermColor,
        origin: fermOrigin.trim() || null,
        notes: fermNotes.trim() || null,
        add_after_boil: fermAddAfterBoil,
      };
      if (isEdit) {
        saved = await ipc(updateFermentable((ingredient as Fermentable).id, input as UpdateFermentableInput));
      } else {
        saved = await ipc(createFermentable(input as CreateFermentableInput));
      }
    } else if (type === 'yeast') {
      const input: CreateYeastInput | UpdateYeastInput = {
        name: yeastName.trim(),
        type_: yeastType,
        form: yeastForm,
        laboratory: yeastLab.trim() || null,
        product_id: yeastProductId.trim() || null,
        attenuation_pct: yeastAttenuation === '' ? null : Number(yeastAttenuation),
        flocculation: yeastFlocculation.trim() || null,
        notes: yeastNotes.trim() || null,
        add_to_secondary: yeastAddToSecondary,
      };
      if (isEdit) {
        saved = await ipc(updateYeast((ingredient as Yeast).id, input as UpdateYeastInput));
      } else {
        saved = await ipc(createYeast(input as CreateYeastInput));
      }
    } else if (type === 'misc') {
      const input: CreateMiscInput | UpdateMiscInput = {
        name: miscName.trim(),
        type_: miscType,
        use_: miscUse,
        time_min: miscTime,
        amount_is_weight: miscAmountIsWeight,
        notes: miscNotes.trim() || null,
        use_for: miscUseFor.trim() || null,
      };
      if (isEdit) {
        saved = await ipc(updateMisc((ingredient as Misc).id, input as UpdateMiscInput));
      } else {
        saved = await ipc(createMisc(input as CreateMiscInput));
      }
    } else {
      const input: CreateWaterInput | UpdateWaterInput = {
        name: waterName.trim(),
        calcium_ppm: waterCa,
        bicarbonate_ppm: waterBicarb,
        sulfate_ppm: waterSulfate,
        chloride_ppm: waterChloride,
        sodium_ppm: waterSodium,
        magnesium_ppm: waterMg,
        notes: waterNotes.trim() || null,
      };
      if (isEdit) {
        saved = await ipc(updateWater((ingredient as Water).id, input as UpdateWaterInput));
      } else {
        saved = await ipc(createWater(input as CreateWaterInput));
      }
    }
    saving = false;
    if (saved) onsave(saved);
  }

  async function handleDuplicate() {
    // onduplicate is called instead of onsave so the caller can reopen in edit mode
    if (saving || !ingredient) return;
    saving = true;
    let saved: AnyIngredient | null | undefined = null;
    if (type === 'hop') {
      saved = await ipc(createHop({
        name: hopName.trim(), alpha_pct: hopAlpha, form: hopForm,
        beta_pct: hopBeta === '' ? null : Number(hopBeta),
        type_: hopType.trim() || null, origin: hopOrigin.trim() || null,
        notes: hopNotes.trim() || null, substitutes: hopSubstitutes.trim() || null,
        forked_from_id: (ingredient as Hop).id,
      }));
    } else if (type === 'fermentable') {
      saved = await ipc(createFermentable({
        name: fermName.trim(), type_: fermType, yield_pct: fermYield,
        color_lovibond: fermColor, origin: fermOrigin.trim() || null,
        notes: fermNotes.trim() || null, add_after_boil: fermAddAfterBoil,
        forked_from_id: (ingredient as Fermentable).id,
      }));
    } else if (type === 'yeast') {
      saved = await ipc(createYeast({
        name: yeastName.trim(), type_: yeastType, form: yeastForm,
        laboratory: yeastLab.trim() || null, product_id: yeastProductId.trim() || null,
        attenuation_pct: yeastAttenuation === '' ? null : Number(yeastAttenuation),
        flocculation: yeastFlocculation.trim() || null,
        notes: yeastNotes.trim() || null, add_to_secondary: yeastAddToSecondary,
        forked_from_id: (ingredient as Yeast).id,
      }));
    } else if (type === 'misc') {
      saved = await ipc(createMisc({
        name: miscName.trim(), type_: miscType, use_: miscUse, time_min: miscTime,
        amount_is_weight: miscAmountIsWeight, notes: miscNotes.trim() || null,
        use_for: miscUseFor.trim() || null,
        forked_from_id: (ingredient as Misc).id,
      }));
    } else {
      saved = await ipc(createWater({
        name: waterName.trim(), calcium_ppm: waterCa, bicarbonate_ppm: waterBicarb,
        sulfate_ppm: waterSulfate, chloride_ppm: waterChloride, sodium_ppm: waterSodium,
        magnesium_ppm: waterMg, notes: waterNotes.trim() || null,
        forked_from_id: (ingredient as Water).id,
      }));
    }
    saving = false;
    if (saved) onduplicate(saved);
  }
</script>

<div class="fixed inset-0 flex items-center justify-center" style="z-index: 1000;">
  <!-- Backdrop -->
  <div class="absolute inset-0" style="background: rgba(0,0,0,0.4);" role="none"
       onclick={oncancel} onkeydown={() => {}}></div>

  <!-- Modal -->
  <div class="relative flex flex-col rounded-lg overflow-hidden"
       style="background: var(--color-bg-elevated); border: 1px solid var(--color-border);
              z-index: 1001; width: 480px; max-width: 95vw; max-height: 80vh;">

    <!-- Header -->
    <div class="flex items-center justify-between px-5 py-3 flex-shrink-0"
         style="border-bottom: 1px solid var(--color-border);">
      <h2 class="text-base font-semibold" style="color: var(--color-text-primary);">{title}</h2>
      <button onclick={oncancel}
              style="background: none; border: none; cursor: pointer; color: var(--color-text-muted); font-size: 18px; padding: 2px 6px; border-radius: 4px;">×</button>
    </div>

    <!-- Body -->
    <div class="flex-1 overflow-y-auto p-5 flex flex-col gap-4">

      {#if isSeeded}
        <div class="text-sm px-3 py-2 rounded flex items-center gap-2"
             style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-secondary);">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="flex-shrink:0;"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
          Built-in ingredients can't be edited. Duplicate to create your own editable copy.
        </div>
      {/if}

      {#if nameCollision}
        <div class="text-sm px-3 py-2 rounded" style="background: #7f1d1d20; border: 1px solid #dc262650; color: #fca5a5;">
          An ingredient with this name already exists. Please choose a different name.
        </div>
      {/if}

      <div style={isSeeded ? 'pointer-events: none; opacity: 0.65; user-select: none;' : ''}>
      {#if type === 'hop'}
        <div class="grid grid-cols-2 gap-3">
          <div class="col-span-2">
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Name *
              <input bind:value={hopName} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Alpha % *
              <input type="number" step="0.1" min="0" bind:value={hopAlpha} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Beta %
              <input type="number" step="0.1" min="0" bind:value={hopBeta} placeholder="—" class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Form *
              <select bind:value={hopForm} class="px-2 py-1.5 rounded text-sm"
                      style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);">
                <option>Pellet</option><option>Cryo</option><option>CO2 Extract</option><option>Plug</option><option>Leaf</option>
              </select>
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Type
              <input bind:value={hopType} placeholder="e.g. Bittering" class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Origin
              <input bind:value={hopOrigin} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div class="col-span-2">
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Substitutes
              <input bind:value={hopSubstitutes} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div class="col-span-2">
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Notes
              <textarea bind:value={hopNotes} rows="3" class="px-2 py-1.5 rounded text-sm resize-none"
                        style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);"></textarea>
            </label>
          </div>
        </div>

      {:else if type === 'fermentable'}
        <div class="grid grid-cols-2 gap-3">
          <div class="col-span-2">
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Name *
              <input bind:value={fermName} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Type *
              <select bind:value={fermType} class="px-2 py-1.5 rounded text-sm"
                      style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);">
                <option>Grain</option><option>Sugar</option><option>Extract</option><option>Dry Extract</option><option>Adjunct</option>
              </select>
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Origin
              <input bind:value={fermOrigin} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Yield % *
              <input type="number" step="0.1" min="0" max="100" bind:value={fermYield} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Color (°L) *
              <input type="number" step="0.1" min="0" bind:value={fermColor} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div class="col-span-2 flex items-center gap-2">
            <input type="checkbox" id="add-after-boil" bind:checked={fermAddAfterBoil} class="rounded" />
            <label for="add-after-boil" class="text-sm" style="color: var(--color-text-primary);">Add after boil</label>
          </div>
          <div class="col-span-2">
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Notes
              <textarea bind:value={fermNotes} rows="3" class="px-2 py-1.5 rounded text-sm resize-none"
                        style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);"></textarea>
            </label>
          </div>
        </div>

      {:else if type === 'yeast'}
        <div class="grid grid-cols-2 gap-3">
          <div class="col-span-2">
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Name *
              <input bind:value={yeastName} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Type *
              <select bind:value={yeastType} class="px-2 py-1.5 rounded text-sm"
                      style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);">
                <option>Ale</option><option>Lager</option><option>Wheat</option><option>Wine</option><option>Champagne</option>
              </select>
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Form *
              <select bind:value={yeastForm} class="px-2 py-1.5 rounded text-sm"
                      style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);">
                <option>Liquid</option><option>Dry</option><option>Slant</option><option>Culture</option>
              </select>
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Laboratory
              <input bind:value={yeastLab} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Product ID
              <input bind:value={yeastProductId} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Attenuation %
              <input type="number" step="1" min="0" max="100" bind:value={yeastAttenuation} placeholder="—" class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Flocculation
              <select bind:value={yeastFlocculation} class="px-2 py-1.5 rounded text-sm"
                      style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);">
                <option value="">—</option><option>Low</option><option>Medium</option><option>High</option><option>Very High</option>
              </select>
            </label>
          </div>
          <div class="col-span-2 flex items-center gap-2">
            <input type="checkbox" id="add-to-secondary" bind:checked={yeastAddToSecondary} class="rounded" />
            <label for="add-to-secondary" class="text-sm" style="color: var(--color-text-primary);">Add to secondary</label>
          </div>
          <div class="col-span-2">
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Notes
              <textarea bind:value={yeastNotes} rows="3" class="px-2 py-1.5 rounded text-sm resize-none"
                        style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);"></textarea>
            </label>
          </div>
        </div>

      {:else if type === 'misc'}
        <div class="grid grid-cols-2 gap-3">
          <div class="col-span-2">
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Name *
              <input bind:value={miscName} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Type *
              <select bind:value={miscType} class="px-2 py-1.5 rounded text-sm"
                      style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);">
                <option>Spice</option><option>Fining</option><option>Water Agent</option><option>Herb</option><option>Flavor</option><option>Other</option>
              </select>
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Default Use *
              <select bind:value={miscUse} class="px-2 py-1.5 rounded text-sm"
                      style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);">
                <option>Boil</option><option>Mash</option><option>Primary</option><option>Secondary</option><option>Bottling</option>
              </select>
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Default Time (min) *
              <input type="number" step="1" min="0" bind:value={miscTime} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div class="col-span-2 flex items-center gap-2">
            <input type="checkbox" id="amount-is-weight" bind:checked={miscAmountIsWeight} class="rounded" />
            <label for="amount-is-weight" class="text-sm" style="color: var(--color-text-primary);">Amount is weight (vs. volume)</label>
          </div>
          <div class="col-span-2">
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Use For
              <input bind:value={miscUseFor} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div class="col-span-2">
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Notes
              <textarea bind:value={miscNotes} rows="3" class="px-2 py-1.5 rounded text-sm resize-none"
                        style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);"></textarea>
            </label>
          </div>
        </div>

      {:else}
        <!-- Water -->
        <div class="grid grid-cols-2 gap-3">
          <div class="col-span-2">
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Name *
              <input bind:value={waterName} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Calcium (ppm) *
              <input type="number" step="1" min="0" bind:value={waterCa} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Bicarbonate (ppm) *
              <input type="number" step="1" min="0" bind:value={waterBicarb} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Sulfate (ppm) *
              <input type="number" step="1" min="0" bind:value={waterSulfate} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Chloride (ppm) *
              <input type="number" step="1" min="0" bind:value={waterChloride} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Sodium (ppm) *
              <input type="number" step="1" min="0" bind:value={waterSodium} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div>
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Magnesium (ppm) *
              <input type="number" step="1" min="0" bind:value={waterMg} class="px-2 py-1.5 rounded text-sm"
                     style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);" />
            </label>
          </div>
          <div class="col-span-2">
            <label class="flex flex-col gap-1 text-xs" style="color: var(--color-text-secondary);">Notes
              <textarea bind:value={waterNotes} rows="2" class="px-2 py-1.5 rounded text-sm resize-none"
                        style="background: var(--color-bg-surface); border: 1px solid var(--color-border); color: var(--color-text-primary);"></textarea>
            </label>
          </div>
        </div>
      {/if}
      </div>
    </div>

    <!-- Footer -->
    <div class="flex justify-end gap-2 px-5 py-3 flex-shrink-0"
         style="border-top: 1px solid var(--color-border);">
      <button onclick={oncancel} class="px-4 py-1.5 rounded text-sm"
              style="background: var(--color-bg-surface); color: var(--color-text-primary); border: 1px solid var(--color-border);">
        {isSeeded ? 'Close' : 'Cancel'}
      </button>
      {#if isSeeded}
        <button onclick={handleDuplicate} disabled={saving} class="px-4 py-1.5 rounded text-sm font-medium"
                style="background: {saving ? 'var(--color-bg-elevated)' : 'var(--color-accent)'}; color: {saving ? 'var(--color-text-muted)' : '#fff'}; border: none; cursor: {saving ? 'default' : 'pointer'};">
          {saving ? 'Duplicating…' : 'Duplicate & Edit'}
        </button>
      {:else}
        <button onclick={handleSave} disabled={saving || nameCollision || !currentName.trim()} class="px-4 py-1.5 rounded text-sm font-medium"
                style="background: {saving || nameCollision || !currentName.trim() ? 'var(--color-bg-elevated)' : 'var(--color-accent)'}; color: {saving || nameCollision || !currentName.trim() ? 'var(--color-text-muted)' : '#fff'}; border: none; cursor: {saving || nameCollision || !currentName.trim() ? 'default' : 'pointer'};">
          {saving ? 'Saving…' : isEdit ? 'Save' : 'Create'}
        </button>
      {/if}
    </div>
  </div>
</div>
