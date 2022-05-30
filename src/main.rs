use std::{rc::Rc, time::Instant};

use glam::Vec3;
use winit::{
    event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = Rc::new(WindowBuilder::new().build(&event_loop).unwrap());
    
    let mut app = iridium::App::new(window.clone()).await;
    app.add_boid(Vec3::new(0.0, 0.0, 0.0));
    let mut cur = Instant::now();
    
    event_loop.run(move |event, _, control_flow| {
        let new_time = Instant::now();
        let delta_t = new_time - cur;
        cur = new_time;
        match event {
            Event::WindowEvent {
                event: ref win_event,
                window_id,
            } if window_id == window.id() => if !app.input(win_event, &event) { // UPDATED!
                match win_event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        app.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        app.resize(**new_inner_size);
                    }
                    _ => {}
                }
            },
            Event::RedrawEventsCleared => {
            },
            _ => {}
        }
        app.update();
        app.render(delta_t);

    });
}

fn main() {
    pollster::block_on(run());
}
