# 项目重命名：MeoClawTuari → MeoClaw Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rename the project from `MeoClawTuari` to `MeoClaw` across all source files, config, docs, and directory structure, preserving git history.

**Architecture:** Pure find-and-replace rename. No logic changes. Build-critical files first, then docs. Folder rename happens last after all internal references are updated, using `git mv` to preserve history.

**Tech Stack:** Tauri v2, Rust, Vue/TypeScript, npm, Git

**Naming Map:**

| Old | New |
|---|---|
| `MeoClawTuari` (product name) | `MeoClaw` |
| `meo-claw-tuari` (npm/pkg builderID/crate) | `meo-claw` |
| `com.meoclawtuari.app` (identifier) | `com.meoclaw.app` |
| `meo_claw_tuadi_lib` (Rust lib, fix typo) | `meo_claw_lib` |
| `~/.local/share/meo-claw-tuari/` (data dir) | `~/.local/share/meo-claw/` |
| `~/Library/Logs/com.meoclawtuari.app/` (log dir) | `~/Library/Logs/com.meoclaw.app/` |
| `meo-claw-tuari` (process/command pattern) | `meo-claw` |

---

### Task 1: Core config files (build-critical)

**Files:**
- Modify: `package.json:2` — npm package name
- Modify: `src-tauri/tauri.conf.json:3,5,16` — productName, identifier, window title
- Modify: `src-tauri/Cargo.toml:2,9` — crate name, lib name
- Modify: `index.html:7` — HTML title

- [ ] **Step 1: Edit package.json**
  Replace `"name": "meo-claw-tuari"` with `"name": "meo-claw"` in `package.json:2`

- [ ] **Step 2: Edit src-tauri/tauri.conf.json**
  3 changes:
  - `"productName": "MeoClawTuari"` → `"productName": "MeoClaw"`
  - `"identifier": "com.meoclawtuari.app"` → `"identifier": "com.meoclaw.app"`
  - `"title": "MeoClawTuari"` → `"title": "MeoClaw"`

- [ ] **Step 3: Edit src-tauri/Cargo.toml**
  2 changes:
  - `name = "meo-claw-tuari"` → `name = "meo-claw"`
  - `name = "meo_claw_tuadi_lib"` → `name = "meo_claw_lib"`

- [ ] **Step 4: Edit index.html**
  `<title>MeoClawTuari</title>` → `<title>MeoClaw</title>`

---

### Task 2: Rust source files (build-critical)

**Files:**
- Modify: `src-tauri/src/main.rs:5` — lib import
- Modify: `src-tauri/src/lib.rs:645,1139` — window title, tooltip
- Modify: `src-tauri/src/backend/manager.rs:130` — data dir path
- Modify: `src-tauri/tests/backend_manager_tests.rs:3` — lib import

- [ ] **Step 1: Edit src-tauri/src/main.rs**
  `meo_claw_tuadi_lib::run()` → `meo_claw_lib::run()`

- [ ] **Step 2: Edit src-tauri/src/lib.rs**
  2 changes:
  - `.title("MeoClaw 选项")` → no change needed (already "MeoClaw")
  - `.tooltip("MeoClawTuari")` → `.tooltip("MeoClaw")`

- [ ] **Step 3: Edit src-tauri/src/backend/manager.rs**
  `.join("meo-claw-tuari")` → `.join("meo-claw")` (Tauri v2 app data dir)

- [ ] **Step 4: Edit src-tauri/tests/backend_manager_tests.rs**
  `use meo_claw_tuadi_lib::backend::` → `use meo_claw_lib::backend::`

- [ ] **Step 5: Verify Rust builds**
  Run: `cargo check --package meo-claw` from `src-tauri/`
  Expected: Compilation succeeds

---

### Task 3: Agent configs + README

**Files:**
- Modify: `AGENTS.md:81-82,105` — screencapture pattern, pkill pattern
- Modify: `CLAUDE.md:62` — screencapture pattern
- Modify: `README.md:1,101,147,231` — title, tree, path, script

- [ ] **Step 1: Edit AGENTS.md**
  3 changes:
  - `owner == "meo-claw-tuari" && name == "MeoClawTuari"` → `owner == "meo-claw" && name == "MeoClaw"`
  - `pkill -f "meo-claw"` (line 105) — no change needed (already `meo-claw`)

- [ ] **Step 2: Edit CLAUDE.md**
  Same screencapture pattern as AGENTS.md:81-82, update `owner`/`name` strings.

- [ ] **Step 3: Edit README.md**
  4 changes:
  - `# MeoClawTuari 桌宠助手` → `# MeoClaw 桌宠助手`
  - Directory tree: `MeoClawTuari/` → `MeoClaw/`
  - `~/.local/share/meo-claw-tuari/backend-config.json` → `~/.local/share/meo-claw/backend-config.json`
  - Screencapture script: update `owner == "meo-claw-tuari" && name == "MeoClawTuari"`

---

### Task 4: DOC/01-SPEC documentation (7 files)

