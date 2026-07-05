<script lang="ts">
  import DatePicker from "./DatePicker.svelte";
  import { fade } from "svelte/transition";

  let { draftStatus = $bindable(), draftPriority = $bindable(), draftRecipient = $bindable(), draftProgress = $bindable(), draftDeadline = $bindable() }: {
    draftStatus: string;
    draftPriority: string;
    draftRecipient: string;
    draftProgress: number;
    draftDeadline: string;
  } = $props();

  const statusCfg: Record<string, { label: string; bg: string; fg: string }> = {
    todo: { label: "Todo", bg: "#ddf4ff", fg: "#0969da" },
    in_progress: { label: "In Progress", bg: "#fff1e5", fg: "#b35900" },
    done: { label: "Done", bg: "#dafbe1", fg: "#1a7f37" },
    cancelled: { label: "Cancelled", bg: "#fff0ee", fg: "#cf222e" },
  };

  const priorityCfg: Record<string, { label: string; bg: string; fg: string }> = {
    suggestion: { label: "Suggestion", bg: "#f3f4f6", fg: "#656d76" },
    low: { label: "Low", bg: "#ddf4ff", fg: "#0969da" },
    medium: { label: "Medium", bg: "#fff1e5", fg: "#b35900" },
    high: { label: "High", bg: "#fff0ee", fg: "#cf222e" },
    urgent: { label: "Urgent", bg: "#8b0000", fg: "#ffffff" },
  };

  let openSelect = $state<string | null>(null);
  let selectEl = $state<HTMLDivElement>();
  let selectBtnEl = $state<HTMLButtonElement>();

  function toggleSelect(name: string) {
    openSelect = openSelect === name ? null : name;
  }

  function pick(name: string, value: string) {
    if (name === "status") draftStatus = value;
    if (name === "priority") draftPriority = value;
    openSelect = null;
  }

  function handleWindowClick(e: MouseEvent) {
    if (selectEl && !selectEl.contains(e.target as Node)) {
      openSelect = null;
    }
  }
</script>

<svelte:window onclick={handleWindowClick} />

<div class="property-editor" bind:this={selectEl}>
  <div class="property-row">
    <span class="property-label">Status</span>
    <div class="sel-wrapper">
      <button class="sel-btn" onclick={() => toggleSelect("status")} bind:this={selectBtnEl}>
        <span class="sel-dot" style="background:{statusCfg[draftStatus]?.bg ?? '#eaeef2'}"></span>
        <span class="sel-label">{statusCfg[draftStatus]?.label ?? draftStatus}</span>
        <span class="sel-arrow">▾</span>
      </button>
      {#if openSelect === "status"}
        <div class="sel-popup" transition:fade={{ duration: 100 }}>
          {#each Object.entries(statusCfg) as [key, opt]}
            <button
              class="sel-option"
              class:sel-active={draftStatus === key}
              onclick={() => pick("status", key)}
            >
              <span class="sel-opt-dot" style="background:{opt.bg}"></span>
              {opt.label}
            </button>
          {/each}
        </div>
      {/if}
    </div>
  </div>
  <div class="property-row">
    <span class="property-label">Priority</span>
    <div class="sel-wrapper">
      <button class="sel-btn" onclick={() => toggleSelect("priority")}>
        <span class="sel-dot" style="background:{priorityCfg[draftPriority]?.bg ?? '#eaeef2'}"></span>
        <span class="sel-label">{priorityCfg[draftPriority]?.label ?? draftPriority}</span>
        <span class="sel-arrow">▾</span>
      </button>
      {#if openSelect === "priority"}
        <div class="sel-popup" transition:fade={{ duration: 100 }}>
          {#each Object.entries(priorityCfg) as [key, opt]}
            <button
              class="sel-option"
              class:sel-active={draftPriority === key}
              onclick={() => pick("priority", key)}
            >
              <span class="sel-opt-dot" style="background:{opt.bg}"></span>
              {opt.label}
            </button>
          {/each}
        </div>
      {/if}
    </div>
  </div>
  <div class="property-row">
    <span class="property-label">Recipient</span>
    <input class="property-input" type="text" bind:value={draftRecipient} placeholder="e.g. Team Alpha" />
  </div>
  <div class="property-row">
    <span class="property-label">Progress</span>
    <input class="property-input" type="range" min="0" max="100" bind:value={draftProgress} />
    <span class="property-value">{draftProgress}%</span>
  </div>
  <div class="property-row">
    <span class="property-label">Deadline</span>
    <DatePicker bind:value={draftDeadline} />
  </div>
</div>

<style>
  .property-editor {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
    position: relative;
  }
  .property-row {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
  }
  .property-label {
    width: 80px;
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    flex-shrink: 0;
  }
  .property-input {
    flex: 1;
    height: 28px;
    padding: 0 6px;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    outline: none;
    transition: border-color 0.15s ease;
  }
  .property-input:focus {
    border-color: var(--color-accent);
  }
  input[type="range"].property-input {
    padding: 0;
    height: auto;
  }
  .property-value {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    min-width: 32px;
    text-align: right;
  }

  .sel-wrapper {
    flex: 1;
    position: relative;
  }
  .sel-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    height: 28px;
    padding: 0 6px;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    cursor: pointer;
    outline: none;
    transition: border-color 0.15s ease;
    font-family: inherit;
  }
  .sel-btn:hover {
    border-color: var(--color-text-secondary);
  }
  .sel-btn:focus {
    border-color: var(--color-accent);
  }
  .sel-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }
  .sel-arrow {
    font-size: 10px;
    color: var(--color-text-secondary);
    margin-left: auto;
  }
  .sel-label {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .sel-popup {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    right: 0;
    background: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-md);
    z-index: 20;
    overflow: hidden;
  }
  .sel-option {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    padding: 6px 10px;
    border: none;
    background: transparent;
    color: var(--color-text-primary);
    font-size: var(--font-size-sm);
    text-align: left;
    cursor: pointer;
    font-family: inherit;
    transition: background 0.1s ease;
  }
  .sel-option:hover {
    background: var(--color-bg-tertiary);
  }
  .sel-option.sel-active {
    color: var(--color-accent);
    font-weight: 600;
  }
  .sel-opt-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }
</style>
