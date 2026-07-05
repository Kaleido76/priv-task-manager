export enum TaskStatus {
  Todo = "todo",
  InProgress = "in_progress",
  Done = "done",
  Cancelled = "cancelled",
}

export enum Priority {
  Suggestion = "suggestion",
  Low = "low",
  Medium = "medium",
  High = "high",
  Urgent = "urgent",
}

export interface Task {
  id: number;
  project_id: number;
  title: string;
  status: TaskStatus;
  priority: Priority;
  recipient: string | null;
  progress: number;
  deadline: string | null;
  create_time: string;
  update_time: string;
}

export interface TaskContent {
  task_id: number;
  note: string;
}

export interface TaskLog {
  id: number;
  task_id: number;
  content: string;
  create_time: string;
}

export interface UpdateTaskRequest {
  id: number;
  title?: string;
  status?: TaskStatus;
  priority?: Priority;
  recipient?: string | null;
  progress?: number;
  deadline?: string | null;
}
