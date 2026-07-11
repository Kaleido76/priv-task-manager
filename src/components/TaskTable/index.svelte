<script lang="ts">
  import { taskStore } from "$stores";
  import { uiStore } from "$stores";
  import { projectStore } from "$stores";
  import TaskRow from "./TaskRow.svelte";
  import { clickOutside } from "$actions/clickOutside";
  import { statusCfg, priorityCfg } from "$config";
  import {
    ArrowUpDown, ArrowUp, ArrowDown, X,
    Trash2, CheckSquare, Square, Move, Filter,
  } from "@lucide/svelte";
  import { fade } from "svelte/transition";
  import { get } from "svelte/store";

  const { tasks, selectedId, selectedTaskIds, hasSelection, searchKeyword } = taskStore;
  const { projects } = projectStore;

  type ColumnType = "text" | "status" | "priority" | "deadline" | "none";

  interface ColDef {
    key: string;
    label: string;
    width: string;
    align: string;
    type: ColumnType;
  }

  const columns: ColDef[] = [
    { key: "title", label: "Title", width: "1fr", align: "left", type: "text" },
    { key: "status", label: "Status", width: "90px", align: "center", type: "status" },
    { key: "priority", label: "Priority", width: "90px", align: "center", type: "priority" },
    { key: "deadline", label: "Deadline", width: "110px", align: "center", type: "deadline" },
    { key: "__deadline_help", label: "", width: "75px", align: "center", type: "none" },
    { key: "recipient", label: "Recipient", width: "120px", align: "left", type: "text" },
  ];

  const gridTemplateCols = $derived("36px " + columns.map(c => c.width).join(" "));

  let sortColumn = $state<string | null>(null);
  let sortDirection = $state<"asc" | "desc">("asc");

  type ColumnFilter = {
    text?: string;
    items?: string[];
    dateFrom?: string;
    dateTo?: string;
  };

  let columnFilters = $state<Record<string, ColumnFilter>>({});

  let filterOpenCol = $state<string | null>(null);

  let filterTextInput = $state("");
  let filterItemsInput = $state<string[]>([]);
  let filterDateFrom = $state("");
  let filterDateTo = $state("");

  let textDebounce: ReturnType<typeof setTimeout>;

  $effect(() => {
    const colKey = filterOpenCol;
    if (colKey) {
      const f = columnFilters[colKey];
      filterTextInput = f?.text ?? "";
      filterItemsInput = f?.items ?? [];
      filterDateFrom = f?.dateFrom ?? "";
      filterDateTo = f?.dateTo ?? "";
    }
  });

  function hasActiveFilter(colKey: string): boolean {
    const f = columnFilters[colKey];
    if (!f) return false;
    if (f.text) return true;
    if (f.items && f.items.length > 0) return true;
    if (f.dateFrom || f.dateTo) return true;
    return false;
  }

  function isColActive(colKey: string): boolean {
    if (!columns.find(c => c.key === colKey)?.label) return false;
    return sortColumn === colKey || hasActiveFilter(colKey) || filterOpenCol === colKey;
  }

  function toggleSort(colKey: string) {
    if (colKey === "__deadline_help") return;
    if (sortColumn === colKey) {
      sortDirection = sortDirection === "asc" ? "desc" : "asc";
    } else {
      sortColumn = colKey;
      sortDirection = "asc";
    }
  }

  function updateColFilter(colKey: string, patch: Partial<ColumnFilter>) {
    const cur = columnFilters[colKey] ?? {};
    const next: ColumnFilter = {};
    if (patch.text !== undefined) next.text = patch.text || undefined;
    else if (cur.text) next.text = cur.text;
    if (patch.items !== undefined) next.items = patch.items.length > 0 ? patch.items : undefined;
    else if (cur.items) next.items = cur.items;
    if (patch.dateFrom !== undefined) next.dateFrom = patch.dateFrom || undefined;
    else if (cur.dateFrom) next.dateFrom = cur.dateFrom;
    if (patch.dateTo !== undefined) next.dateTo = patch.dateTo || undefined;
    else if (cur.dateTo) next.dateTo = cur.dateTo;

    const hasVal = next.text || (next.items && next.items.length > 0) || next.dateFrom || next.dateTo;
    if (hasVal) {
      columnFilters = { ...columnFilters, [colKey]: next };
    } else {
      const { [colKey]: _, ...rest } = columnFilters;
      columnFilters = rest;
    }
  }

  let popupTop = $state(0);
  let popupLeft = $state(0);

  function toggleFilter(colKey: string) {
    if (filterOpenCol === colKey) {
      filterOpenCol = null;
      return;
    }
    filterOpenCol = colKey;
    requestAnimationFrame(() => {
      const el = document.querySelector<HTMLElement>(`[data-fb="${colKey}"]`);
      if (el) {
        const r = el.getBoundingClientRect();
        popupTop = r.bottom + 3;
        popupLeft = r.left + r.width / 2;
      }
    });
  }

  function clearColumn(colKey: string) {
    if (sortColumn === colKey) {
      sortColumn = null;
      sortDirection = "asc";
    }
    const { [colKey]: _, ...rest } = columnFilters;
    columnFilters = rest;
  }

  function handleFilterTextInput() {
    clearTimeout(textDebounce);
    textDebounce = setTimeout(() => {
      const colKey = filterOpenCol;
      if (!colKey) return;
      updateColFilter(colKey, { text: filterTextInput });
    }, 200);
  }

  function toggleFilterItem(item: string) {
    const idx = filterItemsInput.indexOf(item);
    if (idx >= 0) filterItemsInput = filterItemsInput.filter(i => i !== item);
    else filterItemsInput = [...filterItemsInput, item];
    const colKey = filterOpenCol;
    if (colKey) updateColFilter(colKey, { items: filterItemsInput });
  }

  function handleDateFromInput(e: Event) {
    filterDateFrom = (e.target as HTMLInputElement).value;
    const colKey = filterOpenCol;
    if (colKey) updateColFilter(colKey, { dateFrom: filterDateFrom });
  }

  function handleDateToInput(e: Event) {
    filterDateTo = (e.target as HTMLInputElement).value;
    const colKey = filterOpenCol;
    if (colKey) updateColFilter(colKey, { dateTo: filterDateTo });
  }

  function handleFilterPopupClose() {
    filterOpenCol = null;
  }

  let displayTasks = $derived.by(() => {
    let list = $tasks;

    const kw = $searchKeyword.toLowerCase().trim();
    if (kw) {
      list = list.filter(t =>
        t.title.toLowerCase().includes(kw) ||
        (t.recipient ?? "").toLowerCase().includes(kw)
      );
    }

    for (const [key, f] of Object.entries(columnFilters)) {
      if (key === "title" && f.text) {
        const q = f.text.toLowerCase();
        list = list.filter(t => t.title.toLowerCase().includes(q));
      }
      if (key === "recipient" && f.text) {
        const q = f.text.toLowerCase();
        list = list.filter(t => (t.recipient ?? "").toLowerCase().includes(q));
      }
      if (key === "status" && f.items && f.items.length > 0) {
        list = list.filter(t => f.items!.includes(t.status));
      }
      if (key === "priority" && f.items && f.items.length > 0) {
        list = list.filter(t => f.items!.includes(t.priority));
      }
      if (key === "deadline") {
        if (f.dateFrom) list = list.filter(t => t.deadline != null && t.deadline >= f.dateFrom!);
        if (f.dateTo) list = list.filter(t => t.deadline != null && t.deadline <= f.dateTo!);
      }
    }

    if (sortColumn) {
      list = [...list].sort((a: any, b: any) => {
        const va = a[sortColumn!];
        const vb = b[sortColumn!];
        if (va == null) return 1;
        if (vb == null) return -1;
        const cmp = va < vb ? -1 : va > vb ? 1 : 0;
        return sortDirection === "asc" ? cmp : -cmp;
      });
    }

    return list;
  });

  let movePopupOpen = $state(false);

  function toggleMovePopup() {
    movePopupOpen = !movePopupOpen;
  }

  function handleMove(pid: number) {
    if (pid === get(projectStore.selectedId)) return;
    taskStore.batchMove(pid);
    movePopupOpen = false;
  }
