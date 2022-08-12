#[macro_use]
extern crate lazy_static;

mod program;
mod top;
mod windows;
mod registry;
mod tray;

use crate::registry::Registry;
use std::sync::Mutex;
use winapi::{
    shared::windef::HHOOK,
    um::{
        winuser,
        winuser::{ShowWindow, HC_ACTION, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN, SW_HIDE},
        wincon::GetConsoleWindow,
    },
};

static mut HOOK_HANDLE: Option<HHOOK> = None;

lazy_static! {
    static ref SELECTED: Mutex<String> = Mutex::new(String::new());
}

fn main() {
    let registry = Registry::new();
    registry.create();

    SELECTED.lock().unwrap().push_str(program::get().as_str());
    let selected = SELECTED.lock().unwrap().to_string();
    registry.set(selected.clone()).unwrap();

    unsafe {
        let hook_id = winuser::SetWindowsHookExA(
            WH_KEYBOARD_LL,
            Some(hook_callback),
            std::ptr::null_mut(),
            0,
        );
        HOOK_HANDLE = Some(hook_id);

        // hide app
        hide_app();

        // show app running in system tray
        tray::tray(selected);
    }
}

/// winapi methods for Keyboard inputs
// https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms644985(v=vs.85)
extern "system" fn hook_callback(code: i32, wparam: usize, lparam: isize) -> isize {
    if code < HC_ACTION {
        unsafe {
            return if let Some(hook_id) = HOOK_HANDLE {
                winuser::CallNextHookEx(hook_id, code, wparam, lparam)
            } else {
                0
            };
        }
    }

    let keypress: KBDLLHOOKSTRUCT = unsafe { *(lparam as *mut KBDLLHOOKSTRUCT) };

    if wparam == WM_KEYDOWN as usize {
        if from_virtual_key_code(keypress.vkCode) {
            top::assign(SELECTED.lock().unwrap().to_string())
        }
    }
    0
}

// assuming nordic QWERTY layout
// if match char: 1, 2, 3, 4
// TODO: change this
fn from_virtual_key_code(code: u32) -> bool {
    match code {
        49..=52 => true,
        _ => false,
    }
}

// detach console from view
unsafe fn hide_app() {
    let curr_app = GetConsoleWindow();
    if !curr_app.is_null() {
        ShowWindow(curr_app, SW_HIDE);
    }
}