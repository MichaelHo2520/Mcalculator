#![windows_subsystem = "windows"]

use serde::Serialize;
// AppConfig removed - storage migrated to frontend localStorage

// File operations removed - config is now managed by the frontend

#[derive(Serialize)]
struct EvaluateResponse {
    hex: String,
    dec: String,
    error: Option<String>,
    overflowed: bool,
}

#[tauri::command]
fn evaluate(expression: String, bit_depth: u32, is_signed: bool, is_degree: bool, is_float: Option<bool>) -> EvaluateResponse {
    let is_float = is_float.unwrap_or(false);
    let res = calc_core::evaluate(&expression, bit_depth, is_signed, is_degree, is_float);
    EvaluateResponse {
        hex: res.hex,
        dec: res.dec,
        error: res.error,
        overflowed: res.overflowed,
    }
}

// ── Win32 FFI (only on Windows) ──────────────────────────────────────
#[cfg(target_os = "windows")]
mod win32 {
    #[repr(C)]
    #[derive(Default)]
    pub struct RECT {
        pub left: i32,
        pub top: i32,
        pub right: i32,
        pub bottom: i32,
    }

    pub const SWP_NOMOVE: u32 = 0x0002;
    pub const SWP_NOZORDER: u32 = 0x0004;

    extern "system" {
        pub fn GetWindowRect(hwnd: isize, rect: *mut RECT) -> i32;
        pub fn GetClientRect(hwnd: isize, rect: *mut RECT) -> i32;
        pub fn SetWindowPos(
            hwnd: isize,
            insert_after: isize,
            x: i32,
            y: i32,
            cx: i32,
            cy: i32,
            flags: u32,
        ) -> i32;
    }
}

/// Resize the window height via Win32 API, preserving exact current width.
/// This bypasses Tauri's DPI-related size rounding issues entirely.
#[tauri::command]
fn toggle_keyboard(_visible: bool, target_logical_h: f64, window: tauri::Window) {
    #[cfg(target_os = "windows")]
    {
        let hwnd = match window.hwnd() {
            Ok(h) => h.0 as isize,
            Err(_) => return,
        };

        let factor = window.scale_factor().unwrap_or(1.0);

        let mut rect = win32::RECT::default();
        let mut client = win32::RECT::default();

        unsafe {
            win32::GetWindowRect(hwnd, &mut rect);
            win32::GetClientRect(hwnd, &mut client);
        }

        // Compute window chrome (title bar + border) height
        let chrome_h = (rect.bottom - rect.top) - (client.bottom - client.top);

        // Keep the outer width EXACTLY as-is — never touch it
        let outer_w = rect.right - rect.left;

        // Desired inner (client area) height in physical pixels provided by frontend
        let new_inner_h = (target_logical_h * factor).round() as i32;
        let new_outer_h = new_inner_h + chrome_h;

        unsafe {
            win32::SetWindowPos(
                hwnd,
                0,    // ignored (SWP_NOZORDER)
                0, 0, // ignored (SWP_NOMOVE)
                outer_w,
                new_outer_h,
                win32::SWP_NOMOVE | win32::SWP_NOZORDER,
            );
        }
    }
}

#[tauri::command]
fn show_window(window: tauri::Window) {
    let _ = window.show();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![evaluate, toggle_keyboard, show_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
