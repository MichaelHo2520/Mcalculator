---
description: 分析 git 變更並產出標準化的中文 Conventional Commit Message
---
# 產出中文 Git Commit 訊息 (Generate Chinese Git Commit Message)

這個技能會分析工作區或暫存區中的 `git diff` 變更，並根據 Conventional Commits 規範產出結構清晰、語意明確的中文 Commit Message 建議供使用者參考。**此技能不會主動執行 `git commit`。**

## 執行步驟 (Workflow)

1.  **收集變更資訊**：
    *   執行 `git status` 了解目前有哪些檔案被修改或暫存。
    *   執行 `git diff --cached` 取得已暫存 (staged) 的變更。若沒有已暫存的變更，則執行 `git diff` 取得未暫存的變更。
    *   *(注意：如果碰到中文編碼問題，請考慮使用 `git -c core.quotepath=false diff` 或將輸出導向到暫存檔後再讀取。)*

2.  **分析與歸納**：
    *   仔細閱讀 diff 內容，理解變更的實際作用。
    *   將變更按照功能模組或性質進行分類。
    *   提煉出變更的 **「原因 (Why)」** 與 **「具體行為 (What)」**，**避免**逐行翻譯程式碼 (例如：不要寫「將 x 變數從 1 改成 2」，而是寫「調整預設倒數計時秒數」)。

3.  **格式化訊息 (Formatting)**：
    使用以下的 Conventional Commits 格式撰寫中文訊息：

    *   **標題 (Title)**：`<類型>(<可選範圍>): <簡短描述>` (建議不超過 50 個字元)
        *   **類型 (Type)** 限制如下：
            *   `feat`: 新增功能 (Feature)
            *   `fix`: 修復錯誤 (Bug fix)
            *   `docs`: 文件修改 (Documentation)
            *   `style`: 程式碼格式微調 (不影響運作，如空白、排版)
            *   `refactor`: 程式碼重構 (不新增功能也不修復錯誤)
            *   `perf`: 效能優化 (Performance)
            *   `test`: 新增或修改測試 (Tests)
            *   `chore`: 建置過程、輔助工具或套件管理變動 (Chores)
    *   **內文 (Body)**：
        *   使用列點式 (`- `) 條列說明。
        *   若有多個重要變更，請分點描述。

4.  **輸出結果**：
    *   將產出的草稿直接回覆給使用者，並使用程式碼區塊 (code block) 封裝，以便使用者直接複製。

## 輸出範例 (Example Output Format)

為您產出的 Commit 訊息草稿如下，您可以直接複製使用：

```text
feat(UI): 新增複製前一日功能並優化提示訊息

- 新增 handle_copy_previous_day 功能，支援自動略過非 Pass 的日期。
- 加入 QMessageBox 提示視窗，於複製成功後提醒使用者。
- 更新全域版本號 APP_VERSION 至 V1.17。
```
