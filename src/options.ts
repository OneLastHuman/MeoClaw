import { emit } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

const STORAGE_KEY = 'meoclaw.petScale';
const MIN_SCALE = 0.5;
const MAX_SCALE = 2;
const STEP = 0.1;
const DEFAULT_SCALE = 1;
const SLIDER_MIN = 50;
const SLIDER_MAX = 200;

// Response 音效配置
const RESPONSE_SOUND_STORAGE_KEY = 'meoclaw.responseSound';
const RESPONSE_SOUND_OFF = 'off';
const DEFAULT_SOUND = 'chime.mp3';

const RESPONSE_SOUNDS = [
  { id: 'off', label: 'Off', file: null },
  { id: 'chime.mp3', label: 'Chime', file: '/sound/response/chime.mp3' },
  { id: 'soft.mp3', label: 'Soft', file: '/sound/response/soft.mp3' },
  { id: 'notification.ogg', label: 'Notification', file: '/sound/response/notification.ogg' },
  { id: 'classic.wav', label: 'Classic', file: '/sound/response/classic.wav' },
  { id: 'crisp.wav', label: 'Crisp', file: '/sound/response/crisp.wav' },
];

// 音量配置
const VOLUME_STORAGE_KEY = 'meoclaw.volume';
const VOLUME_MIN = 0;
const VOLUME_MAX = 1;

const VOLUME_DEFAULT = 0.5;

function loadVolume(): number {
  const stored = window.localStorage.getItem(VOLUME_STORAGE_KEY);
  if (stored === null) return VOLUME_DEFAULT;
  const parsed = Number(stored);
  if (!Number.isFinite(parsed)) return VOLUME_DEFAULT;
  return Math.min(VOLUME_MAX, Math.max(VOLUME_MIN, parsed));
}

function saveVolume(volume: number) {
  const clamped = Math.min(VOLUME_MAX, Math.max(VOLUME_MIN, volume));
  window.localStorage.setItem(VOLUME_STORAGE_KEY, String(clamped));
  emit('response-volume-changed', { volume: clamped });
}

function volumeToSlider(volume: number): number {
  return Math.round(volume * 100);
}

function sliderToVolume(sliderValue: number): number {
  return sliderValue / 100;
}

function formatVolume(volume: number) {
  return `${Math.round(volume * 100)}%`;
}

let pendingBackend: string | null = null;

function normalizeScale(raw: unknown) {
  const parsed = typeof raw === 'number' ? raw : Number(raw);
  if (!Number.isFinite(parsed)) {
    return DEFAULT_SCALE;
  }

  const clamped = Math.min(MAX_SCALE, Math.max(MIN_SCALE, parsed));
  return Math.round(clamped / STEP) * STEP;
}

function sliderToScale(sliderValue: number): number {
  return sliderValue / 100;
}

function scaleToSlider(scale: number): number {
  return scale * 100;
}

function loadScale() {
  return normalizeScale(window.localStorage.getItem(STORAGE_KEY));
}

async function applyScale(sliderValue: number) {
  const scale = sliderToScale(sliderValue);
  const normalized = normalizeScale(scale);
  window.localStorage.setItem(STORAGE_KEY, String(normalized));
  await emit('pet-scale-changed', { scale: normalized });
}

function formatScale(scale: number) {
  return `${Math.round(scale * 100)}%`;
}

const app = document.querySelector<HTMLDivElement>('#app');

if (!app) {
  throw new Error('options root not found');
}

