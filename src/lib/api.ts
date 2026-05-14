import { invoke } from "@tauri-apps/api/core";
import type { components } from "./api.gen";

export type RecipeSummary = components["schemas"]["RecipeSummary"];
export type EquipmentProfile = components["schemas"]["EquipmentProfile"];
export type Style = components["schemas"]["Style"];
export type RecipeAdditionFermentable = components["schemas"]["RecipeAdditionFermentable"];
export type RecipeAdditionHop = components["schemas"]["RecipeAdditionHop"];
export type RecipeAdditionYeast = components["schemas"]["RecipeAdditionYeast"];
export type RecipeAdditionMisc = components["schemas"]["RecipeAdditionMisc"];
export type RecipeAdditionWater = components["schemas"]["RecipeAdditionWater"];
export type MashStep = components["schemas"]["MashStep"];
export type Mash = components["schemas"]["Mash"];
export type Recipe = components["schemas"]["Recipe"];
export type RecipeStats = components["schemas"]["RecipeStats"];
export type Fermentable = components["schemas"]["Fermentable"];
export type Hop = components["schemas"]["Hop"];
export type Yeast = components["schemas"]["Yeast"];
export type CreateFermentableAdditionInput = components["schemas"]["CreateFermentableAdditionInput"];
export type CreateHopAdditionInput = components["schemas"]["CreateHopAdditionInput"];
export type CreateYeastAdditionInput = components["schemas"]["CreateYeastAdditionInput"];
export type CreateMiscAdditionInput = components["schemas"]["CreateMiscAdditionInput"];
export type CreateWaterAdditionInput = components["schemas"]["CreateWaterAdditionInput"];
export type CreateMashStepInput = components["schemas"]["CreateMashStepInput"];
export type UpdateMashStepInput = components["schemas"]["UpdateMashStepInput"];
export type CreateEquipmentProfileInput = components["schemas"]["CreateEquipmentProfileInput"];
export type UpdateEquipmentProfileInput = components["schemas"]["UpdateEquipmentProfileInput"];
export type CreateRecipeInput = components["schemas"]["CreateRecipeInput"];
export type UpdateRecipeInput = components["schemas"]["UpdateRecipeInput"];
export type UpdateFermentableAdditionInput = components["schemas"]["UpdateFermentableAdditionInput"];
export type UpdateHopAdditionInput = components["schemas"]["UpdateHopAdditionInput"];
export type UpdateYeastAdditionInput = components["schemas"]["UpdateYeastAdditionInput"];
export type UpdateMiscAdditionInput = components["schemas"]["UpdateMiscAdditionInput"];
export type UpdateWaterAdditionInput = components["schemas"]["UpdateWaterAdditionInput"];
export type Misc = components["schemas"]["Misc"];
export type Water = components["schemas"]["Water"];
export type WaterProfile = components["schemas"]["WaterProfile"];
export type CalculatedWaterProfile = components["schemas"]["CalculatedWaterProfile"];
export type RecipeWaterAdjustment = components["schemas"]["RecipeWaterAdjustment"];
export type CreateWaterAdjustmentInput = components["schemas"]["CreateWaterAdjustmentInput"];
export type UpdateWaterAdjustmentInput = components["schemas"]["UpdateWaterAdjustmentInput"];
export type UpdateMashInput = components["schemas"]["UpdateMashInput"];
export type BatchSummary = components["schemas"]["BatchSummary"];
export type Batch = components["schemas"]["Batch"];
export type GravityReading = components["schemas"]["GravityReading"];
export type RecipeVersionSummary = components["schemas"]["RecipeVersionSummary"];
export type CreateBatchInput = components["schemas"]["CreateBatchInput"];
export type UpdateBatchInput = components["schemas"]["UpdateBatchInput"];
export type CreateGravityReadingInput = components["schemas"]["CreateGravityReadingInput"];

export type SugarType = "table_sugar" | "corn_sugar" | "dry_malt_extract";
export type GravityUnit = "sg" | "plato" | "brix";
export type ColorUnit = "srm" | "ebc" | "lovibond";

export interface AbvCaloriesResult {
  abvPct: number;
  attenuationPct: number;
  caloriesPer355ml: number;
}

export interface RefractometerResult {
  sg: number;
}

export interface RefractometerFgResult {
  fgSg: number;
}

export interface GravityConversionResult {
  sg: number;
  plato: number;
  brix: number;
}

export interface PitchRateResult {
  requiredCells: number;
  starterVolumeL: number;
}

export interface ColorConversionResult {
  srm: number;
  ebc: number;
  lovibond: number;
}

