# Times Up, Stand Up!

定時提醒你站起來動一動的桌面小工具。常駐系統匣，時間到了在右下角彈出 GIF + 自訂文字提醒。

支援 Windows / macOS / Linux。

## 功能

- 系統匣常駐，不佔桌面空間
- 可自訂提醒間隔（預設 30 分鐘）
- 可自訂提醒 GIF 和文字
- 兩種提醒模式（二選一）：
  - **提醒模式** — 右下角無框彈窗，自動消失或點擊關閉
  - **桌寵模式** — GIF 角色隨機出現在螢幕上，四處走動後自動消失
- 桌寵模式支援設定 GIF 初始朝向（朝左/朝右），確保移動方向與圖片一致

## 一般提醒模式截圖

![image](https://github.com/user-attachments/assets/5a9a1a81-b719-4323-a967-4c359ad35121)

## 桌寵模式截圖

桌寵會在螢幕上隨意走動，點桌寵可以消滅它(笑)。

![image](https://github.com/user-attachments/assets/502f53c8-fced-45fc-b118-6d6c8161ee30)

## 安裝

### 下載安裝檔

到 [Releases](../../releases) 下載對應平台的安裝檔：

| 平台 | 檔案 |
|------|------|
| Windows | `times-up-stand-up_x.x.x_x64-setup.exe` |
| macOS (Apple Silicon) | `times-up-stand-up_x.x.x_aarch64.dmg` |
| macOS (Intel) | `times-up-stand-up_x.x.x_x64.dmg` |
| Linux | `.deb` 或 `.AppImage` |

### macOS 安裝注意

macOS 可能會提示「無法打開，因為無法驗證開發者」。請使用以下方式開啟：

1. 在 Finder 中**右鍵**點擊 app → 選擇**打開** → 點擊**打開**
2. 或執行：`xattr -cr /Applications/times-up-stand-up.app`

### 從原始碼建置

需要先安裝：

- [Rust](https://rustup.rs/)
- [Node.js](https://nodejs.org/) 18+
- Tauri CLI：`cargo install tauri-cli --version "^2"`
- Linux 額外需要：`libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf`

```bash
git clone https://github.com/你的帳號/times-up-stand-up.git
cd times-up-stand-up
npm install
cargo tauri build
```

產出的安裝檔在 `src-tauri/target/release/bundle/` 底下。

## 使用方式

1. 啟動程式後，系統匣（右下角）會出現圖示
2. 雙擊圖示可開啟設定視窗
3. 右鍵圖示可以：
   - **暫停 / 繼續** 計時
   - **重置計時** 重新開始倒數
   - **設定** 調整間隔、GIF、提醒文字
   - **測試提醒** 立即彈一次看效果（依目前模式顯示提醒或桌寵）
   - **桌寵模式：開/關** 快速切換提醒模式
   - **退出** 關閉程式

## 技術架構

- **Tauri v2** — 桌面應用框架（Rust + WebView）
- **Rust** — 系統匣、計時器、視窗管理
- **HTML/CSS/JS + Vite** — 設定介面和提醒彈窗

```
系統匣右鍵 → Rust (tray) → 暫停/重置/設定/退出
計時器到期 → Rust (timer) → 檢查模式
  ├─ 提醒模式 → reminder.html (右下角 GIF + 文字)
  └─ 桌寵模式 → pet.html (GIF 在螢幕上隨意走動)
設定視窗   → HTML/JS ←invoke→ Rust → settings.json
```

## License

MIT
