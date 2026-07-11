<script lang="ts">
  import type { CardType } from "$types";
  import { File, StickyNote, Link, ListTodo } from "@lucide/svelte";
  import { cardTypeCfg } from "$config";

  let { onAdd }: { onAdd: (type: CardType) => void } = $props();
</script>

<div class="card-adder">
  {#each Object.entries(cardTypeCfg) as [key, cfg]}
    <button class="card-add-btn" onclick={() => onAdd(key as CardType)}>
      {#if key === "file"}
        <File size={13} />
      {:else if key === "note"}
        <StickyNote size={13} />
      {:else if key === "todolist"}
        <ListTodo size={13} />
      {:else}
        <Link size={13} />
      {/if}
      {cfg.label}
    </button>
  {/each}
</div>

<style>
  .card-adder {
    display: flex;
    gap: var(--spacing-xs);
    flex-shrink: 0;
  }
  .card-add-btn {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 5px 12px;
    border: 1px dashed var(--color-border);
    background: transparent;
    color: var(--color-text-secondary);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    cursor: pointer;
    transition: background 0.15s ease, color 0.15s ease, border-color 0.15s ease;
    font-family: inherit;
  }
  .card-add-btn:hover {
    background: var(--color-bg-tertiary);
    color: var(--color-text-primary);
    border-color: var(--color-text-secondary);
  }
</style>
