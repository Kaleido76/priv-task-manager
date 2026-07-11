import { writable } from "svelte/store";

export const saveRequested = writable(0);

function createUiStore() {
  const drawerOpen = writable(false);
  const statusMessage = writable("Ready");
  const saveStatus = writable<"saved" | "saving" | "unsaved">("saved");

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

  function requestSave() {
    saveRequested.update((n) => n + 1);
  }

  return { drawerOpen, statusMessage, saveStatus, toggleDrawer, openDrawer, closeDrawer, requestSave };
}

export const uiStore = createUiStore();
