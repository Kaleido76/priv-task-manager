<script lang="ts">
  let { text = "", onSave }: { text?: string; onSave: () => void } = $props();

  let timeout: ReturnType<typeof setTimeout>;

  function handleInput() {
    clearTimeout(timeout);
    timeout = setTimeout(() => onSave(), 500);
  }
</script>

<textarea
  bind:value={text}
  oninput={handleInput}
  class="text-editor"
></textarea>

<style>
  .text-editor {
    width: 100%;
    min-height: 200px;
    border: 1px solid var(--color-border);
    border-radius: 4px;
    padding: var(--spacing-sm);
    resize: vertical;
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    font-family: inherit;
    font-size: var(--font-size-base);
    line-height: 1.5;
  }
  .text-editor:focus {
    outline: none;
    border-color: var(--color-accent);
  }
</style>
