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
        collapsedH: 170,
        expandedH: 330,
        isDarkTheme: false
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
        const isSigned = cType.startsWith('int');
        const bitDepth = parseInt(cType.replace(/^u?int/, ''));
        return { bitDepth, isSigned };
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
        evaluate(true);
    });

    document.querySelectorAll('input[name="unit"]').forEach(radio => {
        radio.addEventListener('change', (e) => {
            isDegree = e.target.value === 'deg';
            evaluate(true);
        });
    });

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

    toggleBtn.addEventListener('click', async () => {
        isKeyboardVisible = !isKeyboardVisible;
        buttonPanel.classList.toggle('hidden', !isKeyboardVisible);
        await updateWindowSize();
    });

    async function updateWindowSize() {
        const settings = getSettings();
        const targetH = isKeyboardVisible ? settings.expandedH : settings.collapsedH;
        try {
            await invoke('toggle_keyboard', { visible: isKeyboardVisible, targetLogicalH: targetH });
        } catch (e) {
            console.error("Failed to toggle keyboard window size", e);
        }
    }

    // Modal & Settings UI Logic
    const settingsToggle = document.getElementById('settings-toggle');
    const settingsModal = document.getElementById('settings-modal');
    const collapsedInput = document.getElementById('collapsed-h-input');
    const expandedInput = document.getElementById('expanded-h-input');
    const saveBtn = document.getElementById('save-settings');
    const resetBtn = document.getElementById('reset-settings');
    const cancelBtn = document.getElementById('cancel-settings');

    settingsToggle.addEventListener('click', () => {
        const settings = getSettings();
        collapsedInput.value = settings.collapsedH;
        expandedInput.value = settings.expandedH;
        settingsModal.classList.remove('hidden');
    });

    saveBtn.addEventListener('click', async () => {
        const newSettings = {
            collapsedH: parseFloat(collapsedInput.value) || DEFAULT_SETTINGS.collapsedH,
            expandedH: parseFloat(expandedInput.value) || DEFAULT_SETTINGS.expandedH,
            isDarkTheme: document.body.classList.contains('theme-dark')
        };
        saveSettings(newSettings);
        settingsModal.classList.add('hidden');
        await updateWindowSize();
    });

    resetBtn.addEventListener('click', () => {
        collapsedInput.value = DEFAULT_SETTINGS.collapsedH;
        expandedInput.value = DEFAULT_SETTINGS.expandedH;
    });

    cancelBtn.addEventListener('click', () => {
        settingsModal.classList.add('hidden');
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
                expression = item.result;
                inputField.value = expression;
                isCalculated = false;
                hideHistoryPanel();
                evaluate(true);
            });
            historyListContainer.appendChild(div);
        });
    }

    dropdownArrow.addEventListener('click', (e) => {
        e.stopPropagation();
        const isVisible = historyPanel.style.display !== 'none';
        if (isVisible) {
            hideHistoryPanel();
        } else {
            loadHistory();
            renderHistory();
            historyPanel.style.display = 'block';
        }
    });

    function hideHistoryPanel() {
        historyPanel.style.display = 'none';
    }

    document.addEventListener('click', (e) => {
        if (!historyPanel.contains(e.target) && e.target !== dropdownArrow) {
            hideHistoryPanel();
        }
    });

    // Initialization
    async function init() {
        const settings = getSettings();
        if (settings.isDarkTheme) {
            document.body.classList.add('theme-dark');
        }
        loadHistory();

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

        const { bitDepth, isSigned } = parseCType(cTypeSelect.value);

        try {
            const result = await invoke('evaluate', {
                expression: expression.toString(),
                bitDepth: bitDepth,
                isSigned: isSigned,
                isDegree: isDegree
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
            }
        } catch (e) {
            decValue.textContent = "無效輸入";
            hexValue.textContent = "---";
            hideOvf();
        }
    }
});