// --- Recipes ---
export const listRecipes = () => invoke<RecipeSummary[]>("list_recipes");
export const getRecipe = (id: string) => invoke<Recipe>("get_recipe", { id });
export const createRecipe = (input: CreateRecipeInput) => invoke<Recipe>("create_recipe", { input });
export const updateRecipe = (id: string, input: UpdateRecipeInput) =>
  invoke<Recipe>("update_recipe", { id, input });
export const deleteRecipe = (id: string) => invoke<void>("delete_recipe", { id });
export const getRecipeStats = (recipeId: string) =>
  invoke<RecipeStats>("get_recipe_stats", { recipeId });

// --- Recipe additions ---
export const createRecipeFermentable = (recipeId: string, input: CreateFermentableAdditionInput) =>
  invoke<RecipeAdditionFermentable>("create_recipe_fermentable", { recipeId, input });
export const updateRecipeFermentable = (id: string, input: UpdateFermentableAdditionInput) =>
  invoke<RecipeAdditionFermentable>("update_recipe_fermentable", { id, input });
export const deleteRecipeFermentable = (id: string) =>
  invoke<void>("delete_recipe_fermentable", { id });

export const createRecipeHop = (recipeId: string, input: CreateHopAdditionInput) =>
  invoke<RecipeAdditionHop>("create_recipe_hop", { recipeId, input });
export const updateRecipeHop = (id: string, input: UpdateHopAdditionInput) =>
  invoke<RecipeAdditionHop>("update_recipe_hop", { id, input });
export const deleteRecipeHop = (id: string) => invoke<void>("delete_recipe_hop", { id });

export const createRecipeYeast = (recipeId: string, input: CreateYeastAdditionInput) =>
  invoke<RecipeAdditionYeast>("create_recipe_yeast", { recipeId, input });
export const updateRecipeYeast = (id: string, input: UpdateYeastAdditionInput) =>
  invoke<RecipeAdditionYeast>("update_recipe_yeast", { id, input });
export const deleteRecipeYeast = (id: string) => invoke<void>("delete_recipe_yeast", { id });

export const createRecipeMisc = (recipeId: string, input: CreateMiscAdditionInput) =>
  invoke<RecipeAdditionMisc>("create_recipe_misc", { recipeId, input });
export const updateRecipeMisc = (id: string, input: UpdateMiscAdditionInput) =>
  invoke<RecipeAdditionMisc>("update_recipe_misc", { id, input });
export const deleteRecipeMisc = (id: string) => invoke<void>("delete_recipe_misc", { id });

export const createRecipeWater = (recipeId: string, input: CreateWaterAdditionInput) =>
  invoke<RecipeAdditionWater>("create_recipe_water", { recipeId, input });
export const updateRecipeWater = (id: string, input: UpdateWaterAdditionInput) =>
  invoke<RecipeAdditionWater>("update_recipe_water", { id, input });
export const deleteRecipeWater = (id: string) => invoke<void>("delete_recipe_water", { id });

// --- Mash ---
export const getMash = (recipeId: string) => invoke<Mash>("get_mash", { recipeId });
export const updateMash = (recipeId: string, input: UpdateMashInput) =>
  invoke<Mash>("update_mash", { recipeId, input });
export const createMashStep = (mashId: string, input: CreateMashStepInput) =>
  invoke<MashStep>("create_mash_step", { mashId, input });
export const updateMashStep = (id: string, input: UpdateMashStepInput) =>
  invoke<MashStep>("update_mash_step", { id, input });
export const deleteMashStep = (id: string) => invoke<void>("delete_mash_step", { id });
export const updateMashStepOrder = (orderedIds: string[]) =>
  invoke<void>("update_mash_step_order", { orderedIds });

// --- Equipment + library ---
export const listEquipmentProfiles = () => invoke<EquipmentProfile[]>("list_equipment_profiles");
export const createEquipmentProfile = (input: CreateEquipmentProfileInput) =>
  invoke<EquipmentProfile>("create_equipment_profile", { input });
export const updateEquipmentProfile = (id: string, input: UpdateEquipmentProfileInput) =>
  invoke<EquipmentProfile>("update_equipment_profile", { id, input });
export const deleteEquipmentProfile = (id: string) =>
  invoke<void>("delete_equipment_profile", { id });

export const listStyles = () => invoke<Style[]>("list_styles");
export const listFermentableLibrary = () => invoke<Fermentable[]>("list_fermentable_library");
export const listHopLibrary = () => invoke<Hop[]>("list_hop_library");
export const listYeastLibrary = () => invoke<Yeast[]>("list_yeast_library");
export const listMiscLibrary = () => invoke<Misc[]>("list_misc_library");
export const listWaterLibrary = () => invoke<Water[]>("list_water_library");

// --- Settings ---
export const getSettings = () => invoke<Record<string, string>>("get_settings");
export const updateSetting = (key: string, value: string) =>
  invoke<void>("update_setting", { key, value });

