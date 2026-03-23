---
description: 分析 git 變更並產出標準化的中文 Conventional Commit Message
---

分析目前的 git 變更並產出符合 Conventional Commits 規範的中文 Commit Message。

**步驟**：

1.  **取得 git 變更資訊**：
    *   優先執行 `git diff --cached` 取得已暫存的變更。
    *   若無暫存變更，則執行 `git diff` 取得未暫存的變更。
    *   如果都沒有變更，告知使用者目前沒有修改。
    *   *(提示：若遇到中文路徑亂碼，可嘗試 `git -c core.quotepath=false diff`)*

2.  **分析變更**：
    *   閱讀 diff 內容，整理出變更的核心邏輯與目的。
    *   分類為 `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, 或 `chore`。

3.  **格式化訊息 (中文)**：
    *   **標題**：`<type>(<scope>): <簡短描述>`。
    *   **內文**：使用 `- ` 條列具體變更點。

4.  **提供結果**：
    *   將產出的 Commit Message 放在程式碼區塊中提供給使用者。
    *   詢問使用者是否直接採納或需要微調。