// HTML 结构
app.innerHTML = `
  <main class="options-shell">
    <div class="options-window">
      <!-- Title Bar -->
      <div class="title-bar" data-tauri-drag-region>
        <button class="close-btn" id="close-btn" type="button">&times;</button>
        <span class="title-text">MEOCLAW OPTIONS</span>
      </div>

      <!-- Main Content -->
      <div class="options-body">
        <!-- Sidebar -->
        <nav class="sidebar">
          <!-- Logo -->
          <div class="logo-area">
            <div class="logo-icon">
              <span>&lt;/&gt;</span>
            </div>
            <div class="logo-text">MEO-CLAW</div>
          </div>

          <!-- Nav Items -->
          <div class="nav-item active" data-panel="size">SIZE</div>
          <div class="nav-item" data-panel="backend">BACKEND</div>
          <div class="nav-item" data-panel="sound">SOUND</div>
          <div class="nav-item" data-panel="about">ABOUT</div>

          <!-- Version -->
          <div class="version">v1.0.0</div>
        </nav>

        <!-- Content Panel -->
        <div class="content-panel">
          <!-- SIZE Panel -->
          <div class="panel-content active" data-panel="size">
            <div class="section-title">
              <h3>PET SCALE</h3>
              <p>SCALE AFFECTS WINDOW / INPUT / BUBBLE</p>
            </div>
            <div class="scale-display">
              <span class="scale-label">CURRENT</span>
              <span class="scale-value" id="scale-value">100%</span>
            </div>
            <div class="slider-container">
              <div class="slider-track">
                <div class="slider-fill" id="slider-fill"></div>
              </div>
              <input id="scale-slider" class="scale-slider" type="range" min="50" max="200" step="10" value="100" />
            </div>
            <div class="slider-labels">
              <span>50%</span>
              <span class="highlight">100%</span>
              <span>150%</span>
              <span>200%</span>
            </div>
            <div class="panel-actions">
              <button id="reset-btn" class="btn-secondary" type="button">RESET</button>
              <button id="apply-btn" class="btn-primary" type="button">SAVE</button>
            </div>
          </div>

          <!-- BACKEND Panel -->
          <div class="panel-content" data-panel="backend">
            <div class="section-title">
              <h3>BACKEND</h3>
              <p>SELECT MESSAGE PROCESSING BACKEND</p>
            </div>
            <div id="backend-selector" class="selector-list">
              <div class="selector-item" data-backend="openclaw">
                <span class="selector-label">OPENCLAW</span>
                <div class="backend-status">
                  <div class="selector-indicator unknown" id="openclaw-indicator"></div>
                  <span class="status-text" id="openclaw-text">检查中...</span>
                </div>
              </div>
              <div class="selector-item" data-backend="hermes">
                <span class="selector-label">HERMES</span>
                <div class="backend-status">
                  <div class="selector-indicator unknown" id="hermes-indicator"></div>
                  <span class="status-text" id="hermes-text">检查中...</span>
                </div>
              </div>
            </div>
            <div class="backend-current">
              <span class="current-label">CURRENT</span>
              <span class="current-value" id="current-backend-name">--</span>
            </div>
          </div>

          <!-- SOUND Panel -->
          <div class="panel-content" data-panel="sound">
            <div class="section-title">
              <h3>SOUND</h3>
              <p>RESPONSE NOTIFICATION SOUND</p>
            </div>
            <div id="sound-selector" class="selector-list">
              ${RESPONSE_SOUNDS.map((sound) => `
                <div class="selector-item" data-sound="${sound.id}">
                  <span class="selector-label">${sound.label.toUpperCase()}</span>
                  <div class="check-box"></div>
                </div>
              `).join('')}
            </div>
            <p class="sound-hint">CLICK TO SELECT AND PREVIEW</p>
            <div class="volume-section">
              <div class="volume-header">
                <span class="volume-label">VOLUME</span>
                <span class="volume-value" id="volume-value">${formatVolume(loadVolume())}</span>
              </div>
              <div class="slider-container">
                <div class="slider-track">
                  <div class="slider-fill" id="volume-slider-fill"></div>
                </div>
                <input id="volume-slider" class="scale-slider" type="range" min="0" max="100" step="10" value="${volumeToSlider(loadVolume())}" />
              </div>
              <div class="slider-labels">
                <span>0%</span>
                <span>50%</span>
                <span>100%</span>
              </div>
            </div>
          </div>

          <!-- ABOUT Panel -->
          <div class="panel-content" data-panel="about">
            <div class="section-title">
              <h3>ABOUT</h3>
              <p>PROJECT INFO</p>
            </div>
            <div class="about-content">
              <div class="about-logo">
                <div class="about-logo-icon">&lt;/&gt;</div>
                <span class="about-logo-text">MEO CLAW</span>
              </div>
              <p class="about-desc">An openclaw/hermes desktop pet assistant built with Tauri v2</p>
              <div class="about-link" id="github-link">
                <span class="about-link-label">GITHUB</span>
                <span class="about-link-url">github.com/OneLastHuman/MeoClaw</span>
              </div>
              <div class="about-meta">
                <div class="about-meta-item">
                  <span class="about-meta-key">VERSION</span>
                  <span class="about-meta-value">v1.0.0</span>
                </div>
                <div class="about-meta-item">
                  <span class="about-meta-key">LICENSE</span>
                  <span class="about-meta-value">Apache 2.0</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </main>
`;

