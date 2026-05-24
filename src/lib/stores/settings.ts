import { writable } from "svelte/store";
import { getSettings, updateSetting } from "$lib/api";

export interface AppSettings {
  units?: "metric" | "imperial";
  theme?: string;
  default_equipment_profile_id?: string;
  last_route?: string;
}

export const settings = writable<AppSettings>({});

export async function loadSettings() {
  const s = await getSettings();
  settings.set(s as AppSettings);
}

export async function saveSetting(key: keyof AppSettings, value: string) {
  await updateSetting(key, value);
  settings.update((s) => ({ ...s, [key]: value }));
}
