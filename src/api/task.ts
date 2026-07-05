import { invoke } from "@tauri-apps/api/core";
import type { Task, TaskContent, TaskLog, UpdateTaskRequest } from "$types";

function log(method: string, args?: Record<string, unknown>) {
  console.log(`[API] ${method}`, args ?? "");
}

export async function getTasks(projectId: number): Promise<Task[]> {
  log("getTasks", { projectId });
  const r = await invoke<Task[]>("get_tasks", { projectId });
  console.log(`[API] getTasks => ${r.length} tasks`);
  return r;
}

export async function createTask(projectId: number, title: string): Promise<Task> {
  log("createTask", { projectId, title });
  const r = await invoke<Task>("create_task", { projectId, title });
  console.log(`[API] createTask => id=${r.id}`);
  return r;
}

export async function updateTask(req: UpdateTaskRequest): Promise<Task> {
  log("updateTask", { id: req.id });
  const r = await invoke<Task>("update_task", { req });
  return r;
}

export async function deleteTask(id: number): Promise<void> {
  log("deleteTask", { id });
  await invoke("delete_task", { id });
}

export async function searchTasks(
  projectId: number,
  keyword?: string,
  status?: string,
  priority?: string
): Promise<Task[]> {
  log("searchTasks", { projectId, keyword, status, priority });
  const r = await invoke<Task[]>("search_tasks", { projectId, keyword, status, priority });
  console.log(`[API] searchTasks => ${r.length} tasks`);
  return r;
}

export async function getTaskContent(taskId: number): Promise<TaskContent | null> {
  log("getTaskContent", { taskId });
  return invoke<TaskContent | null>("get_task_content", { taskId });
}

export async function updateTaskContent(taskId: number, note: string): Promise<void> {
  log("updateTaskContent", { taskId, noteLen: note.length });
  await invoke("update_task_content", { taskId, note });
}

export async function getTaskLogs(taskId: number): Promise<TaskLog[]> {
  log("getTaskLogs", { taskId });
  return invoke<TaskLog[]>("get_task_logs", { taskId });
}

export async function addTaskLog(taskId: number, content: string): Promise<TaskLog> {
  log("addTaskLog", { taskId });
  return invoke<TaskLog>("add_task_log", { taskId, content });
}

export async function deleteTaskLog(logId: number): Promise<void> {
  log("deleteTaskLog", { logId });
  await invoke("delete_task_log", { logId });
}

export async function deleteTasks(ids: number[]): Promise<void> {
  log("deleteTasks", { ids });
  await invoke("delete_tasks", { ids });
}

export async function moveTasks(ids: number[], projectId: number): Promise<void> {
  log("moveTasks", { ids, projectId });
  await invoke("move_tasks", { ids, projectId });
}
