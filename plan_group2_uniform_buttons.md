# Group2 按鈕統一大小修改計劃

## 目標

Group2（運算符區域，2x4 grid，共 8 格）的所有按鈕，無論算術模式或位元模式，視覺尺寸與外觀必須完全一致。空白按鈕保留顯示、保持與其他按鈕相同外觀，僅無功能（不可點擊）。

## 需修改的檔案

僅修改 `ui/style.css`，共 3 處。不修改 HTML 與 JS。

---

### 修改 1：空白按鈕外觀統一（不再半透明）

**檔案**：`ui/style.css` 第 494-499 行

**問題**：`.btn.op-blank` 設了 `opacity: 0.18`，導致空白按鈕幾乎看不見，與其他按鈕大小外觀不一致。

**修改方式**：移除 `opacity: 0.18`，讓空白按鈕保持與一般按鈕相同的背景、陰影、圓角外觀，僅保留 `pointer-events: none` 禁止點擊。

```css
/* 修改前 */
.btn.op-blank,
.btn.op-blank:disabled {
    pointer-events: none;
    cursor: default;
    opacity: 0.18;
}

/* 修改後 */
.btn.op-blank,
.btn.op-blank:disabled {
    pointer-events: none;
    cursor: default;
}
```

---

### 修改 2：切換按鈕 `⇄` 字體大小統一

**檔案**：`ui/style.css` 第 501-504 行

**問題**：`.btn.switch-page` 單獨設了 `font-size: 13px`，比同組其他按鈕的 `clamp(18px, 2.8vw, 24px)`（繼承自 `.btn`）明顯偏小。

**修改方式**：移除 `font-size: 13px` 這一行，讓它繼承 `.btn` 的預設字體大小。

```css
/* 修改前 */
.btn.switch-page {
    color: var(--color-func);
    font-size: 13px;
}

/* 修改後 */
.btn.switch-page {
    color: var(--color-func);
}
```

---

### 修改 3：位元模式按鈕字體大小統一

**檔案**：`ui/style.css` 第 547-550 行

**問題**：`.btn.bit` 設了 `font-size: 13px`，切換到位元模式後按鈕文字（≪、≫、And、Or、Xor、NOT）明顯比算術模式的符號小。

**修改方式**：將 `font-size` 從固定 `13px` 改為 `clamp(14px, 2.2vw, 18px)`。因為位元按鈕文字較長（如 And、Xor、NOT），不宜與純符號按鈕用完全相同的大字體，但需縮小差距。

```css
/* 修改前 */
.btn.bit {
    color: var(--color-bit);
    font-size: 13px;
}

/* 修改後 */
.btn.bit {
    color: var(--color-bit);
    font-size: clamp(14px, 2.2vw, 18px);
}
```

---

## 不需改動的部分

- `ui/index.html`：Group2 已是 8 格 grid 結構，無需增減按鈕
- `ui/script.js`：`opModes` 配置與 `applyOpMode()` 邏輯不需變更
- Group1、Group3、Group4：不受此修改影響

## 驗證方式

1. 開啟計算機，確認 Group2 的 8 個按鈕（含空白）視覺大小一致
2. 點擊 `⇄` 切換至位元模式，確認所有按鈕大小仍一致
3. 確認空白按鈕可見但不可點擊
4. 切換深色/淺色主題，確認兩種主題下外觀皆正常