</script>

<div class="task-table">
  <div class="table-header" style="grid-template-columns: {gridTemplateCols}">
    <div class="header-cell header-checkbox"></div>
    {#each columns as col (col.key)}
      <div class="header-col" class:col-active={isColActive(col.key)} style="text-align:{col.align}">
        {#if col.label}
          <div class="col-buttons" class:cb-visible={isColActive(col.key)} style="justify-content:{col.align === 'left' ? 'flex-start' : col.align === 'right' ? 'flex-end' : 'center'}">
            <button
              class="col-btn-col"
              class:active={sortColumn === col.key}
              onclick={() => toggleSort(col.key)}
              title={sortColumn === col.key ? (sortDirection === "asc" ? "Ascending" : "Descending") : "Sort"}
            >
              {#if sortColumn === col.key && sortDirection === "asc"}
                <ArrowUp size={11} />
              {:else if sortColumn === col.key && sortDirection === "desc"}
                <ArrowDown size={11} />
              {:else}
                <ArrowUpDown size={11} />
              {/if}
            </button>
            {#if col.type !== "none"}
              <div class="col-filter-wrap">
                <button
                  class="col-btn-col"
                  class:active={hasActiveFilter(col.key) || filterOpenCol === col.key}
                  onclick={() => toggleFilter(col.key)}
                  title="Filter"
                  data-fb={col.key}
                >
                  <Filter size={11} />
                </button>
                {#if filterOpenCol === col.key}
                  <div class="col-filter-popup" style="top:{popupTop}px;left:{popupLeft}px;transform:translateX(-50%)" use:clickOutside={handleFilterPopupClose} transition:fade={{ duration: 80 }}>
                    {#if col.type === "text"}
                      <div class="cf-text">
                        <input
                          class="cf-text-input"
                          type="text"
                          placeholder="Filter..."
                          bind:value={filterTextInput}
                          oninput={handleFilterTextInput}
                        />
                      </div>
                    {:else if col.type === "status"}
                      <div class="cf-items">
                        {#each Object.entries(statusCfg) as [key, cfg]}
                          <div class="cf-item" role="button" tabindex="0" onclick={() => toggleFilterItem(key)} onkeydown={(e) => { if (e.key === "Enter" || e.key === " ") { e.preventDefault(); toggleFilterItem(key); } }}>
                            {#if filterItemsInput.includes(key)}
                              <span class="cf-item-check checked"><CheckSquare size={14} /></span>
                            {:else}
                              <span class="cf-item-check"><Square size={14} /></span>
                            {/if}
                            <span class="cf-item-dot" style="background:{cfg.bg}"></span>
                            <span class="cf-item-label">{cfg.label}</span>
                          </div>
                        {/each}
                      </div>
                    {:else if col.type === "priority"}
                      <div class="cf-items">
                        {#each Object.entries(priorityCfg) as [key, cfg]}
                          <div class="cf-item" role="button" tabindex="0" onclick={() => toggleFilterItem(key)} onkeydown={(e) => { if (e.key === "Enter" || e.key === " ") { e.preventDefault(); toggleFilterItem(key); } }}>
                            {#if filterItemsInput.includes(key)}
                              <span class="cf-item-check checked"><CheckSquare size={14} /></span>
                            {:else}
                              <span class="cf-item-check"><Square size={14} /></span>
                            {/if}
                            <span class="cf-item-dot" style="background:{cfg.bg}"></span>
                            <span class="cf-item-label">{cfg.label}</span>
                          </div>
                        {/each}
                      </div>
                    {:else if col.type === "deadline"}
                      <div class="cf-date">
                        <span class="cf-date-label">From</span>
                        <input class="cf-date-input" type="date" value={filterDateFrom} oninput={handleDateFromInput} aria-label="Filter date from" />
                        <span class="cf-date-label">To</span>
                        <input class="cf-date-input" type="date" value={filterDateTo} oninput={handleDateToInput} aria-label="Filter date to" />
                      </div>
                    {/if}
                  </div>
                {/if}
              </div>
            {/if}
            <button
              class="col-btn-col col-btn-col-clear"
              class:visible={isColActive(col.key)}
              onclick={() => clearColumn(col.key)}
              title="Clear sort and filter"
            >
              <X size={11} />
            </button>
          </div>
        {/if}
        <div
          class="header-cell-label"
          class:label-active={isColActive(col.key)}
          class:label-sorted={sortColumn === col.key}
        >
          {col.label}
        </div>
      </div>
    {/each}
  </div>
  <div class="table-body">
    {#each displayTasks as task (task.id)}
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
    flex-shrink: 0;
    background: var(--table-header-bg, var(--color-bg-secondary));
  }
  .table-header > * {
    border-bottom: 1px solid var(--color-border);
  }
  .header-cell {
    padding: 9px var(--spacing-sm);
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--color-text-secondary);
    user-select: none;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    display: flex;
    align-items: center;
  }
  .header-checkbox {
    cursor: default;
  }

  .header-col {
    position: relative;
    transition: border-color 0.15s ease;
    min-height: 40px;
  }
  .header-col.col-active {
    border-bottom: 2px solid var(--color-accent);
  }

  .col-buttons {
    position: absolute;
    bottom: calc(100% - 11px);
    left: 4px;
    right: 4px;
    height: 22px;
    display: flex;
    align-items: center;
    gap: 2px;
    opacity: 0;
    visibility: hidden;
    pointer-events: none;
    z-index: 20;
    transition: opacity 80ms ease, visibility 80ms ease;
  }
  .header-col:hover .col-buttons,
  .col-buttons.cb-visible {
    opacity: 1;
    visibility: visible;
    pointer-events: auto;
  }

  .col-btn-col {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 22px;
    height: 22px;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-secondary);
    cursor: pointer;
    border-radius: 50%;
    padding: 0;
    transition: color 0.12s ease, background 0.12s ease, border-color 0.12s ease;
    flex-shrink: 0;
    line-height: 0;
    box-shadow: 0 1px 2px rgba(0,0,0,0.06);
  }
  .col-btn-col:hover {
    background: var(--color-bg-tertiary);
    color: var(--color-text-primary);
  }
  .col-btn-col.active {
    background: var(--color-accent);
    border-color: var(--color-accent);
    color: var(--color-bg-primary);
  }
  .col-btn-col-clear {
    visibility: hidden;
  }
  .col-btn-col-clear.visible {
    visibility: visible;
  }
  .col-btn-col-clear:hover {
    background: rgba(207, 34, 46, 0.12);
    border-color: rgba(207, 34, 46, 0.3);
    color: #cf222e;
  }

  .header-cell-label {
    padding: 9px var(--spacing-sm) 8px;
    font-size: var(--font-size-base);
    font-weight: 600;
    color: var(--color-text-secondary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    transition: color 0.15s ease;
    text-align: inherit;
  }
  .header-cell-label.label-active {
    color: var(--color-text-primary);
  }
  .header-cell-label.label-sorted {
    color: var(--color-accent);
  }

  .col-filter-wrap {
    position: relative;
  }

  .col-filter-popup {
    position: fixed;
    z-index: 999;
    background: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-md);
    padding: var(--spacing-xs);
    min-width: 140px;
  }

  .cf-text {
    padding: 2px;
  }
  .cf-text-input {
    width: 120px;
    height: 22px;
    padding: 0 6px;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    outline: none;
    font-family: inherit;
    transition: border-color 0.12s ease;
  }
  .cf-text-input:focus {
    border-color: var(--color-accent);
  }

  .cf-items {
    display: flex;
    flex-direction: column;
    gap: 1px;
    padding: 2px 0;
  }
  .cf-item {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 3px 8px;
    cursor: pointer;
    border-radius: var(--radius-sm);
    transition: background 0.1s ease;
    font-size: var(--font-size-sm);
    color: var(--color-text-primary);
    white-space: nowrap;
  }
  .cf-item:hover {
    background: var(--color-bg-tertiary);
  }
  .cf-item-check {
    display: flex;
    align-items: center;
    color: var(--color-text-secondary);
    transition: color 0.12s ease;
    flex-shrink: 0;
  }
  .cf-item-check.checked {
    color: var(--color-accent);
  }
  .cf-item-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }
  .cf-item-label {
    flex: 1;
  }

  .cf-date {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 4px 6px;
  }
  .cf-date-label {
    font-size: 12px;
    color: var(--color-text-secondary);
    font-weight: 500;
  }
  .cf-date-input {
    width: 130px;
    height: 22px;
    padding: 0 4px;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    outline: none;
    font-family: inherit;
    transition: border-color 0.12s ease;
  }
  .cf-date-input:focus {
    border-color: var(--color-accent);
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
    font-size: 12px;
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
