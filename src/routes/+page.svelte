<script lang="ts">
  import { onMount } from "svelte";
  import Sidebar from "$components/Sidebar/index.svelte";
  import Toolbar from "$components/Toolbar/index.svelte";
  import TaskTable from "$components/TaskTable/index.svelte";
  import TaskDrawer from "$components/TaskDrawer/index.svelte";
  import StatusBar from "$components/StatusBar/index.svelte";
  import { projectStore, taskStore, uiStore } from "$stores";
  import { Pencil, Check, X } from "@lucide/svelte";
  import { formatDateTime } from "$utils";
  import { fly, fade } from "svelte/transition";

  const { selectedId, selectedProject } = projectStore;
  const { drawerOpen } = uiStore;

  let editMode = $state(false);
  let editName = $state("");
  let editDesc = $state("");
  let colorPickerOpen = $state(false);
  let colorPickerEl = $state<HTMLDivElement>();

  const COLORS = [
    "#e34d4d", "#f0883e", "#f7c843", "#3fb950",
    "#58a6ff", "#bc8cff", "#f083b7", "#8b949e",
  ];

  onMount(() => {
    console.log("[Page] onMount: loading projects...");
    projectStore.load();
  });

  $effect(() => {
    const pid = $selectedId;
    console.log("[Page] $effect: selectedId =", pid);
    if (pid !== null) {
      taskStore.load(pid);
      uiStore.closeDrawer();
      editMode = false;
    }
  });

  function toggleColorPicker() {
    colorPickerOpen = !colorPickerOpen;
  }

  function handleColorPick(color: string | null) {
    const p = $selectedProject;
    if (p) {
      projectStore.updateColor(p.id, color);
    }
    colorPickerOpen = false;
  }

  function handleWindowClick(e: MouseEvent) {
    if (colorPickerOpen && colorPickerEl && !colorPickerEl.contains(e.target as Node)) {
      colorPickerOpen = false;
    }
  }

  function startEdit() {
    const p = $selectedProject;
    if (p) {
      editName = p.name;
      editDesc = p.description;
      editMode = true;
    }
  }

  async function submitEdit() {
    const p = $selectedProject;
    if (!p) return;
    if (editName.trim() && editName.trim() !== p.name) {
      await projectStore.rename(p.id, editName.trim());
    }
    if (editDesc !== p.description) {
      await projectStore.updateDescription(p.id, editDesc);
    }
    editMode = false;
  }

  function cancelEdit() {
    editMode = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      if (colorPickerOpen) { colorPickerOpen = false; return; }
      if (editMode) cancelEdit();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} onclick={handleWindowClick} />

