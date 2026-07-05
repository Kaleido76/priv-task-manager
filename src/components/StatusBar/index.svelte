<script lang="ts">
  import { projectStore, taskStore, uiStore } from "$stores";
  import { TaskStatus } from "$types";

  const { projects, selectedProject } = projectStore;
  const { tasks } = taskStore;
  const { saveStatus } = uiStore;

  let taskCount = $derived($tasks.length);
  let pendingCount = $derived(
    $tasks.filter(
      (t: { status: string }) => t.status === TaskStatus.Todo || t.status === TaskStatus.InProgress
    ).length
  );

  const saveLabelMap: Record<string, string> = {
    saved: "Saved",
    saving: "Saving...",
    unsaved: "Unsaved changes",
  };
</script>

<footer class="statusbar">
  <span class="statusbar-left">
    {$selectedProject?.name ?? "No project"}
  </span>
  <span class="statusbar-center">
    {taskCount} tasks · {pendingCount} pending
  </span>
  <span class="statusbar-right">
    {saveLabelMap[$saveStatus]}
  </span>
</footer>

<style>
  .statusbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: var(--statusbar-height);
    padding: 0 var(--spacing-lg);
    background: var(--color-bg-secondary);
    border-top: 1px solid var(--color-border);
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
  }
</style>