// 确认对话框 HTML
const confirmModal = document.createElement('div');
confirmModal.id = 'confirm-modal';
confirmModal.className = 'modal-overlay hidden';
confirmModal.innerHTML = `
  <div class="modal-window">
    <h3>CONFIRM SWITCH?</h3>
    <p id="confirm-message">Switch backend to Hermes?</p>
    <div class="modal-actions">
      <button id="confirm-cancel" class="btn-secondary" type="button">CANCEL</button>
      <button id="confirm-ok" class="btn-primary" type="button">CONFIRM</button>
    </div>
  </div>
`;
document.querySelector('.options-shell')?.appendChild(confirmModal);

// 气泡提示 HTML
const toast = document.createElement('div');
toast.id = 'toast';
toast.className = 'toast hidden';
document.body.appendChild(toast);

// CSS 样式
const style = document.createElement('style');
style.textContent = `
  @font-face {
    font-family: 'Press Start 2P';
    src: url('/fonts/PressStart2P.woff2') format('woff2');
    font-weight: 400;
    font-style: normal;
    font-display: swap;
  }

  :root {
    --bg-light: #fff3dc;
    --bg-dark: #ffd59f;
    --accent: #fd811e;
    --border: #d07b1f;
    --text-dark: #934500;
    --text-muted: #865012;
    --panel-bg: #fffaf5;
  }

  * {
    box-sizing: border-box;
  }

  html, body, #app {
    width: 100%;
    height: 100%;
    margin: 0;
    background: transparent;
  }

  body {
    font-family: monospace;
    background: transparent;
    color: var(--text-dark);
  }

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

  .options-window {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--panel-bg);
    box-shadow: 0 12px 40px rgba(147, 69, 0, 0.15);
  }

  /* Title Bar */
  .title-bar {
    background: linear-gradient(180deg, var(--bg-light), var(--bg-dark));
    padding: 10px 14px;
    display: flex;
    align-items: center;
    gap: 10px;
    border-bottom: 3px solid var(--border);
  }

  .title-text {
    margin: 0 auto;
    font-family: 'Press Start 2P', monospace;
    font-size: 9px;
    color: var(--text-dark);
  }

  /* Main Body */
  .options-body {
    display: flex;
    flex: 1;
    min-height: 0;
  }

  /* Sidebar */
  .sidebar {
    width: 140px;
    background: linear-gradient(180deg, var(--bg-light), #ffe4c4);
    border-right: 3px solid var(--border);
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .logo-area {
    text-align: center;
    padding-bottom: 12px;
    border-bottom: 3px solid var(--border);
    margin-bottom: 4px;
  }

  .logo-icon {
    width: 36px;
    height: 36px;
    background: var(--accent);
    border: 3px solid var(--border);
    margin: 0 auto 6px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .logo-icon span {
    font-family: 'Press Start 2P', monospace;
    font-size: 10px;
    color: #fff;
  }

  .logo-text {
    font-family: 'Press Start 2P', monospace;
    font-size: 6px;
    color: var(--text-dark);
  }

  .nav-item {
    padding: 10px 12px;
    background: var(--bg-light);
    color: var(--text-dark);
    border: 3px solid var(--border);
    font-family: 'Press Start 2P', monospace;
    font-size: 7px;
    cursor: pointer;
    transition: background 150ms ease, color 150ms ease;
  }

  .nav-item:hover {
    background: var(--bg-dark);
  }

  .nav-item.active {
    background: var(--accent);
    color: #fff;
  }

  .version {
    margin-top: auto;
    text-align: center;
    font-family: 'Press Start 2P', monospace;
    font-size: 6px;
    color: var(--text-muted);
    padding-top: 12px;
    border-top: 3px solid var(--border);
  }

  /* Content Panel */
  .content-panel {
    flex: 1;
    padding: 18px;
    display: flex;
    flex-direction: column;
    background: var(--panel-bg);
    min-height: 0;
    position: relative;
  }

  .panel-content {
    display: none;
    flex-direction: column;
    height: 100%;
  }

  .panel-content.active {
    display: flex;
  }

  .section-title {
    margin-bottom: 14px;
  }

  .section-title h3 {
    font-family: 'Press Start 2P', monospace;
    font-size: 11px;
    color: var(--text-dark);
    margin: 0 0 4px;
  }

  .section-title p {
    font-family: monospace;
    font-size: 10px;
    color: var(--text-muted);
    margin: 0;
  }

  /* Scale Display */
  .scale-display {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
    padding: 12px;
    background: var(--bg-light);
    border: 3px solid var(--border);
  }

  .scale-label {
    font-family: 'Press Start 2P', monospace;
    font-size: 8px;
    color: var(--text-muted);
  }

  .scale-value {
    font-family: 'Press Start 2P', monospace;
    font-size: 20px;
    color: var(--accent);
  }

  /* Slider */
  .slider-container {
    margin-bottom: 6px;
    position: relative;
    height: 16px;
  }

  .slider-track {
    height: 16px;
    background: var(--bg-light);
    border: 3px solid var(--border);
    position: relative;
    overflow: hidden;
  }

  .slider-fill {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 50%;
    background: linear-gradient(90deg, var(--accent), #ff9a3f);
  }

  .scale-slider {
    position: absolute;
    left: 0;
    top: -6px;
    width: 100%;
    height: 28px;
    opacity: 0;
    cursor: pointer;
    margin: 0;
  }

  .slider-labels {
    display: flex;
    justify-content: space-between;
    margin-bottom: 14px;
    padding: 0 4px;
  }

  .slider-labels span {
    font-family: 'Press Start 2P', monospace;
    font-size: 7px;
    color: var(--text-muted);
  }

  .slider-labels .highlight {
    color: var(--accent);
  }

  .panel-actions {
    display: flex;
    gap: 10px;
    margin-top: auto;
    padding-top: 12px;
    border-top: 3px solid var(--bg-dark);
  }

  /* Buttons */
  button {
    border: 3px solid var(--border);
    font-family: 'Press Start 2P', monospace;
    font-size: 8px;
    cursor: pointer;
    transition: transform 100ms ease;
  }

  button:hover {
    transform: translateY(-1px);
  }

  button:active {
    transform: translateY(1px);
  }

  .btn-secondary {
    flex: 1;
    padding: 10px;
    background: var(--bg-light);
    color: var(--text-dark);
  }

  .btn-primary {
    flex: 2;
    padding: 10px;
    background: var(--accent);
    color: #fff;
  }

  /* Selector List (Backend & Sound) */
  .selector-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    flex: 1;
  }

  .selector-item {
    padding: 10px 12px;
    background: #fff;
    border: 3px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
    transition: background 150ms ease;
  }

  .selector-item:hover {
    background: var(--bg-light);
  }

  .selector-item.selected {
    background: var(--accent);
    color: #fff;
  }

  .selector-item.selected .selector-label {
    color: #fff;
  }

  .selector-item.selected .check-box {
    background: #fff;
    border-color: var(--border);
  }

  .selector-label {
    font-family: 'Press Start 2P', monospace;
    font-size: 7px;
    color: var(--text-dark);
  }

  .selector-indicator {
    width: 10px;
    height: 10px;
    border: 2px solid var(--border);
  }

  .selector-indicator.running {
    background: #39ff14;
  }

  .selector-indicator.stopped {
    background: #ff6b6b;
  }

  .selector-indicator.unknown {
    background: var(--text-muted);
  }

  .backend-status {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .status-text {
    font-family: 'Press Start 2P', monospace;
    font-size: 7px;
    color: var(--text-muted);
  }

  .check-box {
    width: 10px;
    height: 10px;
    background: var(--text-muted);
    border: 2px solid var(--border);
  }

  /* Backend Current */
  .backend-current {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px;
    background: var(--bg-light);
    border: 3px solid var(--border);
    margin-top: auto;
  }

  .current-label {
    font-family: 'Press Start 2P', monospace;
    font-size: 7px;
    color: var(--text-muted);
  }

  .current-value {
    font-family: 'Press Start 2P', monospace;
    font-size: 9px;
    color: var(--accent);
  }

  /* Sound Hint */
  .sound-hint {
    padding-top: 12px;
    border-top: 3px solid var(--bg-dark);
    font-family: monospace;
    font-size: 9px;
    color: var(--text-muted);
    text-align: center;
    margin-bottom: 14px;
  }

  /* Volume Section */
  .volume-section {
    margin-top: auto;
  }

  .volume-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }

  .volume-label {
    font-family: 'Press Start 2P', monospace;
    font-size: 8px;
    color: var(--text-muted);
  }

  .volume-value {
    font-family: 'Press Start 2P', monospace;
    font-size: 14px;
    color: var(--accent);
  }

  /* Modal */
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-overlay.hidden {
    display: none;
  }

  .modal-window {
    background: var(--panel-bg);
    border: 3px solid var(--border);
    border-radius: 12px;
    padding: 24px 20px;
    width: min(90%, 320px);
    text-align: center;
  }

  .modal-window h3 {
    font-family: 'Press Start 2P', monospace;
    font-size: 10px;
    color: var(--text-dark);
    margin: 0 0 16px;
  }

  .modal-window p {
    font-family: monospace;
    font-size: 12px;
    color: var(--text-muted);
    margin: 0 0 20px;
  }

  .modal-actions {
    display: flex;
    gap: 10px;
  }

  .modal-actions button {
    flex: 1;
    padding: 10px;
  }

  /* Toast */
  .toast {
    position: fixed;
    bottom: 20px;
    right: 20px;
    padding: 12px 20px;
    border: 3px solid var(--border);
    border-radius: 12px;
    font-family: 'Press Start 2P', monospace;
    font-size: 8px;
    z-index: 1001;
    animation: slideIn 300ms ease;
  }

  .toast.hidden {
    display: none;
  }

  .toast.success {
    background: #d4edda;
    color: #155724;
    border-color: #28a745;
  }

  .toast.error {
    background: #f8d7da;
    color: #721c24;
    border-color: #dc3545;
  }

  @keyframes slideIn {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

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

  /* About Panel */
  .about-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    padding-top: 12px;
  }

  .about-logo {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }

  .about-logo-icon {
    width: 48px;
    height: 48px;
    background: var(--accent);
    border: 3px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: 'Press Start 2P', monospace;
    font-size: 14px;
    color: #fff;
  }

  .about-logo-text {
    font-family: 'Press Start 2P', monospace;
    font-size: 10px;
    color: var(--text-dark);
  }

  .about-desc {
    font-family: monospace;
    font-size: 10px;
    color: var(--text-muted);
    text-align: center;
    margin: 0;
    max-width: 320px;
    line-height: 1.5;
  }

  .about-link {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 12px 20px;
    background: var(--bg-light);
    border: 3px solid var(--border);
    cursor: pointer;
    transition: background 150ms ease;
    width: 100%;
    max-width: 360px;
  }

  .about-link:hover {
    background: var(--bg-dark);
  }

  .about-link-label {
    font-family: 'Press Start 2P', monospace;
    font-size: 7px;
    color: var(--text-muted);
  }

  .about-link-url {
    font-family: monospace;
    font-size: 11px;
    color: var(--accent);
    word-break: break-all;
    text-align: center;
  }

  .about-meta {
    display: flex;
    gap: 20px;
    margin-top: 4px;
  }

  .about-meta-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }

  .about-meta-key {
    font-family: 'Press Start 2P', monospace;
    font-size: 6px;
    color: var(--text-muted);
  }

  .about-meta-value {
    font-family: 'Press Start 2P', monospace;
    font-size: 8px;
    color: var(--text-dark);
  }
`;
document.head.appendChild(style);

