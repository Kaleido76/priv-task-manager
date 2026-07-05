<script lang="ts">
  import { TaskStatus } from "$types";

  let { status }: { status: TaskStatus } = $props();

  const labelMap: Record<TaskStatus, string> = {
    [TaskStatus.Todo]: "Todo",
    [TaskStatus.InProgress]: "In Progress",
    [TaskStatus.Done]: "Done",
    [TaskStatus.Cancelled]: "Cancelled",
  };
</script>

<span
  class="status-badge"
  class:status--todo={status === TaskStatus.Todo}
  class:status--in-progress={status === TaskStatus.InProgress}
  class:status--done={status === TaskStatus.Done}
  class:status--cancelled={status === TaskStatus.Cancelled}
>
  {labelMap[status]}
</span>

<style>
  .status-badge {
    display: inline-block;
    padding: 1px 8px;
    font-size: var(--font-size-sm);
    border-radius: 3px;
    font-weight: 500;
  }
  .status--todo { background: #e0e0e0; color: #616161; }
  .status--in-progress { background: #deecf9; color: #0078d4; }
  .status--done { background: #dff6dd; color: #107c10; }
  .status--cancelled { background: #fde7e9; color: #d32f2f; }
  @media (prefers-color-scheme: dark) {
    .status--todo { background: #3c3c3c; color: #969696; }
    .status--in-progress { background: #1a3a5c; color: #0098ff; }
    .status--done { background: #1a3a1a; color: #4ec44e; }
    .status--cancelled { background: #3a1a1a; color: #f44336; }
  }
</style>
