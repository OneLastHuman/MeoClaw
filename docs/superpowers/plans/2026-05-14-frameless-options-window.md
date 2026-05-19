# Frameless Options Window Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Convert the options window from native chrome to a frameless window with a retro-styled close button and drag region.

**Architecture:** Rust `decorations(false)` + `transparent(true)` removes native title bar; frontend replaces fake traffic lights with a real close button and adds `data-tauri-drag-region` for window dragging. Body background becomes transparent, window border with border-radius sits flush against window edges.

**Tech Stack:** Tauri v2 (Rust + WebviewWindowBuilder), Vanilla TypeScript, CSS

---

### Task 1: Rust — Make options window frameless with transparent background

**Files:**
- Modify: `src-tauri/src/lib.rs:640-653`

- [ ] **Step 1: Update `show_options_window` builder**

  Locate the `show_options_window` function in `src-tauri/src/lib.rs`, find the `WebviewWindowBuilder::new(...)` block (around line 640-653). Make the following changes:

  1. Add `.decorations(false)` — removes native title bar and border
  2. Add `.transparent(true)` — enables window background transparency so border-radius on the content shows clean corners
  3. Keep `.title("MeoClaw 选项")` — still needed for Mission Control / accessibility

  The modified block should look like this:

  ```rust
  let options_window = WebviewWindowBuilder::new(
      app,
      "options",
      WebviewUrl::App("options.html".into()),
  )
  .title("MeoClaw 选项")
  .inner_size(680.0, 550.0)
  .min_inner_size(620.0, 480.0)
  .resizable(false)
  .maximizable(false)
  .minimizable(false)
  .always_on_top(true)
  .skip_taskbar(true)
  .decorations(false)
  .transparent(true)
  .build()?;
  ```

- [ ] **Step 2: Verify Rust compiles**

  Run: `cargo check --manifest-path src-tauri/Cargo.toml`
  Expected: Compilation succeeds with no errors.

- [ ] **Step 3: Commit**

  ```bash
  git add src-tauri/src/lib.rs
  git commit -m "feat: make options window frameless with transparent bg"
  ```

---

### Task 2: Frontend — Replace traffic lights with close button, add drag region

**Files:**
- Modify: `src/options.ts` (HTML template, CSS, and event binding)

- [ ] **Step 1: Update HTML title bar**

  In `src/options.ts`, find the title bar HTML block (lines 74-79):

  ```html
  <!-- Title Bar -->
  <div class="title-bar">
    <div class="traffic-light"></div>
    <div class="traffic-light" style="opacity: 0.5;"></div>
    <div class="traffic-light" style="opacity: 0.3;"></div>
    <span class="title-text">MEOCLAW OPTIONS</span>
  </div>
  ```

  Replace with:

  ```html
  <!-- Title Bar -->
  <div class="title-bar" data-tauri-drag-region>
    <button class="close-btn" id="close-btn" type="button">&times;</button>
    <span class="title-text">MEOCLAW OPTIONS</span>
  </div>
  ```

