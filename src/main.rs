extern crate ash;
extern crate core;
extern crate winit;

use ash::version::EntryV1_0;
use ash::vk;
use ash::Entry;
use core::slice;
use winit::{ControlFlow, Event, VirtualKeyCode, WindowEvent};

fn main() {
    println!("Starting...");

    let mut events_loop = winit::EventsLoop::new();
    let _window = winit::WindowBuilder::new()
        .with_title("Test Window")
        .build(&events_loop)
        .unwrap();

    let entry = Entry::new().unwrap();
    let extensions = entry.enumerate_instance_extension_properties().unwrap();
    println!("{} extensions supported:", extensions.len());
    for extension in extensions {
        println!("{}", array_to_utf8(&extension.extension_name));
    }

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

fn array_to_utf8(i8slice: &[i8]) -> &str {
    let u8slice = unsafe { slice::from_raw_parts(i8slice.as_ptr() as *const u8, i8slice.len()) };
    return std::str::from_utf8(u8slice).unwrap();
}
