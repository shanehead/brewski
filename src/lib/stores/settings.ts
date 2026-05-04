import { writable } from "svelte/store";
import { getSettings, updateSetting } from "$lib/api";

export const settings = writable<Record<string, string>>({});

export async function loadSettings() {
  const s = await getSettings();
  settings.set(s);
}

export async function saveSetting(key: string, value: string) {
  await updateSetting(key, value);
  settings.update((s) => ({ ...s, [key]: value }));
}