// ========== 面板切换逻辑 ==========
function switchPanel(panelName: string) {
  // Update nav items
  document.querySelectorAll('.nav-item').forEach(item => {
    item.classList.toggle('active', item.getAttribute('data-panel') === panelName);
  });

  // Update panel content
  document.querySelectorAll('.panel-content').forEach(panel => {
    panel.classList.toggle('active', panel.getAttribute('data-panel') === panelName);
  });
}

document.querySelectorAll('.nav-item').forEach(item => {
  item.addEventListener('click', () => {
    const panel = item.getAttribute('data-panel');
    if (panel) {
      switchPanel(panel);
    }
  });
});

// ========== Scale 逻辑 ==========
const slider = document.querySelector<HTMLInputElement>('#scale-slider');
const scaleValue = document.querySelector<HTMLElement>('#scale-value');
const sliderFill = document.querySelector<HTMLElement>('#slider-fill');
const resetButton = document.querySelector<HTMLButtonElement>('#reset-btn');
const applyButton = document.querySelector<HTMLButtonElement>('#apply-btn');

if (!slider || !scaleValue || !sliderFill || !resetButton || !applyButton) {
  throw new Error('options controls not found');
}

function updateSliderUI(sliderValue: number) {
  if (!sliderFill) return;
  const percent = ((sliderValue - SLIDER_MIN) / (SLIDER_MAX - SLIDER_MIN)) * 100;
  sliderFill.style.width = `${percent}%`;
}

