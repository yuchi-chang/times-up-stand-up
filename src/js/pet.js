import { getCurrentWindow } from '@tauri-apps/api/window';
import { LogicalPosition } from '@tauri-apps/api/dpi';
import { invoke } from '@tauri-apps/api/core';
import { load } from '@tauri-apps/plugin-store';
import { setLanguage, t } from './i18n.js';

const WIN_W = 200;
const WIN_H = 170;
const SPEED = 1;
const BUBBLE_DURATION = 4000; // show bubble for 4 seconds

let win;
let x, y, vx, vy;
let screenW, screenH;
let dirTimer;
let alive = true;
let gifFacesRight = true;

async function init() {
  win = getCurrentWindow();

  let autoDismiss = 15;
  let customText = '';

  // Load settings
  try {
    const store = await load('settings.json', { autoSave: false });
    const lang = await store.get('language');
    if (lang) setLanguage(lang);
    autoDismiss = (await store.get('auto_dismiss_seconds')) || 15;
    const savedText = await store.get('custom_text');
    if (savedText != null) customText = savedText;
    const dir = await store.get('gif_direction');
    if (dir === 'left') gifFacesRight = false;
  } catch (e) {}

  // Load custom GIF if set
  try {
    const base64Data = await invoke('load_gif_base64');
    if (base64Data) {
      document.getElementById('gif').src = 'data:image/gif;base64,' + base64Data;
    }
  } catch (e) {}

  // Get screen size from Tauri monitor API
  try {
    const monitor = await win.currentMonitor();
    if (monitor) {
      const scale = monitor.scaleFactor;
      screenW = monitor.size.width / scale;
      screenH = monitor.size.height / scale;
    }
  } catch (e) {}

  // Fallback
  if (!screenW || !screenH) {
    screenW = window.screen.availWidth || 1920;
    screenH = window.screen.availHeight || 1080;
  }

  // Random start position
  x = Math.random() * (screenW - WIN_W);
  y = Math.random() * (screenH - WIN_H - 48);

  try {
    await win.setPosition(new LogicalPosition(Math.round(x), Math.round(y)));
  } catch (e) {
    console.error('setPosition failed:', e);
  }

  // Show speech bubble (use i18n default if empty)
  showBubble(customText || t('reminder.default'));

  // Start moving in the same direction as GIF faces
  newDirection(true);

  // Movement loop
  requestTick();

  // Change direction every 3~8 seconds
  scheduleDirectionChange();

  // Auto-dismiss
  setTimeout(() => dismiss(), autoDismiss * 1000);

  // Click to dismiss
  document.body.addEventListener('click', () => dismiss());
}

function showBubble(text) {
  const bubble = document.getElementById('bubble');
  bubble.textContent = text;
  bubble.classList.remove('hidden');

  setTimeout(() => {
    bubble.classList.add('hidden');
  }, BUBBLE_DURATION);
}

function newDirection(matchGif) {
  if (matchGif) {
    const base = gifFacesRight ? 0 : Math.PI;
    const angle = base + (Math.random() - 0.5) * Math.PI * 0.6;
    vx = Math.cos(angle) * SPEED;
    vy = Math.sin(angle) * SPEED;
  } else {
    const angle = Math.random() * Math.PI * 2;
    vx = Math.cos(angle) * SPEED;
    vy = Math.sin(angle) * SPEED;
  }
  updateFlip();
}

function updateFlip() {
  const movingRight = vx >= 0;
  const needFlip = gifFacesRight ? !movingRight : movingRight;
  document.getElementById('gif').style.transform = needFlip ? 'scaleX(-1)' : '';
}

function scheduleDirectionChange() {
  const delay = 3000 + Math.random() * 5000;
  dirTimer = setTimeout(() => {
    newDirection();
    scheduleDirectionChange();
  }, delay);
}

async function requestTick() {
  while (alive) {
    x += vx;
    y += vy;

    if (x <= 0 || x >= screenW - WIN_W) {
      vx = -vx;
      x = Math.max(0, Math.min(x, screenW - WIN_W));
      updateFlip();
    }
    if (y <= 0 || y >= screenH - WIN_H - 48) {
      vy = -vy;
      y = Math.max(0, Math.min(y, screenH - WIN_H - 48));
    }

    try {
      await win.setPosition(new LogicalPosition(Math.round(x), Math.round(y)));
    } catch (e) { break; }

    await new Promise(r => setTimeout(r, 30));
  }
}

async function dismiss() {
  alive = false;
  clearTimeout(dirTimer);
  try {
    await win.close();
  } catch (e) {}
}

init();
