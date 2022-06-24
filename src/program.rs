use dialoguer::{theme::ColorfulTheme, Select};
use std::io::stdin;
use winapi::{
    shared::{
        minwindef::{BOOL, LPARAM},
        windef::HWND,
    },
    um::{
        winnt::LPWSTR,
        winuser::{EnumWindows, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible},
    },
};

pub fn get() -> String {
    let mut custom_program = String::new();
    let program_titles = get_program_titles().unwrap();

    let index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select desired window")
        .default(1)
        .items(program_titles.as_slice())
        .interact()
        .unwrap();

    if index == 0 {
        println!("Specify the program that you want to open:");
        stdin()
            .read_line(&mut custom_program)
            .expect("incorrect program");
        if let Some('\n') = custom_program.chars().next_back() {
            custom_program.pop();
        }
        if let Some('\r') = custom_program.chars().next_back() {
            custom_program.pop();
        }
        custom_program
    } else {
        program_titles[index].to_owned()
    }
}

fn get_program_titles() -> Result<Vec<String>, ()> {
    // TODO: save custom program to registry, then next time is showed with others
    let state: Box<Vec<String>> = Box::new(vec![String::from("Open Custom Program")]);
    let ptr = Box::into_raw(state);
    let state;

    unsafe {
        EnumWindows(Some(enumerate_windows), ptr as LPARAM);
        state = Box::from_raw(ptr);
    }
    Ok(*state)
}

unsafe extern "system" fn enumerate_windows(window: HWND, state: LPARAM) -> BOOL {
    if IsWindowVisible(window) == 0 {
        return true.into();
    }

    let state = state as *mut Vec<String>;
    let mut len = GetWindowTextLengthW(window);
    if len == 0 {
        return true.into();
    }

    len = len + 1;
    let mut title: Vec<u16> = vec![0; len as usize];
    let tw = GetWindowTextW(window, title.as_mut_ptr() as LPWSTR, len);
    if tw != 0 {
        if let Ok(title) = String::from_utf16(title[0..(tw as usize)].as_ref()) {
            (*state).push(title);
        }
    }
    true.into()
}