let currentSliderValue = scaleToSlider(loadScale());
slider.value = String(currentSliderValue);
scaleValue.textContent = formatScale(sliderToScale(currentSliderValue));
updateSliderUI(currentSliderValue);

slider.addEventListener('input', async () => {
  currentSliderValue = Number(slider.value);
  const scale = sliderToScale(currentSliderValue);
  scaleValue.textContent = formatScale(scale);
  updateSliderUI(currentSliderValue);
  await applyScale(currentSliderValue);
});

resetButton.addEventListener('click', async () => {
  currentSliderValue = scaleToSlider(DEFAULT_SCALE);
  slider.value = String(currentSliderValue);
  scaleValue.textContent = formatScale(DEFAULT_SCALE);
  updateSliderUI(currentSliderValue);
  await applyScale(currentSliderValue);
});

applyButton.addEventListener('click', async () => {
  await applyScale(currentSliderValue);
  await getCurrentWindow().close();
});

// ========== 关闭窗口 ==========
document.getElementById('close-btn')?.addEventListener('click', () => {
  getCurrentWindow().close();
});

// ========== 后端状态检查和切换逻辑 ==========
interface BackendHealth {
  available: boolean;
  backend: string;
  endpoint: string;
  error?: string;
}

// 后端状态检查（检查所有已注册后端）
async function checkBackendStatus() {
  try {
    const healthList = await invoke<BackendHealth[]>('check_all_backends_health');
    healthList.forEach(health => {
      const indicator = document.getElementById(`${health.backend}-indicator`);
      const textEl = document.getElementById(`${health.backend}-text`);
      if (indicator) {
        indicator.className = `selector-indicator ${health.available ? 'running' : 'stopped'}`;
      }
      if (textEl) {
        textEl.textContent = health.available ? 'ON' : 'OFF';
      }
    });
    return healthList;
  } catch (e) {
    // 如果检查失败，所有状态显示为未知
    document.querySelectorAll('.selector-indicator').forEach(el => {
      el.className = 'selector-indicator unknown';
    });
    document.querySelectorAll('.status-text').forEach(el => {
      el.textContent = '??';
    });
    return null;
  }
}

