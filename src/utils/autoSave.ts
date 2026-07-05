import { writable } from "svelte/store";

export type SaveStatus = "saved" | "saving" | "unsaved";

export function createAutoSaver<T>(
  saveFn: (data: T) => Promise<void>,
  delay: number = 500
) {
  const dirty = writable(false);
  const saveStatus = writable<SaveStatus>("saved");
  let currentData: T;
  let timer: ReturnType<typeof setTimeout> | null = null;

  async function flush() {
    if (timer) {
      clearTimeout(timer);
      timer = null;
    }
    if (!dirty) return;
    saveStatus.set("saving");
    try {
      await saveFn(currentData);
      dirty.set(false);
      saveStatus.set("saved");
    } catch {
      saveStatus.set("unsaved");
    }
  }

  function markDirty(data: T) {
    currentData = data;
    dirty.set(true);
    saveStatus.set("unsaved");
    if (timer) clearTimeout(timer);
    timer = setTimeout(flush, delay);
  }

  return { dirty, saveStatus, markDirty, flush };
}
