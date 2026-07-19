# TaskManager — Agent Guide

## Project Overview

A portable desktop task management app built with **Tauri v2** + **Svelte 5** + **SvelteKit 2** + **SQLite (rusqlite)**.

Users create **Projects** (sidebar) and manage **Tasks** (table + drawer) under each project. Features inline rename, per-column filter/sort with popups, deadline helper capsules, notes, activity logs, batch operations, and custom colored dropdown/picker components.

## Tech Stack

| Layer | Technology | Notes |
|---|---|---|
| Desktop Shell | Tauri v2 (Rust) | WebView-based, portable |
| Frontend | Svelte 5 + SvelteKit 2 + TypeScript | SPA mode (`adapter-static`, `ssr=false`) |
| Styling | Plain CSS with CSS custom properties | No Tailwind/Bootstrap. `src/styles/` |
| Icons | `@lucide/svelte` | GitHub-style consistent icon set |
| Database | SQLite via rusqlite 0.31 | `bundled` feature, WAL mode |
| State | Svelte `writable/derived` stores | `src/stores/` |
| Today Store | `todayStore.ts` | YYYY-MM-DD string, auto-refreshes at midnight |
| IPC | `@tauri-apps/api/core` `invoke()` | Frontend ↔ Rust commands |
| Open/Launch | `tauri-plugin-opener` / `@tauri-apps/plugin-opener` | Registered in lib.rs + capabilities |
| Build | Vite 6 (frontend) + Cargo (backend) | |

## Portability (Critical!)

The app is **portable**: `taskmanager.db` is created next to the executable (`std::env::current_exe().parent()`), **not** in AppData. Never use `app.path().app_data_dir()` or any OS-specific directory.

If you need to reset the database, delete `taskmanager.db` in the exe directory.

**Known portability bug**: `open_file_location` and `open_url` in `card_cmds.rs` use Windows-only commands (`explorer` / `cmd /c start`). The `tauri-plugin-opener` is already a dependency but not used for these. To fix, replace with `tauri_plugin_opener::open_path()` / `open_url()`.

## Project Structure

