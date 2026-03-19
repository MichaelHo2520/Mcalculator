## Why

KK計算機（現名為麥克計算機）目前以 Electron (HTML/JS/CSS) 架構運行，打包後體積約 150MB+，對一個計算機應用而言過於龐大。將後端邏輯遷移至 Rust + Tauri 框架，可將打包體積降至 ~5-10MB，同時保留精緻的 Neumorphism/Dark Mode 雙主題 UI（重用現有 HTML/CSS 前端），並消除 `eval()` 帶來的安全隱患，改用安全的運算式解析器。

## What Changes

- **新增 Rust 計算核心**：安全的運算式解析器（Tokenizer → AST → Evaluator），取代 JS `eval()` 字串替換
- **新增 Tauri 桌面外殼**：取代 Electron，提供 WebView 容器 + Rust 後端 IPC 通訊
- **修改前端 JS 邏輯**：將 `script.js` 中的計算邏輯改為透過 Tauri `invoke()` 呼叫 Rust 後端
- **保留前端 UI 層**：`calculator.html` 和 `style.css` 基本不需改動
- **移除 Electron 依賴**：不再需要 `main.js` (Electron main process)
- **產出 portable exe**：單一 Windows 可執行檔

## Capabilities

### New Capabilities
- `expression-parser`: Rust 運算式解析器——Tokenizer、遞迴下降 Parser、AST 求值，支援四則運算、位元運算、科學函數、HEX 字面值、隱式乘法、括號自動補全
- `number-formatter`: 數值格式化引擎——HEX/DEC 輸出、Bit-Depth (64/32/16) 遮罩
- `tauri-shell`: Tauri 桌面外殼——WebView 前端容器、Rust 後端 IPC 命令、視窗設定、打包設定
- `frontend-integration`: 前端整合層——將計算邏輯從 JS `eval()` 遷移至 Tauri invoke，保留 UI 和主題系統

### Modified Capabilities
（無現有 spec 需修改）

## Impact

- **新增依賴**：Rust toolchain、Tauri CLI、tauri crate
- **移除依賴**：Electron、Node.js runtime
- **打包產物變更**：從 Electron 安裝包 (~150MB) → Tauri portable exe (~5-10MB)
- **前端影響最小**：HTML/CSS 完全保留，JS 僅計算呼叫路徑改變
- **安全性提升**：消除 `eval()` 注入風險
