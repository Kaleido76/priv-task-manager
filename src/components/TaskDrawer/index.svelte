<script lang="ts">
  import { taskStore, uiStore } from "$stores";
  import { getTaskContent, updateTaskContent, getTaskLogs, addTaskLog, updateTask, deleteTaskLog } from "$api";
  import type { TaskContent, TaskLog, UpdateTaskRequest } from "$types";
  import PropertyEditor from "./PropertyEditor.svelte";
  import { formatDateTime } from "$utils";
  import { X, Pencil, Check, LoaderCircle, Plus, Trash2, AlertTriangle } from "@lucide/svelte";

  const { selectedTask } = taskStore;
  const { drawerOpen, saveStatus, activeTab } = uiStore;

  let content = $state<TaskContent | null>(null);
  let logs = $state<TaskLog[]>([]);
  let noteText = $state("");
  let logInput = $state("");
  let loading = $state(false);
  let noteEditing = $state(false);
  let noteClearState = $state<"idle" | "confirm">("idle");
  let confirmingLogId = $state<number | null>(null);

  let draftTitle = $state("");
  let draftStatus = $state("");
  let draftPriority = $state("");
  let draftRecipient = $state("");
  let draftProgress = $state(0);
  let draftDeadline = $state("");

  let originalNote = $state("");

  let saveState = $state<"idle" | "saving" | "saved">("idle");
  let editingTaskId = $state<number | null>(null);

  $effect(() => {
    const task = $selectedTask;
    if (task && $drawerOpen && task.id !== editingTaskId) {
      editingTaskId = task.id;
      draftTitle = task.title;
      draftStatus = task.status;
      draftPriority = task.priority;
      draftRecipient = task.recipient ?? "";
      draftProgress = task.progress;
      draftDeadline = task.deadline ? task.deadline.split("T")[0] : "";
      noteEditing = false;
      noteClearState = "idle";
      saveState = "idle";
      loading = true;
      Promise.all([
        getTaskContent(task.id).then((c) => {
          content = c;
          noteText = c?.note ?? "";
          originalNote = c?.note ?? "";
        }),
        getTaskLogs(task.id).then((l) => logs = l),
      ]).finally(() => loading = false);
    }
  });

  async function handleSave() {
    const task = $selectedTask;
    if (!task) return;
    saveState = "saving";

    const changes: Record<string, unknown> = {};
    if (draftTitle !== task.title) changes.title = draftTitle;
    if (draftStatus !== task.status) changes.status = draftStatus;
    if (draftPriority !== task.priority) changes.priority = draftPriority;
    if (draftRecipient !== (task.recipient ?? "")) changes.recipient = draftRecipient || null;
    if (draftProgress !== task.progress) changes.progress = draftProgress;
    const deadlineVal = task.deadline ? task.deadline.split("T")[0] : "";
    if (draftDeadline !== deadlineVal) changes.deadline = draftDeadline || null;

    try {
      if (Object.keys(changes).length > 0) {
        await updateTask({ id: task.id, ...changes } as unknown as UpdateTaskRequest);
      }
      if (noteText !== originalNote) {
        await updateTaskContent(task.id, noteText);
        originalNote = noteText;
      }
      saveState = "saved";
      saveStatus.set("saved");
      taskStore.refresh();
      await new Promise((r) => setTimeout(r, 2000));
      saveState = "idle";
    } catch {
      saveState = "idle";
      saveStatus.set("unsaved");
    }
  }

  function discardAndClose() {
    uiStore.closeDrawer();
  }

  async function handleAddLog() {
    const task = $selectedTask;
    if (!task || !logInput.trim()) return;
    const newLog = await addTaskLog(task.id, logInput.trim());
    logs = [...logs, newLog];
    logInput = "";
  }

  async function handleDeleteLog(logId: number) {
    if (confirmingLogId === logId) {
      await deleteTaskLog(logId);
      logs = logs.filter(l => l.id !== logId);
      confirmingLogId = null;
    } else {
      confirmingLogId = logId;
    }
  }

  function resetLogConfirm() {
    confirmingLogId = null;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") discardAndClose();
  }

  function handleNoteSave() {
    const task = $selectedTask;
    if (!task) return;
    updateTaskContent(task.id, noteText).then(() => {
      originalNote = noteText;
      noteEditing = false;
    }).catch(() => {});
  }

  function toggleNoteEdit() {
    if (noteEditing) {
      const task = $selectedTask;
      if (task) {
        updateTaskContent(task.id, noteText).then(() => {
          originalNote = noteText;
          noteEditing = false;
        }).catch(() => {});
      }
    } else {
      noteEditing = true;
      noteClearState = "idle";
    }
  }

  function handleNoteClear() {
    if (noteClearState === "confirm") {
      noteText = "";
      const task = $selectedTask;
      if (task) {
        updateTaskContent(task.id, "").then(() => {
          originalNote = "";
          noteClearState = "idle";
        }).catch(() => {});
      }
    } else {
      noteClearState = "confirm";
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if $drawerOpen && $selectedTask}
  <div class="drawer">
    <div class="drawer-header">
      <input class="drawer-title-input" bind:value={draftTitle} />
      <button class="drawer-close" onclick={discardAndClose}>
        <X size={16} />
      </button>
    </div>

    <div class="drawer-body">
      <PropertyEditor
        bind:draftStatus bind:draftPriority
        bind:draftRecipient bind:draftProgress bind:draftDeadline
      />

      <div class="drawer-meta">
        <span class="drawer-meta-item">Created: {formatDateTime($selectedTask!.create_time)}</span>
        <span class="drawer-meta-item">Updated: {formatDateTime($selectedTask!.update_time)}</span>
      </div>

      <div class="drawer-section">
        <div class="drawer-tabs">
          <button class="tab" class:active={$activeTab === "note"} onclick={() => activeTab.set("note")}>Note</button>
          <button class="tab" class:active={$activeTab === "logs"} onclick={() => activeTab.set("logs")}>Logs</button>
        </div>

        {#if $activeTab === "note"}
          <div class="drawer-content">
            {#if loading}
              <div class="drawer-loading">Loading...</div>
            {:else if noteEditing}
              <div class="note-edit-area">
                <textarea class="note-textarea" bind:value={noteText}></textarea>
                <div class="note-edit-actions">
                  <button class="btn-note-cancel" onclick={() => { noteText = originalNote; noteEditing = false; }}>Cancel</button>
                  <button class="btn-note-save" onclick={handleNoteSave}>Save Note</button>
                </div>
              </div>
            {:else}
              <div class="note-display">
                <div class="note-actions">
                  <button class="btn-note-action" class:confirm={noteClearState === "confirm"} onclick={handleNoteClear} title="Clear note">
                    {#if noteClearState === "confirm"}
                      <AlertTriangle size={14} />
                    {:else}
                      <Trash2 size={14} />
                    {/if}
                  </button>
                  <button class="btn-note-action" onclick={toggleNoteEdit} title="Edit note">
                    <Pencil size={14} />
                  </button>
                </div>
                {#if noteText}
                  <div class="note-text">{noteText}</div>
                {:else}
                  <div class="note-empty">No note added</div>
                {/if}
              </div>
            {/if}
          </div>
        {:else}
          <div class="drawer-content">
            <div class="log-list">
              {#each logs as log}
                <div class="log-item" onclick={() => resetLogConfirm()} onkeydown={() => {}} role="button" tabindex="0">
                  <div class="log-top">
                    <span class="log-time">{formatDateTime(log.create_time)}</span>
                    <button
                      class="btn-log-del"
                      class:confirm={confirmingLogId === log.id}
                      onclick={(e) => { e.stopPropagation(); handleDeleteLog(log.id); }}
                      title={confirmingLogId === log.id ? "Click again to delete" : "Delete log"}
                    >
                      {#if confirmingLogId === log.id}
                        <AlertTriangle size={12} />
                      {:else}
                        <Trash2 size={12} />
                      {/if}
                    </button>
                  </div>
                  <span class="log-text">{log.content}</span>
                </div>
              {/each}
            </div>
            <div class="log-input-row">
              <input class="log-input" type="text" placeholder="Add a log..." bind:value={logInput}
                onkeydown={(e) => e.key === "Enter" && handleAddLog()} />
              <button class="btn-log-add" onclick={handleAddLog}><Plus size={16} /></button>
            </div>
          </div>
        {/if}
      </div>
    </div>

    <div class="drawer-footer">
      <button class="btn-save" onclick={handleSave} disabled={saveState !== "idle"}>
        {#if saveState === "saving"}
          <span class="spin"><LoaderCircle size={16} /></span> Saving...
        {:else if saveState === "saved"}
          <Check size={16} /> Saved
        {:else}
          <Check size={16} /> Save
        {/if}
      </button>
    </div>
  </div>
{/if}

<style>
  .drawer {
    width: var(--drawer-width);
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--color-bg-primary);
    overflow: hidden;
  }
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  .spin {
    animation: spin 1s linear infinite;
  }

  .drawer-header {
    display: flex;
    align-items: center;
    padding: var(--spacing-md);
    border-bottom: 1px solid var(--color-border);
    gap: var(--spacing-sm);
  }
  .drawer-title-input {
    flex: 1;
    border: 1px solid transparent;
    background: transparent;
    color: var(--color-text-primary);
    font-size: var(--font-size-lg);
    font-weight: 600;
    outline: none;
    padding: 2px 4px;
    border-radius: var(--radius-sm);
    transition: border-color 0.15s ease, background 0.15s ease;
  }
  .drawer-title-input:focus {
    border-color: var(--color-accent);
    background: var(--color-bg-primary);
  }
  .drawer-close {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: var(--color-text-secondary);
    cursor: pointer;
    border-radius: var(--radius-sm);
    transition: background 0.15s ease, color 0.15s ease;
  }
  .drawer-close:hover {
    background: var(--color-bg-tertiary);
    color: var(--color-text-primary);
  }

  .drawer-body {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: var(--spacing-md);
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg);
  }
  .drawer-section {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
    min-width: 0;
  }
  .drawer-tabs {
    display: flex;
    gap: 0;
    border-bottom: 1px solid var(--color-border);
  }
  .tab {
    padding: 6px 16px;
    border: none;
    background: transparent;
    color: var(--color-text-secondary);
    cursor: pointer;
    font-size: var(--font-size-sm);
    border-bottom: 2px solid transparent;
    margin-bottom: -1px;
    transition: color 0.15s ease, border-color 0.15s ease;
  }
  .tab.active {
    color: var(--color-accent);
    border-bottom-color: var(--color-accent);
  }
  .tab:hover:not(.active) {
    color: var(--color-text-primary);
  }
  .drawer-content {
    flex: 1;
    min-width: 0;
  }
  .drawer-loading {
    padding: var(--spacing-lg);
    text-align: center;
    color: var(--color-text-secondary);
  }

  /* Note display / edit */
  .note-display {
    position: relative;
    padding: var(--spacing-sm) var(--spacing-md);
    background: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    min-height: 80px;
  }
  .note-text {
    font-size: var(--font-size-base);
    color: var(--color-text-primary);
    line-height: 1.5;
    white-space: pre-wrap;
    word-break: break-word;
  }
  .note-empty {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    font-style: italic;
  }
  .note-actions {
    position: absolute;
    top: 6px;
    right: 6px;
    display: flex;
    gap: 4px;
    opacity: 0;
    transition: opacity 0.15s ease;
  }
  .note-display:hover .note-actions {
    opacity: 1;
  }
  .btn-note-action {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-secondary);
    cursor: pointer;
    border-radius: var(--radius-sm);
    transition: background 0.15s ease, color 0.15s ease, border-color 0.15s ease;
  }
  .btn-note-action:hover {
    background: var(--color-bg-tertiary);
    color: var(--color-text-primary);
  }
  .btn-note-action.confirm {
    border-color: #cf222e;
    color: #cf222e;
    background: #fff0ee;
  }
  .btn-note-action.confirm:hover {
    background: #ffdedb;
    border-color: #a0111f;
  }
  .note-edit-area {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
  }
  .note-textarea {
    width: 100%;
    min-height: 150px;
    padding: var(--spacing-sm);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    font-family: inherit;
    font-size: var(--font-size-base);
    line-height: 1.5;
    resize: vertical;
    outline: none;
    transition: border-color 0.15s ease;
  }
  .note-textarea:focus {
    border-color: var(--color-accent);
  }
  .note-edit-actions {
    display: flex;
    gap: var(--spacing-xs);
    justify-content: flex-end;
  }
  .btn-note-cancel {
    padding: 4px 12px;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    cursor: pointer;
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    transition: background 0.15s ease;
  }
  .btn-note-cancel:hover {
    background: var(--color-bg-tertiary);
  }
  .btn-note-save {
    padding: 4px 12px;
    border: 1px solid var(--color-accent);
    background: var(--color-accent);
    color: #ffffff;
    cursor: pointer;
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    transition: background 0.15s ease;
  }
  .btn-note-save:hover {
    background: var(--color-accent-hover);
  }

  /* Logs */
  .log-list {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
    max-height: 300px;
    overflow-y: auto;
    margin-bottom: var(--spacing-sm);
  }
  .log-item {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 6px 8px;
    background: var(--color-bg-secondary);
    border-radius: var(--radius-sm);
    position: relative;
  }
  .log-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--spacing-xs);
  }
  .log-time {
    font-size: 11px;
    color: var(--color-text-secondary);
  }
  .btn-log-del {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: var(--color-text-secondary);
    cursor: pointer;
    border-radius: 3px;
    opacity: 0;
    transition: opacity 0.12s ease, background 0.12s ease, color 0.12s ease;
    flex-shrink: 0;
  }
  .log-item:hover .btn-log-del {
    opacity: 0.5;
  }
  .log-item:hover .btn-log-del:hover {
    opacity: 1;
    color: #cf222e;
  }
  .btn-log-del.confirm {
    opacity: 1 !important;
    color: #cf222e;
    background: rgba(207, 34, 46, 0.12);
  }
  .log-text {
    font-size: var(--font-size-sm);
    color: var(--color-text-primary);
  }
  .log-input-row {
    display: flex;
    gap: var(--spacing-xs);
  }
  .log-input {
    flex: 1;
    height: 28px;
    padding: 0 8px;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    outline: none;
    transition: border-color 0.15s ease;
  }
  .log-input:focus {
    border-color: var(--color-accent);
  }
  .btn-log-add {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid var(--color-border);
    background: var(--color-accent);
    color: #ffffff;
    cursor: pointer;
    border-radius: var(--radius-sm);
    font-size: 16px;
    transition: background 0.15s ease;
  }
  .btn-log-add:hover {
    background: var(--color-accent-hover);
  }

  .drawer-meta {
    display: flex;
    gap: var(--spacing-lg);
    font-size: 11px;
    color: var(--color-text-secondary);
    padding: 2px 0;
  }
  .drawer-meta-item {
    white-space: nowrap;
  }

  /* Footer Save button */
  .drawer-footer {
    padding: var(--spacing-sm) var(--spacing-md);
    border-top: 1px solid var(--color-border);
    display: flex;
    justify-content: flex-end;
  }
  .btn-save {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 20px;
    border: 1px solid var(--color-accent);
    background: var(--color-accent);
    color: #ffffff;
    cursor: pointer;
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    font-weight: 500;
    transition: background 0.15s ease, opacity 0.15s ease;
  }
  .btn-save:hover:not(:disabled) {
    background: var(--color-accent-hover);
  }
  .btn-save:disabled {
    opacity: 0.7;
    cursor: default;
  }
</style>
