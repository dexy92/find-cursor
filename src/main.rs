#![windows_subsystem = "windows"]

mod monitor_canvas;
use monitor_canvas::MonitorCanvas;
use std::process;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

fn run_loop(event_loop: EventLoop<()>, mut monitor_canvas: MonitorCanvas) {
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::CursorMoved { position, .. } => {
                    monitor_canvas.draw(position);
                    monitor_canvas.render().expect("Error on render");
                }
                _ => (),
            },
            _ => (),
        }
    });
}

fn main() {
    let event_loop = EventLoop::new();
    let monitor_canvas = MonitorCanvas::new(&event_loop, None);
    let monitor_canvas = match monitor_canvas {
        Ok(monitor_canvas) => monitor_canvas,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    run_loop(event_loop, monitor_canvas);
}
