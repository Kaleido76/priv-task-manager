import { writable } from "svelte/store";

function createUiStore() {
  const drawerOpen = writable(false);
  const statusMessage = writable("Ready");
  const saveStatus = writable<"saved" | "saving" | "unsaved">("saved");
  const activeTab = writable<"note" | "logs">("note");

  function toggleDrawer() {
    console.log("[Store] uiStore.toggleDrawer");
    drawerOpen.update((v) => !v);
  }

  function openDrawer() {
    console.log("[Store] uiStore.openDrawer");
    drawerOpen.set(true);
  }

  function closeDrawer() {
    console.log("[Store] uiStore.closeDrawer");
    drawerOpen.set(false);
  }

  return { drawerOpen, statusMessage, saveStatus, activeTab, toggleDrawer, openDrawer, closeDrawer };
}

export const uiStore = createUiStore();
