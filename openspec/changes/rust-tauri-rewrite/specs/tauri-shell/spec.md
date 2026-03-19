## ADDED Requirements

### Requirement: Tauri 應用視窗
系統 SHALL 建立 Tauri 應用視窗，載入現有 `calculator.html` 前端：
- 預設視窗大小：860×650 px
- 最小視窗大小：860×650 px
- 可調整大小：是
- 視窗標題：`"麥克計算機 V1.0"`
- 自動隱藏選單列

#### Scenario: 應用啟動
- **WHEN** 執行 exe
- **THEN** 顯示計算機視窗，載入 `calculator.html`，視窗大小為 860×650

---

### Requirement: Tauri IPC evaluate 命令
系統 SHALL 提供 Tauri command `evaluate`，接收以下參數：
- `expression: String` — 運算式字串
- `bit_depth: u8` — 位元深度 (64/32/16)
- `is_degree: bool` — 是否為角度模式

回傳結構：
- `hex: String` — HEX 格式化結果
- `dec: String` — DEC 格式化結果
- `error: Option<String>` — 錯誤訊息（如有）

#### Scenario: 成功計算
- **WHEN** 前端呼叫 `invoke('evaluate', { expression: "2+3", bit_depth: 64, is_degree: false })`
- **THEN** 回傳 `{ hex: "5", dec: "5", error: null }`

#### Scenario: 無效運算式
- **WHEN** 前端呼叫 `invoke('evaluate', { expression: "2++", bit_depth: 64, is_degree: false })`
- **THEN** 回傳 `{ hex: "---", dec: "無效輸入", error: "ParseError: ..." }`

---

### Requirement: 打包為 portable exe
系統 SHALL 使用 `tauri build` 產出單一 Windows 可執行檔（portable exe），不需安裝程式。

#### Scenario: 打包產出
- **WHEN** 執行 `cargo tauri build`
- **THEN** 產出可在 Windows 上直接執行的 `.exe` 檔案，體積小於 15MB
