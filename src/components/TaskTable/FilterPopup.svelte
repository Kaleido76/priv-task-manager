<script lang="ts">
  import { clickOutside } from "$actions/clickOutside";
  import { statusCfg, priorityCfg } from "$config";
  import { CheckSquare, Square } from "@lucide/svelte";
  import { fade } from "svelte/transition";

  type ColumnType = "text" | "status" | "priority" | "deadline" | "none";

  type ColumnFilter = {
    text?: string;
    items?: string[];
    dateFrom?: string;
    dateTo?: string;
  };

  let {
    colKey,
    colType,
    filter,
    top,
    left,
    onUpdate,
    onClose,
  }: {
    colKey: string;
    colType: ColumnType;
    filter: ColumnFilter | undefined;
    top: number;
    left: number;
    onUpdate: (colKey: string, patch: Partial<ColumnFilter>) => void;
    onClose: () => void;
  } = $props();

  let filterTextInput = $state(filter?.text ?? "");
  let filterItemsInput = $state<string[]>(filter?.items ?? []);
  let filterDateFrom = $state(filter?.dateFrom ?? "");
  let filterDateTo = $state(filter?.dateTo ?? "");

  let textDebounce: ReturnType<typeof setTimeout>;

  function handleFilterTextInput() {
    clearTimeout(textDebounce);
    textDebounce = setTimeout(() => {
      onUpdate(colKey, { text: filterTextInput });
    }, 200);
  }

  function toggleFilterItem(item: string) {
    const idx = filterItemsInput.indexOf(item);
    if (idx >= 0) filterItemsInput = filterItemsInput.filter(i => i !== item);
    else filterItemsInput = [...filterItemsInput, item];
    onUpdate(colKey, { items: filterItemsInput });
  }

  function handleDateFromInput(e: Event) {
    filterDateFrom = (e.target as HTMLInputElement).value;
    onUpdate(colKey, { dateFrom: filterDateFrom });
  }

  function handleDateToInput(e: Event) {
    filterDateTo = (e.target as HTMLInputElement).value;
    onUpdate(colKey, { dateTo: filterDateTo });
  }
</script>

<div
  class="col-filter-popup"
  style="top:{top}px;left:{left}px;transform:translateX(-50%)"
  use:clickOutside={onClose}
  transition:fade={{ duration: 80 }}
>
  {#if colType === "text"}
    <div class="cf-text">
      <input
        class="cf-text-input"
        type="text"
        placeholder="Filter..."
        bind:value={filterTextInput}
        oninput={handleFilterTextInput}
      />
    </div>
  {:else if colType === "status"}
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
  {:else if colType === "priority"}
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
  {:else if colType === "deadline"}
    <div class="cf-date">
      <span class="cf-date-label">From</span>
      <input class="cf-date-input" type="date" value={filterDateFrom} oninput={handleDateFromInput} aria-label="Filter date from" />
      <span class="cf-date-label">To</span>
      <input class="cf-date-input" type="date" value={filterDateTo} oninput={handleDateToInput} aria-label="Filter date to" />
    </div>
  {/if}
</div>

<style>
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
</style>
