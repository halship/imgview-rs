mod state;

use glutin::dpi::LogicalSize;
use glutin::event::*;
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use state::State;
use std::process;

fn main() {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("imgview")
        .with_inner_size(LogicalSize {
            width: 800.0,
            height: 600.0,
        });
    let context = ContextBuilder::new()
        .build_windowed(wb, &event_loop)
        .unwrap();
    let context = unsafe { context.make_current().unwrap() };

    let mut state = match State::new(&context) {
        Ok(state) => state,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { window_id, event } if window_id == context.window().id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => state.resize(physical_size),
                    _ => (),
                }
            }
            Event::MainEventsCleared => context.window().request_redraw(),
            Event::RedrawRequested(_) => {
                state.render();
                context.swap_buffers().unwrap();
            }
            _ => (),
        };
    });
}
