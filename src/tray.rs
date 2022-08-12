use trayicon::{MenuBuilder, TrayIconBuilder};
use winit::{
    event::{Event},
    event_loop::{ControlFlow, EventLoop}
};

#[derive(Clone, Eq, PartialEq)]
enum Events {
    Exit
}

// show app running in system tray
// also add an icon with a status
pub fn tray(program: String) {
    let event_loop = EventLoop::<Events>::with_user_event();
    let proxy = event_loop.create_proxy();

    let tray_icon = TrayIconBuilder::new()
        .sender_winit(proxy)
        .icon_from_buffer(include_bytes!("../assets/icon1.ico"))
        .tooltip(format!("prioritizing: {}", program).as_str())
        .menu(
            MenuBuilder::new()
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
            },
            _ => (),
        }
    });
}