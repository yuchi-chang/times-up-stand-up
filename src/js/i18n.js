const translations = {
  tw: {
    'app.title': '站立提醒設定',
    'label.language': '語言',
    'label.interval': '提醒間隔（分鐘）',
    'label.text': '提醒文字',
    'label.gif': '自訂 GIF',
    'label.dismiss': '自動消失秒數',
    'label.pet': '桌寵模式（GIF 在螢幕上隨意走動）',
    'label.direction': 'GIF 初始朝向',
    'direction.right': '朝右 →',
    'direction.left': '← 朝左',
    'btn.browse': '選擇檔案',
    'btn.clear': '清除',
    'btn.save': '儲存',
    'btn.test': '測試提醒',
    'btn.saving': '儲存中...',
    'msg.saved': '已儲存！',
    'msg.saveFailed': '儲存失敗: ',
    'msg.testFailed': '測試失敗: ',
    'placeholder.gif': '使用預設 GIF',
    'reminder.default': '時間到了！站起來動一動！',
  },
  en: {
    'app.title': 'Stand-up Reminder Settings',
    'label.language': 'Language',
    'label.interval': 'Reminder Interval (minutes)',
    'label.text': 'Reminder Text',
    'label.gif': 'Custom GIF',
    'label.dismiss': 'Auto-dismiss (seconds)',
    'label.pet': 'Desktop Pet (GIF walks around screen)',
    'label.direction': 'GIF Facing Direction',
    'direction.right': 'Right →',
    'direction.left': '← Left',
    'btn.browse': 'Browse',
    'btn.clear': 'Clear',
    'btn.save': 'Save',
    'btn.test': 'Test',
    'btn.saving': 'Saving...',
    'msg.saved': 'Saved!',
    'msg.saveFailed': 'Save failed: ',
    'msg.testFailed': 'Test failed: ',
    'placeholder.gif': 'Use default GIF',
    'reminder.default': "Time's up! Stand up and stretch!",
  },
};

let currentLang = 'tw';

export function setLanguage(lang) {
  currentLang = lang;
}

export function t(key) {
  return (translations[currentLang] && translations[currentLang][key]) ||
    translations['tw'][key] || key;
}

export function applyTranslations() {
  document.querySelectorAll('[data-i18n]').forEach(el => {
    el.textContent = t(el.dataset.i18n);
  });
  document.querySelectorAll('[data-i18n-placeholder]').forEach(el => {
    el.placeholder = t(el.dataset.i18nPlaceholder);
  });
}
