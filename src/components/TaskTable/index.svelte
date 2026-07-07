<script lang="ts">
  import { taskStore } from "$stores";
  import { uiStore } from "$stores";
  import { projectStore } from "$stores";
  import TaskRow from "./TaskRow.svelte";
  import { clickOutside } from "$actions/clickOutside";
  import { Trash2, CheckSquare, Square, Move } from "@lucide/svelte";
  import { fade } from "svelte/transition";

  const { tasks, selectedId, selectedTaskIds, hasSelection } = taskStore;
  const { projects } = projectStore;

  let sortColumn = $state<string | null>(null);
  let sortDirection = $state<"asc" | "desc">("asc");

  function toggleSort(column: string) {
    if (column === "__deadline_help") return;
    if (sortColumn === column) {
      sortDirection = sortDirection === "asc" ? "desc" : "asc";
    } else {
      sortColumn = column;
      sortDirection = "asc";
    }
  }

  let sortedTasks = $derived.by(() => {
    const tasksList = $tasks;
    if (!sortColumn) return tasksList;
    return [...tasksList].sort((a: any, b: any) => {
      const va = a[sortColumn!];
      const vb = b[sortColumn!];
      if (va == null) return 1;
      if (vb == null) return -1;
      const cmp = va < vb ? -1 : va > vb ? 1 : 0;
      return sortDirection === "asc" ? cmp : -cmp;
    });
  });

  const columns: { key: string; label: string; width: string; align: string }[] = [
    { key: "title", label: "Title", width: "1fr", align: "left" },
    { key: "status", label: "Status", width: "90px", align: "center" },
    { key: "priority", label: "Priority", width: "90px", align: "center" },
    { key: "deadline", label: "Deadline", width: "110px", align: "center" },
    { key: "__deadline_help", label: "", width: "75px", align: "center" },
    { key: "recipient", label: "Recipient", width: "120px", align: "left" },
  ];

  const gridTemplateCols = $derived("36px " + columns.map(c => c.width).join(" "));

  const sortIndicator = $derived((col: string) => {
    if (sortColumn !== col) return "";
    return sortDirection === "asc" ? " ▲" : " ▼";
  });

  let movePopupOpen = $state(false);

  function toggleMovePopup() {
    movePopupOpen = !movePopupOpen;
  }

  function handleMove(pid: number) {
    taskStore.batchMove(pid);
    movePopupOpen = false;
  }
</script>

