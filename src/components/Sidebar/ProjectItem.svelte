<script lang="ts">
  import type { Project } from "$types";
  import { Trash2, AlertTriangle } from "@lucide/svelte";

  let { project, isSelected, onSelect, onDelete }: {
    project: Project;
    isSelected: boolean;
    onSelect: () => void;
    onDelete: () => void;
  } = $props();

  let deleteConfirm = $state(false);

  function handleDelete(e: MouseEvent) {
    e.stopPropagation();
    if (deleteConfirm) {
      onDelete();
      deleteConfirm = false;
    } else {
      deleteConfirm = true;
      setTimeout(() => deleteConfirm = false, 3000);
    }
  }

  function resetDelete() {
    deleteConfirm = false;
  }
</script>

<div
  class="project-item"
  class:selected={isSelected}
  onclick={() => { resetDelete(); onSelect(); }}
  onkeydown={(e) => { if (e.key === "Enter") { resetDelete(); onSelect(); } }}
  role="button"
  tabindex="0"
>
  <span class="project-dot" style="background: {project.color ?? '#888'}"></span>
  <span class="project-name">{project.name}</span>
  {#if isSelected}
    <button
      class="btn-delete"
      class:confirm={deleteConfirm}
      onclick={handleDelete}
      title={deleteConfirm ? "Click again to delete" : "Delete project"}
    >
      {#if deleteConfirm}
        <AlertTriangle size={14} />
      {:else}
        <Trash2 size={14} />
      {/if}
    </button>
  {/if}
</div>

<style>
  .project-item {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-sm) var(--spacing-md);
    cursor: pointer;
    font-size: var(--font-size-base);
    color: var(--color-text-primary);
    background: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    transition: background 0.15s ease, color 0.15s ease, border-color 0.15s ease, box-shadow 0.15s ease;
  }
  .project-item:hover {
    background: var(--sidebar-hover-bg);
    border-color: var(--color-text-secondary);
    box-shadow: var(--shadow-sm);
  }
  .project-item.selected {
    background: var(--color-accent);
    color: #ffffff;
    border-color: var(--color-accent);
    box-shadow: 0 1px 4px rgba(9, 105, 218, 0.3);
  }
  .project-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }
  .project-name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: var(--font-size-base);
  }
  .btn-delete {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: inherit;
    cursor: pointer;
    opacity: 0.4;
    flex-shrink: 0;
    border-radius: 3px;
    transition: opacity 0.15s ease, background 0.15s ease, color 0.15s ease;
  }
  .btn-delete:hover {
    opacity: 1;
  }
  .btn-delete.confirm {
    opacity: 1;
    color: #cf222e;
    background: rgba(207, 34, 46, 0.2);
  }
</style>
