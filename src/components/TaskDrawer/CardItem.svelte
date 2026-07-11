<script lang="ts">
  import type { TaskCard } from "$types";
  import { Trash2, Pencil, GripVertical, AlertTriangle, FolderOpen, ExternalLink, File, StickyNote, Link, ListTodo, Check } from "@lucide/svelte";
  import { cardTypeCfg } from "$config";
  import { truncatePath } from "$utils";
  import { openFileLocation, openUrl, checkPathExists } from "$api";

  let {
    card, onEdit, onDelete, confirmingDelete,
    onMouseDown, onToggleTodoItem, onResetDeleteConfirm, dropTarget,
  }: {
    card: TaskCard;
    onEdit: (id: number) => void;
    onDelete: (id: number) => void;
    confirmingDelete: boolean;
    onMouseDown: (e: MouseEvent, id: number) => void;
    onToggleTodoItem?: (cardId: number, itemId: string) => void;
    onResetDeleteConfirm?: () => void;
    dropTarget?: boolean;
  } = $props();

  const cfg = $derived(cardTypeCfg[card.card_type]);

  let valid = $state<boolean | null>(null);

  $effect(() => {
    if (card.card_type === "file" && card.data.path) {
      checkPathExists(card.data.path).then((exists) => { valid = exists; });
    } else if (card.card_type === "link" && card.data.url) {
      valid = /^(https?:\/\/)?[\w.-]+\.[a-z]{2,}/i.test(card.data.url);
    } else if (card.card_type === "todolist") {
      valid = true;
    } else {
      valid = true;
    }
  });

  function handleCardClick(e: MouseEvent | KeyboardEvent) {
    const t = e.target as HTMLElement;
    if (t.closest(".card-header") || t.closest(".card-actions") || t.closest(".todo-item")) return;
    if (card.card_type === "file" && card.data.path) {
      openFileLocation(card.data.path);
    } else if (card.card_type === "link" && card.data.url) {
      openUrl(card.data.url);
    }
  }
</script>

<div
  class="card-item"
  class:clickable={card.card_type === "file" || card.card_type === "link"}
  class:drop-target={dropTarget}
  ondblclick={handleCardClick}
  onkeydown={(e) => { if (e.key === "Enter") handleCardClick(e); }}
  role="button"
  tabindex="0"
  data-card-id={card.id}
  onmouseleave={onResetDeleteConfirm}
