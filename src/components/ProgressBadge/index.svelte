<script lang="ts">
  let { progress }: { progress: number } = $props();
</script>

<div class="progress-badge">
  <div class="progress-bar">
    <div class="progress-fill" style="width: {progress}%"></div>
  </div>
  <span class="progress-text">{progress}%</span>
</div>

<style>
  .progress-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
  }
  .progress-bar {
    width: 50px;
    height: 6px;
    background: var(--color-bg-tertiary);
    border-radius: 3px;
    overflow: hidden;
  }
  .progress-fill {
    height: 100%;
    background: var(--color-accent);
    transition: width 0.2s;
  }
  .progress-text {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    min-width: 32px;
  }
</style>
