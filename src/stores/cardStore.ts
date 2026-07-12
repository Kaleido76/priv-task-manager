import { writable, get } from "svelte/store";
import type { TaskCard, CardData, CardType } from "$types";
import * as api from "$api";

function createCardStore() {
  const cards = writable<TaskCard[]>([]);
  const loading = writable(false);

  async function loadCards(taskId: number) {
    loading.set(true);
    try {
      const result = await api.getTaskCards(taskId);
      cards.set(result);
      console.log(`[Store] cardStore.loadCards => ${result.length} cards`);
    } catch (e) {
      console.error("[Store] cardStore.loadCards error:", e);
    } finally {
      loading.set(false);
    }
  }

  async function addCard(taskId: number, type: CardType, data: CardData = {}): Promise<TaskCard | null> {
    console.log("[Store] cardStore.addCard", taskId, type);
    try {
      const created = await api.createTaskCard(taskId, type, data);
      await loadCards(taskId);
      return created;
    } catch (e) {
      console.error("[Store] cardStore.addCard error:", e);
      return null;
    }
  }

  async function saveCard(id: number, data: CardData, taskId: number, cardType: string) {
    console.log("[Store] cardStore.saveCard", id);
    try {
      let shouldDelete = false;
      if (cardType === "note") shouldDelete = !data.content;
      else if (cardType === "file") shouldDelete = !data.path;
      else if (cardType === "link") shouldDelete = !data.url;
      else if (cardType === "todolist") shouldDelete = !data.items || data.items.length === 0;

      if (shouldDelete) {
        await api.deleteTaskCard(id);
        cards.update(c => c.filter(card => card.id !== id));
      } else {
        await api.updateTaskCard(id, data);
        await loadCards(taskId);
      }
    } catch (e) {
      console.error("[Store] cardStore.saveCard error:", e);
    }
  }

  async function deleteCard(id: number) {
    console.log("[Store] cardStore.deleteCard", id);
    try {
      await api.deleteTaskCard(id);
      cards.update(c => c.filter(card => card.id !== id));
      console.log("[Store] cardStore.deleteCard done");
    } catch (e) {
      console.error("[Store] cardStore.deleteCard error:", e);
    }
  }

  async function reorderCards(ids: number[], taskId: number) {
    console.log("[Store] cardStore.reorderCards");
    try {
      await api.reorderTaskCards(ids);
      await loadCards(taskId);
    } catch (e) {
      console.error("[Store] cardStore.reorderCards error:", e);
    }
  }

  async function toggleTodoItem(cardId: number, itemId: string, taskId: number) {
    console.log("[Store] cardStore.toggleTodoItem", cardId, itemId);
    try {
      const currentCards = get(cards);
      const card = currentCards.find(c => c.id === cardId);
      if (!card || card.card_type !== "todolist" || !card.data.items) return;
      const items = card.data.items.map(i =>
        i.id === itemId ? { ...i, done: !i.done } : i
      );
      await api.updateTaskCard(cardId, { ...card.data, items });
      await loadCards(taskId);
    } catch (e) {
      console.error("[Store] cardStore.toggleTodoItem error:", e);
    }
  }

  function reset() {
    cards.set([]);
    loading.set(false);
  }

  return {
    cards,
    loading,
    loadCards,
    addCard,
    saveCard,
    deleteCard,
    reorderCards,
    toggleTodoItem,
    reset,
  };
}

export const cardStore = createCardStore();
