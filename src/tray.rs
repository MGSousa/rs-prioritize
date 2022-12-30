use std::sync::atomic::{AtomicBool, Ordering};
use trayicon::{Icon, MenuBuilder, MenuItem, TrayIcon, TrayIconBuilder};
use winit::{
    event::{Event},
    event_loop::{ControlFlow, EventLoop}
};
use crate::registry::Registry;

#[derive(Clone, Eq, PartialEq)]
enum Events {
    Exit,
    Pause,
    Resume,
    Switch
}

const DEFAULT_ORDER: Ordering = Ordering::Relaxed;

lazy_static! {
    pub static ref PAUSED: AtomicBool = AtomicBool::new(false);
    static ref RESET: AtomicBool = AtomicBool::new(false);
}

trait MainTray {
    fn set_state(&mut self, pause: bool, state: &Icon);
}

impl MainTray for TrayIcon<Events> {
    fn set_state(&mut self, pause: bool, state: &Icon) {
        PAUSED.swap(pause, Ordering::Relaxed);
        self::TrayIcon::set_icon(self, state).unwrap();
        self::TrayIcon::set_menu(self, &build_opts(!pause)).unwrap()
    }
}

fn build_opts(auto_start: bool) -> MenuBuilder<Events> {
    let id;
    let event;
    if auto_start {
        id = Events::Pause;
        event = "Stop";
    } else {
        id = Events::Resume;
        event = "Start";
    }

    MenuBuilder::new()
        .item(&event, id)
        .with(MenuItem::Checkable {
            name: "Choose another app on next start".into(),
            disabled: RESET.load(DEFAULT_ORDER),
            is_checked: RESET.load(DEFAULT_ORDER),
            id: Events::Switch,
            icon: None
        })
        .item("Exit", Events::Exit)
}

// show app running in system tray
// also add an icon with a status
pub fn tray(program: String) {
    let registry = Registry::new();

    let event_loop = EventLoop::<Events>::with_user_event();
    let proxy = event_loop.create_proxy();

    let enabled_icon = include_bytes!("../assets/icon1.ico");
    let disabled_icon = include_bytes!("../assets/icon2.ico");

    let enabled = Icon::from_buffer(
        enabled_icon, None, None).unwrap();
    let disabled = Icon::from_buffer(
        disabled_icon, None, None).unwrap();

    let mut tray = TrayIconBuilder::new()
        .sender_winit(proxy)
        .icon_from_buffer(disabled_icon)
        .tooltip(format!("prioritizing: {}", program).as_str())
        .menu(build_opts(false))
        .build()
        .unwrap();
    tray.set_state(true, &disabled);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // for proper closure and removal of the tray icon
        // it must be moved inside loop as unbound variable binding
        let _ = tray;

        match event {
            Event::UserEvent(e) => match e {
                Events::Exit => {
                    *control_flow = ControlFlow::Exit
                }
                Events::Pause => {
                    tray.set_state(true, &disabled);
                }
                Events::Resume => {
                    tray.set_state(false, &enabled);
                }
                Events::Switch => {
                    if !RESET.load(DEFAULT_ORDER) {
                        if let Ok(_) = registry.delete() {
                            RESET.swap(true, DEFAULT_ORDER);
                        }
                    }

                    let mut start = true;
                    if PAUSED.fetch_or(true, DEFAULT_ORDER) {
                        start = false;
                    }
                    match tray.set_menu(&build_opts(start)) {
                        Ok(_) => {},
                        Err(e) => println!("error setting up menu, {}", e)
                    }
                }
            },
            _ => (),
        }
    });
}