- [ ] **Step 2: Update CSS — remove padding/layout wrapper, adjust title bar and body styles**

  Find the `.options-shell` rule (lines 235-241):

  ```css
  .options-shell {
    min-height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 20px;
  }
  ```

  Replace with:

  ```css
  .options-shell {
    width: 100%;
    height: 100vh;
    display: flex;
    flex-direction: column;
    background: linear-gradient(160deg, var(--bg-light), var(--bg-dark));
    border: 3px solid var(--border);
    border-radius: 12px;
    overflow: hidden;
  }
  ```

  Find the `.options-window` rule (lines 243-251):

  ```css
  .options-window {
    width: 100%;
    max-width: 720px;
    background: var(--panel-bg);
    border: 3px solid var(--border);
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 12px 40px rgba(147, 69, 0, 0.15);
  }
  ```

  Replace with:

  ```css
  .options-window {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--panel-bg);
    box-shadow: 0 12px 40px rgba(147, 69, 0, 0.15);
  }
  ```

  Find the `html, body, #app` rule (lines 223-227):

  ```css
  html, body, #app {
    width: 100%;
    height: 100%;
    margin: 0;
  }
  ```

  Replace with:

  ```css
  html, body, #app {
    width: 100%;
    height: 100%;
    margin: 0;
    background: transparent;
  }
  ```

  Find the `body` rule (lines 229-233):

  ```css
  body {
    font-family: monospace;
    background: linear-gradient(160deg, var(--bg-light), var(--bg-dark));
    color: var(--text-dark);
  }
  ```

  Replace with:

  ```css
  body {
    font-family: monospace;
    background: transparent;
    color: var(--text-dark);
  }
  ```

  Find the `.traffic-light` rule (lines 263-268) — delete the entire rule:

  ```css
  .traffic-light {
    width: 14px;
    height: 14px;
    background: var(--border);
    border-radius: 50%;
  }
  ```

  Find the `.title-bar` rule (lines 254-261):

  ```css
  .title-bar {
    background: linear-gradient(180deg, var(--bg-light), var(--bg-dark));
    padding: 12px 16px;
    display: flex;
    align-items: center;
    gap: 12px;
    border-bottom: 3px solid var(--border);
  }
  ```

  Replace with:

  ```css
  .title-bar {
    background: linear-gradient(180deg, var(--bg-light), var(--bg-dark));
    padding: 10px 14px;
    display: flex;
    align-items: center;
    gap: 10px;
    border-bottom: 3px solid var(--border);
  }
  ```

  Find the `.title-text` rule (lines 270-275):

  ```css
  .title-text {
    margin-left: auto;
    font-family: 'Press Start 2P', monospace;
    font-size: 9px;
    color: var(--text-dark);
  }
  ```

  Replace with:

  ```css
  .title-text {
    margin: 0 auto;
    font-family: 'Press Start 2P', monospace;
    font-size: 9px;
    color: var(--text-dark);
  }
  ```

- [ ] **Step 3: Add close button styles**

  Find the end of the CSS injection block (before `document.head.appendChild(style);` on line 713), add the close button styles:

  ```css
  .close-btn {
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: #e74c3c;
    border: 2px solid #c0392b;
    color: #fff;
    font-family: monospace;
    font-size: 12px;
    font-weight: bold;
    line-height: 1;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    flex-shrink: 0;
    z-index: 1;
  }

  .close-btn:hover {
    background: #ff6b6b;
  }

  .close-btn:active {
    background: #c0392b;
    transform: translateY(0);
  }
  ```

- [ ] **Step 4: Add close button event binding**

  Find the event listener section (after the scale logic, before backend logic — around line 737). Add:

  ```typescript
  // ========== 关闭窗口 ==========
  document.getElementById('close-btn')?.addEventListener('click', () => {
    getCurrentWindow().close();
  });
  ```

  Also add the import at the top if not already present — check line 2 already imports `getCurrentWindow`:

  ```typescript
  import { getCurrentWindow } from '@tauri-apps/api/window';
  ```

- [ ] **Step 5: Adjust `.options-body` height**

  Find the `.options-body` rule (lines 278-281). Since the window now uses flex-grow instead of fixed height:

  ```css
  .options-body {
    display: flex;
    height: 440px;
  }
  ```

  Replace with:

  ```css
  .options-body {
    display: flex;
    flex: 1;
    min-height: 0;
  }
  ```

- [ ] **Step 6: Verify TypeScript compiles**

  Run: `npx tsc --noEmit`
  Expected: No type errors.

- [ ] **Step 7: Run full dev build**

  Run: `npm run tauri dev` — let it start, verify the options window opens correctly from tray menu.
  Expected: Options window opens with no native title bar, has a red circle close button on the left, can be dragged by the title bar, close button works.

- [ ] **Step 8: Commit**

  ```bash
  git add src/options.ts
  git commit -m "feat: add close button and drag region to frameless options window"
  ```
