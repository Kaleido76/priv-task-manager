<script lang="ts">
  import type { CardType, CardData, TodoItem } from "$types";
  import { cardTypeCfg } from "$config";
  import { Plus, Trash2, Check } from "@lucide/svelte";
  import { tick } from "svelte";

  let { cardType, data, isNew, onSave, onCancel }: {
    cardType: CardType;
    data: CardData;
    isNew: boolean;
    onSave: (data: CardData) => void;
    onCancel: () => void;
  } = $props();

  let editPath = $state(data.path ?? "");
  let editName = $state(data.name ?? "");
  let editContent = $state(data.content ?? "");
  let editUrl = $state(data.url ?? "");
  let editItems = $state<TodoItem[]>(data.items?.length ? data.items.map(i => ({ ...i })) : []);

  let inputRefs: (HTMLInputElement | null)[] = [];

  const cfg = $derived(cardTypeCfg[cardType]);

  const canSave = $derived(
    cardType === "note" ? editContent.trim().length > 0
    : cardType === "file" ? editPath.trim().length > 0
    : cardType === "link" ? editUrl.trim().length > 0
    : cardType === "todolist" ? editItems.length > 0
    : true
  );
  const canSaveNew = $derived(!isNew || canSave);

  const showDeleteLabel = $derived(
    cardType === "note" && !isNew && !editContent.trim()
    || cardType === "todolist" && !isNew && editItems.length === 0
  );

  function addItem() {
    const id = crypto.randomUUID();
    editItems = [...editItems, { id, text: "", done: false }];
  }

  function removeItem(id: string) {
    editItems = editItems.filter(i => i.id !== id);
  }

  function toggleItem(id: string) {
    editItems = editItems.map(i => i.id === id ? { ...i, done: !i.done } : i);
  }

  function updateItemText(id: string, text: string) {
    editItems = editItems.map(i => i.id === id ? { ...i, text } : i);
  }

  async function handleItemKeydown(e: KeyboardEvent, index: number) {
    if (e.key === "Enter") {
      e.preventDefault();
      const id = crypto.randomUUID();
      editItems = [
        ...editItems.slice(0, index + 1),
        { id, text: "", done: false },
        ...editItems.slice(index + 1),
      ];
      await tick();
      inputRefs[index + 1]?.focus();
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      if (index > 0) {
        inputRefs[index - 1]?.focus();
      }
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      if (index < editItems.length - 1) {
        inputRefs[index + 1]?.focus();
      }
    }
  }

  function handleSave() {
    onSave({ path: editPath, name: editName, content: editContent, url: editUrl, items: editItems });
  }
</script>

