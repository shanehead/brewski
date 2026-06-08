import { writable } from "svelte/store";
import { getSettings, updateSetting, type GravityUnit } from "$lib/api";

export interface AppSettings {
  units?: "metric" | "imperial";
  gravity_unit?: GravityUnit;
  theme?: string;
  default_equipment_profile_id?: string;
  last_route?: string;
  last_route_recipes?: string;
  last_route_batches?: string;
  last_route_tools?: string;
  last_route_equipment?: string;
  last_route_library?: string;
  last_route_settings?: string;
  starters_collapsed?: boolean;
  hide_example_recipes?: boolean;
  show_tooltips?: boolean;
}

export const settings = writable<AppSettings>({});

export async function loadSettings() {
  const s = await getSettings();
  const parsed = Object.fromEntries(
    Object.entries(s).map(([k, v]) => [k, v === "true" ? true : v === "false" ? false : v])
  );
  settings.set(parsed as AppSettings);
}

export async function saveSetting(key: keyof AppSettings, value: string) {
  await updateSetting(key, value);
  const parsed: string | boolean = value === "true" ? true : value === "false" ? false : value;
  settings.update((s) => ({ ...s, [key]: parsed }));
}
