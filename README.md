<p align="right">
  <strong>English</strong> | <a href="./README_CN.md">中文</a>
</p>

# MeoClaw 🐾

<div align="center">

![License](https://img.shields.io/badge/license-Apache%202.0-blue)
![Tauri](https://img.shields.io/badge/Tauri-v2-green)
![Vue](https://img.shields.io/badge/Vue-3-4FC08D)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange)

**An openclaw/hermes desktop pet assistant built with Tauri v2**

</div>

No need to open terminals, web browsers, or any third-party channels — directly chat with your configured assistant right on your desktop.

---

## Features

- **Easy Chat** — Double-click the pet to open an input box and start a conversation with your AI backend
- **File Drag & Drop** — Drop files onto the pet window, auto-detects file types and sends them to AI for processing
- **Dual Backend Support** — Compatible with both OpenClaw (WebSocket) and Hermes (HTTP/SSE), switch freely at runtime
- **Free Dragging & Resizing** — Drag the pet anywhere on screen, auto-snaps to edges, adjustable scale from 50% to 200%

## Screenshots

| | | |
|:---:|:---:|:---:|
| <img width="240" src="assets/screenshots/idle.png"> | <img width="240" src="assets/screenshots/bubble.png"> | <img width="240" src="assets/screenshots/file-drop.png"> |
| Pet idle on desktop | Tool call bubble | File drag & drop |
| <img width="240" src="assets/screenshots/chat.png"> | <img width="240" src="assets/screenshots/drag.png"> | <img width="240" src="assets/screenshots/settings.png"> |
| Response display | Drag & edge snap | Settings panel |

## Quick Start

### Prerequisites

- Node.js 18+
- Rust 1.70+
- macOS 11+ / Windows 10+ / Ubuntu 20.04+

### Install & Run

```bash
git clone https://github.com/yourusername/meoclaw.git
cd meoclaw
npm install
npm run tauri dev    # development with hot-reload
npm run tauri build  # production build
```

### Run Tests

```bash
npm test
```

## Tech Stack

| Layer | Tech |
|-------|------|
| Frontend | Vue 3 + TypeScript (Composition API) |
| Desktop | Tauri v2 |
| Backend | Rust |
| AI Backends | OpenClaw / Hermes |
| Communication | WebSocket + HTTP/SSE |
| Testing | Vitest + jsdom |

## Interactions

| Action | Result |
|--------|--------|
| Double-click | Opens input box for AI chat |
| Right-click | Context menu with state switches and commands |
| Long-press + drag | Move the pet; auto-snaps to screen edge |
| Drop file | Sends file to AI for processing |
| Tray left-click | Toggle window visibility |
| Tray right-click | Quit / Restart / Options |

## Animation States

| State | Description | Playback |
|-------|-------------|----------|
| `idle` | Idle loop | ping-pong loop |
| `shock` | Surprise | play once |
| `EnterInput` | Entering input mode | loop |
| `startworking` | Starting work | once → `working` |
| `working` | Working loop | forward loop |
| `EnterReceiving` | Entering receive | once → `Receiving` |
| `Receiving` | Receiving loop | loop |
| `received` | Received | static single frame |
| `Response` | Displaying response | ping-pong loop |

**Window behavior:** `dragging`, `edgeHiddenLeft/Right`, `edgePeekLeft/Right`

## Configuration

**Backend config** at `~/.local/share/meo-claw/backend-config.json`:

```json
{
  "selected": "openclaw",
  "openclaw": { "endpoint": "ws://127.0.0.1:18789" },
  "hermes": { "endpoint": "http://127.0.0.1:8642" }
}
```

**Frontend settings** (localStorage):

| Key | Description | Default |
|-----|-------------|---------|
| `meoclaw.petScale` | Scale (0.5–2.0) | 1 |
| `meoclaw.responseSound` | Response sound | `chime.mp3` |

## Project Structure

```
MeoClaw/
├── src/                    # Vue 3 frontend
│   ├── components/         # Pet components
│   ├── config/             # Animation & icon configs
│   ├── stores/             # State management
│   └── menu/               # Context menu
├── src-tauri/              # Rust backend
│   └── src/
│       ├── backend/        # OpenClaw & Hermes clients
│       └── openclaw/       # OpenClaw protocol & auth
├── public/                 # Static assets (sprites, sounds)
├── DOC/                    # Project documentation
└── package.json
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request. Make sure all tests pass before submitting:

```bash
npm test
```

## License

Apache 2.0 © 2026 Hue