>
  <div
    class="card-header"
    style="background:{cfg.color}15; border-bottom: 1px solid {cfg.color}30"
    onmousedown={(e) => onMouseDown(e, card.id)}
    onkeydown={() => {}}
    role="button"
    tabindex="0"
  >
    <span class="card-header-icon" style="color:{cfg.color}"><GripVertical size={14} /></span>
    <span class="card-header-type" style="color:{cfg.color}">
      {#if card.card_type === "file"}
        <File size={14} />
      {:else if card.card_type === "note"}
        <StickyNote size={14} />
      {:else if card.card_type === "todolist"}
        <ListTodo size={14} />
      {:else}
        <Link size={14} />
      {/if}
      {cfg.label}
    </span>
  </div>

  <div class="card-body">
    {#if card.card_type === "file"}
      {#if card.data.name}
        <div class="card-name">{card.data.name}</div>
        {#if card.data.path}
          <div class="card-secondary">{truncatePath(card.data.path)}</div>
        {/if}
      {:else}
        <div class="card-name">{card.data.path}</div>
      {/if}
    {:else if card.card_type === "note"}
      <div class="card-note-content">
        {#if card.data.content}
          {card.data.content}
        {:else}
          <span class="card-empty">Empty note</span>
        {/if}
      </div>
    {:else if card.card_type === "todolist"}
      <div class="card-todo-list">
        {#if card.data.items && card.data.items.length > 0}
          {#each card.data.items.slice(0, 8) as item}
            <div class="todo-item" class:done={item.done} onclick={() => onToggleTodoItem?.(card.id, item.id)} onkeydown={() => {}} role="button" tabindex="0">
              <span class="todo-checkbox" style="color:{cfg.color}">
                {#if item.done}
                  <Check size={12} />
                {:else}
                  <div class="todo-checkbox-empty" style="border-color:{cfg.color}40"></div>
                {/if}
              </span>
              <span class="todo-text">{item.text}</span>
            </div>
          {/each}
          {#if card.data.items.length > 8}
            <div class="todo-more">+{card.data.items.length - 8} more</div>
          {/if}
        {:else}
          <span class="card-empty">Empty list</span>
        {/if}
      </div>
    {:else if card.card_type === "link"}
      {#if card.data.name}
        <div class="card-name">{card.data.name}</div>
        {#if card.data.url}
          <div class="card-secondary">{card.data.url}</div>
        {/if}
      {:else}
        <div class="card-name">{card.data.url}</div>
      {/if}
    {/if}

    {#if card.card_type === "file" || card.card_type === "link"}
      <div class="card-footer">
        <span class="card-click-hint">
          {#if card.card_type === "file"}
            <FolderOpen size={11} /> Double-click to open file location
          {:else}
            <ExternalLink size={11} /> Double-click to open link
          {/if}
        </span>
        {#if valid === false}
          <span class="card-validation-warn">
            <AlertTriangle size={12} />
            {#if card.card_type === "file"}
              Path not found
            {:else}
              Invalid URL format
            {/if}
          </span>
        {/if}
      </div>
    {/if}

    <div class="card-actions" onclick={(e) => e.stopPropagation()} onkeydown={() => {}} role="presentation">
      <button class="card-action-btn card-action-edit" onclick={() => onEdit(card.id)} title="Edit"><Pencil size={12} /></button>
      <button
        class="card-action-btn"
        class:confirm={confirmingDelete}
        onclick={() => onDelete(card.id)}
        title={confirmingDelete ? "Click again to delete" : "Delete"}
      >
        {#if confirmingDelete}
          <AlertTriangle size={12} />
        {:else}
          <Trash2 size={12} />
        {/if}
      </button>
    </div>
  </div>
</div>

<style>
  .card-item {
    display: flex;
    flex-direction: column;
    gap: 6px;
    background: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    position: relative;
    transition: box-shadow 0.15s ease;
    overflow: hidden;
    flex-shrink: 0;
  }
  .card-item.clickable {
    cursor: pointer;
  }
  .card-item.clickable:hover {
    box-shadow: var(--shadow-sm);
  }
  .card-item.drop-target {
    outline: 2px solid var(--color-accent);
    outline-offset: -2px;
    border-radius: var(--radius-md);
  }
  .card-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: -1px;
    gap: 8px;
  }
  .card-click-hint {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 12px;
    color: var(--color-text-secondary);
    opacity: 0.6;
  }
  .card-validation-warn {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    color: #9a6700;
    white-space: nowrap;
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 6px 12px;
    cursor: grab;
    user-select: none;
    margin: 0;
  }
  .card-header:active {
    cursor: grabbing;
  }
  .card-header-icon {
    display: inline-flex;
    align-items: center;
    opacity: 0.6;
  }
  .card-header:hover .card-header-icon {
    opacity: 1;
  }
  .card-header-type {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.4px;
  }

  .card-body {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
    padding: 0 12px 10px;
    position: relative;
  }
  .card-body .card-name {
    font-size: var(--font-size-base);
    font-weight: 500;
    color: var(--color-text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .card-body .card-secondary {
    font-size: 12px;
    color: var(--color-text-secondary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .card-note-content {
    font-size: var(--font-size-sm);
    color: var(--color-text-primary);
    line-height: 1.4;
    white-space: pre-wrap;
    word-break: break-word;
    user-select: text;
  }
  .card-empty {
    color: var(--color-text-secondary);
    font-style: italic;
  }

  .card-actions {
    position: absolute;
    top: 0;
    right: 4px;
    display: flex;
    gap: 4px;
    opacity: 0;
    transition: opacity 0.12s ease;
  }
  .card-item:hover .card-actions {
    opacity: 1;
  }
  .card-action-btn {
    width: 22px;
    height: 22px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-secondary);
    cursor: pointer;
    border-radius: 3px;
    transition: background 0.12s ease, color 0.12s ease, border-color 0.12s ease;
    padding: 0;
  }
  .card-action-btn:hover {
    background: var(--color-bg-tertiary);
    color: var(--color-text-primary);
  }
  .card-action-edit {
    color: var(--color-accent);
  }
  .card-action-edit:hover {
    background: var(--color-accent);
    color: #ffffff;
    border-color: var(--color-accent);
  }
  .card-action-btn.confirm {
    border-color: #cf222e;
    color: #cf222e;
    background: #fff0ee;
  }
  .card-action-btn.confirm:hover {
    background: #ffdedb;
    border-color: #a0111f;
  }

  .card-todo-list {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }
  .todo-item {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: var(--font-size-sm);
    color: var(--color-text-primary);
    padding: 2px 0;
    line-height: 1.3;
  }
  .todo-item.done .todo-text {
    text-decoration: line-through;
    opacity: 0.6;
  }
  .todo-checkbox {
    display: inline-flex;
    align-items: center;
    flex-shrink: 0;
  }
  .todo-checkbox-empty {
    width: 12px;
    height: 12px;
    border-radius: 2px;
    border: 1px solid;
  }
  .todo-text {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .todo-more {
    font-size: 12px;
    color: var(--color-text-secondary);
    opacity: 0.7;
    padding-top: 2px;
  }
</style>
