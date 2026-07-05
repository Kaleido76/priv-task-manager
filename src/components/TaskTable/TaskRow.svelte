<script lang="ts">
  import type { Task } from "$types";
  import { formatDate } from "$utils";
  import { taskStore } from "$stores";
  import { CheckSquare, Square } from "@lucide/svelte";

  let { task, isSelected, onSelect, gridTemplateCols }: {
    task: Task;
    isSelected: boolean;
    onSelect: () => void;
    gridTemplateCols: string;
  } = $props();

  const { selectedTaskIds } = taskStore;

  let checked = $derived($selectedTaskIds.has(task.id));

  const statusCfg: Record<string, { label: string; bg: string; fg: string }> = {
    todo: { label: "Todo", bg: "#ddf4ff", fg: "#0969da" },
    in_progress: { label: "In Progress", bg: "#fff1e5", fg: "#b35900" },
    done: { label: "Done", bg: "#dafbe1", fg: "#1a7f37" },
    cancelled: { label: "Cancelled", bg: "#fff0ee", fg: "#cf222e" },
  };

  const priorityCfg: Record<string, { label: string; bg: string; fg: string }> = {
    suggestion: { label: "Suggestion", bg: "#f3f4f6", fg: "#656d76" },
    low: { label: "Low", bg: "#ddf4ff", fg: "#0969da" },
    medium: { label: "Medium", bg: "#fff1e5", fg: "#b35900" },
    high: { label: "High", bg: "#fff0ee", fg: "#cf222e" },
    urgent: { label: "Urgent", bg: "#8b0000", fg: "#ffffff" },
  };

  let deadlineInfo = $derived.by(() => {
    if (!task.deadline) return { text: "—", cls: "", bg: "" };
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    const d = new Date(task.deadline);
    const due = new Date(d.getFullYear(), d.getMonth(), d.getDate());
    const diffMs = due.getTime() - today.getTime();
    const diffDays = Math.round(diffMs / 86400000);
    if (diffDays < 0) return { text: `${-diffDays} Day`, cls: "dl-overdue", bg: "#fff0ee" };
    if (diffDays === 0) return { text: "Today", cls: "dl-today", bg: "#fff1e5" };
    return { text: `${diffDays} Day`, cls: "dl-remaining", bg: "#dafbe1" };
  });

  function handleCheck(e: Event) {
    e.stopPropagation();
    taskStore.toggleSelect(task.id);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      onSelect();
    }
  }
</script>

<div
  class="task-row"
  class:selected={isSelected}
  class:task-done={task.status === "done" || task.status === "cancelled"}
  style="grid-template-columns: {gridTemplateCols}"
  onclick={onSelect}
  onkeydown={handleKeydown}
  role="button"
  tabindex="0"
>
  <div class="cell cell-check" onclick={handleCheck} onkeydown={(e) => { if (e.key === "Enter" || e.key === " ") { e.preventDefault(); handleCheck(e); } }} role="button" tabindex="-1">
    {#if checked}
      <span class="check-icon checked"><CheckSquare size={14} /></span>
    {:else}
      <span class="check-icon"><Square size={14} /></span>
    {/if}
  </div>
  <div class="cell cell-title" class:cell-title-done={task.status === "done" || task.status === "cancelled"}>{task.title}</div>
  <div class="cell cell-status" style="text-align: center;">
    <span class="tag" style="background:{statusCfg[task.status]?.bg ?? '#eaeef2'};color:{statusCfg[task.status]?.fg ?? '#656d76'}">{statusCfg[task.status]?.label ?? task.status}</span>
  </div>
  <div class="cell cell-priority" style="text-align: center;">
    <span class="tag" style="background:{priorityCfg[task.priority]?.bg ?? '#eaeef2'};color:{priorityCfg[task.priority]?.fg ?? '#656d76'}">{priorityCfg[task.priority]?.label ?? task.priority}</span>
  </div>
  <div class="cell cell-recipient">{task.recipient ?? "—"}</div>
  <div class="cell cell-progress" style="text-align: center;">{task.progress}%</div>
  <div class="cell cell-deadline" style="text-align: right;">{task.deadline ? formatDate(task.deadline) : "—"}</div>
  <div class="cell cell-dl-help" style="text-align: center;">
    {#if task.status !== "done" && task.status !== "cancelled"}
      {#if deadlineInfo.text === "—"}
        —
      {:else}
        <span class="tag" style="background:{deadlineInfo.bg};color:{deadlineInfo.cls === 'dl-overdue' ? '#cf222e' : deadlineInfo.cls === 'dl-today' ? '#b35900' : '#1a7f37'}">{deadlineInfo.text}</span>
      {/if}
    {/if}
  </div>
</div>

<style>
  .task-row {
    display: grid;
    align-items: center;
    border-bottom: 1px solid var(--color-border);
    cursor: pointer;
    transition: background 0.15s ease;
    min-height: 34px;
  }
  .task-row:hover {
    background: var(--table-row-hover-bg, rgba(0,0,0,0.04));
  }
  .task-row.selected {
    background: var(--table-row-selected-bg, rgba(0,120,212,0.1));
  }
  .task-row.task-done {
    opacity: 0.65;
  }
  .cell {
    padding: 4px var(--spacing-sm);
    font-size: var(--font-size-sm);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .cell-title { font-weight: 500; }
  .cell-title-done {
    text-decoration: line-through;
    color: var(--color-text-secondary);
    opacity: 0.65;
  }
  .cell-check {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    cursor: pointer;
    color: var(--color-text-secondary);
  }
  .cell-check:hover {
    color: var(--color-accent);
  }
  .check-icon {
    display: block;
    transition: color 0.12s ease;
  }
  .check-icon.checked {
    color: var(--color-accent);
  }

  .tag {
    display: inline-block;
    padding: 2px 8px;
    border-radius: 12px;
    font-size: 11px;
    font-weight: 500;
    line-height: 1.5;
    white-space: nowrap;
  }
</style>
