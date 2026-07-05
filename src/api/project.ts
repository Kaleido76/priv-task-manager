import { invoke } from "@tauri-apps/api/core";
import type { Project } from "$types";

function log(method: string, args?: Record<string, unknown>) {
  console.log(`[API] ${method}`, args ?? "");
}

export async function getProjects(): Promise<Project[]> {
  log("getProjects");
  const r = await invoke<Project[]>("get_projects");
  console.log(`[API] getProjects => ${r.length} projects`);
  return r;
}

export async function createProject(name: string): Promise<Project> {
  log("createProject", { name });
  const r = await invoke<Project>("create_project", { name });
  console.log(`[API] createProject => id=${r.id}`);
  return r;
}

export async function renameProject(id: number, name: string): Promise<Project> {
  log("renameProject", { id, name });
  return invoke<Project>("rename_project", { id, name });
}

export async function updateProjectDescription(id: number, description: string): Promise<Project> {
  log("updateProjectDescription", { id });
  return invoke<Project>("update_project_description", { id, description });
}

export async function updateProjectColor(id: number, color: string | null): Promise<Project> {
  log("updateProjectColor", { id, color });
  return invoke<Project>("update_project_color", { id, color });
}

export async function deleteProject(id: number): Promise<void> {
  log("deleteProject", { id });
  await invoke("delete_project", { id });
}

export async function reorderProjects(ids: number[]): Promise<void> {
  log("reorderProjects", { ids });
  await invoke("reorder_projects", { ids });
}
