import { writable, derived, get } from "svelte/store";
import type { Project } from "$types";
import * as api from "$api";

function createProjectStore() {
  const projects = writable<Project[]>([]);
  const selectedId = writable<number | null>(null);

  const selectedProject = derived([projects, selectedId], ([$projects, $selectedId]) =>
    $projects.find((p) => p.id === $selectedId) ?? null
  );

  async function load() {
    console.log("[Store] projectStore.load");
    try {
      const list = await api.getProjects();
      projects.set(list);
      const currentId = get(selectedId);
      if (currentId !== null && !list.some((p) => p.id === currentId)) {
        selectedId.set(null);
      }
      console.log(`[Store] projectStore.load => ${list.length} projects`);
    } catch (e) {
      console.error("[Store] projectStore.load error:", e);
    }
  }

  async function create(name: string) {
    console.log("[Store] projectStore.create", name);
    try {
      const p = await api.createProject(name);
      await load();
      selectedId.set(p.id);
      console.log("[Store] projectStore.create => selected", p.id);
    } catch (e) {
      console.error("[Store] projectStore.create error:", e);
    }
  }

  async function remove(id: number) {
    console.log("[Store] projectStore.remove", id);
    try {
      await api.deleteProject(id);
      await load();
    } catch (e) {
      console.error("[Store] projectStore.remove error:", e);
    }
  }

  async function rename(id: number, name: string) {
    console.log("[Store] projectStore.rename", id, name);
    try {
      await api.renameProject(id, name);
      await load();
    } catch (e) {
      console.error("[Store] projectStore.rename error:", e);
    }
  }

  async function updateDescription(id: number, description: string) {
    console.log("[Store] projectStore.updateDescription", id);
    try {
      await api.updateProjectDescription(id, description);
      await load();
    } catch (e) {
      console.error("[Store] projectStore.updateDescription error:", e);
    }
  }

  async function updateColor(id: number, color: string | null) {
    console.log("[Store] projectStore.updateColor", id, color);
    try {
      await api.updateProjectColor(id, color);
      await load();
    } catch (e) {
      console.error("[Store] projectStore.updateColor error:", e);
    }
  }

  return { projects, selectedId, selectedProject, load, create, remove, rename, updateDescription, updateColor };
}

export const projectStore = createProjectStore();
