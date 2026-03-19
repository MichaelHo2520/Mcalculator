document.addEventListener('DOMContentLoaded', () => {
    const hexValue = document.getElementById('hex-value');
    const decValue = document.getElementById('dec-value');
    const inputField = document.getElementById('expression-input');
    const buttons = document.querySelectorAll('.btn');

    let expression = "";
    let bitDepth = 64;
    let isDegree = false;

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

    // Radio buttons handlers
    document.querySelectorAll('input[name="bit-depth"]').forEach(radio => {
        radio.addEventListener('change', (e) => {
            bitDepth = parseInt(e.target.value);
            evaluate(true);
        });
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

    function handleInput(val) {
        if (!val) return;

        const start = inputField.selectionStart || 0;
        const end = inputField.selectionEnd || 0;

        if (val === 'entry-clear') {
            if (start !== end) {
                // Delete selected range
                expression = expression.substring(0, start) + expression.substring(end);
                inputField.value = expression;
                inputField.setSelectionRange(start, start);
            } else if (start > 0) {
                // Delete one char before cursor
                expression = expression.substring(0, start - 1) + expression.substring(start);
                inputField.value = expression;
                inputField.setSelectionRange(start - 1, start - 1);
            }
        } else if (val === 'clear-all') {
            expression = "";
            inputField.value = expression;
        } else if (val === '=') {
            evaluate();
        } else {
            // Insert at cursor position
            expression = expression.substring(0, start) + val + expression.substring(end);
            inputField.value = expression;
            inputField.setSelectionRange(start + val.length, start + val.length);
        }
        
        // Keep focus on input field
        inputField.focus();

        // Auto evaluate if it's a simple number or if user wants real-time?
        // Let's stick to manual evaluate or simple real-time for DEC/HEX if it's just numbers
        tryUpdateQuickDisplay();
    }

    // Capture keyboard input
    document.addEventListener('keydown', (e) => {
        const key = e.key;

        // Handle functional keys first
        if (key === 'Enter') {
            e.preventDefault();
            evaluate();
            return;
        } else if (key === 'Backspace') {
            e.preventDefault();
            expression = expression.slice(0, -1);
            inputField.value = expression;
            evaluate(true);
            return;
        } else if (key === 'Escape') {
            e.preventDefault();
            expression = "";
            inputField.value = expression;
            evaluate(true);
            return;
        }

        // Handle mathematical characters (only single chars)
        // Include 'a-z' for scientific functions like sin, cos, log, etc.
        if (key.length === 1 && /[0-9a-zA-Z\+\-\*\/\.\(\)]/.test(key)) {
            // If focused on input field, let the default input happen and the 'input' event will handle it
            if (document.activeElement !== inputField) {
                expression += key; // Keep case as typed for functions
                inputField.value = expression;
                evaluate(true);
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
            expandedH: parseFloat(expandedInput.value) || DEFAULT_SETTINGS.expandedH
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
            inputField.focus(); // Keep focus when clicking theme button
        });
    }

    function tryUpdateQuickDisplay() {
        evaluate(true);
    }

    const tauri = window.__TAURI__;
    const invoke = tauri && tauri.core ? tauri.core.invoke : (tauri ? tauri.invoke : null);

    // Initialization: Set correct size and then show window (Anti-Flicker)
    async function init() {
        const settings = getSettings();
        if (settings.isDarkTheme) {
            document.body.classList.add('theme-dark');
        }

        // Try to update size first
        try {
            await updateWindowSize();
        } catch (e) {
            console.error("Initial size update failed", e);
        }

        // Always try to show the window even if size update failed
        try {
            await invoke('show_window');
        } catch (e) {
            console.error("Failed to show window", e);
        }
    }
    init().catch(console.error);

    async function evaluate(isPreview = false) {
        if (!expression) {
            hexValue.textContent = "0";
            decValue.textContent = "0";
            return;
        }

        try {
            const result = await invoke('evaluate', {
                expression: expression.toString(),
                bitDepth: bitDepth,
                isDegree: isDegree
            });

            if (result.error) {
                decValue.textContent = "無效輸入";
                hexValue.textContent = "---";
            } else {
                hexValue.textContent = result.hex;
                decValue.textContent = result.dec;
                if (!isPreview) {
                    expression = result.dec;
                    inputField.value = expression;
                }
            }
        } catch (e) {
            decValue.textContent = "無效輸入";
            hexValue.textContent = "---";
        }
    }

    // obsolete functions removed

    function updateDisplays() {
        evaluate(true);
    }
});
