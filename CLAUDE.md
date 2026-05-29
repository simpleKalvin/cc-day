# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run Commands

- **Dev server (frontend only):** `pnpm dev` — starts Vite on port 1420
- **Dev (Tauri desktop app):** `pnpm tauri dev` — builds Rust backend + launches frontend via Vite
- **Build for production:** `pnpm tauri build` — compiles TypeScript, bundles frontend, builds Rust release binary
- **Type check:** `tsc --noEmit`
- **Lint:** No linter configured yet

## Architecture

Tauri v2 desktop application with a React frontend and Rust backend.

### Frontend (`src/`)
- React 19 + TypeScript (strict mode) with Vite 7 bundler
- Entry: `src/main.tsx` → `src/App.tsx`
- Uses `@tauri-apps/api` to invoke Rust commands via `invoke()`

### Backend (`src-tauri/`)
- Rust binary crate, lib named `cc_day_lib`
- Tauri commands defined in `src-tauri/src/lib.rs`, registered via `tauri::generate_handler![]`
- `src-tauri/src/main.rs` is the binary entry point — calls `cc_day_lib::run()`
- Config: `src-tauri/tauri.conf.json` (app identity, window settings, CSP, bundle config)
- Capabilities: `src-tauri/capabilities/default.json` (permissions for main window)

### Tauri Command Pattern

New commands follow this pattern:
1. Add `#[tauri::command]` fn in `src-tauri/src/lib.rs`
2. Register it in `tauri::generate_handler![greet, new_command]`
3. Call from frontend: `invoke("command_name", { args })`

### Key Config
- Package manager: **pnpm** (required by `tauri.conf.json` build commands)
- Vite dev port: **1420** (fixed, required by Tauri)
- CSP is disabled (`"csp": null`) — tighten before production