<div class="card-editor" style="border-color:{cfg.color}">
  <div class="editor-heading" style="color:{cfg.color}">Edit {cfg.label}</div>

  {#if cardType === "file"}
    <div class="editor-field">
      <label class="editor-label" for="editor-path">File Path *</label>
      <input id="editor-path" class="editor-input" type="text" bind:value={editPath} placeholder="e.g. C:\path\to\file.pdf" />
    </div>
    <div class="editor-field">
      <label class="editor-label" for="editor-name">Display Name</label>
      <input id="editor-name" class="editor-input" type="text" bind:value={editName} placeholder="Display name (optional)" />
    </div>
  {:else if cardType === "note"}
    <div class="editor-field">
      <label class="editor-label" for="editor-content">Content</label>
      <textarea id="editor-content" class="editor-textarea" bind:value={editContent} placeholder="Write your note..."></textarea>
    </div>
  {:else if cardType === "todolist"}
    <div class="editor-field">
      <div class="todo-editor-list">
        {#each editItems as item, i (item.id)}
          <div class="todo-editor-row" class:done={item.done}>
            <button class="todo-editor-toggle" onclick={() => toggleItem(item.id)} title={item.done ? "Uncheck" : "Check"}>
              {#if item.done}
                <Check size={12} />
              {:else}
                <div class="todo-editor-empty"></div>
              {/if}
            </button>
            <input class="todo-editor-input" type="text" bind:value={item.text} placeholder="Item {i + 1}" bind:this={inputRefs[i]} onkeydown={(e) => handleItemKeydown(e, i)} />
            <button class="todo-editor-remove" onclick={() => removeItem(item.id)} title="Remove item"><Trash2 size={12} /></button>
          </div>
        {/each}
      </div>
      <button class="todo-editor-add" onclick={addItem}><Plus size={12} /> Add item</button>
    </div>
  {:else if cardType === "link"}
    <div class="editor-field">
      <label class="editor-label" for="editor-url">URL *</label>
      <input id="editor-url" class="editor-input" type="text" bind:value={editUrl} placeholder="e.g. https://example.com" />
    </div>
    <div class="editor-field">
      <label class="editor-label" for="editor-name2">Display Name</label>
      <input id="editor-name2" class="editor-input" type="text" bind:value={editName} placeholder="Display name (optional)" />
    </div>
  {/if}

  <div class="editor-actions">
    <button class="editor-btn editor-btn-cancel" onclick={onCancel}>Cancel</button>
    <button class="editor-btn editor-btn-save" onclick={handleSave} disabled={!canSaveNew}>{showDeleteLabel ? "Delete" : "Save"}</button>
  </div>
</div>

<style>
  .card-editor {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
    padding: var(--spacing-sm) var(--spacing-md);
    background: var(--color-bg-primary);
    border: 1px solid;
    border-radius: var(--radius-md);
  }
  .editor-heading {
    font-size: var(--font-size-sm);
    font-weight: 600;
    padding-bottom: 2px;
  }
  .editor-field {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }
  .editor-label {
    font-size: 12px;
    color: var(--color-text-secondary);
    font-weight: 500;
  }
  .editor-input {
    height: 32px;
    padding: 0 8px;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    outline: none;
    transition: border-color 0.15s ease;
    font-family: inherit;
  }
  .editor-input:focus {
    border-color: var(--color-accent);
  }
  .editor-textarea {
    min-height: 80px;
    padding: 8px;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    outline: none;
    resize: vertical;
    font-family: inherit;
    line-height: 1.4;
    transition: border-color 0.15s ease;
  }
  .editor-textarea:focus {
    border-color: var(--color-accent);
  }
  .editor-actions {
    display: flex;
    gap: var(--spacing-xs);
    justify-content: flex-end;
  }
  .editor-btn {
    padding: 4px 14px;
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    cursor: pointer;
    font-family: inherit;
    transition: background 0.15s ease, opacity 0.15s ease;
  }
  .editor-btn-cancel {
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
  }
  .editor-btn-cancel:hover {
    background: var(--color-bg-tertiary);
  }
  .editor-btn-save {
    border: 1px solid var(--color-accent);
    background: var(--color-accent);
    color: #ffffff;
  }
  .editor-btn-save:hover:not(:disabled) {
    background: var(--color-accent-hover);
  }
  .editor-btn-save:disabled {
    opacity: 0.5;
    cursor: default;
  }

  /* ── TodoList Editor ── */
  .todo-editor-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
    max-height: 200px;
    overflow-y: auto;
  }
  .todo-editor-row {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .todo-editor-row.done .todo-editor-input {
    text-decoration: line-through;
    opacity: 0.6;
  }
  .todo-editor-toggle {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: var(--color-accent);
    cursor: pointer;
    padding: 0;
    flex-shrink: 0;
    border-radius: 3px;
  }
  .todo-editor-toggle:hover {
    background: var(--color-bg-tertiary);
  }
  .todo-editor-empty {
    width: 14px;
    height: 14px;
    border-radius: 3px;
    border: 1.5px solid var(--color-border);
  }
  .todo-editor-input {
    flex: 1;
    height: 30px;
    padding: 0 8px;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    outline: none;
    font-family: inherit;
    min-width: 0;
  }
  .todo-editor-input:focus {
    border-color: var(--color-accent);
  }
  .todo-editor-remove {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: var(--color-text-secondary);
    cursor: pointer;
    padding: 0;
    flex-shrink: 0;
    border-radius: 3px;
  }
  .todo-editor-remove:hover {
    color: #cf222e;
    background: #fff0ee;
  }
  .todo-editor-add {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 4px 10px;
    margin-top: 4px;
    border: 1px dashed var(--color-border);
    background: transparent;
    color: var(--color-text-secondary);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    cursor: pointer;
    font-family: inherit;
    transition: background 0.15s ease, color 0.15s ease;
  }
  .todo-editor-add:hover {
    background: var(--color-bg-tertiary);
    color: var(--color-text-primary);
  }
</style>
