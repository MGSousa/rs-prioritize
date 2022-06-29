use crate::windows::get_window_by_pid;
use std::{ffi::CString, mem, process::Command, process::exit, sync::Mutex};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use winapi::{
    shared::windef::HWND,
    um::winuser::{FindWindowA, INPUT, INPUT_KEYBOARD, SendInput, ShowWindow, SetForegroundWindow}
};

lazy_static! {
    static ref WINDOW: Mutex<String> = Mutex::new(String::new());
}

pub fn assign(window: String) {
    let mut window_name = format_name(window);
    if WINDOW.lock().unwrap().to_string() != "" {
        window_name = format_name(WINDOW.lock().unwrap().to_string());
    }
    if window_name.to_str().unwrap().trim() == "" {
        println!("using process name");
        exit(1);
    }
    println!("window: {:?}", window_name);

    let handler = unsafe { FindWindowA(std::ptr::null_mut(), window_name.as_ptr()) };
    if handler.is_null() {
        match window_name.to_str() {
            Ok(name) => {
                if find_process(name) == false {
                    let res = Command::new("explorer").arg(name).spawn();
                    match res {
                        Ok(child) => {
                            println!("created new PID: {}", child.id());
                        }
                        Err(e) => println!("error: {}", e),
                    }
                }
            }
            _ => {}
        }
    } else {
        unsafe { set_top(handler) };
    }
}

fn format_name(window: String) -> CString {
    CString::new(window).unwrap()
}

fn find_process(name: &str) -> bool {
    let mut sys = System::new_all();

    // sync all information of our `System` struct.
    sys.refresh_all();

    let process_name = name.split("\\").collect::<Vec<&str>>();
    for (pid, process) in sys.processes() {
        if process.name() == process_name[process_name.len() - 1] {
            let handler = get_window_by_pid(pid.as_u32())
                .unwrap()
                .unwrap();

            unsafe { set_top(handler) };
            println!("[{}] {}", pid, process.name());
            return true;
        }
    }
    false
}

unsafe fn set_top(handler: HWND) {
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: Default::default(),
    };
    SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
    ShowWindow(handler, 9);
    SetForegroundWindow(handler);
}
