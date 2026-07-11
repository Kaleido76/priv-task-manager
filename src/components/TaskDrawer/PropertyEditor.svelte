<script lang="ts">
  import DatePicker from "./DatePicker.svelte";
  import { statusCfg, priorityCfg } from "$config";
  import { clickOutside } from "$actions/clickOutside";
  import { fade } from "svelte/transition";

  let { draftStatus = $bindable(), draftPriority = $bindable(), draftRecipient = $bindable(), draftDeadline = $bindable() }: {
    draftStatus: string;
    draftPriority: string;
    draftRecipient: string;
    draftDeadline: string;
  } = $props();

  let openSelect = $state<string | null>(null);

  function toggleSelect(name: string) {
    openSelect = openSelect === name ? null : name;
  }

  function pick(name: string, value: string) {
    if (name === "status") draftStatus = value;
    if (name === "priority") draftPriority = value;
    openSelect = null;
  }

  function handleWindowClick() {
    openSelect = null;
  }
</script>

<div class="property-editor" use:clickOutside={handleWindowClick}>
  <div class="property-row">
    <span class="property-label">Status</span>
    <div class="sel-wrapper">
      <button class="sel-btn" onclick={() => toggleSelect("status")}>
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
    min-width: 0;
    height: 34px;
    padding: 0 10px;
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

  .sel-wrapper {
    flex: 1;
    min-width: 0;
    position: relative;
  }
  .sel-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    height: 34px;
    padding: 0 10px;
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
    font-size: 12px;
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
