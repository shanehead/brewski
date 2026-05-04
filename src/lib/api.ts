import { invoke } from "@tauri-apps/api/core";

export interface RecipeSummary {
  id: string;
  name: string;
  style_name: string | null;
  type_: string;
  batch_size_l: number;
  created_at: number;
  updated_at: number;
}

export interface EquipmentProfile {
  id: string;
  name: string;
  notes: string | null;
  boil_size_l: number;
  batch_size_l: number;
  boil_time_min: number;
  evap_rate_pct_hr: number;
  trub_chiller_loss_l: number;
  fermenter_loss_l: number;
  efficiency_pct: number;
  created_at: number;
  updated_at: number;
}

export interface Style {
  id: string;
  name: string;
  category: string;
  og_min: number;
  og_max: number;
  fg_min: number;
  fg_max: number;
  ibu_min: number;
  ibu_max: number;
  color_min_srm: number;
  color_max_srm: number;
  abv_min_pct: number | null;
  abv_max_pct: number | null;
}

export interface RecipeAdditionFermentable {
  id: string;
  recipe_id: string;
  fermentable_id: string | null;
  name: string;
  type_: string;
  yield_pct: number;
  color_lovibond: number;
  amount_kg: number;
  add_after_boil: boolean;
  addition_order: number;
}

export interface RecipeAdditionHop {
  id: string;
  recipe_id: string;
  hop_id: string | null;
  name: string;
  alpha_pct: number;
  form: string;
  amount_kg: number;
  use_: string;
  time_min: number;
  addition_order: number;
}

export interface RecipeAdditionYeast {
  id: string;
  recipe_id: string;
  yeast_id: string | null;
  name: string;
  type_: string;
  form: string;
  laboratory: string | null;
  product_id: string | null;
  attenuation_pct: number | null;
  amount: number | null;
}

export interface RecipeAdditionMisc {
  id: string;
  recipe_id: string;
  name: string;
  type_: string;
  use_: string;
  amount: number;
  time_min: number;
  addition_order: number;
}

export interface RecipeAdditionWater {
  id: string;
  recipe_id: string;
  name: string;
  amount_l: number;
}

export interface MashStep {
  id: string;
  mash_id: string;
  name: string;
  type_: string;
  infuse_amount_l: number | null;
  step_temp_c: number;
  step_time_min: number;
  ramp_time_min: number | null;
  step_order: number;
}

export interface Mash {
  id: string;
  recipe_id: string;
  name: string;
  grain_temp_c: number;
  sparge_temp_c: number | null;
  ph: number | null;
  notes: string | null;
  steps: MashStep[];
}

export interface Recipe {
  id: string;
  name: string;
  type_: string;
  brewer: string | null;
  batch_size_l: number;
  boil_size_l: number;
  boil_time_min: number;
  efficiency_pct: number | null;
  style_id: string | null;
  equipment_profile_id: string | null;
  notes: string | null;
  taste_notes: string | null;
  taste_rating: number | null;
  fermentation_stages: number;
  primary_age_days: number | null;
  primary_temp_c: number | null;
  secondary_age_days: number | null;
  secondary_temp_c: number | null;
  carbonation_vols: number | null;
  forced_carbonation: boolean;
  date: string | null;
  equipment_profile: EquipmentProfile | null;
  style: Style | null;
  fermentables: RecipeAdditionFermentable[];
  hops: RecipeAdditionHop[];
  yeasts: RecipeAdditionYeast[];
  miscs: RecipeAdditionMisc[];
  waters: RecipeAdditionWater[];
  mash: Mash | null;
}

export interface RecipeStats {
  og: number;
  fg: number;
  abv_pct: number;
  ibu: number;
  srm: number;
  calories_per_355ml: number;
  bu_gu_ratio: number;
  pre_boil_gravity: number;
  pre_boil_volume_l: number;
  post_boil_volume_l: number;
}

export interface Fermentable {
  id: string;
  name: string;
  type_: string;
  yield_pct: number;
  color_lovibond: number;
}

export interface Hop {
  id: string;
  name: string;
  alpha_pct: number;
  form: string;
}

export interface Yeast {
  id: string;
  name: string;
  type_: string;
  form: string;
  laboratory: string | null;
  product_id: string | null;
  attenuation_pct: number | null;
}

