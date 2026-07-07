<script lang="ts">
  import { ChevronLeft, ChevronRight, Calendar } from "@lucide/svelte";
  import { clickOutside } from "$actions/clickOutside";
  import { fly } from "svelte/transition";

  let { value = $bindable("") }: { value: string } = $props();

  let open = $state(false);
  let viewYear = $state(new Date().getFullYear());
  let viewMonth = $state(new Date().getMonth());

  function initView(dateStr: string) {
    if (dateStr) {
      const d = new Date(dateStr + "T00:00:00");
      if (!isNaN(d.getTime())) {
        viewYear = d.getFullYear();
        viewMonth = d.getMonth();
        return;
      }
    }
    const today = new Date();
    viewYear = today.getFullYear();
    viewMonth = today.getMonth();
  }

  function toggleOpen() {
    if (!open) initView(value);
    open = !open;
  }

  function prevMonth() {
    if (viewMonth === 0) { viewYear--; viewMonth = 11; }
    else { viewMonth--; }
  }

  function nextMonth() {
    if (viewMonth === 11) { viewYear++; viewMonth = 0; }
    else { viewMonth++; }
  }

  function selectDate(year: number, month: number, day: number) {
    const m = String(month + 1).padStart(2, "0");
    const d = String(day).padStart(2, "0");
    value = `${year}-${m}-${d}`;
    open = false;
  }

  function handleWindowClick() {
    open = false;
  }

  let calendarDays = $derived.by(() => {
    const firstDay = new Date(viewYear, viewMonth, 1).getDay();
    const daysInMonth = new Date(viewYear, viewMonth + 1, 0).getDate();
    const days: ({ day: number } | null)[] = [];
    for (let i = 0; i < firstDay; i++) days.push(null);
    for (let i = 1; i <= daysInMonth; i++) days.push({ day: i });
    while (days.length % 7 !== 0) days.push(null);
    return days;
  });

  const todayStr = $derived(new Date().toISOString().split("T")[0]);
  const monthName = $derived(new Date(viewYear, viewMonth).toLocaleString("en-US", { month: "long" }));
</script>

<div class="dp-root" use:clickOutside={handleWindowClick}>
  <div class="dp-trigger" onclick={toggleOpen} onkeydown={(e) => { if (e.key === "Enter") toggleOpen(); }} role="button" tabindex="0">
    <span class="dp-icon"><Calendar size={14} /></span>
    <span class="dp-text">{value || "No deadline"}</span>
  </div>
  {#if open}
    <div class="dp-popup" transition:fly={{ duration: 150, y: -4, opacity: 0 }}>
      <div class="dp-header">
        <button class="dp-nav" onclick={prevMonth}><ChevronLeft size={14} /></button>
        <span class="dp-title">{monthName} {viewYear}</span>
        <button class="dp-nav" onclick={nextMonth}><ChevronRight size={14} /></button>
      </div>
      <div class="dp-grid">
        {#each ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] as wd}
          <span class="dp-wd">{wd}</span>
        {/each}
        {#each calendarDays as cell}
          {#if cell}
            {@const dateStr = `${viewYear}-${String(viewMonth + 1).padStart(2, "0")}-${String(cell.day).padStart(2, "0")}`}
            <button
              class="dp-day"
              class:selected={value === dateStr}
              class:today={todayStr === dateStr}
              onclick={() => selectDate(viewYear, viewMonth, cell.day)}
            >{cell.day}</button>
          {:else}
            <span class="dp-empty"></span>
          {/if}
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .dp-root {
    flex: 1;
    position: relative;
  }
  .dp-trigger {
    display: flex;
    align-items: center;
    height: 28px;
    padding: 0 6px;
    border: 1px solid var(--color-border);
    background: var(--color-bg-primary);
    border-radius: var(--radius-sm);
    cursor: pointer;
    gap: 6px;
    transition: border-color 0.15s ease;
    outline: none;
  }
  .dp-trigger:hover {
    border-color: var(--color-text-secondary);
  }
  .dp-trigger:focus {
    border-color: var(--color-accent);
  }
  .dp-icon {
    display: flex;
    align-items: center;
    color: var(--color-text-secondary);
    flex-shrink: 0;
  }
  .dp-text {
    flex: 1;
    font-size: var(--font-size-sm);
    color: var(--color-text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .dp-popup {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    width: 240px;
    background: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-md);
    z-index: 100;
    padding: var(--spacing-sm);
  }
  .dp-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--spacing-sm);
  }
  .dp-nav {
    width: 26px;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: var(--color-text-secondary);
    cursor: pointer;
    border-radius: var(--radius-sm);
    transition: background 0.15s ease, color 0.15s ease;
  }
  .dp-nav:hover {
    background: var(--color-bg-tertiary);
    color: var(--color-text-primary);
  }
  .dp-title {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--color-text-primary);
  }
  .dp-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 2px;
  }
  .dp-wd {
    text-align: center;
    font-size: 11px;
    font-weight: 600;
    color: var(--color-text-secondary);
    padding: 4px 0;
  }
  .dp-day {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 28px;
    border: none;
    background: transparent;
    color: var(--color-text-primary);
    cursor: pointer;
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    transition: background 0.12s ease, color 0.12s ease;
  }
  .dp-day:hover {
    background: var(--color-bg-tertiary);
  }
  .dp-day.today {
    font-weight: 600;
    color: var(--color-accent);
  }
  .dp-day.selected {
    background: var(--color-accent);
    color: #ffffff;
    font-weight: 500;
  }
  .dp-empty {
    height: 28px;
  }
</style>
