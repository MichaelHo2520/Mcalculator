## 1. 專案設定與腳手架

- [x] 1.1 建立 Cargo workspace（workspace Cargo.toml, `calc-core` lib crate, `calc-app` bin crate）
- [x] 1.2 初始化 Tauri 專案結構（`tauri.conf.json`、`src-tauri/` 目錄、`src-tauri/src/main.rs`）
- [x] 1.3 將現有 `calculator.html`、`style.css`、`icon.png` 複製到 Tauri 的前端資源目錄
- [x] 1.4 設定 `tauri.conf.json`：視窗大小 860×650、最小尺寸、標題 `"麥克計算機 V1.0"`、隱藏選單列
- [x] 1.5 確認 `cargo tauri dev` 可成功啟動並顯示 HTML 頁面

## 2. 運算式解析核心（calc-core）

- [x] 2.1 定義 Token 列舉（Num, Hex, Op, BitOp, Fn, Const, LParen, RParen, Factorial）
- [x] 2.2 實作 Tokenizer：將字串分解為 Token 序列，支援數字、HEX(0x)、運算子、函數名、PI
- [x] 2.3 實作隱式乘法注入：在 Token 序列中適當位置自動插入乘號 Token
- [x] 2.4 定義 AST 節點列舉（Num, BinOp, UnaryOp, FnCall, Factorial）
- [x] 2.5 實作 Recursive Descent Parser：token 序列 → AST，遵守運算子優先順序
- [x] 2.6 實作括號自動補全邏輯（preview 模式下）
- [x] 2.7 為 Tokenizer 撰寫單元測試（至少涵蓋：四則、hex、函數、PI、括號）
- [x] 2.8 為 Parser 撰寫單元測試（優先順序、巢狀表達式、隱式乘法）

## 3. AST 求值器（calc-core）

- [x] 3.1 實作 Evaluator：遞迴走訪 AST，計算結果，回傳 `Result<f64, EvalError>`
- [x] 3.2 實作科學函數（sin, cos, tan, log, exp, sqrt）
- [x] 3.3 實作角度單位切換（弧度 ↔ 角度，角度模式時自動轉換 trig 輸入）
- [x] 3.4 實作階乘函數（非負整數，負數回傳 NaN）
- [x] 3.5 實作位元運算（^, |, &）
- [x] 3.6 為 Evaluator 撰寫單元測試（複合運算式、位元運算、trig 弧度/角度、不完整表達式容錯）

## 4. 數值格式化（calc-core）

- [x] 4.1 實作 `to_hex(val, bit_depth)` → String：整數部分套用 bit-depth 遮罩後轉大寫 HEX
- [x] 4.2 實作 `to_dec(val, bit_depth)` → String：整數顯示遮罩值，浮點顯示完整值
- [x] 4.3 實作 Bit-Depth 遮罩邏輯（64/32/16 位元 BigInt AND 運算）
- [x] 4.4 為格式化函數撰寫單元測試（各 bit-depth 下的 hex/dec 輸出、溢位截斷、負數二補數）

## 5. calc-core 公共 API

- [x] 5.1 實作 `evaluate(expression, bit_depth, is_degree)` → `EvalResult { hex, dec, error }` 統一入口
- [x] 5.2 為統一 API 撰寫整合測試（端到端：字串輸入 → hex/dec 輸出）

## 6. Tauri 後端整合（calc-app）

- [x] 6.1 實作 Tauri command `evaluate`：接收 JS invoke 參數，呼叫 calc-core，回傳 JSON 結果
- [x] 6.2 定義 Tauri command 的序列化結構（EvaluateRequest, EvaluateResponse）
- [x] 6.3 在 `main.rs` 中註冊 command handler
- [x] 6.4 測試 Tauri command IPC 可正確接收參數和回傳結果

## 7. 前端 JS 遷移

- [x] 7.1 修改 `script.js`：將 `evaluate()` 函數中的 `eval()` 替換為 `window.__TAURI__.invoke('evaluate', ...)`
- [x] 7.2 處理 invoke 的 async/await 回傳，更新 HEX/DEC 顯示
- [x] 7.3 移除所有 Electron 相關程式碼（`main.js` 不再需要）
- [x] 7.4 確認所有 UI 互動保持正常（按鈕、radio buttons、主題切換、鍵盤隱藏）
- [x] 7.5 確認即時預覽在 Tauri 下正常運作（每次按鍵觸發 invoke）

## 8. 打包與驗收

- [x] 8.1 設定 Tauri bundler 產出 portable exe（非 installer）
- [x] 8.2 執行 `cargo tauri build`，確認產出 exe 可獨立執行
- [x] 8.3 驗證 exe 體積小於 15MB
- [x] 8.4 端到端功能驗收：四則運算、位元運算、科學函數、HEX/DEC、Bit-Depth、角度切換、主題切換
