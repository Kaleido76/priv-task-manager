# TaskManager — Agent Guide

## Project Overview

A portable desktop task management app built with **Tauri v2** + **Svelte 5** + **SvelteKit 2** + **SQLite (rusqlite)**.

Users create **Projects** (sidebar) and manage **Tasks** (table + drawer) under each project. Features inline rename, search/filter, deadline with helper capsules, notes, activity logs, batch operations, and custom colored dropdown/picker components.

## Tech Stack

| Layer | Technology | Notes |
|---|---|---|
| Desktop Shell | Tauri v2 (Rust) | WebView-based, portable |
| Frontend | Svelte 5 + SvelteKit 2 + TypeScript | SPA mode (`adapter-static`, `ssr=false`) |
| Styling | Plain CSS with CSS custom properties | No Tailwind/Bootstrap. `src/styles/` |
| Icons | `@lucide/svelte` | GitHub-style consistent icon set |
| Database | SQLite via rusqlite 0.31 | `bundled` feature, WAL mode |
| State | Svelte `writable/derived` stores | `src/stores/` |
| IPC | `@tauri-apps/api/core` `invoke()` | Frontend ↔ Rust commands |
| Build | Vite 6 (frontend) + Cargo (backend) | |

## Portability (Critical!)

The app is **portable**: `taskmanager.db` is created next to the executable (`std::env::current_exe().parent()`), **not** in AppData. Never use `app.path().app_data_dir()` or any OS-specific directory.

If you need to reset the database, delete `taskmanager.db` in the exe directory.

## Project Structure

```
TaskManager/
├── src/                          # Frontend (SvelteKit)
│   ├── app.html                  # Root HTML shell
│   ├── routes/
│   │   ├── +layout.svelte        # Global layout — imports variables.css + global.css
│   │   ├── +layout.ts            # SSR disabled
│   │   └── +page.svelte          # Single-page app shell
│   ├── components/
│   │   ├── Sidebar/              # Project list + create (card-style items)
│   │   ├── Toolbar/              # Search, filters, "+ New Task"
│   │   ├── TaskTable/            # Sortable 7-column table + TaskRow (capsule tags, helper col)
│   │   ├── TaskDrawer/           # Detail panel: title, properties (custom dropdowns), notes, logs
│   │   ├── StatusBar/            # Project name, counts, save status
│   │   ├── StatusBadge/          # (unused) Colored status badge
│   │   └── ProgressBadge/        # (unused) Progress bar
│   ├── stores/
│   │   ├── projectStore.ts       # Project CRUD, selectedId
│   │   ├── taskStore.ts          # Task CRUD + search/filter + batch select
│   │   └── uiStore.ts            # Drawer, saveStatus, activeTab
│   ├── api/
│   │   ├── project.ts            # invoke("get_projects"), etc.
│   │   └── task.ts               # invoke("get_tasks"), etc.
│   ├── types/
│   │   ├── task.ts               # Task, TaskStatus, Priority, UpdateTaskRequest
│   │   └── project.ts            # Project interface
│   ├── utils/
│   │   └── index.ts              # formatDate, formatDateTime, nowISO
│   └── styles/
│       ├── variables.css         # :root CSS custom properties (colors, spacing, etc.)
│       └── global.css            # Resets, scrollbar, base html/body
├── src-tauri/                    # Backend (Rust)
│   ├── src/
│   │   ├── lib.rs                # Tauri setup, register commands
│   │   ├── main.rs               # Entry point
│   │   ├── database/
│   │   │   ├── mod.rs            # Database struct, init(), PRAGMAs
│   │   │   └── migrations/
│   │   │       ├── mod.rs        # Schema init (runs 0001_init.sql)
│   │   │       └── 0001_init.sql # Final schema (projects with description/update_time, tasks with recipient, no progress)
│   │   ├── models/
│   │   │   ├── task.rs           # Task, TaskContent, TaskLog, TaskStatus, Priority
│   │   │   └── project.rs        # Project
│   │   ├── repository/           # Raw SQL queries
│   │   │   ├── project_repo.rs   # CRUD + sort_order
│   │   │   ├── task_repo.rs      # CRUD + search + content upsert + batch ops
│   │   │   └── task_log_repo.rs  # CRUD + delete
│   │   ├── services/             # Business logic
│   │   │   ├── project_service.rs
│   │   │   └── task_service.rs
│   │   ├── commands/             # #[tauri::command] handlers
│   │   │   ├── project_cmds.rs
│   │   │   └── task_cmds.rs
│   │   ├── config/
│   │   │   └── mod.rs            # Empty
│   │   └── utils/
│   │       └── mod.rs            # Empty
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
├── svelte.config.js              # adapter-static, path aliases ($api, $stores, etc.)
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

See `src-tauri/src/database/migrations/0001_init.sql`. Key tables:

- **projects** — `id, name, description, color, sort_order, create_time, update_time`
- **tasks** — `id, project_id, title, status, priority, recipient, deadline, create_time, update_time`
- **task_contents** — `task_id (PK), note` (upsert pattern)
- **task_logs** — `id, task_id, content, create_time`

The schema is applied once at startup via `CREATE TABLE IF NOT EXISTS`. There is no migration versioning — the `0001_init.sql` is the single source of truth for the database layout.

## Logging System

All key function calls log at both layers:

| Layer | Prefix | Output |
|---|---|---|
| Rust backend | `[DB]` / `[CMD]` | stdout (visible in terminal) |
| Frontend API | `[API]` | Browser DevTools console |
| Frontend Store | `[Store]` | Browser DevTools console |
| Frontend Component | `[Page]` / `[Sidebar]` / `[Toolbar]` | Browser DevTools console |

To trace an interaction, watch the terminal for `[CMD]` and DevTools for `[API]` / `[Store]`.

## Development Workflow

```bash
# Install dependencies
pnpm install

