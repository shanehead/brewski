import { writable } from "svelte/store";

export const lastError = writable<string | null>(null);

export const lastSuccess = writable<string | null>(null);

let clearTimer: ReturnType<typeof setTimeout> | undefined;

export function setSuccess(message: string) {
  clearTimeout(clearTimer);
  lastSuccess.set(message);
  clearTimer = setTimeout(() => lastSuccess.set(null), 3000);
}

/** Await an IPC promise, routing any rejection to the error toast. Returns undefined on failure. */
export async function ipc<T>(promise: Promise<T>): Promise<T | undefined> {
  try {
    return await promise;
  } catch (e) {
    lastError.set(String(e));
    return undefined;
  }
}
