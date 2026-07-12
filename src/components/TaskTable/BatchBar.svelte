<script lang="ts">
  import { clickOutside } from "$actions/clickOutside";
  import type { Project } from "$types";
  import { Trash2, CheckSquare, Square, Move } from "@lucide/svelte";
  import { fade } from "svelte/transition";

  let {
    selectedCount,
    projects,
    currentProjectId,
    onSelectAll,
    onDeselectAll,
    onDelete,
    onMove,
  }: {
    selectedCount: number;
    projects: Project[];
    currentProjectId: number | null;
    onSelectAll: () => void;
    onDeselectAll: () => void;
    onDelete: () => void;
    onMove: (projectId: number) => void;
  } = $props();

  let movePopupOpen = $state(false);

  function toggleMovePopup() {
    movePopupOpen = !movePopupOpen;
  }

  function handleMove(pid: number) {
    if (pid === currentProjectId) return;
    onMove(pid);
    movePopupOpen = false;
  }

  function handleMovePopupClose() {
    movePopupOpen = false;
  }
</script>

<div class="batch-bar" transition:fade={{ duration: 150 }}>
  <span class="batch-count">{selectedCount} selected</span>
  <div class="batch-actions" use:clickOutside={handleMovePopupClose}>
    <button class="batch-btn batch-btn-label" onclick={onSelectAll} title="Select all">
      <CheckSquare size={14} /> Select All
    </button>
    <button class="batch-btn batch-btn-label" onclick={onDeselectAll} title="Deselect all">
      <Square size={14} /> Clear
    </button>
    <div class="batch-divider"></div>
    <button class="batch-btn batch-btn-label batch-btn-danger" onclick={onDelete} title="Delete selected">
      <Trash2 size={14} /> Delete
    </button>
    <div class="batch-divider"></div>
    <button class="batch-btn batch-btn-label" onclick={toggleMovePopup} title="Move to project">
      <Move size={14} /> Move
    </button>
    {#if movePopupOpen}
      <div class="batch-move-popup">
        <div class="batch-move-header">Move to...</div>
        {#each projects as p}
          <button class="batch-move-item" onclick={() => handleMove(p.id)}>{p.name}</button>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
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
