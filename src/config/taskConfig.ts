export interface StatusConfig {
  label: string;
  bg: string;
  fg: string;
}

export interface PriorityConfig {
  label: string;
  bg: string;
  fg: string;
}

export interface CapsuleConfig {
  text: string;
  bg: string;
  fg: string;
}

export const statusCfg: Record<string, StatusConfig> = {
  todo: { label: "Todo", bg: "#0969da", fg: "#ffffff" },
  in_progress: { label: "In Progress", bg: "#b35900", fg: "#ffffff" },
  done: { label: "Done", bg: "#1a7f37", fg: "#ffffff" },
  cancelled: { label: "Cancelled", bg: "#cf222e", fg: "#ffffff" },
};

export const priorityCfg: Record<string, PriorityConfig> = {
  suggestion: { label: "Suggestion", bg: "#656d76", fg: "#ffffff" },
  low: { label: "Low", bg: "#1a7f37", fg: "#ffffff" },
  medium: { label: "Medium", bg: "#b35900", fg: "#ffffff" },
  high: { label: "High", bg: "#cf222e", fg: "#ffffff" },
  urgent: { label: "Urgent", bg: "#8b0000", fg: "#ffffff" },
};

export const cardTypeCfg: Record<string, { label: string; color: string }> = {
  file: { label: "File", color: "#0969da" },
  note: { label: "Note", color: "#1a7f37" },
  link: { label: "Link", color: "#8250df" },
  todolist: { label: "TO-DO", color: "#da4b09" },
};

export function getDeadlineCapsule(deadline: string | null, status: string, todayStr?: string): CapsuleConfig | null {
  if (!deadline || status === "done" || status === "cancelled") return null;
  const seed = todayStr ? new Date(todayStr + "T00:00:00") : new Date();
  const today = new Date(seed.getFullYear(), seed.getMonth(), seed.getDate());
  const d = new Date(deadline);
  const due = new Date(d.getFullYear(), d.getMonth(), d.getDate());
  const diffMs = due.getTime() - today.getTime();
  const diffDays = Math.round(diffMs / 86400000);
  if (diffDays < 0) return { text: `${-diffDays} Day`, bg: "#cf222e", fg: "#ffffff" };
  if (diffDays === 0) return { text: "Today", bg: "#b35900", fg: "#ffffff" };
  return { text: `${diffDays} Day`, bg: "#1a7f37", fg: "#ffffff" };
}