**Files (all under `DOC/01-SPEC/`):**
- Modify: `DOC/README.md:1` — title
- Modify: `DOC/01-SPEC/01-产品定义/桌宠助手PRD.md:287,359` — config path, tree
- Modify: `DOC/01-SPEC/02-架构设计/架构总览.md:5,102` — description, config path
- Modify: `DOC/01-SPEC/02-架构设计/状态管理.md:5,110,209` — description, config paths
- Modify: `DOC/01-SPEC/02-架构设计/多后端管理.md:5,93` — description, config path
- Modify: `DOC/01-SPEC/03-接口规范/事件系统.md:3` — description
- Modify: `DOC/01-SPEC/03-接口规范/接口总览.md:3` — description
- Modify: `DOC/01-SPEC/03-接口规范/窗口通信.md:3,76` — description
- Modify: `DOC/01-SPEC/05-UI设计/窗口配置.md:7` — window title
- Modify: `DOC/01-SPEC/06-流程规范/调试流程.md:67,84,87,103` — script, pkill
- Modify: `DOC/01-SPEC/06-流程规范/测试流程-文件监控方式.md:25` — pkill

All changes are find-and-replace of:
- `MeoClawTuari` → `MeoClaw`
- `meo-claw-tuari` → `meo-claw`
- `com.meoclawtuari.app` → `com.meoclaw.app`

- [ ] **Step 1: Bulk replace in DOC/01-SPEC**
  Run from project root:
  ```
  rg -l "MeoClawTuari|meo-claw-tuari|com.meoclawtuari" DOC/01-SPEC/ --type md | while read f; do
    sed -i '' 's/MeoClawTuari/MeoClaw/g; s/meo-claw-tuari/meo-claw/g; s/com\.meoclawtuari\.app/com.meoclaw.app/g' "$f"
  done
  ```

---

### Task 5: DOC/02-PLAN documentation (key references)

**Files (under `DOC/02-PLAN/`):**
These are historical planning docs. Only update key references (MeoClawTuari → MeoClaw) where they affect readability. Skip absolute file paths that reference the old folder name (those are historical artifacts and will be correct after folder rename).

Files with `meo-claw-tuari` / `MeoClawTuari` content references:
- `DOC/02-PLAN/Response状态与窗口位置调整.md:281` — log path
- `DOC/02-PLAN/MenuBarTrayIcon菜单栏图标.md:50` — tooltip
- `DOC/02-PLAN/OpenClaw状态检测集成.md:23,126` — diagram, text
- `DOC/02-PLAN/OpenClaw连接稳定性改进.md:61` — pkill
- `DOC/02-PLAN/OpenClaw响应时间测试.md:56` — screencapture
- `DOC/02-PLAN/Hermes后端集成.md:159,172,237,316` — API key, logs
- `DOC/02-PLAN/选项菜单后端切换修复-2026-04-22.md:74` — config path
- `DOC/02-PLAN/2026-04-24-分身功能设计.md:398,444` — window title

- [ ] **Step 1: Bulk replace in DOC/02-PLAN**
  Same pattern as Task 4 — replace all occurrences.
  Note: Some files like `桌宠大小调整功能实现-2026-04-17.md` only have absolute file paths (`/Users/hue/Documents/桌宠项目/MeoClawTuari/...`) which will be correct after folder rename — those need no change.

---

### Task 6: Misc docs + docs/superpowers

**Files:**
- Modify: `DOC/03-CHANGELOG/2026-04-17-缩放功能开发总结.md:14` — "MeoClaw Arcade"
- Modify: `docs/superpowers/specs/2026-04-19-多后端接入设计.md:5` — description
- Modify: `docs/superpowers/plans/*.md` — cargo check commands with `--package meo-claw-tuari`

- [ ] **Step 1: Update docs/superpowers files**
  Replace `MeoClawTuari` → `MeoClaw` and `meo-claw-tuari` → `meo-claw` in spec/plan files.

---

### Task 7: Regenerate lock files

- [ ] **Step 1: Regenerate package-lock.json**
  Run: `npm install` (from project root)
  Expected: `package-lock.json` updates with new package name

- [ ] **Step 2: Regenerate Cargo.lock**
  Run: `cargo check` (from `src-tauri/`)
  Expected: Compilation succeeds, `Cargo.lock` auto-updates

---

### Task 8: Folder rename + git commit

- [ ] **Step 1: Rename directory with git mv**
  ```bash
  cd /Users/hue/Documents/pet
  git mv MeoClawTuari MeoClaw
  ```
  This stages all file renames and preserves history.

- [ ] **Step 2: Verify git status**
  ```bash
  cd /Users/hue/Documents/pet/MeoClaw
  git status
  ```
  Expected: Shows renamed files + content changes. No deleted/untracked confusion.

- [ ] **Step 3: Commit the rename**
  ```bash
  cd /Users/hue/Documents/pet/MeoClaw
  git add -A
  git commit -m "rename: MeoClawTuari → MeoClaw"
  ```

---

### Task 9: Full build verification

- [ ] **Step 1: Rust build check**
  ```bash
  cd src-tauri && cargo check
  ```
  Expected: Compilation succeeds with no warnings related to name references.

- [ ] **Step 2: npm build check**
  ```bash
  npm run build
  ```
  Expected: Vite build succeeds.

- [ ] **Step 3: Tauri build check (optional)**
  ```bash
  npm run tauri build
  ```
  Expected: App builds without name-related errors.

---

### Files NOT modified (intentionally skipped)

| File | Reason |
|---|---|
| `options.html:6` | Already `MeoClaw 选项` — correct |
| `.kanban/tasks.json:26` | Internal kanban tool data, not user-facing |
| `.superpowers/brainstorm/*.html` | Historical generated design snapshots |
| `src-tauri/Cargo.lock` | Auto-regenerated in Task 7 |
| Absolute file path references in old planning docs | Fixed automatically by folder rename |
| `Hermes后端集成.md` API key `meo-claw-secret-key-2024` | This is a config value, not a name reference |
