import { writable } from "svelte/store";

export const lastError = writable<string | null>(null);

/** Await an IPC promise, routing any rejection to the error toast. Returns undefined on failure. */
export async function ipc<T>(promise: Promise<T>): Promise<T | undefined> {
  try {
    return await promise;
  } catch (e) {
    lastError.set(String(e));
    return undefined;
  }
}