<div class="app-shell">
  <Toolbar />
  <div class="app-main">
    <Sidebar />
    <div class="app-content">
      {#if $selectedProject}
        <div class="project-header" class:editing={editMode}>
          <div class="ph-top">
            <div class="ph-dot-wrap" bind:this={colorPickerEl}>
              <button
                class="ph-dot-btn"
                onclick={toggleColorPicker}
                title="Change project color"
              >
                <span class="ph-dot" style="background: {$selectedProject.color ?? '#888'}"></span>
              </button>
              {#if colorPickerOpen}
                <div class="cp-popup" transition:fly={{ duration: 120, y: -4, opacity: 0 }}>
                  <div class="cp-swatches">
                    {#each COLORS as c}
                      <button
                        class="cp-swatch"
                        class:active={$selectedProject.color === c}
                        style="background: {c}"
                        title={c}
                        onclick={() => handleColorPick(c)}
                      ></button>
                    {/each}
                    <button
                      class="cp-swatch cp-swatch-none"
                      class:active={$selectedProject.color === null}
                      title="No color"
                      onclick={() => handleColorPick(null)}
                    ></button>
                  </div>
                </div>
              {/if}
            </div>
            <div class="ph-title-area">
              {#if editMode}
                <input class="ph-input ph-name-input" bind:value={editName} placeholder="Project name" />
              {:else}
                <span class="ph-name">{$selectedProject.name}</span>
              {/if}
            </div>
            <div class="ph-actions">
              {#if editMode}
                <button class="ph-btn ph-btn-confirm" onclick={submitEdit} title="Save"><Check size={14} /></button>
                <button class="ph-btn ph-btn-cancel" onclick={cancelEdit} title="Cancel"><X size={14} /></button>
              {:else}
                <button class="ph-btn ph-btn-edit" onclick={startEdit} title="Edit project info"><Pencil size={13} /></button>
              {/if}
            </div>
          </div>
          {#if editMode}
            <textarea class="ph-input ph-desc-input" bind:value={editDesc} placeholder="Project description" rows="2"></textarea>
          {:else if $selectedProject.description}
            <div class="ph-desc">{$selectedProject.description}</div>
          {/if}
          <div class="ph-meta">
            <span class="ph-meta-item">Created: {formatDateTime($selectedProject.create_time)}</span>
            {#if $selectedProject.update_time}
              <span class="ph-meta-item">Updated: {formatDateTime($selectedProject.update_time)}</span>
            {/if}
          </div>
        </div>
        <TaskTable />
      {:else}
        <div class="empty-state">
          <div class="empty-icon">
            <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2"/><rect x="9" y="3" width="6" height="4" rx="1"/></svg>
          </div>
          <p class="empty-text">Select or create a project to get started</p>
          <p class="empty-hint">Use the sidebar to choose an existing project, or click <strong>+</strong> to create a new one.</p>
        </div>
      {/if}
    </div>
  </div>
  <StatusBar />
</div>

{#if $drawerOpen}
  <div
    class="drawer-backdrop"
    role="button"
    tabindex="0"
    transition:fade={{ duration: 150 }}
    onclick={() => uiStore.closeDrawer()}
    onkeydown={(e) => { if (e.key === "Enter" || e.key === " ") { e.preventDefault(); uiStore.closeDrawer(); } }}
  ></div>
  <div class="app-drawer" transition:fly={{ duration: 200, x: 20, opacity: 0 }}>
    <TaskDrawer />
  </div>
{/if}

<style>
  .app-shell {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }
  .app-main {
    display: flex;
    flex: 1;
    overflow: hidden;
  }
  .app-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .project-header {
    flex-shrink: 0;
    padding: var(--spacing-lg) var(--spacing-md);
    border-bottom: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
  }
  .ph-top {
    display: flex;
    align-items: center;
    gap: var(--spacing-md);
  }
  .ph-dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    flex-shrink: 0;
  }
  .ph-title-area {
    flex: 1;
    min-width: 0;
  }
  .ph-name {
    font-size: var(--font-size-lg);
    font-weight: 600;
    color: var(--color-text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .ph-input {
    width: 100%;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    border-radius: var(--radius-sm);
    font-family: inherit;
    outline: none;
    font-size: var(--font-size-base);
    transition: border-color 0.15s ease;
  }
  .ph-input:focus {
    border-color: var(--color-accent);
  }
  .ph-name-input {
    height: 28px;
    padding: 0 6px;
    font-weight: 600;
    font-size: var(--font-size-lg);
  }
  .ph-desc-input {
    padding: var(--spacing-xs) var(--spacing-sm);
    font-size: var(--font-size-sm);
    resize: vertical;
    line-height: 1.4;
  }
  .ph-desc {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    line-height: 1.4;
    white-space: pre-wrap;
    word-break: break-word;
  }
  .ph-meta {
    display: flex;
    gap: var(--spacing-lg);
    font-size: 11px;
    color: var(--color-text-secondary);
  }
  .ph-meta-item {
    white-space: nowrap;
  }
  .ph-actions {
    display: flex;
    gap: 4px;
    flex-shrink: 0;
  }
  .ph-btn {
    width: 26px;
    height: 26px;
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
  .ph-btn-edit {
    opacity: 0;
  }
  .project-header:hover .ph-btn-edit {
    opacity: 1;
  }
  .ph-btn-edit:hover {
    background: var(--color-bg-tertiary);
    color: var(--color-text-primary);
  }
  .ph-btn-confirm {
    border-color: var(--color-accent);
    background: var(--color-accent);
    color: #ffffff;
  }
  .ph-btn-confirm:hover {
    background: var(--color-accent-hover);
    border-color: var(--color-accent-hover);
  }
  .ph-btn-cancel {
    border-color: var(--color-border);
    color: var(--color-text-secondary);
  }
  .ph-btn-cancel:hover {
    background: var(--color-bg-tertiary);
    color: var(--color-text-primary);
  }

  .ph-dot-wrap {
    position: relative;
    line-height: 0;
  }
  .ph-dot-btn {
    border: none;
    background: transparent;
    cursor: pointer;
    padding: 0;
    line-height: 0;
    border-radius: 50%;
    transition: transform 0.12s ease;
  }
  .ph-dot-btn:hover {
    transform: scale(1.15);
  }
  .ph-dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    display: inline-block;
    flex-shrink: 0;
  }
  .cp-popup {
    position: absolute;
    top: calc(100% + 6px);
    left: 0;
    background: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-md);
    padding: var(--spacing-sm);
    z-index: 20;
  }
  .cp-swatches {
    display: flex;
    gap: 4px;
    flex-wrap: wrap;
    max-width: 160px;
  }
  .cp-swatch {
    width: 22px;
    height: 22px;
    border-radius: 50%;
    border: 2px solid transparent;
    cursor: pointer;
    padding: 0;
    transition: border-color 0.12s ease, transform 0.12s ease;
  }
  .cp-swatch:hover {
    transform: scale(1.2);
    border-color: var(--color-text-secondary);
  }
  .cp-swatch.active {
    border-color: var(--color-text-primary);
  }
  .cp-swatch-none {
    border: 2px solid var(--color-border);
    background: var(--color-bg-primary) !important;
    position: relative;
  }
  .cp-swatch-none::after {
    content: "";
    position: absolute;
    inset: -1px;
    border-radius: 50%;
    border: 1.5px solid var(--color-border);
    clip-path: polygon(0 100%, 100% 0, 100% 100%);
  }

  .empty-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-sm);
    color: var(--color-text-secondary);
    padding: var(--spacing-lg);
  }
  .empty-icon {
    color: var(--color-text-secondary);
    opacity: 0.4;
    margin-bottom: var(--spacing-sm);
  }
  .empty-text {
    font-size: var(--font-size-base);
    font-weight: 500;
    color: var(--color-text-secondary);
  }
  .empty-hint {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    opacity: 0.7;
    text-align: center;
    max-width: 280px;
    line-height: 1.5;
  }

  .drawer-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(31, 35, 40, 0.3);
    z-index: 10;
  }
  .app-drawer {
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    width: var(--drawer-width, 400px);
    background: var(--color-bg-primary);
    z-index: 11;
    box-shadow: var(--shadow-md);
  }
</style>
