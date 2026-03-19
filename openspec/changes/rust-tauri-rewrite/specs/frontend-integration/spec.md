## ADDED Requirements

### Requirement: 計算邏輯遷移至 Tauri invoke
前端 `script.js` SHALL 將 `evaluate()` 函數中的 `eval()` 呼叫替換為 Tauri `invoke('evaluate', ...)` 非同步呼叫，傳送運算式和設定，接收 HEX/DEC 結果後更新顯示。

#### Scenario: 按鈕輸入觸發計算
- **WHEN** 使用者點擊按鈕 `"7"` 後點擊 `"+"`，再點擊 `"3"`
- **THEN** 前端即時透過 Tauri invoke 傳送 `"7+3"`，取得並顯示 HEX=`"A"` / DEC=`"10"`

#### Scenario: 即時預覽
- **WHEN** 使用者在輸入框打字 `"sin("`
- **THEN** 前端每次按鍵後呼叫 Tauri invoke，若回傳 error 則保留上次有效結果

---

### Requirement: 保留所有 UI 互動
前端 SHALL 保留以下所有互動功能，不做任何改變：
- 按鈕面板（Hex/數字/運算子/科學/功能按鍵）
- Bit-Depth 切換（64/32/16 radio buttons）
- 角度單位切換（徑/角 radio buttons）
- 主題切換（Neumorphism ↔ Dark Mode）
- 鍵盤面板顯示/隱藏切換
- 鍵盤快捷鍵（Enter/Backspace/Escape + 字元輸入）
- BKS（退格）和 CLR（清除）按鈕

#### Scenario: 主題切換保持正常
- **WHEN** 使用者點擊 🎨 按鈕
- **THEN** UI 在 Neumorphism 與 Dark Mode 之間切換

#### Scenario: 鍵盤隱藏/顯示
- **WHEN** 使用者點擊鍵盤切換按鈕
- **THEN** 按鈕面板在顯示與隱藏之間切換

---

### Requirement: 移除 Electron 依賴
前端 SHALL NOT 依賴任何 Electron API（`require('electron')`、`nodeIntegration`）。`main.js` (Electron) SHALL 被 Tauri 取代。

#### Scenario: 無 Node.js API 使用
- **WHEN** 檢查前端程式碼
- **THEN** 不存在 `require()`、`process`、`module.exports` 等 Node.js API 呼叫
