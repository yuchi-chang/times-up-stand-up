import { load } from '@tauri-apps/plugin-store';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { setLanguage, t, applyTranslations } from './i18n.js';

let store;

async function init() {
  try {
    store = await load('settings.json', { autoSave: false });

    // Load saved settings
    const lang = await store.get('language');
    const interval = await store.get('interval_minutes');
    const customText = await store.get('custom_text');
    const gifPath = await store.get('gif_path');
    const autoDismiss = await store.get('auto_dismiss_seconds');
    const petMode = await store.get('pet_mode');
    const gifDirection = await store.get('gif_direction');

    if (lang) {
      setLanguage(lang);
      document.getElementById('language').value = lang;
    }
    if (interval) document.getElementById('interval').value = interval;
    if (customText != null) document.getElementById('custom-text').value = customText;
    if (gifPath) document.getElementById('gif-path').value = gifPath;
    if (autoDismiss) document.getElementById('auto-dismiss').value = autoDismiss;
    if (petMode) document.getElementById('pet-mode').checked = petMode;
    if (gifDirection) {
      const radio = document.querySelector(`input[name="gif-direction"][value="${gifDirection}"]`);
      if (radio) radio.checked = true;
    }
    toggleDirectionField();
    applyTranslations();
    updateTextPlaceholder();
  } catch (e) {
    console.error('Failed to load settings:', e);
  }

  // Language change — apply immediately
  document.getElementById('language').addEventListener('change', (e) => {
    setLanguage(e.target.value);
    applyTranslations();
    updateTextPlaceholder();
  });

  // Browse GIF button
  document.getElementById('browse-btn').addEventListener('click', async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: 'GIF', extensions: ['gif'] }],
      });
      if (selected) {
        document.getElementById('gif-path').value = selected;
      }
    } catch (e) {
      console.error('Failed to open file dialog:', e);
    }
  });

  // Clear GIF button
  document.getElementById('clear-gif-btn').addEventListener('click', () => {
    document.getElementById('gif-path').value = '';
  });

  // Pet mode toggle shows/hides direction field
  document.getElementById('pet-mode').addEventListener('change', toggleDirectionField);

  // Save button
  document.getElementById('save-btn').addEventListener('click', saveSettings);

  // Test button
  document.getElementById('test-btn').addEventListener('click', async () => {
    await saveSettings();
    try {
      const petMode = document.getElementById('pet-mode').checked;
      await invoke(petMode ? 'show_pet' : 'show_reminder');
    } catch (e) {
      console.error('Failed to show test:', e);
      showStatus(t('msg.testFailed') + e, true);
    }
  });
}

function updateTextPlaceholder() {
  document.getElementById('custom-text').placeholder = t('reminder.default');
}

async function saveSettings() {
  const btn = document.getElementById('save-btn');
  btn.disabled = true;
  btn.textContent = t('btn.saving');

  try {
    if (!store) {
      store = await load('settings.json', { autoSave: false });
    }

    await store.set('language', document.getElementById('language').value);
    await store.set('interval_minutes', parseInt(document.getElementById('interval').value, 10));
    await store.set('custom_text', document.getElementById('custom-text').value);
    await store.set('gif_path', document.getElementById('gif-path').value);
    await store.set('pet_mode', document.getElementById('pet-mode').checked);
    await store.set('gif_direction', document.querySelector('input[name="gif-direction"]:checked').value);
    await store.set('auto_dismiss_seconds', parseInt(document.getElementById('auto-dismiss').value, 10));
    await store.save();

    // Notify backend to update timer
    await invoke('update_timer_settings');

    showStatus(t('msg.saved'));
  } catch (e) {
    console.error('Failed to save settings:', e);
    showStatus(t('msg.saveFailed') + e, true);
  } finally {
    btn.disabled = false;
    btn.textContent = t('btn.save');
  }
}

function toggleDirectionField() {
  const petMode = document.getElementById('pet-mode').checked;
  document.getElementById('gif-direction-field').style.display = petMode ? '' : 'none';
}

function showStatus(msg, isError) {
  let statusEl = document.getElementById('status-msg');
  if (!statusEl) {
    statusEl = document.createElement('div');
    statusEl.id = 'status-msg';
    document.querySelector('.actions').appendChild(statusEl);
  }
  statusEl.textContent = msg;
  statusEl.style.color = isError ? '#e53935' : '#43a047';
  statusEl.style.fontSize = '14px';
  statusEl.style.marginTop = '8px';

  if (!isError) {
    setTimeout(() => { statusEl.textContent = ''; }, 3000);
  }
}

init();
