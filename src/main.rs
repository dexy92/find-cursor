#![windows_subsystem = "windows"]

mod monitor_canvas;
use monitor_canvas::{MonitorCanvas, PercentagePosition};
use std::{collections::HashMap, process};
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowId,
};

fn run_loop(event_loop: EventLoop<()>, mut monitor_canvases: HashMap<WindowId, MonitorCanvas>) {
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, window_id } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::CursorMoved { position, .. } => {
                    let percentage_position = PercentagePosition::from_physical(
                        position,
                        monitor_canvases[&window_id].window_size(),
                    );

                    for (_, canvas) in &mut monitor_canvases {
                        let monitor_position = if canvas.id() == window_id {
                            percentage_position.clone()
                        } else {
                            PercentagePosition {
                                x: -1.0,
                                y: percentage_position.y,
                            }
                        };

                        canvas.draw(monitor_position);
                        canvas.render().expect("Error on render");
                    }
                }
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button: MouseButton::Left,
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });
}

fn main() {
    let event_loop = EventLoop::new();

    let mut monitor_canvases: HashMap<WindowId, MonitorCanvas> = HashMap::new();
    for monitor in event_loop.available_monitors() {
        let monitor_canvas = MonitorCanvas::new(&event_loop, Some(monitor));
        let monitor_canvas = match monitor_canvas {
            Ok(monitor_canvas) => monitor_canvas,
            Err(err) => {
                eprintln!("{}", err);
                process::exit(1);
            }
        };
        monitor_canvases.insert(monitor_canvas.id(), monitor_canvas);
    }

    run_loop(event_loop, monitor_canvases);
}
