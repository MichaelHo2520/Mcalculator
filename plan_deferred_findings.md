# Deferred Findings (007-010) 修改計劃

## 目標

修復 design audit 報告中 4 項 deferred findings，提升可及性與設計一致性。僅修改 `ui/style.css`，不修改 HTML 與 JS，不影響任何功能。

## 實施順序

按風險由低到高，每項獨立 commit，格式：`style(design): FINDING-0XX — 簡述`

---

### 修改 1：FINDING-008 — 54px 魔術數字 token 化

**檔案**：`ui/style.css`

**問題**：`54px` 在 3 處重複出現（right-panel、side-control、group-4），未使用 CSS variable。

**修改方式**：在 `:root` 新增 `--sidebar-width: 54px`，替換所有硬編碼的 `54px`。

**步驟 A**：在 `:root` 區塊（第 1-55 行）的 `--font-family` 之前新增變數

```css
/* 在 --divider-color-subtle 之後加入 */
--sidebar-width: 54px;
```

**步驟 B**：修改第 231-232 行 `.right-panel`

```css
/* 修改前 */
.right-panel {
    flex: 0 0 54px;
    min-width: 54px;

/* 修改後 */
.right-panel {
    flex: 0 0 var(--sidebar-width);
    min-width: var(--sidebar-width);
```

**步驟 C**：修改第 241-242 行 `.side-control`

```css
/* 修改前 */
.side-control {
    flex: 0 0 54px;
    min-width: 54px;

/* 修改後 */
.side-control {
    flex: 0 0 var(--sidebar-width);
    min-width: var(--sidebar-width);
```

**步驟 D**：修改第 542 行 `.group-4`

```css
/* 修改前 */
.group-4 { grid-template-columns: repeat(1, 1fr); grid-template-rows: repeat(4, 1fr); flex: 0 0 54px; min-width: 54px; }

/* 修改後 */
.group-4 { grid-template-columns: repeat(1, 1fr); grid-template-rows: repeat(4, 1fr); flex: 0 0 var(--sidebar-width); min-width: var(--sidebar-width); }
```

**風險**：零。數值完全不變，只是抽成 token。

---

### 修改 2：FINDING-010 — `--color-hex` 對比度提升至 WCAG AA

**檔案**：`ui/style.css` 第 38 行

**問題**：`--color-hex: #798d99` 在 `#e0e5ec` 背景上對比度僅 ~3.5:1，未達 WCAG AA 標準（4.5:1）。影響 HEX 按鈕（A-F）、tab 文字、bit-label。

**修改方式**：僅修改 light theme 的 `:root`，dark theme（第 93 行）不動。

```css
/* 修改前 */
--color-hex: #798d99;

/* 修改後 */
--color-hex: #5e7682;
```

**說明**：`#5e7682` 在 `#e0e5ec` 上對比度約 4.6:1，色相不變（藍灰），僅降低明度。Dark theme 第 93 行的 `--color-hex` 有獨立值，不受影響。

**風險**：低。HEX 按鈕和 tab 文字稍深，更易讀。

---

### 修改 3：FINDING-009 — 9px 字體提升至 10px

**檔案**：`ui/style.css` 共 3 處

**問題**：`9px` 低於 WCAG 建議最小可讀性。出現在 ovf-badge、trunc-badge、bit-label。

**步驟 A**：修改第 276 行 `.ovf-badge`

```css
/* 修改前 */
font-size: 9px;

/* 修改後 */
font-size: 10px;
```

**步驟 B**：修改第 298 行 `.trunc-badge`

```css
/* 修改前 */
font-size: 9px;

/* 修改後 */
font-size: 10px;
```

**步驟 C**：修改第 855 行 `.bit-label`

```css
/* 修改前 */
font-size: 9px;

/* 修改後 */
font-size: 10px;
```

**風險**：極低。Badge 為 absolute positioned，不影響排版。bit-label 在 25% 寬格子中，10px 仍有空間。

---

### 修改 4：FINDING-007 — 觸控目標保障 44px 最低限

**檔案**：`ui/style.css` 共 2 處

**問題**：按鈕尺寸由 flex/grid 隱性決定，小視窗下可能低於 44px 觸控目標標準。

**步驟 A**：修改第 559-561 行 `.btn`，在 `width: 100%;` 之後加入 `min-height`

```css
/* 修改前 */
.btn {
    height: 100%;
    width: 100%;
    background-color: var(--btn-bg);

/* 修改後 */
.btn {
    height: 100%;
    width: 100%;
    min-height: 44px;
    background-color: var(--btn-bg);
```

**步驟 B**：修改第 544-547 行 `.group-2 .btn, .group-3 .btn` 的 `min-width`

```css
/* 修改前 */
.group-2 .btn,
.group-3 .btn {
    min-width: 36px;
}

/* 修改後 */
.group-2 .btn,
.group-3 .btn {
    min-width: 44px;
}
```

**風險**：低。只設下限，大視窗下按鈕本就超過 44px。需目測確認小視窗（375px 寬）下 group-4 不溢出。

---

## 不需改動的部分

- `ui/index.html`：不新增、不刪除、不修改任何 HTML 元素
- `ui/script.js`：不動任何 JavaScript 邏輯
- Dark theme 的 `--color-hex`（第 93 行）：深色背景下對比度已足夠
- 已修復的 FINDING-001~006：不重複處理

## 驗證方式

1. 每次修改後切換 light/dark 主題，確認外觀正常
2. 確認所有按鈕可點擊、計算結果正確（功能未受影響）
3. 用 DevTools 檢查按鈕 rendered size >= 44px（修改 4 之後）
4. 用對比度工具確認 `#5e7682` on `#e0e5ec` >= 4.5:1（修改 2 之後）
5. 確認 badge（OVF/TRUNC）在 10px 下未超出容器（修改 3 之後）
