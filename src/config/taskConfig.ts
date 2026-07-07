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
  todo: { label: "Todo", bg: "#ddf4ff", fg: "#0969da" },
  in_progress: { label: "In Progress", bg: "#fff1e5", fg: "#b35900" },
  done: { label: "Done", bg: "#dafbe1", fg: "#1a7f37" },
  cancelled: { label: "Cancelled", bg: "#fff0ee", fg: "#cf222e" },
};

export const priorityCfg: Record<string, PriorityConfig> = {
  suggestion: { label: "Suggestion", bg: "#f3f4f6", fg: "#656d76" },
  low: { label: "Low", bg: "#ddf4ff", fg: "#0969da" },
  medium: { label: "Medium", bg: "#fff1e5", fg: "#b35900" },
  high: { label: "High", bg: "#fff0ee", fg: "#cf222e" },
  urgent: { label: "Urgent", bg: "#8b0000", fg: "#ffffff" },
};

export function getDeadlineCapsule(deadline: string | null, status: string): CapsuleConfig | null {
  if (!deadline || status === "done" || status === "cancelled") return null;
  const now = new Date();
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
  const d = new Date(deadline);
  const due = new Date(d.getFullYear(), d.getMonth(), d.getDate());
  const diffMs = due.getTime() - today.getTime();
  const diffDays = Math.round(diffMs / 86400000);
  if (diffDays < 0) return { text: `${-diffDays} Day`, bg: "#fff0ee", fg: "#cf222e" };
  if (diffDays === 0) return { text: "Today", bg: "#fff1e5", fg: "#b35900" };
  return { text: `${diffDays} Day`, bg: "#dafbe1", fg: "#1a7f37" };
}