// --- Recipes ---
export const listRecipes = () => invoke<RecipeSummary[]>("list_recipes");
export const getRecipe = (id: string) => invoke<Recipe>("get_recipe", { id });
export const createRecipe = (input: {
  name: string;
  type_?: string;
  batch_size_l?: number;
  boil_size_l?: number;
  boil_time_min?: number;
  equipment_profile_id?: string;
  source_id?: string;
}) => invoke<Recipe>("create_recipe", { input });
export const updateRecipe = (id: string, input: Partial<Recipe>) =>
  invoke<Recipe>("update_recipe", { id, input });
export const deleteRecipe = (id: string) => invoke<void>("delete_recipe", { id });
export const getRecipeStats = (recipeId: string) =>
  invoke<RecipeStats>("get_recipe_stats", { recipeId });

// --- Recipe additions ---
export const createRecipeFermentable = (recipeId: string, input: object) =>
  invoke<RecipeAdditionFermentable>("create_recipe_fermentable", { recipeId, input });
export const updateRecipeFermentable = (id: string, input: object) =>
  invoke<RecipeAdditionFermentable>("update_recipe_fermentable", { id, input });
export const deleteRecipeFermentable = (id: string) =>
  invoke<void>("delete_recipe_fermentable", { id });

export const createRecipeHop = (recipeId: string, input: object) =>
  invoke<RecipeAdditionHop>("create_recipe_hop", { recipeId, input });
export const updateRecipeHop = (id: string, input: object) =>
  invoke<RecipeAdditionHop>("update_recipe_hop", { id, input });
export const deleteRecipeHop = (id: string) => invoke<void>("delete_recipe_hop", { id });

export const createRecipeYeast = (recipeId: string, input: object) =>
  invoke<RecipeAdditionYeast>("create_recipe_yeast", { recipeId, input });
export const updateRecipeYeast = (id: string, input: object) =>
  invoke<RecipeAdditionYeast>("update_recipe_yeast", { id, input });
export const deleteRecipeYeast = (id: string) => invoke<void>("delete_recipe_yeast", { id });

export const createRecipeMisc = (recipeId: string, input: object) =>
  invoke<RecipeAdditionMisc>("create_recipe_misc", { recipeId, input });
export const updateRecipeMisc = (id: string, input: object) =>
  invoke<RecipeAdditionMisc>("update_recipe_misc", { id, input });
export const deleteRecipeMisc = (id: string) => invoke<void>("delete_recipe_misc", { id });

export const createRecipeWater = (recipeId: string, input: object) =>
  invoke<RecipeAdditionWater>("create_recipe_water", { recipeId, input });
export const updateRecipeWater = (id: string, input: object) =>
  invoke<RecipeAdditionWater>("update_recipe_water", { id, input });
export const deleteRecipeWater = (id: string) => invoke<void>("delete_recipe_water", { id });

// --- Mash ---
export const getMash = (recipeId: string) => invoke<Mash>("get_mash", { recipeId });
export const updateMash = (recipeId: string, input: object) =>
  invoke<Mash>("update_mash", { recipeId, input });
export const createMashStep = (mashId: string, input: object) =>
  invoke<MashStep>("create_mash_step", { mashId, input });
export const updateMashStep = (id: string, input: object) =>
  invoke<MashStep>("update_mash_step", { id, input });
export const deleteMashStep = (id: string) => invoke<void>("delete_mash_step", { id });
export const updateMashStepOrder = (orderedIds: string[]) =>
  invoke<void>("update_mash_step_order", { orderedIds });

// --- Equipment + library ---
export const listEquipmentProfiles = () => invoke<EquipmentProfile[]>("list_equipment_profiles");
export const createEquipmentProfile = (input: object) =>
  invoke<EquipmentProfile>("create_equipment_profile", { input });
export const updateEquipmentProfile = (id: string, input: object) =>
  invoke<EquipmentProfile>("update_equipment_profile", { id, input });
export const deleteEquipmentProfile = (id: string) =>
  invoke<void>("delete_equipment_profile", { id });

export const listStyles = () => invoke<Style[]>("list_styles");
export const listFermentableLibrary = () => invoke<Fermentable[]>("list_fermentable_library");
export const listHopLibrary = () => invoke<Hop[]>("list_hop_library");
export const listYeastLibrary = () => invoke<Yeast[]>("list_yeast_library");

// --- Settings ---
export const getSettings = () => invoke<Record<string, string>>("get_settings");
export const updateSetting = (key: string, value: string) =>
  invoke<void>("update_setting", { key, value });

// --- Import / export ---
export const getRecipeBeerxml = (recipeId: string) =>
  invoke<string>("get_recipe_beerxml", { recipeId });
export const createRecipesFromBeerxml = (xml: string) =>
  invoke<RecipeSummary[]>("create_recipes_from_beerxml", { xml });
