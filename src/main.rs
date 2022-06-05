use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, Window, WindowBuilder},
};

fn draw(frame: &mut [u8], cursor_position: PhysicalPosition<f64>, window_size: PhysicalSize<u32>) {
    let cursor_x = cursor_position.x as i16;
    let cursor_y = cursor_position.y as i16;

    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % window_size.width as usize) as i16;
        let y = (i / window_size.width as usize) as i16;

        let rgba = if x == cursor_x || y == cursor_y {
            [0xff, 0x00, 0x00, 0xff]
        } else {
            [0x00, 0x00, 0x00, 0x00]
        };

        pixel.copy_from_slice(&rgba);
    }
}

fn run_loop(event_loop: EventLoop<()>, window: Window) {
    let window_size = window.inner_size();
    let mut pixels = {
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_size.width, window_size.height, surface_texture)
    }
    .unwrap();
    pixels.set_clear_color(Color::TRANSPARENT);

    let mut cursor_position = PhysicalPosition::new(0.0, 0.0);

    let mut redraw_requested = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::CursorMoved { position, .. } => {
                    cursor_position = position;
                    if !redraw_requested {
                        redraw_requested = true;
                        window.request_redraw();
                    }
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                draw(pixels.get_frame(), cursor_position, window_size);
                pixels.render().expect("Render failed");
                redraw_requested = false;
            }
            _ => (),
        }
    });
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_transparent(true)
        .with_decorations(false)
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .build(&event_loop)
        .unwrap();

    run_loop(event_loop, window);
}