# Start Tauri dev (with hot-reload)
pnpm tauri dev

# Build for production
pnpm tauri build

# Frontend-only dev (no Tauri backend)
pnpm dev

# Type-check frontend
pnpm check

# Build frontend (outputs to build/)
pnpm build
```

## How to Add a New Feature

1. **Database**: Edit `src-tauri/src/database/migrations/0001_init.sql` to add/alter tables. The schema is applied via `CREATE TABLE IF NOT EXISTS` — for column changes, edit the SQL directly (dev-only, no migration versioning).
2. **Rust model**: Define struct in `src-tauri/src/models/` with `Serialize + Deserialize`.
3. **Repository**: Add SQL queries in `src-tauri/src/repository/`.
4. **Service**: Business logic in `src-tauri/src/services/`.
5. **Command**: `#[tauri::command]` in `src-tauri/src/commands/`, register in `lib.rs` `generate_handler![]`.
6. **Frontend API**: Add `invoke()` wrapper in `src/api/`.
7. **Frontend Store**: State management in `src/stores/`.
8. **Component**: UI in `src/components/`.

## Key Components

### DatePicker (`src/components/TaskDrawer/DatePicker.svelte`)
Custom calendar popup replacing native `<input type="date">`. Supports month navigation, today/selected highlighting, hover states, and click-outside close. Used via `<DatePicker bind:value={draftDeadline} />`.

### PropertyEditor (`src/components/TaskDrawer/PropertyEditor.svelte`)
Custom button+popup widgets replacing native `<select>` for Status and Priority. Each option rendered as a colored capsule dot + label. Popup uses Svelte `transition:fade` for smooth enter/exit. Recipient is a plain text input; Deadline uses the DatePicker.

### TaskTable (`src/components/TaskTable/`)
7-column CSS Grid layout (checkbox + 6 data columns). Columns are hardcoded with specific widths and text-align values — no `columns` prop (to prevent index mismatch). Header cells are sortable by clicking (except `__deadline_help` which is an empty-label non-sortable spacer). The `__deadline_help` column shows colored capsules ("3 Day" green / "Today" orange / "2 Day" red) based on days until deadline; hidden for done/cancelled tasks.

### TaskDrawer (`src/components/TaskDrawer/`)
Floating fixed-position panel with backdrop. Edits use a local draft state; save button with `saving→saved→idle` animation does NOT close drawer. Discard on Escape/backdrop/× click. Notes section: read-only display with hover Pencil edit toggle; Trash2 shows red AlertTriangle confirm on first click, clears on second. Logs each have a two-step delete button. Read-only created/updated timestamps displayed in a `.drawer-meta` divider.

### Batch Operations
Checkbox column enables multi-select. When `selectedTaskIds.size > 0`, row click toggles selection instead of opening drawer. Floating action bar (`transition:fade`) with Select All / Clear / Delete (single-step) / Move to Project (popup with project list). Backend `delete_tasks` / `move_tasks` accept `Vec<i64>`.

### Sidebar (`src/components/Sidebar/`)
Card-style project items with white background, border, rounded corners, and hover shadow. Skip reload when clicking already-selected project. "+" button creates a project with default name "New Project". Clicking color dot opens a popup with 8 swatches + no-color option; selection saves via `updateProjectColor`.

## UI Conventions

- **Capsule tags**: Status and Priority rendered as rounded pills with GitHub-style color pairs (bg/fg). Used in both TaskTable and PropertyEditor dropdown.
- **Deadline helper**: Separate `__deadline_help` column with text-only capsule ("3 Day" green / "Today" orange / "-2 Day" red). Only rendered for active tasks (not done/cancelled).
- **Done/Cancelled tasks**: Title gets `text-decoration: line-through; opacity: 0.65`; row has `opacity: 0.65`.
- **Svelte transitions**: `transition:fly` / `transition:fade` for bidirectional enter/exit animations (drawer, backdrop, color picker, date picker, batch bar).
- **Global UI sizing**: Increased font sizes (+1px base), spacing (+1–2px), sidebar 240px, toolbar 44px, statusbar 28px, drawer 420px.
- **Uniform padding**: `--spacing-lg` on Toolbar, StatusBar, project header, drawer body.

## Known Gotchas

- `<option value={null}>` in Svelte 5 may serialize to string `"null"` — prefer using a sentinel string like `""` or `"__all__"` for "all" options.
- Lucide Svelte icons (`@lucide/svelte`) don't forward scoped CSS classes to their SVG root — wrap in `<span class="...">` instead of `class="..."` on the component.
- CSS custom properties are defined in `variables.css` and must be imported via `+layout.svelte`.
- The app ONLY works with `adapter-static` in SPA mode; SSR is disabled.
- All Tauri command parameters must be owned types (`Option<String>` not `Option<&str>`).
- When using `bundled` SQLite, the feature flag must be on in `Cargo.toml`.
- Unused legacy code: `TaskStatus`/`Priority` Rust enums, `get_task` service fn, `StatusBadge`/`ProgressBadge` components — can be removed or refactored.
- `ProjectItem.svelte:14` `$state(project.name)` captures only initial prop value (pre-existing Svelte 5 warning).
- `formatDateTime` shows date + minutes; `formatDate` shows date-only (used for deadlines).
- Deadline helper `__deadline_help` column has an empty label, `tabindex=-1`, and `toggleSort` returns early — it is non-interactive.
