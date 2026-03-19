## Context

KK計算機（現名為麥克計算機）目前是 Electron 桌面應用（HTML/JS/CSS），核心計算邏輯在 `script.js` 中透過字串替換 + `eval()` 實現。打包體積約 150MB。

現有程式碼結構：
- `calculator/main.js` — Electron 主程序 (37 行)
- `calculator/calculator.html` — UI 結構 (133 行)
- `calculator/script.js` — 計算邏輯 (225 行)
- `calculator/style.css` — 雙主題樣式 (401 行)

目標：遷移到 Tauri + Rust，重用 HTML/CSS 前端，以 Rust 實作安全的計算核心。

## Goals / Non-Goals

**Goals:**
- 1:1 復刻所有現有計算功能（四則運算、位元運算、科學函數、HEX/DEC 顯示、Bit-Depth 遮罩）
- 以安全的 Recursive Descent Parser 取代 `eval()` 字串替換
- 打包體積從 ~150MB 降至 ~5-10MB (portable exe)
- 保留 Neumorphism + Dark Mode 雙主題視覺效果
- 保留即時預覽（每次按鍵即時計算）

**Non-Goals:**
- 不新增任何新計算功能
- 不改變 UI 佈局或視覺設計
- 不支援 macOS/Linux（僅 Windows exe）
- 不做 CI/CD 自動化
- 不做安裝程式（portable exe 即可）

## Decisions

### Decision 1: 使用 Tauri 而非 iced/egui

**選擇**: Tauri 2.x

**理由**:
- 可完全重用現有 HTML/CSS/JS 前端，大幅減少 UI 重寫工作量
- 打包體積只需約 5-10MB（使用系統 WebView2）
- 視覺品質：可保留原始的 Neumorphism/Dark Mode CSS，不需在 Rust GUI 框架中重建
- 學習曲線低：前端不需變化，僅需學習 Tauri IPC

**替代方案**:
- **iced**: 純 Rust，但需完全重建 UI（~15-20h 額外工作），且 Neumorphism 效果極難復刻
- **egui**: 外觀過於陽春，不符合精緻 UI 需求

### Decision 2: 自建 Parser 而非使用現有 crate

**選擇**: 自建 Recursive Descent Parser

**理由**:
- 計算機語法相對簡單，自建更靈活
- 需要自訂 HEX 字面值偵測（裸 `FF` vs `0xFF`）
- 需要括號自動補全支援（即時預覽）
- 可完全掌控錯誤處理

**替代方案**:
- **nom / pest**: 通用 parser 庫，對此場景 overkill
- **evalexpr crate**: 不支援 hex、bit operations、自訂函數

### Decision 3: 計算核心作為獨立 library crate

**選擇**: Cargo workspace — `calc-core` (lib) + `calc-app` (bin/Tauri)

**理由**:
- 計算邏輯可獨立單元測試，不依賴 GUI
- 日後可被其他 GUI 框架或 CLI 工具重用
- 關注點分離清晰

### Decision 4: 前端 IPC 設計

**選擇**: 單一 Tauri command `evaluate`，接收運算式 + 設定，回傳 HEX/DEC 結果

```
invoke('evaluate', {
  expression: "2+sin(PI/4)",
  bitDepth: 64,
  isDegree: false
}) → { hex: "4", dec: "4.707..." }
```

**理由**:
- 簡化前端改動：僅需將 `eval()` 呼叫替換為 `invoke()`
- 所有計算邏輯集中在 Rust 端
- 前端 JS 只負責 UI 互動和顯示

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| WebView2 在某些 Windows 版本可能缺失 | Tauri bundler 可選擇嵌入 WebView2 installer |
| Parser 對不完整表達式（即時預覽）容錯 | 設計 Parser 回傳 `Result<f64, ParseError>`，前端遇 Err 保留上次結果 |
| Tauri IPC 延遲影響即時預覽體驗 | 計算極輕量，IPC 延遲可忽略（<1ms） |
| HEX 自動偵測可能與函數名衝突 | Token 階段明確區分：已知函數名不作為 hex 處理 |
