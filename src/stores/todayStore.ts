import { writable } from "svelte/store";

function getTodayStr(): string {
  const d = new Date();
  const m = String(d.getMonth() + 1).padStart(2, "0");
  const day = String(d.getDate()).padStart(2, "0");
  return `${d.getFullYear()}-${m}-${day}`;
}

function createTodayStore() {
  const { subscribe, set } = writable(getTodayStr());

  function scheduleNext() {
    const now = new Date();
    const tomorrow = new Date(now.getFullYear(), now.getMonth(), now.getDate() + 1);
    const msUntilMidnight = tomorrow.getTime() - now.getTime();
    setTimeout(() => {
      set(getTodayStr());
      scheduleNext();
    }, msUntilMidnight);
  }

  scheduleNext();

  return { subscribe };
}

export const todayStore = createTodayStore();