// 获取当前后端
async function getCurrentBackend() {
  try {
    const backend = await invoke<string | null>('get_current_backend');
    const nameEl = document.getElementById('current-backend-name');
    if (nameEl && backend) {
      nameEl.textContent = backend === 'openclaw' ? 'OPENCLAW' : 'HERMES';
    }
    updateBackendSelection(backend);
    return backend;
  } catch (e) {
    console.error('Failed to get current backend:', e);
    updateBackendSelection(null);
    return null;
  }
}

function updateBackendSelection(backend: string | null) {
  document.querySelectorAll('#backend-selector .selector-item').forEach(item => {
    const isSelected = item.getAttribute('data-backend') === backend;
    item.classList.toggle('selected', isSelected);
  });
}

function openConfirmModal(backend: string) {
  pendingBackend = backend;
  const modal = document.getElementById('confirm-modal');
  const message = document.getElementById('confirm-message');
  if (modal && message) {
    const backendName = backend === 'hermes' ? 'Hermes' : 'OpenClaw';
    message.textContent = `Switch backend to ${backendName}?`;
    modal.classList.remove('hidden');
  }
}

function closeConfirmModal() {
  pendingBackend = null;
  const modal = document.getElementById('confirm-modal');
  if (modal) {
    modal.classList.add('hidden');
  }
}

