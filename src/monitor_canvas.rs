use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalPosition,
    event_loop::EventLoop,
    monitor::MonitorHandle,
    window::{Fullscreen, Window, WindowBuilder},
};

pub struct MonitorCanvas {
    window: Window,
    pixels: Pixels,
}

impl MonitorCanvas {
    pub fn new(
        event_loop: &EventLoop<()>,
        monitor_handle: Option<MonitorHandle>,
    ) -> Result<MonitorCanvas, String> {
        let window = WindowBuilder::new()
            .with_transparent(true)
            .with_decorations(false)
            .with_fullscreen(Some(Fullscreen::Borderless(monitor_handle)))
            .build(&event_loop);

        let window = match window {
            Ok(window) => window,
            Err(err) => return Err(format!("Error creating window: {}", err)),
        };

        let window_size = window.inner_size();

        let pixels = {
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(window_size.width, window_size.height, surface_texture)
        };

        let mut pixels = match pixels {
            Ok(pixels) => pixels,
            Err(err) => return Err(format!("Error creating pixels: {}", err)),
        };

        pixels.set_clear_color(Color::TRANSPARENT);

        Ok(MonitorCanvas { window, pixels })
    }

    pub fn draw(&mut self, cursor_position: PhysicalPosition<f64>) {
        let frame = self.pixels.get_frame();
        let window_size = self.window.inner_size();
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

    pub fn render(&self) -> Result<(), pixels::Error> {
        self.pixels.render()
    }
}
