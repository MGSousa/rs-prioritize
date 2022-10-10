use std::sync::atomic::{AtomicBool, Ordering};
use trayicon::{Icon, MenuBuilder, TrayIconBuilder};
use winit::{
    event::{Event},
    event_loop::{ControlFlow, EventLoop}
};

#[derive(Clone, Eq, PartialEq)]
enum Events {
    Exit,
    Pause,
    Resume
}

lazy_static! {
    pub static ref PAUSED: AtomicBool = AtomicBool::new(false);
}

// show app running in system tray
// also add an icon with a status
pub fn tray(program: String) {
    let event_loop = EventLoop::<Events>::with_user_event();
    let proxy = event_loop.create_proxy();

    let enabled_icon = include_bytes!("../assets/icon1.ico");

    let enabled = Icon::from_buffer(
        enabled_icon, None, None).unwrap();
    let disabled = Icon::from_buffer(
        include_bytes!("../assets/icon2.ico"), None, None).unwrap();

    let mut tray_icon = TrayIconBuilder::new()
        .sender_winit(proxy)
        .icon_from_buffer(enabled_icon)
        .tooltip(format!("prioritizing: {}", program).as_str())
        .menu(
            MenuBuilder::new()
                .item("Pause", Events::Pause)
                .item("Exit", Events::Exit),
        )
        .build()
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        let _ = tray_icon;
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(e) => match e {
                Events::Exit => {
                    println!("application exited!");
                    *control_flow = ControlFlow::Exit
                }
                Events::Pause => {
                    PAUSED.swap(true, Ordering::Relaxed);
                    tray_icon.set_icon(&disabled).unwrap();
                    tray_icon
                        .set_menu(
                            &MenuBuilder::new()
                                .item("Resume", Events::Resume)
                                .item("Exit", Events::Exit),
                        )
                        .unwrap();
                }
                Events::Resume => {
                    PAUSED.swap(false, Ordering::Relaxed);
                    tray_icon.set_icon(&enabled).unwrap();
                    tray_icon
                        .set_menu(
                            &MenuBuilder::new()
                                .item("Pause", Events::Pause)
                                .item("Exit", Events::Exit),
                        )
                        .unwrap();
                }
            },
            _ => (),
        }
    });
}