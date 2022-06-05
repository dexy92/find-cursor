use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Fullscreen}, 
    dpi::PhysicalPosition
};


fn run_loop(event_loop: EventLoop<()>) {
    let mut cursor_position = PhysicalPosition::new(0.0, 0.0);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        println!("{:?}", cursor_position);
        
        match event {
            Event::WindowEvent {event, ..} => match event{
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::CursorMoved { position, .. } => cursor_position = position,
                _ => ()
            }
            _ => ()
        }
        
    });
}


fn main() {
    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new().with_fullscreen(Some(Fullscreen::Borderless(None))).build(&event_loop).unwrap();

    run_loop(event_loop);
}