function showToast(message: string, isSuccess: boolean) {
  const toast = document.getElementById('toast');
  if (toast) {
    toast.textContent = isSuccess ? `OK ${message}` : `FAIL ${message}`;
    toast.className = `toast ${isSuccess ? 'success' : 'error'}`;
    setTimeout(() => {
      toast.classList.add('hidden');
    }, 2000);
  }
}

async function verifySwitch(targetBackend: string, previousBackend: string | null): Promise<{success: boolean, message: string}> {
  // 验证1：检查后端是否已切换（与切换前不同）
  if (previousBackend === targetBackend) {
    const backendName = targetBackend === 'hermes' ? 'Hermes' : 'OpenClaw';
    return { success: false, message: `${backendName} Already Active` };
  }

  // 验证2：检查当前后端是否切换到目标后端
  const current = await invoke<string | null>('get_current_backend');
  if (current !== targetBackend) {
    return { success: false, message: 'Switch Failed: State Error' };
  }

  // 验证3：检查目标后端是否可用
  const healthList = await invoke<BackendHealth[]>('check_all_backends_health');
  const targetHealth = healthList.find(h => h.backend === targetBackend);
  if (!targetHealth?.available) {
    const backendName = targetBackend === 'hermes' ? 'Hermes' : 'OpenClaw';
    return { success: false, message: `Switch Failed: ${backendName} Not Responding` };
  }

  const backendName = targetBackend === 'hermes' ? 'Hermes' : 'OpenClaw';
  return { success: true, message: `Switched to ${backendName}` };
}

// 切换后端
async function switchBackend(backend: string) {
  try {
    // 0. 获取切换前的状态
    const previousBackend = await invoke<string | null>('get_current_backend');

    // 1. 执行切换
    await invoke('switch_backend', { backend });

    // 2. 双重验证
    const result = await verifySwitch(backend, previousBackend);

    // 3. 显示结果
    showToast(result.message, result.success);

    // 4. 更新 UI
    await getCurrentBackend();
    await checkBackendStatus();

    return result;
  } catch (e) {
    console.error('Failed to switch backend:', e);
    showToast('Switch Failed: Exec Error', false);
    return { success: false, message: 'Switch Failed: Exec Error' };
  }
}

// 绑定单选按钮点击事件
document.querySelectorAll('#backend-selector .selector-item').forEach(item => {
  item.addEventListener('click', () => {
    const backend = item.getAttribute('data-backend');
    if (backend) {
      openConfirmModal(backend);
    }
  });
});

document.getElementById('confirm-cancel')?.addEventListener('click', () => {
  closeConfirmModal();
});

document.getElementById('confirm-ok')?.addEventListener('click', async () => {
  if (pendingBackend) {
    const backendToSwitch = pendingBackend;  // 先保存值
    closeConfirmModal();  // closeConfirmModal 会把 pendingBackend 设为 null，所以先保存
    await switchBackend(backendToSwitch);
  }
});

