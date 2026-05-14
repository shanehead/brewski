import { writable } from "svelte/store";
import type { BatchSummary } from "$lib/api";
import { listBatches } from "$lib/api";

export const batchList = writable<BatchSummary[]>([]);

export async function refreshBatchList(): Promise<void> {
  const batches = await listBatches();
  batchList.set(batches);
}
