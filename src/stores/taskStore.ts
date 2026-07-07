import { writable, derived, get } from "svelte/store";
import type { Task, UpdateTaskRequest } from "$types";
import * as api from "$api";

function createTaskStore() {
  const tasks = writable<Task[]>([]);
  const selectedId = writable<number | null>(null);
  const currentProjectId = writable<number | null>(null);
  const searchKeyword = writable("");
  const filterStatus = writable<string>("__all__");
  const filterPriority = writable<string>("__all__");
  const selectedTaskIds = writable<Set<number>>(new Set());

  const selectedTask = derived([tasks, selectedId], ([$tasks, $selectedId]) =>
    $tasks.find((t) => t.id === $selectedId) ?? null
  );

  const hasSelection = derived(selectedTaskIds, ($ids) => $ids.size > 0);

  async function load(projectId: number) {
    console.log("[Store] taskStore.load", projectId);
    try {
      currentProjectId.set(projectId);
      selectedId.set(null);
      selectedTaskIds.set(new Set());
      searchKeyword.set("");
      filterStatus.set("__all__");
      filterPriority.set("__all__");
      const result = await api.getTasks(projectId);
      tasks.set(result);
      console.log(`[Store] taskStore.load => ${result.length} tasks`);
    } catch (e) {
      console.error("[Store] taskStore.load error:", e);
    }
  }

  async function refresh() {
    const pid = get(currentProjectId);
    if (!pid) return;
    console.log("[Store] taskStore.refresh", pid);
    try {
      const kw = get(searchKeyword) || undefined;
      const fs = get(filterStatus) !== "__all__" ? get(filterStatus) : undefined;
      const fp = get(filterPriority) !== "__all__" ? get(filterPriority) : undefined;
      if (kw || fs || fp) {
        const result = await api.searchTasks(pid, kw, fs, fp);
        tasks.set(result);
      } else {
        const result = await api.getTasks(pid);
        tasks.set(result);
      }
      console.log(`[Store] taskStore.refresh => ${get(tasks).length} tasks`);
    } catch (e) {
      console.error("[Store] taskStore.refresh error:", e);
    }
  }

  async function create(title: string) {
    console.log("[Store] taskStore.create", title);
    try {
      const pid = get(currentProjectId);
      if (!pid) {
        console.warn("[Store] taskStore.create: no project selected");
        return;
      }
      await api.createTask(pid, title);
      await refresh();
    } catch (e) {
      console.error("[Store] taskStore.create error:", e);
    }
  }

  async function update(req: UpdateTaskRequest) {
    console.log("[Store] taskStore.update", req.id);
    try {
      await api.updateTask(req);
      await refresh();
    } catch (e) {
      console.error("[Store] taskStore.update error:", e);
    }
  }

  async function remove(id: number) {
    console.log("[Store] taskStore.remove", id);
    try {
      await api.deleteTask(id);
      await refresh();
    } catch (e) {
      console.error("[Store] taskStore.remove error:", e);
    }
  }

  async function search(projectId: number) {
    console.log("[Store] taskStore.search", projectId);
    try {
      const result = await api.searchTasks(
        projectId,
        get(searchKeyword) || undefined,
        get(filterStatus) !== "__all__" ? get(filterStatus) : undefined,
        get(filterPriority) !== "__all__" ? get(filterPriority) : undefined
      );
      tasks.set(result);
      console.log(`[Store] taskStore.search => ${result.length} tasks`);
    } catch (e) {
      console.error("[Store] taskStore.search error:", e);
    }
  }

  function toggleSelect(id: number) {
    selectedTaskIds.update((s) => {
      const next = new Set(s);
      if (next.has(id)) next.delete(id); else next.add(id);
      return next;
    });
  }

  function selectAll() {
    const ids = get(tasks).map((t) => t.id);
    selectedTaskIds.set(new Set(ids));
  }

  function deselectAll() {
    selectedTaskIds.set(new Set());
  }

  async function batchDelete() {
    const ids = [...get(selectedTaskIds)];
    if (!ids.length) return;
    console.log("[Store] taskStore.batchDelete", ids);
    try {
      await api.deleteTasks(ids);
      selectedTaskIds.set(new Set());
      await refresh();
    } catch (e) {
      console.error("[Store] taskStore.batchDelete error:", e);
    }
  }

  async function batchMove(projectId: number) {
    const ids = [...get(selectedTaskIds)];
    if (!ids.length) return;
    console.log("[Store] taskStore.batchMove", ids, projectId);
    try {
      await api.moveTasks(ids, projectId);
      selectedTaskIds.set(new Set());
      await refresh();
    } catch (e) {
      console.error("[Store] taskStore.batchMove error:", e);
    }
  }

  return {
    tasks, selectedId, selectedTask, currentProjectId,
    searchKeyword, filterStatus, filterPriority, selectedTaskIds, hasSelection,
    load, create, update, remove, search, refresh,
    toggleSelect, selectAll, deselectAll, batchDelete, batchMove,
  };
}

export const taskStore = createTaskStore();
