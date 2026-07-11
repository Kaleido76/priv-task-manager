<script lang="ts">
  import { onMount } from "svelte";
  import Sidebar from "$components/Sidebar/index.svelte";
  import Toolbar from "$components/Toolbar/index.svelte";
  import TaskTable from "$components/TaskTable/index.svelte";
  import TaskDrawer from "$components/TaskDrawer/index.svelte";
  import StatusBar from "$components/StatusBar/index.svelte";
  import ProjectHeader from "$components/ProjectHeader/index.svelte";
  import { projectStore, taskStore, uiStore, saveRequested } from "$stores";
  import { fade, fly } from "svelte/transition";

  const { selectedId, selectedProject } = projectStore;
  const { drawerOpen } = uiStore;

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
    }
  });
</script>

<div class="app-shell">
  <Toolbar />
  <div class="app-main">
    <Sidebar />
    <div class="app-content">
      {#if $selectedProject}
        <ProjectHeader project={$selectedProject} />
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
    onclick={() => uiStore.requestSave()}
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