```
TaskManager/
├── src/                          # Frontend (SvelteKit)
│   ├── app.html                  # Root HTML shell
│   ├── routes/
│   │   ├── +layout.svelte        # Global layout — imports variables.css + global.css
│   │   ├── +layout.ts            # SSR disabled
│   │   └── +page.svelte          # Shell (imports ProjectHeader, Toolbar, TaskTable, etc.)
│   ├── components/
│   │   ├── Sidebar/              # Project list + create (card-style items)
│   │   │   ├── index.svelte      # Sidebar shell + "+" button
│   │   │   └── ProjectItem.svelte # Project card with name, colored dot, delete
│   │   ├── ProjectHeader/        # Project name, description, color picker (8 swatches)
│   │   │   └── index.svelte      # Inline edit, color dot + popup, created/updated meta
│   │   ├── Toolbar/              # Global search (+ New Task)
│   │   ├── TaskTable/            # 7-column grid + column header buttons + filter popups
│   │   ├── TaskDrawer/           # Detail panel: title, properties, cards (file/note/link/todolist)
│   │   │   ├── index.svelte      # Drawer shell, cards list, drag-drop listener, auto-save
│   │   │   ├── CardItem.svelte   # Read-only card display with validation, drop target
│   │   │   ├── CardEditor.svelte # Inline card editor with todolist keyboard nav
│   │   │   ├── CardAdder.svelte  # "+" button to add new cards
│   │   │   ├── DatePicker.svelte # Custom calendar popup for deadline
│   │   │   └── PropertyEditor.svelte # Colored dropdowns for status/priority/recipient/deadline
│   │   └── StatusBar/            # Project name, counts, save status
│   ├── config/
│   │   ├── taskConfig.ts         # statusCfg, priorityCfg, deadline capsule colors
│   │   ├── locale.ts             # Locale constants (e.g., short date format "yyyy-MM-dd")
│   │   └── index.ts              # Barrel export
│   ├── actions/
│   │   └── clickOutside.ts       # clickOutside Svelte action
│   ├── stores/
│   │   ├── projectStore.ts       # Project CRUD, selectedId
│   │   ├── taskStore.ts          # Task CRUD + searchKeyword + batch select
│   │   ├── cardStore.ts          # Card CRUD + reorder
│   │   ├── uiStore.ts            # Drawer, saveStatus, statusMessage, saveRequested counter
│   │   ├── todayStore.ts         # YYYY-MM-DD string, auto-refreshes at midnight
│   │   └── index.ts              # Barrel export
│   ├── api/
│   │   ├── project.ts            # invoke("get_projects"), etc.
│   │   ├── task.ts               # invoke("get_tasks"), etc.
│   │   └── index.ts              # Barrel export
│   ├── types/
│   │   ├── task.ts               # Task, CardData, TodoItem, CardType, TaskStatus/Priority enums
│   │   ├── project.ts            # Project interface
│   │   └── index.ts              # Barrel export
│   ├── utils/
│   │   ├── index.ts              # formatDate, formatDateTime, nowISO
│   │   └── autoSave.ts           # Debounced auto-save utility
│   └── styles/
│       ├── variables.css         # :root CSS custom properties (colors, spacing, etc.)
│       └── global.css            # Resets, scrollbar, base html/body
├── src-tauri/                    # Backend (Rust)
│   ├── src/
│   │   ├── lib.rs                # Tauri setup, register 22 commands, opener plugin
│   │   ├── main.rs               # Entry point
│   │   ├── database/
│   │   │   ├── mod.rs            # Database struct, init(), PRAGMAs
│   │   │   └── migrations/
│   │   │       ├── mod.rs        # Versioned migration runner (PRAGMA user_version)
│   │   │       └── 001_init.sql  # Final schema (projects with description/update_time, tasks with recipient, no progress)
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── task.rs           # Task, TaskContent, TaskLog (enums removed — plain strings on Rust side)
│   │   │   └── project.rs        # Project
│   │   ├── repository/           # Raw SQL queries
│   │   │   ├── mod.rs
│   │   │   ├── project_repo.rs   # CRUD + sort_order
│   │   │   ├── task_repo.rs      # CRUD + search + content upsert + batch ops
│   │   │   ├── task_log_repo.rs  # CRUD by task_id
│   │   │   └── card_repo.rs      # CRUD + reorder for task_cards
│   │   ├── services/             # Business logic
│   │   │   ├── mod.rs
│   │   │   ├── project_service.rs
│   │   │   ├── task_service.rs
│   │   │   └── card_service.rs
│   │   ├── commands/             # #[tauri::command] handlers
│   │   │   ├── mod.rs
│   │   │   ├── project_cmds.rs
│   │   │   ├── task_cmds.rs
│   │   │   └── card_cmds.rs
│   │   └── utils/
│   │       └── mod.rs            # deserialize_some helper for nullable Option fields
│   ├── build.rs
│   ├── capabilities/
│   │   └── default.json          # Permissions: core:default + opener:default
│   ├── Cargo.toml
│   └── tauri.conf.json
├── static/
│   └── favicon.png
├── package.json
├── svelte.config.js              # adapter-static, path aliases ($api, $stores, $config, $actions, etc.)
├── tsconfig.json
└── vite.config.js
```

## Path Aliases (svelte.config.js)

| Alias | Path |
|---|---|
| `$api` | `./src/api` |
| `$stores` | `./src/stores` |
| `$components` | `./src/components` |
| `$types` | `./src/types` |
| `$utils` | `./src/utils` |
| `$styles` | `./src/styles` |
| `$config` | `./src/config` |
| `$actions` | `./src/actions` |

## Architecture: Data Flow

```
User Click → Svelte Component → Store method → API fn → invoke("cmd_name", args)
                                                              ↓
Tauri IPC bridge                                       Rust #[tauri::command]
                                                              ↓
API fn returns ← invoke resolves ←      Service layer → Repository → SQLite
```

## Backend: Rust Command Pattern

All commands follow this pattern in `commands/`:

```rust
#[tauri::command]
pub fn some_command(db: tauri::State<Database>, param: Type) -> Result<ReturnType, String> {
    service::some_fn(db.inner(), param)
}
```

- `db: tauri::State<Database>` is injected by Tauri, no need to pass from frontend.
- Frontend invoke uses `camelCase` keys; Tauri auto-converts to `snake_case` for Rust.
- All errors are `String` (mapped via `.map_err(|e| e.to_string())`).

## Database Schema

See `src-tauri/src/database/migrations/001_init.sql`. Key tables:

- **projects** — `id, name, description, color, sort_order, create_time, update_time`
- **tasks** — `id, project_id, title, status, priority, recipient, deadline, create_time, update_time`
- **task_contents** — `task_id (PK), note` (upsert pattern)
- **task_logs** — `id, task_id, content, create_time`
- **task_cards** — `id, task_id, sort_order, card_type, data (JSON), create_time, update_time`