// 初始化状态
checkBackendStatus();
getCurrentBackend();

// 监听后端切换事件
listen('backend-switched', (event: any) => {
  const nameEl = document.getElementById('current-backend-name');
  if (nameEl) {
    nameEl.textContent = event.payload === 'openclaw' ? 'OPENCLAW' : 'HERMES';
  }
  checkBackendStatus();
});

// ========== Response 音效选择逻辑 ==========

function loadResponseSound(): string {
  return window.localStorage.getItem(RESPONSE_SOUND_STORAGE_KEY) || DEFAULT_SOUND;
}

function saveResponseSound(soundId: string) {
  window.localStorage.setItem(RESPONSE_SOUND_STORAGE_KEY, soundId);
  // 通知前端音效设置已更改
  emit('response-sound-changed', { soundId });
}

function playSoundPreview(soundId: string) {
  if (soundId === RESPONSE_SOUND_OFF) {
    return;
  }

  const sound = RESPONSE_SOUNDS.find((s) => s.id === soundId);
  if (!sound || !sound.file) {
    return;
  }

  try {
    const audio = new Audio(sound.file);
    audio.volume = loadVolume();
    audio.play().catch((err) => {
      console.warn('[Sound Preview] Failed to play:', err);
    });
  } catch (err) {
    console.warn('[Sound Preview] Failed to create audio:', err);
  }
}

function updateSoundSelection(soundId: string | null) {
  document.querySelectorAll('#sound-selector .selector-item').forEach((item) => {
    const isSelected = item.getAttribute('data-sound') === soundId;
    item.classList.toggle('selected', isSelected);
  });
}

// 初始化音效选择状态
const currentSound = loadResponseSound();
updateSoundSelection(currentSound);

// 绑定音效选择点击事件（预览播放）
document.querySelectorAll('#sound-selector .selector-item').forEach((item) => {
  item.addEventListener('click', () => {
    const soundId = item.getAttribute('data-sound');
    if (soundId) {
      updateSoundSelection(soundId);
      playSoundPreview(soundId);
      saveResponseSound(soundId);
    }
  });
});

// ========== 音量滑块逻辑 ==========
const volumeSlider = document.querySelector<HTMLInputElement>('#volume-slider')!;
const volumeValue = document.querySelector<HTMLElement>('#volume-value')!;
const volumeSliderFill = document.querySelector<HTMLElement>('#volume-slider-fill')!;

function updateVolumeSliderUI(sliderValue: number) {
  const percent = (sliderValue / 100) * 100;
  volumeSliderFill.style.width = `${percent}%`;
}

let currentVolumeSliderValue = volumeToSlider(loadVolume());
volumeSlider.value = String(currentVolumeSliderValue);
volumeValue.textContent = formatVolume(sliderToVolume(currentVolumeSliderValue));
updateVolumeSliderUI(currentVolumeSliderValue);

volumeSlider.addEventListener('input', () => {
  currentVolumeSliderValue = Number(volumeSlider.value);
  const vol = sliderToVolume(currentVolumeSliderValue);
  volumeValue.textContent = formatVolume(vol);
  updateVolumeSliderUI(currentVolumeSliderValue);
  saveVolume(vol);
});

// 监听音量事件（用于同步多窗口）
listen('response-volume-changed', (event: any) => {
  if (typeof event.payload?.volume === 'number') {
    const vol = event.payload.volume;
    currentVolumeSliderValue = volumeToSlider(vol);
    volumeSlider.value = String(currentVolumeSliderValue);
    volumeValue.textContent = formatVolume(vol);
    updateVolumeSliderUI(currentVolumeSliderValue);
  }
});

// 监听音效切换事件（用于同步多窗口）
listen('response-sound-changed', (event: any) => {
  if (event.payload?.soundId) {
    updateSoundSelection(event.payload.soundId);
  }
});

// ========== About 面板逻辑 ==========
document.getElementById('github-link')?.addEventListener('click', () => {
  invoke('open_url', { url: 'https://github.com/OneLastHuman/MeoClaw' });
});