<div class="task-table">
  <div class="table-header" style="grid-template-columns: {gridTemplateCols}">
    <div class="header-cell header-checkbox"></div>
    {#each columns as col}
      <div
        class="header-cell"
        style="text-align: {col.align}"
        onclick={() => toggleSort(col.key)}
        onkeydown={(e) => { if (e.key === "Enter") toggleSort(col.key); }}
        role="button"
        tabindex={col.label ? 0 : -1}
      >
        {#if col.label}{col.label}{sortIndicator(col.key)}{/if}
      </div>
    {/each}
  </div>
  <div class="table-body">
    {#each sortedTasks as task (task.id)}
      <TaskRow
        {task}
        {gridTemplateCols}
        isSelected={task.id === $selectedId}
        onSelect={() => {
          if ($selectedTaskIds.size > 0) {
            taskStore.toggleSelect(task.id);
          } else {
            selectedId.set(task.id);
            uiStore.openDrawer();
          }
        }}
      />
    {:else}
      <div class="table-empty">
        No tasks yet. Click '+ New Task' to create one.
      </div>
    {/each}
  </div>

  {#if $hasSelection}
    <div class="batch-bar" transition:fade={{ duration: 150 }}>
      <span class="batch-count">{$selectedTaskIds.size} selected</span>
      <div class="batch-actions" use:clickOutside={() => movePopupOpen = false}>
        <button class="batch-btn batch-btn-label" onclick={() => taskStore.selectAll()} title="Select all">
          <CheckSquare size={14} /> Select All
        </button>
        <button class="batch-btn batch-btn-label" onclick={() => taskStore.deselectAll()} title="Deselect all">
          <Square size={14} /> Clear
        </button>
        <div class="batch-divider"></div>
        <button class="batch-btn batch-btn-label batch-btn-danger" onclick={() => taskStore.batchDelete()} title="Delete selected">
          <Trash2 size={14} /> Delete
        </button>
        <div class="batch-divider"></div>
        <button class="batch-btn batch-btn-label" onclick={toggleMovePopup} title="Move to project">
          <Move size={14} /> Move
        </button>
        {#if movePopupOpen}
          <div class="batch-move-popup">
            <div class="batch-move-header">Move to...</div>
            {#each $projects as p}
              <button class="batch-move-item" onclick={() => handleMove(p.id)}>{p.name}</button>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .task-table {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
    position: relative;
  }
  .table-header {
    display: grid;
    align-items: center;
    background: var(--table-header-bg, var(--color-bg-secondary));
    border-bottom: 1px solid var(--color-border);
    min-height: 28px;
    flex-shrink: 0;
  }
  .header-cell {
    padding: 4px var(--spacing-sm);
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--color-text-secondary);
    cursor: pointer;
    user-select: none;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    transition: color 0.15s ease;
  }
  .header-cell:hover {
    color: var(--color-text-primary);
  }
  .header-checkbox {
    cursor: default;
  }
  .table-body {
    flex: 1;
    overflow-y: auto;
  }
  .table-empty {
    padding: var(--spacing-lg);
    text-align: center;
    color: var(--color-text-secondary);
    font-size: var(--font-size-sm);
  }

  .batch-bar {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: 6px var(--spacing-md);
    background: var(--color-bg-primary);
    border-top: 1px solid var(--color-border);
    border-bottom: 1px solid var(--color-border);
    flex-shrink: 0;
  }
  .batch-count {
    font-size: var(--font-size-sm);
    font-weight: 500;
    color: var(--color-text-primary);
    margin-right: auto;
  }
  .batch-actions {
    display: flex;
    align-items: center;
    gap: 4px;
    position: relative;
  }
  .batch-btn {
    display: flex;
    align-items: center;
    gap: 4px;
    height: 28px;
    padding: 0 8px;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-secondary);
    cursor: pointer;
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    transition: background 0.15s ease, color 0.15s ease, border-color 0.15s ease;
    white-space: nowrap;
  }
  .batch-btn:hover {
    background: var(--color-bg-tertiary);
    color: var(--color-text-primary);
    border-color: var(--color-text-secondary);
  }
  .batch-btn-danger:hover {
    color: #cf222e;
    border-color: #cf222e;
    background: #fff0ee;
  }
  .batch-divider {
    width: 1px;
    height: 20px;
    background: var(--color-border);
    margin: 0 4px;
  }
  .batch-move-popup {
    position: absolute;
    bottom: calc(100% + 6px);
    right: 0;
    background: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-md);
    padding: var(--spacing-xs) 0;
    z-index: 20;
    min-width: 160px;
    animation: mp-enter 0.12s ease-out;
  }
  @keyframes mp-enter {
    from { opacity: 0; transform: translateY(4px); }
    to { opacity: 1; transform: translateY(0); }
  }
  .batch-move-header {
    padding: 4px var(--spacing-md);
    font-size: 11px;
    font-weight: 600;
    color: var(--color-text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  .batch-move-item {
    display: block;
    width: 100%;
    padding: 5px var(--spacing-md);
    border: none;
    background: transparent;
    color: var(--color-text-primary);
    cursor: pointer;
    font-size: var(--font-size-sm);
    text-align: left;
    transition: background 0.1s ease;
  }
  .batch-move-item:hover {
    background: var(--color-bg-tertiary);
  }
</style>
