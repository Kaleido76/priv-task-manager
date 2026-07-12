<script lang="ts">
  import { taskStore, uiStore, saveRequested, cardStore } from "$stores";
  import type { TaskCard, CardData, CardType, UpdateTaskRequest } from "$types";
  import PropertyEditor from "./PropertyEditor.svelte";
  import CardAdder from "./CardAdder.svelte";
  import CardItem from "./CardItem.svelte";
  import CardEditor from "./CardEditor.svelte";
  import { formatDateTime } from "$utils";
  import { X, AlertTriangle } from "@lucide/svelte";
  import { getCurrentWebview } from "@tauri-apps/api/webview";

  const { selectedTask } = taskStore;
  const { drawerOpen } = uiStore;
  const { cards, loading } = cardStore;

  let editingCardId = $state<number | null>(null);
  let editingCardData = $state<CardData>({});
  let editingIsNew = $state(false);
  let confirmingDeleteCardId = $state<number | null>(null);

  // Drag state
  let dragSourceId: number | null = null;
  let dragGhostEl: HTMLElement | null = null;
  let dropIndicatorIndex: number = $state(-1);
  let dropIndicatorTop: number = $state(0);

  let draftTitle = $state("");
  let draftStatus = $state("");
  let draftPriority = $state("");
  let draftRecipient = $state("");
  let draftDeadline = $state("");

  let editingTaskId = $state<number | null>(null);
  let confirmDiscard = $state(false);
  let originalTitle = $state("");
  let originalStatus = $state("");
  let originalPriority = $state("");
  let originalRecipient = $state("");
  let originalDeadline = $state("");
  let savingBeforeClose = $state(false);
  let dropTargetCardId = $state<number | null>(null);
  let dragDropUnlisten: (() => void) | null = null;

  $effect(() => {
    if ($saveRequested > 0 && !savingBeforeClose) {
      savingBeforeClose = true;
      handleSave().finally(() => {
        saveRequested.set(0);
        savingBeforeClose = false;
        uiStore.closeDrawer();
      });
    }
  });

  $effect(() => {
    const task = $selectedTask;
    if (task && $drawerOpen && task.id !== editingTaskId) {
      editingTaskId = task.id;
      draftTitle = task.title;
      draftStatus = task.status;
      draftPriority = task.priority;
      draftRecipient = task.recipient ?? "";
      draftDeadline = task.deadline ? task.deadline.split("T")[0] : "";
      originalTitle = task.title;
      originalStatus = task.status;
      originalPriority = task.priority;
      originalRecipient = task.recipient ?? "";
      originalDeadline = task.deadline ? task.deadline.split("T")[0] : "";
      editingCardId = null;
      editingIsNew = false;
      confirmingDeleteCardId = null;
      confirmDiscard = false;
      savingBeforeClose = false;
      cardStore.loadCards(task.id);
    }
  });

  function handleDragDropEvent(event: { payload: { type: string; paths?: string[]; position?: { x: number; y: number } } }) {
    const p = event.payload;
    if (p.type === 'leave') { dropTargetCardId = null; return; }
    if (!p.position) return;
    const el = document.elementFromPoint(p.position.x, p.position.y);
    const cardEl = el?.closest('[data-card-id]') as HTMLElement | null;
    if (!cardEl) { dropTargetCardId = null; return; }
    const cardId = parseInt(cardEl.dataset.cardId!);
    const card = $cards.find(c => c.id === cardId);

    if (p.type === 'enter' || p.type === 'over') {
      dropTargetCardId = card?.card_type === "file" ? cardId : null;
      return;
    }

    if (p.type === 'drop' && p.paths?.length) {
      dropTargetCardId = null;
      if (card?.card_type === "file") {
        handleSaveCard(cardId, { path: p.paths[0], name: "" });
      }
    }
  }

  $effect(() => {
    dropTargetCardId = null;
    dragDropUnlisten?.();
    dragDropUnlisten = null;

    if ($drawerOpen && $selectedTask) {
      let cancelled = false;
      getCurrentWebview().onDragDropEvent(handleDragDropEvent).then(fn => {
        if (cancelled) { fn(); return; }
        dragDropUnlisten = fn;
      });
      return () => {
        cancelled = true;
        dragDropUnlisten?.();
        dragDropUnlisten = null;
        dropTargetCardId = null;
      };
    }
  });

  async function handleSave() {
    const task = $selectedTask;
    if (!task) return;

    const changes: Record<string, unknown> = {};
    if (draftTitle !== task.title) changes.title = draftTitle;
    if (draftStatus !== task.status) changes.status = draftStatus;
    if (draftPriority !== task.priority) changes.priority = draftPriority;
    if (draftRecipient !== (task.recipient ?? "")) changes.recipient = draftRecipient || null;
    const deadlineVal = task.deadline ? task.deadline.split("T")[0] : "";
    if (draftDeadline !== deadlineVal) changes.deadline = draftDeadline || null;

    if (Object.keys(changes).length > 0) {
      await taskStore.update({ id: task.id, ...changes } as unknown as UpdateTaskRequest);
    }
  }

  function discardChangesAndClose() {
    draftTitle = originalTitle;
    draftStatus = originalStatus;
    draftPriority = originalPriority;
    draftRecipient = originalRecipient;
    draftDeadline = originalDeadline;
    uiStore.closeDrawer();
  }

  function handleSaveAndClose() {
    confirmDiscard = false;
    handleSave().then(() => uiStore.closeDrawer());
  }

  function handleDiscardClick() {
    if (confirmDiscard) {
      discardChangesAndClose();
    } else {
      confirmDiscard = true;
      setTimeout(() => { confirmDiscard = false; }, 3000);
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      handleSaveAndClose();
    }
  }

  // Cards
  async function handleAddCard(type: CardType) {
    const task = $selectedTask;
    if (!task) return;
    const created = await cardStore.addCard(task.id, type);
    if (created) {
      editingCardData = { ...created.data };
      editingCardId = created.id;
      editingIsNew = true;
    }
  }

  async function handleSaveCard(id: number, data: CardData) {
    const card = $cards.find((c) => c.id === id);
    if (!card) return;
    await cardStore.saveCard(id, data, $selectedTask!.id, card.card_type);
    editingCardId = null;
  }

  async function handleCancelCard(id: number) {
    if (editingIsNew) {
      await cardStore.deleteCard(id);
    }
    editingCardId = null;
  }

  function handleEditCard(id: number) {
    const card = $cards.find((c) => c.id === id);
    if (card) {
      editingCardData = { ...card.data };
      editingCardId = id;
      editingIsNew = false;
    }
  }

  async function handleDeleteCard(id: number) {
    if (confirmingDeleteCardId === id) {
      await cardStore.deleteCard(id);
      confirmingDeleteCardId = null;
    } else {
      confirmingDeleteCardId = id;
    }
  }

  function handleResetDeleteConfirm() {
    confirmingDeleteCardId = null;
  }

  async function handleToggleTodoItem(cardId: number, itemId: string) {
    await cardStore.toggleTodoItem(cardId, itemId, $selectedTask!.id);
  }

  function handleCardMouseDown(e: MouseEvent, cardId: number) {
    const cardEl = (e.currentTarget as HTMLElement).closest("[data-card-id]") as HTMLElement | null;
    if (!cardEl) return;

    const rect = cardEl.getBoundingClientRect();
    dragSourceId = cardId;

    const clone = cardEl.cloneNode(true) as HTMLElement;
    clone.dataset.ghostOffsetX = String(e.clientX - rect.left);
    clone.dataset.ghostOffsetY = String(e.clientY - rect.top);
    clone.style.position = "fixed";
    clone.style.left = `${rect.left}px`;
    clone.style.top = `${rect.top}px`;
    clone.style.width = `${rect.width}px`;
    clone.style.pointerEvents = "none";
    clone.style.opacity = "0.85";
    clone.style.zIndex = "9999";
    clone.style.boxShadow = "0 4px 12px rgba(0,0,0,0.15)";
    clone.style.transition = "none";
    document.body.appendChild(clone);
    dragGhostEl = clone;

    document.addEventListener("mousemove", handleDragMove);
    document.addEventListener("mouseup", handleDragEnd);
  }

  function handleDragMove(e: MouseEvent) {
    if (!dragGhostEl || dragSourceId === null) return;

    const ghostOffsetX = parseFloat(dragGhostEl.dataset.ghostOffsetX ?? "0");
    const ghostOffsetY = parseFloat(dragGhostEl.dataset.ghostOffsetY ?? "0");
    dragGhostEl.style.left = `${e.clientX - ghostOffsetX}px`;
    dragGhostEl.style.top = `${e.clientY - ghostOffsetY}px`;

    const cardsList = document.querySelector(".cards-list");
    if (!cardsList) { dropIndicatorIndex = -1; return; }

    const cardEls = cardsList.querySelectorAll<HTMLElement>("[data-card-id]");
    const cardRects = Array.from(cardEls, (el) => el.getBoundingClientRect());
    const listRect = cardsList.getBoundingClientRect();

    const gaps: { y: number; index: number }[] = [];
    for (let i = 0; i <= cardRects.length; i++) {
      let y: number;
      if (cardRects.length === 0) {
        y = listRect.top;
      } else if (i === 0) {
        y = cardRects[0].top - 2;
      } else if (i === cardRects.length) {
        y = cardRects[cardRects.length - 1].bottom + 2;
      } else {
        y = (cardRects[i - 1].bottom + cardRects[i].top) / 2;
      }
      gaps.push({ y, index: i });
    }

    let closestDist = Infinity;
    let closestGap = gaps[0];
    for (const gap of gaps) {
      const dist = Math.abs(e.clientY - gap.y);
      if (dist < closestDist) { closestDist = dist; closestGap = gap; }
    }

    const srcIndex = $cards.findIndex((c) => c.id === dragSourceId);
    const targetIndex = closestGap.index;

    cardEls.forEach((el) => el.classList.remove("drag-over", "drag-source"));

    const isOriginal = targetIndex === srcIndex || targetIndex === srcIndex + 1;

    if (isOriginal || srcIndex < 0) {
      dropIndicatorIndex = -1;
      if (srcIndex >= 0) cardEls[srcIndex]?.classList.add("drag-over");
    } else {
      dropIndicatorIndex = targetIndex;
      dropIndicatorTop = closestGap.y - listRect.top + cardsList.scrollTop;
      cardEls[srcIndex]?.classList.add("drag-source");
    }
  }

  async function handleDragEnd() {
    document.removeEventListener("mousemove", handleDragMove);
    document.removeEventListener("mouseup", handleDragEnd);

    const ghostEl = dragGhostEl;
    const sourceId = dragSourceId;
    const targetIdx = dropIndicatorIndex;

    dragGhostEl = null;
    dragSourceId = null;
    dropIndicatorIndex = -1;

    if (ghostEl) document.body.removeChild(ghostEl);

    document.querySelectorAll(".cards-list [data-card-id]").forEach((el) => {
      el.classList.remove("drag-over", "drag-source");
    });

    if (sourceId === null || targetIdx < 0) return;

    const ids = $cards.map((c) => c.id);
    const fromIdx = ids.indexOf(sourceId);
    if (fromIdx < 0) return;

    ids.splice(fromIdx, 1);
    const insertIdx = targetIdx > fromIdx ? targetIdx - 1 : targetIdx;
    ids.splice(insertIdx, 0, sourceId);

    await cardStore.reorderCards(ids, $selectedTask!.id);
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if $drawerOpen && $selectedTask}
  <div class="drawer">
    <div class="drawer-header">
      <span class="drawer-heading">Task Details</span>
      <div class="drawer-header-actions">
        <button class="drawer-discard" class:confirm={confirmDiscard} onclick={handleDiscardClick}>
          {#if confirmDiscard}
            <AlertTriangle size={13} />
            <span>Discard anyway</span>
          {:else}
            <X size={14} />
            <span>Discard</span>
          {/if}
        </button>
      </div>
    </div>

    <div class="drawer-body">
      <!-- Left Column: Metadata -->
      <div class="drawer-col drawer-col-left">
        <input class="drawer-title-input" bind:value={draftTitle} placeholder="Task title" />
        <PropertyEditor
          bind:draftStatus bind:draftPriority
          bind:draftRecipient bind:draftDeadline
        />
        <div class="drawer-meta">
          <span class="drawer-meta-item">Created: {formatDateTime($selectedTask!.create_time)}</span>
          <span class="drawer-meta-item">Updated: {formatDateTime($selectedTask!.update_time)}</span>
        </div>
      </div>

      <!-- Right Column: Cards -->
      <div class="drawer-col drawer-col-cards">
        <div class="col-heading">Cards</div>
        <div class="cards-list">
          {#if $loading}
            <div class="drawer-loading">Loading...</div>
          {:else if $cards.length === 0}
            <div class="cards-empty">No cards yet</div>
          {:else}
            {#each $cards as card}
              {#if editingCardId === card.id}
                <CardEditor
                  cardType={card.card_type}
                  data={editingCardData}
                  isNew={editingIsNew}
                  onSave={(d) => handleSaveCard(card.id, d)}
                  onCancel={() => handleCancelCard(card.id)}
                />
              {:else}
                <CardItem
                  {card}
                  onEdit={handleEditCard}
                  onDelete={handleDeleteCard}
                  confirmingDelete={confirmingDeleteCardId === card.id}
                  onMouseDown={handleCardMouseDown}
                  onToggleTodoItem={handleToggleTodoItem}
                  onResetDeleteConfirm={handleResetDeleteConfirm}
                  dropTarget={dropTargetCardId === card.id}
                />
              {/if}
            {/each}
          {/if}
          {#if dropIndicatorIndex >= 0}
            <div class="drop-indicator" style="top: {dropIndicatorTop}px;"></div>
          {/if}
        </div>
        <CardAdder onAdd={handleAddCard} />
      </div>
    </div>
  </div>
{/if}

<style>
  .drawer {
    width: var(--drawer-width);
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--color-bg-primary);
    overflow: hidden;
  }
  .drawer-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--spacing-md);
    border-bottom: 1px solid var(--color-border);
  }
  .drawer-heading {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--color-text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  .drawer-header-actions {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .drawer-discard {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 3px 10px;
    height: 30px;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-secondary);
    cursor: pointer;
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    font-family: inherit;
    transition: background 0.15s ease, color 0.15s ease, border-color 0.15s ease;
    white-space: nowrap;
  }
  .drawer-discard:hover {
    background: var(--color-bg-tertiary);
    color: #cf222e;
    border-color: #cf222e40;
  }
  .drawer-discard.confirm {
    border-color: #cf222e;
    color: #cf222e;
    background: #fff0ee;
  }
  .drawer-discard.confirm:hover {
    background: #ffdedb;
    border-color: #a0111f;
  }

  .drawer-body {
    flex: 1;
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-template-rows: 1fr;
    overflow: hidden;
  }
  .drawer-col {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
    padding: var(--spacing-md);
    overflow: hidden;
    min-width: 0;
    min-height: 0;
  }
  .drawer-col + .drawer-col {
    border-left: 1px solid var(--color-border);
  }
  .drawer-col-left {
    overflow: visible;
  }

  .col-heading {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--color-text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    padding-bottom: var(--spacing-xs);
    border-bottom: 1px solid var(--color-border);
    flex-shrink: 0;
  }

  .drawer-title-input {
    width: 100%;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    font-size: var(--font-size-base);
    font-weight: 600;
    outline: none;
    padding: 10px 12px;
    border-radius: var(--radius-sm);
    transition: border-color 0.15s ease;
    box-sizing: border-box;
  }
  .drawer-title-input:focus {
    border-color: var(--color-accent);
  }

  .drawer-meta {
    display: flex;
    flex-direction: column;
    gap: 2px;
    font-size: 12px;
    color: var(--color-text-secondary);
    padding-top: var(--spacing-xs);
    border-top: 1px solid var(--color-border);
    margin-top: auto;
  }

  .drawer-col-cards {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
    overflow: hidden;
  }
  .cards-list {
    flex: 1;
    overflow-y: auto;
    min-height: 0;
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 4px 0;
    position: relative;
  }
  :global(.cards-list > [data-card-id].drag-over) {
    outline: 2px solid var(--color-accent);
    outline-offset: -2px;
    border-radius: var(--radius-md);
  }
  :global(.cards-list > [data-card-id].drag-source) {
    opacity: 0.35;
  }
  :global(.cards-list > .drop-indicator) {
    position: absolute;
    left: 0;
    right: 0;
    height: 2px;
    background: var(--color-accent);
    pointer-events: none;
    z-index: 10;
  }
  .cards-empty {
    padding: var(--spacing-lg);
    text-align: center;
    color: var(--color-text-secondary);
    font-size: var(--font-size-sm);
    font-style: italic;
  }
  .drawer-loading {
    padding: var(--spacing-lg);
    text-align: center;
    color: var(--color-text-secondary);
  }
</style>