// --- Tools ---
export const calculateAbvCalories = (og: number, fg: number) =>
  invoke<AbvCaloriesResult>("calculate_abv_calories", { og, fg });
export const correctHydrometerTemp = (
  measuredSg: number,
  measuredTempC: number,
  calibrationTempC: number,
) =>
  invoke<number>("correct_hydrometer_temp", { measuredSg, measuredTempC, calibrationTempC });
export const calculateRefractometer = (brix: number, wortCorrectionFactor: number) =>
  invoke<RefractometerResult>("calculate_refractometer", { brix, wortCorrectionFactor });
export const correctRefractometerFg = (
  ogBrix: number,
  fgBrix: number,
  wortCorrectionFactor: number,
) =>
  invoke<RefractometerFgResult>("correct_refractometer_fg", {
    ogBrix,
    fgBrix,
    wortCorrectionFactor,
  });
export const calculatePrimingSugar = (
  targetVols: number,
  batchSizeL: number,
  tempC: number,
  sugarType: SugarType,
) =>
  invoke<number>("calculate_priming_sugar", { targetVols, batchSizeL, tempC, sugarType });
export const calculateCo2Pressure = (targetVols: number, tempC: number) =>
  invoke<number>("calculate_co2_pressure", { targetVols, tempC });
export const convertGravity = (value: number, fromUnit: GravityUnit) =>
  invoke<GravityConversionResult>("convert_gravity", { value, fromUnit });
export const calculatePitchRate = (
  og: number,
  batchSizeL: number,
  pitchRate: number,
  yeastPackCells: number,
  viabilityPct: number,
) =>
  invoke<PitchRateResult>("calculate_pitch_rate", {
    og,
    batchSizeL,
    pitchRate,
    yeastPackCells,
    viabilityPct,
  });
export const convertColor = (value: number, fromUnit: ColorUnit) =>
  invoke<ColorConversionResult>("convert_color", { value, fromUnit });

// --- Import / export ---
export const getRecipeBeerxml = (recipeId: string) =>
  invoke<string>("get_recipe_beerxml", { recipeId });
export const createRecipesFromBeerxml = (xml: string) =>
  invoke<RecipeSummary[]>("create_recipes_from_beerxml", { xml });

// --- Water Chemistry ---
export const setRecipeWaterSources = (recipeId: string, mashWaterId: string | null, spargeWaterId: string | null) =>
  invoke<Recipe>("set_recipe_water_sources", { recipeId, mashWaterId, spargeWaterId });
export const calculateWaterProfile = (recipeId: string) =>
  invoke<CalculatedWaterProfile>("calculate_water_profile", { recipeId });
export const createWaterAdjustment = (recipeId: string, input: CreateWaterAdjustmentInput) =>
  invoke<RecipeWaterAdjustment>("create_water_adjustment", { recipeId, input });
export const updateWaterAdjustment = (id: string, input: UpdateWaterAdjustmentInput) =>
  invoke<RecipeWaterAdjustment>("update_water_adjustment", { id, input });
export const deleteWaterAdjustment = (id: string) =>
  invoke<void>("delete_water_adjustment", { id });

// --- Batches ---
export const createBatch = (input: CreateBatchInput) =>
  invoke<Batch>("create_batch", { input });
export const listBatches = () =>
  invoke<BatchSummary[]>("list_batches");
export const listBatchesForRecipe = (recipeId: string) =>
  invoke<BatchSummary[]>("list_batches_for_recipe", { recipeId });
export const getBatch = (id: string) =>
  invoke<Batch>("get_batch", { id });
export const updateBatch = (id: string, input: UpdateBatchInput) =>
  invoke<Batch>("update_batch", { id, input });
export const deleteBatch = (id: string) =>
  invoke<void>("delete_batch", { id });

// --- Gravity Readings ---
export const addGravityReading = (batchId: string, input: CreateGravityReadingInput) =>
  invoke<GravityReading>("add_gravity_reading", { batchId, input });
export const deleteGravityReading = (id: string) =>
  invoke<void>("delete_gravity_reading", { id });

// --- Recipe Versions ---
export const listRecipeVersions = (recipeId: string) =>
  invoke<RecipeVersionSummary[]>("list_recipe_versions", { recipeId });

export type SaveRecipeVersionInput = components["schemas"]["SaveRecipeVersionInput"];

export const getRecipeVersion = (id: string) =>
  invoke<Recipe>("get_recipe_version", { id });

export const saveRecipeVersion = (input: SaveRecipeVersionInput) =>
  invoke<RecipeVersionSummary>("save_recipe_version", { input });

export const branchFromVersion = (recipeId: string, versionId: string) =>
  invoke<void>("branch_from_version", { recipeId, versionId });

export const deleteRecipeVersion = (id: string) =>
  invoke<void>("delete_recipe_version", { id });
