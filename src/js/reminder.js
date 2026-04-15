import { getCurrentWindow } from '@tauri-apps/api/window';
import { load } from '@tauri-apps/plugin-store';
import { invoke } from '@tauri-apps/api/core';
import { setLanguage, t } from './i18n.js';

async function init() {
  let customText = '';
  let autoDismiss = 8;

  try {
    const store = await load('settings.json', { autoSave: false });
    const lang = await store.get('language');
    if (lang) setLanguage(lang);
    const savedText = await store.get('custom_text');
    if (savedText != null) customText = savedText;
    autoDismiss = (await store.get('auto_dismiss_seconds')) || autoDismiss;
  } catch (e) {
    console.error('Failed to load settings:', e);
  }

  const gifEl = document.getElementById('gif');
  const msgEl = document.getElementById('message');

  // Load GIF via Rust backend (handles local file reading)
  try {
    const base64Data = await invoke('load_gif_base64');
    if (base64Data) {
      gifEl.src = 'data:image/gif;base64,' + base64Data;
    }
  } catch (e) {
    console.error('Failed to load custom GIF, using default:', e);
  }

  // Set message text (use i18n default if empty)
  msgEl.textContent = customText || t('reminder.default');

  // Auto dismiss
  const dismissTimeout = setTimeout(() => dismiss(), autoDismiss * 1000);

  // Click to dismiss
  document.getElementById('reminder-container').addEventListener('click', () => {
    clearTimeout(dismissTimeout);
    dismiss();
  });
}

async function dismiss() {
  const container = document.getElementById('reminder-container');
  container.classList.add('fade-out');

  setTimeout(async () => {
    try {
      const win = getCurrentWindow();
      await win.hide();
    } catch (e) {
      console.error('Failed to destroy window:', e);
    }
  }, 400);
}

init();