Migrations use `PRAGMA user_version` for version tracking. See `migrations/mod.rs`.

## Key Components

### DatePicker (`src/components/TaskDrawer/DatePicker.svelte`)
Custom calendar popup replacing native `<input type="date">`. Supports month navigation, today/selected highlighting, hover states, and click-outside close. Used via `<DatePicker bind:value={draftDeadline} />`.

### PropertyEditor (`src/components/TaskDrawer/PropertyEditor.svelte`)
Custom button+popup widgets replacing native `<select>` for Status and Priority. Each option rendered as a colored capsule dot + label. Popup uses Svelte `transition:fade` for smooth enter/exit. Recipient is a plain text input; Deadline uses the DatePicker.

### TaskTable (`src/components/TaskTable/`)
7-column CSS Grid layout (checkbox + 6 data columns). Columns are hardcoded with specific widths and text-align values — no `columns` prop (to prevent index mismatch). Each column header has three **circular buttons** ([Sort] [Filter] [Clear]) that appear on hover and straddle the header's top border. Active buttons turn accent blue with white icon. Filter popups use `position: fixed` (JS-anchored from button `getBoundingClientRect`) to avoid clipping. Sorting and filtering are **pure frontend** (local state in TaskTable, no store or backend calls). The `__deadline_help` column shows colored capsules ("3 Day" green / "Today" orange / "-2 Day" red) based on days until deadline; hidden for done/cancelled tasks. Capsules auto-refresh at midnight via `todayStore`.

### TaskDrawer (`src/components/TaskDrawer/`)
Floating fixed-position 840px panel with backdrop. Two-column grid layout: metadata (title, status/priority/recipient/deadline via PropertyEditor, created/updated timestamps) left, cards right. Edits use a local draft state with `saveRequested` store signal for auto-save.

**Save/Discard**: Merged X+Discard button (2‑step confirm: first click shows AlertTriangle + "Discard anyway", second reverts and closes; 3s auto-reset). Escape saves metadata then closes. Backdrop click saves via `saveRequested` counter signal (`$effect` watches it, resets to 0 after consumption).

**Cards system**: Four card types — **file** (path+name, double-click opens file location), **note** (full-height text, selectable without editing, no line‑clamp), **link** (url+name, double-click opens with auto-`https://`), **todolist** (checklist items with inline toggle). Cards are flex items with `flex-shrink: 0` and `overflow: hidden` (corner clipping). Editor replaces card inline with keyboard nav (todolist: Enter/Up/Down). Adder button at column bottom. Dragging card headers reorders via `reorderTaskCards`. External file drag-drop onto File Cards uses Tauri's `getCurrentWebview().onDragDropEvent()` — highlights with `document.elementFromPoint()` hit‑test, replaces path + clears name on drop. Validation: file path checked via `check_path_exists` Rust command, link URL via regex; invalid cards show yellow `AlertTriangle`.

**Card scrolling**: `.drawer-body` uses `grid-template-rows: 1fr` + `min-height: 0` on grid columns, `.cards-list` uses `flex: 1; overflow-y: auto`, cards use `flex-shrink: 0` to prevent compression when content overflows.

### CardEditor (`src/components/TaskDrawer/CardEditor.svelte`)
Inline edit card for all four types. Todolist editor: keyed `{#each}` items with `bind:this` refs, keyboard navigation (Enter inserts new item after current, Up/Down moves focus). `canSave` derived (checks non-empty content for the card type), `showDeleteLabel` when existing content fully cleared (destroys card on save).

### CardItem (`src/components/TaskDrawer/CardItem.svelte`)
Read-only card display. File/Link: click‑hint "Double‑click to open…" left, validation warning right in `.card-footer`. Todolist: inline checkbox toggle calls `onToggleTodoItem`. Action buttons (edit, delete with 2‑step confirm) positioned absolute in `.card-body`. Delete confirm resets on `onmouseleave`. Accepts `dropTarget` boolean prop for external drag highlight (accent outline).

### Batch Operations
Checkbox column enables multi-select. When `selectedTaskIds.size > 0`, row click toggles selection instead of opening drawer. Floating action bar (`transition:fade`) with Select All / Clear / Delete (single-step) / Move to Project (popup with project list). Backend `delete_tasks` / `move_tasks` accept `Vec<i64>`.

