<script lang="ts">
  import { taskStore } from "$stores";
  import { Plus, Search } from "@lucide/svelte";

  const { searchKeyword } = taskStore;

  let searchValue = $state("");

  function handleSearchInput() {
    console.log("[Toolbar] handleSearchInput", searchValue);
    searchKeyword.set(searchValue);
  }

  async function handleNewTask() {
    console.log("[Toolbar] handleNewTask");
    await taskStore.create("New Task");
  }
</script>

<header class="toolbar">
  <div class="toolbar-left">
    <button class="btn btn-primary" onclick={handleNewTask}>
      <Plus size={16} /> New Task
    </button>
  </div>
  <div class="toolbar-center">
    <div class="search-wrapper">
      <span class="search-icon"><Search size={14} /></span>
      <input
        class="search-input"
        type="text"
        placeholder="Search tasks..."
        bind:value={searchValue}
        oninput={handleSearchInput}
      />
    </div>
  </div>
</header>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    height: var(--toolbar-height);
    padding: 0 var(--spacing-lg);
    background: var(--color-bg-primary);
    border-bottom: 1px solid var(--color-border);
    gap: var(--spacing-md);
  }
  .toolbar-left {
    display: flex;
    align-items: center;
  }
  .toolbar-center {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .btn {
    display: inline-flex;
    align-items: center;
    padding: 4px 12px;
    border: 1px solid var(--color-border);
    background: var(--button-bg, transparent);
    color: var(--color-text-primary);
    cursor: pointer;
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    height: 28px;
    transition: background 0.15s ease, border-color 0.15s ease;
  }
  .btn:hover {
    background: var(--button-hover-bg, var(--color-bg-tertiary));
  }
  .btn-primary {
    background: var(--color-accent);
    color: #ffffff;
    border-color: var(--color-accent);
  }
  .btn-primary:hover {
    background: var(--color-accent-hover);
    border-color: var(--color-accent-hover);
  }
  .search-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }
  .search-icon {
    position: absolute;
    left: 8px;
    color: var(--color-text-secondary);
    pointer-events: none;
  }
  .search-input {
    width: 300px;
    height: 28px;
    padding: 0 8px 0 28px;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    border-radius: var(--radius-sm);
    outline: none;
    font-size: var(--font-size-sm);
    transition: border-color 0.15s ease, box-shadow 0.15s ease;
  }
  .search-input:focus {
    border-color: var(--color-accent);
    box-shadow: 0 0 0 3px rgba(9, 105, 218, 0.12);
  }
</style>
