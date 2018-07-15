extern crate ash;
extern crate winit;

use winit::{ControlFlow, Event, VirtualKeyCode, WindowEvent};

fn main() {
    println!("Starting...");

    let mut events_loop = winit::EventsLoop::new();
    let _window = winit::WindowBuilder::new()
        .with_title("Test Window")
        .build(&events_loop)
        .unwrap();

    events_loop.run_forever(|event| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                    ControlFlow::Break
                } else {
                    ControlFlow::Continue
                }
            }
            WindowEvent::CloseRequested => winit::ControlFlow::Break,
            _ => ControlFlow::Continue,
        },
        _ => ControlFlow::Continue,
    });

    println!("Started!");
}