### Sidebar (`src/components/Sidebar/`)
Card-style project items with white background, border, rounded corners, and hover shadow. Skip reload when clicking already-selected project. "+" button creates a project with default name "New Project". Static colored dot (non-interactive) — color editing is in ProjectHeader.

### ProjectHeader (`src/components/ProjectHeader/`)
Inline-editable project name + description. Color dot with **clickable popup** showing 8 swatches + no-color option. Selection saves via `projectStore.updateColor`. Created/Updated timestamps shown.

## UI Conventions

- **Capsule tags**: Status and Priority rendered as rounded pills with dark backgrounds and white text (high contrast). Color config centralized in `src/config/taskConfig.ts`. Used in both TaskTable and PropertyEditor dropdown.
- **Deadline helper**: Separate `__deadline_help` column with text-only capsule ("3 Day" green / "Today" orange / "-2 Day" red). Only rendered for active tasks (not done/cancelled). Column header is non-interactive (`toggleSort` returns early).
- **Date-boundary auto-refresh**: `getDeadlineCapsule` in `taskConfig.ts` and `todayStr` in `DatePicker.svelte` derive from `todayStore` (a Svelte writable storing `YYYY-MM-DD`). A `setTimeout` schedules the next midnight and updates the store, ensuring deadline capsules and today highlighting refresh automatically when the date changes, even if the app stays open across midnight.
- **Done/Cancelled tasks**: Title gets `text-decoration: line-through; opacity: 0.65`; row has `opacity: 0.65`.
- **Svelte transitions**: `transition:fly` / `transition:fade` for bidirectional enter/exit animations (drawer, backdrop, color picker, date picker, batch bar).
- **Global UI sizing**: Increased font sizes (+1px base), spacing (+1–2px), sidebar 250px, toolbar 46px, statusbar 30px, drawer 840px. Font-size variables: `--font-size-sm` 15px, `--font-size-base` 16px, `--font-size-lg` 18px. Spacing: xs 7px, sm 12px, md 17px, lg 22px.
- **Uniform padding**: `--spacing-lg` on Toolbar, StatusBar, project header, drawer body.
- **Input sizing**: `.property-input`/`.sel-btn`/`.dp-trigger` 34px height, `.editor-input` 32px, `.todo-editor-input` 30px; horizontal padding 8–12px. Card padding: header `6px 12px`, body `0 12px 10px`, drawer title input `10px 12px`.
- **Card action buttons**: 22×22px, positioned `absolute; top: 0; right: 4px` inside `.card-body`, visible on card hover.

## Known Gotchas

- `<option value={null}>` in Svelte 5 may serialize to string `"null"` — prefer using a sentinel string like `""` or `"__all__"` for "all" options.
- Lucide Svelte icons (`@lucide/svelte`) don't forward scoped CSS classes to their SVG root — wrap in `<span class="...">` instead of `class="..."` on the component.
- CSS custom properties are defined in `variables.css` and must be imported via `+layout.svelte`.
- The app ONLY works with `adapter-static` in SPA mode; SSR is disabled.
- All Tauri command parameters must be owned types (`Option<String>` not `Option<&str>`).
- When using `bundled` SQLite, the feature flag must be on in `Cargo.toml`.
- TypeScript has `TaskStatus`/`Priority` **string-valued enums**; the Rust side uses plain strings (not enums).
- `formatDateTime` shows date + minutes; `formatDate` shows date-only (used for deadlines).
- Deadline helper `__deadline_help` column header is non-interactive — `toggleSort` returns early.
- **Tauri drag-drop**: Native HTML5 `drop`/`dragover` events don't fire for file drops because Tauri v2 intercepts them at the window level (`dragDropEnabled` defaults to `true`). Use `getCurrentWebview().onDragDropEvent()` from `@tauri-apps/api/webview` instead. Hit-test with `document.elementFromPoint(position)` to find the target element.
- **Flex compression**: `overflow: hidden` on a flex item (`.card-item`) makes its main-axis overflow non-`visible`, causing `min-height: auto` to compute to `0`. Cards then shrink proportionally instead of scrolling. Fix: `flex-shrink: 0` prevents compression, and `overflow-y: auto` on the parent scroll container handles overflow.
- **Card column scroll**: The card column needs `grid-template-rows: 1fr` on `.drawer-body` (forces grid row to container height) AND `min-height: 0` on the grid column (overrides `min-height: auto`). Without both, the column expands to content height and `.cards-list` never scrolls.
