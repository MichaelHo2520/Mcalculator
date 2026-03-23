document.addEventListener('DOMContentLoaded', () => {
    const hexValue = document.getElementById('hex-value');
    const decValue = document.getElementById('dec-value');
    const inputField = document.getElementById('expression-input');
    const buttons = document.querySelectorAll('.btn');
    const cTypeSelect = document.getElementById('c-type-select');
    const hexOvf = document.getElementById('hex-ovf');
    const decOvf = document.getElementById('dec-ovf');
    const historyPanel = document.getElementById('history-panel');
    const historyListContainer = document.getElementById('history-list');
    const dropdownArrow = document.querySelector('.dropdown-arrow');

    let expression = "";
    let isDegree = false;
    let isCalculated = false;       // 上次按 = 後設為 true
    let historyList = [];           // [{expression, result, cType, timestamp}]
    const MAX_HISTORY = 50;

    // Settings Management
    const DEFAULT_SETTINGS = {
        collapsedH: 165,
        expandedH: 600,
        windowW: 540,
        isDarkTheme: false,
        selectedType: 'int64',
        isDegree: false
    };

    function getSettings() {
        const saved = localStorage.getItem('calc_settings');
        if (saved) {
            try {
                const parsed = JSON.parse(saved);
                return { ...DEFAULT_SETTINGS, ...parsed };
            } catch (e) {}
        }
        return { ...DEFAULT_SETTINGS };
    }

    function saveSettings(settings) {
        localStorage.setItem('calc_settings', JSON.stringify(settings));
    }

    // New Helper: Parse C Type
    function parseCType(cType) {
        if (cType === 'f32' || cType === 'f64') {
            return {
                bitDepth: cType === 'f32' ? 32 : 64,
                isSigned: true,
                isFloat: true
            };
        }
        const isSigned = cType.startsWith('int');
        const bitDepth = parseInt(cType.replace(/^u?int/, ''));
        return { bitDepth, isSigned, isFloat: false };
    }

    // New Helper: Insert at Cursor
    function insertAtCursor(text) {
        const start = inputField.selectionStart || 0;
        const end = inputField.selectionEnd || 0;
        expression = expression.substring(0, start) + text + expression.substring(end);
        inputField.value = expression;
        const newPos = start + text.length;
        inputField.setSelectionRange(newPos, newPos);
    }

    // Event Listeners
    cTypeSelect.addEventListener('change', () => {
        const settings = getSettings();
        saveSettings({ ...settings, selectedType: cTypeSelect.value });
        evaluate(true);
        inputField.focus();
    });

    const unitToggleBtn = document.getElementById('unit-toggle-btn');

    function updateUnitBtnDisplay() {
        if (!unitToggleBtn) return;
        if (isDegree) {
            unitToggleBtn.innerHTML = '角度<br>(°)';
            unitToggleBtn.title = '三角函數單位：\n' +
                '角度 (Degree)\n' +
                '常見角度表示方式\n' +
                '例如：sin(90°) = 1\n\n' +
                '轉換公式：\n' +
                '弧度 = 角度 × π/180\n' +
                '角度 = 弧度 × 180/π\n\n' +
                '點擊切換為弧度模式';
        } else {
            unitToggleBtn.innerHTML = '弧度<br>(rad)';
            unitToggleBtn.title = '三角函數單位：\n' +
                '弧度 (Radian)\n' +
                '數學與程式預設單位\n' +
                '例如：sin(π/2) = 1\n\n' +
                '轉換公式：\n' +
                '弧度 = 角度 × π/180\n' +
                '角度 = 弧度 × 180/π\n\n' +
                '點擊切換為角度模式';
        }
    }

    if (unitToggleBtn) {
        unitToggleBtn.addEventListener('click', () => {
            isDegree = !isDegree;
            updateUnitBtnDisplay();
            
            const settings = getSettings();
            saveSettings({ ...settings, isDegree: isDegree });
            
            evaluate(true);
            inputField.focus();
        });
    }

    buttons.forEach(btn => {
        btn.addEventListener('click', () => {
            const val = btn.getAttribute('data-val');
            handleInput(val);
        });
    });

    function isOperatorChar(ch) {
        return ['+', '-', '*', '/', '%', '&', '|', '^'].includes(ch);
    }

    function handleInput(val) {
        if (!val) return;

        if (val === 'entry-clear') {
            const start = inputField.selectionStart || 0;
            const end = inputField.selectionEnd || 0;
            if (start !== end) {
                expression = expression.substring(0, start) + expression.substring(end);
                inputField.value = expression;
                inputField.setSelectionRange(start, start);
            } else if (start > 0) {
                expression = expression.substring(0, start - 1) + expression.substring(start);
                inputField.value = expression;
                inputField.setSelectionRange(start - 1, start - 1);
            }
            isCalculated = false;
        } else if (val === 'clear-all') {
            expression = "";
            inputField.value = "";
            isCalculated = false;
        } else if (val === '=') {
            evaluate(false);
        } else if (/[0-9A-F.]/.test(val) || val === '0x') {
            if (isCalculated) {
                expression = "";
                inputField.value = "";
                isCalculated = false;
            }
            insertAtCursor(val);
        } else if (val === '(' || val === ')') {
            if (isCalculated && val === '(') {
                expression = "";
                inputField.value = "";
                isCalculated = false;
            }
            insertAtCursor(val);
        } else {
            // Operator or function
            if (isCalculated) {
                isCalculated = false;
            }
            // Simple double-operator prevention
            const cursorPos = inputField.selectionStart || expression.length;
            if (cursorPos > 0 && val.length === 1 && isOperatorChar(val)) {
                const prevChar = expression[cursorPos - 1];
                if (isOperatorChar(prevChar)) {
                    expression = expression.substring(0, cursorPos - 1) + expression.substring(cursorPos);
                    inputField.value = expression;
                    inputField.setSelectionRange(cursorPos - 1, cursorPos - 1);
                }
            }
            insertAtCursor(val);
        }
        
        inputField.focus();
        tryUpdateQuickDisplay();
    }

    // Capture keyboard input
    document.addEventListener('keydown', (e) => {
        const key = e.key;

        if (key === 'Enter') {
            e.preventDefault();
            evaluate(false);
            return;
        } else if (key === 'Backspace') {
            // Let the input field handle it if focused, otherwise handle manually
            if (document.activeElement !== inputField) {
                e.preventDefault();
                handleInput('entry-clear');
            } else {
                isCalculated = false;
            }
            return;
        } else if (key === 'Escape') {
            e.preventDefault();
            handleInput('clear-all');
            return;
        }

        // Handle mathematical characters (only single chars)
        if (key.length === 1 && /[0-9a-fA-F\+\-\*\/\.\(\)\%\&\|\^]/.test(key)) {
            if (document.activeElement !== inputField) {
                e.preventDefault();
                handleInput(key.toUpperCase());
            } else {
                // If calculated, clear first
                if (isCalculated && /[0-9a-fA-F.]/.test(key)) {
                    expression = "";
                    inputField.value = "";
                    isCalculated = false;
                } else if (isCalculated) {
                    isCalculated = false;
                }
            }
        }
    });

    // Manual input in text field
    inputField.addEventListener('input', (e) => {
        expression = e.target.value;
        evaluate(true);
    });

    // Keyboard toggle
    const toggleBtn = document.getElementById('toggle-keyboard');
    const buttonPanel = document.querySelector('.button-panel');
    let isKeyboardVisible = false;
    let isHistoryVisible = false;

    toggleBtn.addEventListener('click', async () => {
        isKeyboardVisible = !isKeyboardVisible;
        
        // Hide/show the currently active panel
        const tabBits = document.getElementById('tab-bits');
        const isBitPanelActive = tabBits && tabBits.classList.contains('active');
        const bitPanel = document.getElementById('bit-panel');
        
        if (isBitPanelActive) {
            bitPanel.classList.toggle('hidden', !isKeyboardVisible);
            buttonPanel.classList.add('hidden');
        } else {
            buttonPanel.classList.toggle('hidden', !isKeyboardVisible);
            if (bitPanel) bitPanel.classList.add('hidden');
        }
        
        // Hide tabs wrapper when collapsed
        const tabsRow = document.getElementById('middle-tabs-row');
        if (tabsRow) tabsRow.classList.toggle('hidden', !isKeyboardVisible);
        
        await updateWindowSize();
    });

    let isAutoResizing = false;
    let resizeTimeout = null;

    async function updateWindowSize() {
        const settings = getSettings();
        // 如果鍵盤顯示 或 歷史紀錄顯示，就用展開高度
        const shouldExpand = isKeyboardVisible || isHistoryVisible;
        const targetH = shouldExpand ? settings.expandedH : settings.collapsedH;
        
        isAutoResizing = true;
        
        try {
            await invoke('toggle_keyboard', { visible: shouldExpand, targetLogicalH: targetH });
        } catch (e) {
            console.error("Failed to update window size", e);
        } finally {
            // 防呆：給予系統足夠時間完成 resize 事件傳遞
            setTimeout(() => {
                isAutoResizing = false;
            }, 300);
        }
    }

    // 監聽使用者手動拖曳 OS 邊框高度
    window.addEventListener('resize', () => {
        if (isAutoResizing) return;

        if (resizeTimeout) clearTimeout(resizeTimeout);

        resizeTimeout = setTimeout(() => {
            if (isAutoResizing) return; // 再次確認防呆標記

            const currentInnerHeight = Math.max(160, Math.min(800, window.innerHeight));
            const settings = getSettings();
            
            // 判斷當下處於哪種高度模式
            const isExpandedDataMode = isKeyboardVisible || isHistoryVisible;

            if (isExpandedDataMode) {
                settings.expandedH = currentInnerHeight;
                expandedInput.value = currentInnerHeight;
            } else {
                settings.collapsedH = currentInnerHeight;
                collapsedInput.value = currentInnerHeight;
            }

            // Save current width
            settings.windowW = window.innerWidth;
            if (windowWInput) windowWInput.value = window.innerWidth;

            saveSettings(settings);
            
        }, 150);
    });

    // Modal & Settings UI Logic
    const settingsToggle = document.getElementById('settings-toggle');
    const settingsModal = document.getElementById('settings-modal');
    const collapsedInput = document.getElementById('collapsed-h-input');
    const windowWInput = document.getElementById('window-w-input');
    const expandedInput = document.getElementById('expanded-h-input');
    const saveBtn = document.getElementById('save-settings');
    const resetBtn = document.getElementById('reset-settings');
    const cancelBtn = document.getElementById('cancel-settings');

    settingsToggle.addEventListener('click', () => {
        const settings = getSettings();
        collapsedInput.value = settings.collapsedH;
        windowWInput.value = settings.windowW || 540;
        expandedInput.value = settings.expandedH;
        settingsModal.classList.remove('hidden');
    });

    saveBtn.addEventListener('click', async () => {
        const current = getSettings();
        const newSettings = {
            ...current,
            collapsedH: parseFloat(collapsedInput.value) || DEFAULT_SETTINGS.collapsedH,
            windowW: parseFloat(windowWInput.value) || DEFAULT_SETTINGS.windowW,
            expandedH: parseFloat(expandedInput.value) || DEFAULT_SETTINGS.expandedH,
            isDarkTheme: document.body.classList.contains('theme-dark')
        };
        saveSettings(newSettings);
        settingsModal.classList.add('hidden');
        
        // Apply width immediately
        try {
            await invoke('init_window', { targetLogicalW: newSettings.windowW });
        } catch (e) {}

        await updateWindowSize();
        inputField.focus();
    });

    resetBtn.addEventListener('click', () => {
        collapsedInput.value = DEFAULT_SETTINGS.collapsedH;
        windowWInput.value = DEFAULT_SETTINGS.windowW;
        expandedInput.value = DEFAULT_SETTINGS.expandedH;
    });

    cancelBtn.addEventListener('click', () => {
        settingsModal.classList.add('hidden');
        inputField.focus();
    });

    // Theme toggle
    const themeBtn = document.getElementById('theme-toggle-btn');
    if (themeBtn) {
        themeBtn.addEventListener('click', () => {
            document.body.classList.toggle('theme-dark');
            const isDark = document.body.classList.contains('theme-dark');
            const settings = getSettings();
            saveSettings({ ...settings, isDarkTheme: isDark });
            inputField.focus();
        });
    }

    function tryUpdateQuickDisplay() {
        evaluate(true);
    }

    const tauri = window.__TAURI__;
    const invoke = tauri && tauri.core ? tauri.core.invoke : (tauri ? tauri.invoke : null);

    // OVF Helpers
    function showOvf() {
        hexOvf.style.display = 'block';
        decOvf.style.display = 'block';
    }
    function hideOvf() {
        hexOvf.style.display = 'none';
        decOvf.style.display = 'none';
    }

    // History Management
    function loadHistory() {
        const saved = localStorage.getItem('calc_history');
        if (saved) {
            try { historyList = JSON.parse(saved); } catch(e) { historyList = []; }
        }
    }
    function saveHistory() {
        localStorage.setItem('calc_history', JSON.stringify(historyList));
    }
    function addHistory(expr, result, cType) {
        // Prevent duplicate consecutive entries
        if (historyList.length > 0 && historyList[0].expression === expr && historyList[0].result === result) return;
        
        historyList.unshift({
            expression: expr,
            result: result,
            cType: cType,
            timestamp: Date.now()
        });
        if (historyList.length > MAX_HISTORY) historyList.pop();
        saveHistory();
    }
    function renderHistory() {
        historyListContainer.innerHTML = '';
        historyList.forEach(item => {
            const div = document.createElement('div');
            div.className = 'history-item';
            div.innerHTML = `
                <span class="hist-expr">${item.expression}</span>
                <span class="hist-result">= ${item.result}</span>
                <span class="hist-type">${item.cType}</span>
            `;
            div.addEventListener('click', () => {
                expression = item.expression;
                inputField.value = expression;
                isCalculated = false;
                hideHistoryPanel();
                evaluate(true);
                inputField.focus();
            });
            historyListContainer.appendChild(div);
        });
    }

    dropdownArrow.addEventListener('click', async (e) => {
        e.stopPropagation();
        const isCurrentlyVisible = historyPanel.style.display !== 'none';
        if (isCurrentlyVisible) {
            await hideHistoryPanel();
        } else {
            loadHistory();
            renderHistory();
            historyPanel.style.display = 'block';
            isHistoryVisible = true;
            await updateWindowSize();
        }
    });

    const clearHistoryBtn = document.getElementById('clear-history');
    if (clearHistoryBtn) {
        clearHistoryBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            historyList = [];
            saveHistory();
            renderHistory();
        });
    }

    async function hideHistoryPanel() {
        if (!isHistoryVisible) return;
        historyPanel.style.display = 'none';
        isHistoryVisible = false;
        await updateWindowSize();
        inputField.focus();
    }

    document.addEventListener('click', (e) => {
        if (isHistoryVisible && !historyPanel.contains(e.target) && e.target !== dropdownArrow) {
            hideHistoryPanel();
        }
    });

    // Initialization
    async function init() {
        const settings = getSettings();
        if (settings.isDarkTheme) {
            document.body.classList.add('theme-dark');
        }
        
        // Restore Type selection
        if (settings.selectedType) {
            cTypeSelect.value = settings.selectedType;
        }
        
        // Restore Unit selection
        isDegree = !!settings.isDegree;
        updateUnitBtnDisplay();

        loadHistory();
        
        // Restore window width
        try {
            await invoke('init_window', { targetLogicalW: settings.windowW || 540 });
        } catch (e) {}

        try {
            await updateWindowSize();
        } catch (e) {}

        try {
            await invoke('show_window');
        } catch (e) {}
    }
    init().catch(console.error);

    async function evaluate(isPreview = false) {
        if (!expression) {
            hexValue.textContent = "0";
            decValue.textContent = "0";
            hideOvf();
            return;
        }

        const { bitDepth, isSigned, isFloat } = parseCType(cTypeSelect.value);

        try {
            const result = await invoke('evaluate', {
                expression: expression.toString(),
                bitDepth: bitDepth,
                isSigned: isSigned,
                isDegree: isDegree,
                isFloat: isFloat
            });

            if (result.error) {
                decValue.textContent = "無效輸入";
                hexValue.textContent = "---";
                hideOvf();
            } else {
                hexValue.textContent = result.hex;
                decValue.textContent = result.dec;
                
                if (result.overflowed) { showOvf(); } else { hideOvf(); }

                if (!isPreview) {
                    addHistory(expression, result.dec, cTypeSelect.value);
                    expression = result.dec;
                    inputField.value = expression;
                    isCalculated = true;
                }
                
                updateBitPanel();
            }
        } catch (e) {
            decValue.textContent = "無效輸入";
            hexValue.textContent = "---";
            hideOvf();
        }
    }

    // Tabs & Bit Display Logic
    const tabKeyboard = document.getElementById('tab-keyboard');
    const tabBits = document.getElementById('tab-bits');
    const bitPanel = document.getElementById('bit-panel');
    let bitCells = [];

    function switchTab(isBitPanel) {
        if (!isKeyboardVisible) return; // Cannot switch if panel is hidden entirely
        
        if (isBitPanel) {
            tabKeyboard.classList.remove('active');
            tabBits.classList.add('active');
            buttonPanel.classList.add('hidden');
            bitPanel.classList.remove('hidden');
        } else {
            tabBits.classList.remove('active');
            tabKeyboard.classList.add('active');
            bitPanel.classList.add('hidden');
            buttonPanel.classList.remove('hidden');
        }
    }

    if (tabKeyboard && tabBits) {
        tabKeyboard.addEventListener('click', () => switchTab(false));
        tabBits.addEventListener('click', () => switchTab(true));
    }

    function createBitPanel() {
        if (!bitPanel) return;
        bitPanel.innerHTML = '';
        bitCells = [];

        let currentBit = 63;
        for (let r = 0; r < 4; r++) {
            const rowDiv = document.createElement('div');
            rowDiv.className = 'bit-row';

            for (let g = 0; g < 4; g++) {
                const groupDiv = document.createElement('div');
                groupDiv.className = 'bit-group';

                const cellsDiv = document.createElement('div');
                cellsDiv.className = 'bit-cells';

                const lowestBit = currentBit - 3;
                for (let b = 0; b < 4; b++) {
                    const bitIndex = currentBit;
                    const cell = document.createElement('button');
                    cell.className = 'bit-cell';
                    cell.textContent = '0';
                    cell.onclick = () => toggleBit(bitIndex);
                    cellsDiv.appendChild(cell);
                    bitCells[bitIndex] = cell; // index 0 is least significant bit
                    currentBit--;
                }
                groupDiv.appendChild(cellsDiv);

                const label = document.createElement('div');
                label.className = 'bit-label';
                label.textContent = lowestBit;
                groupDiv.appendChild(label);

                rowDiv.appendChild(groupDiv);
            }
            bitPanel.appendChild(rowDiv);

            if (r === 1) { // between 2nd and 3rd row -> bits 31 and 32
                const divider = document.createElement('div');
                divider.className = 'h-divider';
                bitPanel.appendChild(divider);
            }
        }
    }

    function getBinStrFromHex(hexStr, bitDepth) {
        hexStr = hexStr.replace(/^0x/i, '').replace(/[^0-9A-Fa-f]/ig, '');
        if (!hexStr) return '0'.repeat(64);
        let bin = '';
        for (let i = 0; i < hexStr.length; i++) {
            bin += parseInt(hexStr[i], 16).toString(2).padStart(4, '0');
        }
        // Pad to required bits and then to 64
        return bin.padStart(bitDepth, '0').padStart(64, '0');
    }

    function updateBitPanel() {
        if (!bitPanel || bitCells.length === 0) return;

        const { bitDepth } = parseCType(cTypeSelect.value);
        const hexText = hexValue.textContent === '---' ? '0' : hexValue.textContent;
        const binStr = getBinStrFromHex(hexText, bitDepth);

        for (let i = 0; i < 64; i++) {
            const cell = bitCells[i];
            if (!cell) continue;

            if (i >= bitDepth) {
                cell.textContent = '0';
                cell.classList.remove('active-bit');
                cell.classList.add('inactive-bit');
            } else {
                cell.classList.remove('inactive-bit');

                // index 0 of binStr is MSB
                const isOne = binStr[63 - i] === '1';
                cell.textContent = isOne ? '1' : '0';
                
                if (isOne) {
                    cell.classList.add('active-bit');
                } else {
                    cell.classList.remove('active-bit');
                }
            }
        }
    }

    async function toggleBit(bitIndex) {
        const { bitDepth } = parseCType(cTypeSelect.value);
        if (bitIndex >= bitDepth) return;

        const hexText = hexValue.textContent === '---' ? '0' : hexValue.textContent;
        let binStr = getBinStrFromHex(hexText, bitDepth);

        const strIndex = 63 - bitIndex;
        const newBit = binStr[strIndex] === '1' ? '0' : '1';
        binStr = binStr.substring(0, strIndex) + newBit + binStr.substring(strIndex + 1);

        const activeBits = binStr.slice(64 - bitDepth);

        let newHex = '';
        let paddedActiveBits = activeBits.padStart(Math.ceil(activeBits.length / 4) * 4, '0');
        for (let i = 0; i < paddedActiveBits.length; i += 4) {
            const nibble = paddedActiveBits.slice(i, i + 4);
            newHex += parseInt(nibble, 2).toString(16).toUpperCase();
        }

        const newExpr = '0x' + (newHex || '0');
        
        expression = newExpr;
        inputField.value = newExpr;
        isCalculated = true; // Act as if evaluated manually
        
        await evaluate(false);
    }

    // Call createBitPanel on load
    createBitPanel();
    updateBitPanel();

    // Disable right-click context menu
    document.addEventListener('contextmenu', (e) => {
        e.preventDefault();
    });

    // Global Click-to-Focus (僅限真正點擊到背景區域時)
    const refocusHandler = (e) => {
        if (e.target === e.currentTarget && e.target !== inputField) {
            setTimeout(() => inputField.focus(), 10);
        }
    };

    // 只有點擊到這些容器本身（不包含子元素）才回焦
    ['calculator-body', 'calculator-container', 'display-section', 'input-section', 'input-wrapper'].forEach(cls => {
        const el = document.querySelector('.' + cls);
        if (el) el.addEventListener('mousedown', refocusHandler);
    });
});
