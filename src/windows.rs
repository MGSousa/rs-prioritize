use winapi::{
    shared::{
        minwindef::LPARAM,
        windef::HWND,
    },
    um::winuser::{EnumChildWindows, EnumWindows, GetWindowThreadProcessId},
};
use windows_error::WindowsError;

pub fn get_window_by_pid(pid: u32) -> Result<Option<HWND>, WindowsError> {
    let mut found_window: Option<HWND> = None;

    let res = enum_windows_by_until(None, |handle: HWND| {
        let (process_pid, _) = get_windows_thread_process_id(handle);
        if process_pid == pid {
            found_window = Some(handle);
            return 0;
        }
        1
    });
    if res.is_err() {
        res.err().unwrap();
    }
    Ok(found_window)
}

fn enum_windows_by_until<T: FnMut(HWND) -> i32>(
    parent: Option<HWND>,
    mut cmp_func: T,
) -> Result<(), WindowsError> {
    let lparam = &mut cmp_func as *mut _ as LPARAM;
    let result: i32;

    if let Some(parent_window) = parent {
        result = unsafe {
            EnumChildWindows(
                parent_window,
                Some(callback_enum_windows_until::<T>),
                lparam,
            )
        };
    } else {
        result = unsafe {
            EnumWindows(Some(callback_enum_windows_until::<T>), lparam)
        };
    }

    if result == 0 {
        return Err(WindowsError::from_last_err());
    }
    Ok(())
}

fn get_windows_thread_process_id(window: HWND) -> (u32, u32) {
    let mut process_pid: u32 = 0;
    let thread_pid = unsafe {
        GetWindowThreadProcessId(window, &mut process_pid)
    };
    (process_pid, thread_pid)
}

unsafe extern "system" fn callback_enum_windows_until<T: FnMut(HWND)
    -> i32>(window: HWND, param: LPARAM) -> i32 {
    let func = &mut *(param as *mut T);
    func(window)
}
