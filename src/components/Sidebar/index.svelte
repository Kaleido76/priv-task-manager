<script lang="ts">
  import { projectStore } from "$stores";
  import { taskStore } from "$stores";
  import ProjectItem from "./ProjectItem.svelte";
  import { Plus } from "@lucide/svelte";

  const { projects, selectedId } = projectStore;

  async function handleCreate() {
    console.log("[Sidebar] handleCreate");
    await projectStore.create("New Project");
  }
</script>

<aside class="sidebar">
  <div class="sidebar-header">
    <span class="sidebar-title">Projects</span>
    <button class="btn-add" onclick={handleCreate}>
      <Plus size={16} />
    </button>
  </div>
  <div class="project-list">
    {#each $projects as project}
      <ProjectItem
        {project}
        isSelected={project.id === $selectedId}
        onSelect={() => {
          console.log("[Sidebar] select project", project.id, project.name);
          projectStore.selectedId.set(project.id);
          taskStore.load(project.id);
        }}
        onDelete={() => projectStore.remove(project.id)}
      />
    {/each}
  </div>
  {#if $projects.length === 0}
    <div class="sidebar-empty">No projects yet</div>
  {/if}
</aside>

<style>
  .sidebar {
    width: var(--sidebar-width);
    background: var(--color-bg-secondary);
    border-right: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--spacing-md);
    border-bottom: 1px solid var(--color-border);
  }
  .sidebar-title {
    font-size: var(--font-size-sm);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--color-text-secondary);
  }
  .btn-add {
    width: 26px;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: var(--color-text-secondary);
    cursor: pointer;
    border-radius: var(--radius-sm);
    font-size: 16px;
    transition: background 0.15s ease, color 0.15s ease;
  }
  .btn-add:hover {
    background: var(--color-bg-tertiary);
    color: var(--color-text-primary);
  }
  .project-list {
    flex: 1;
    overflow-y: auto;
    padding: var(--spacing-md);
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
  }
  .sidebar-empty {
    padding: var(--spacing-lg);
    text-align: center;
    color: var(--color-text-secondary);
    font-size: var(--font-size-sm);
  }
</style>
