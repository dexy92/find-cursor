use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event_loop::EventLoop,
    monitor::MonitorHandle,
    window::{Fullscreen, Window, WindowBuilder},
};

pub struct PercentagePosition {
    pub x: f64,
    pub y: f64,
}

impl PercentagePosition {
    pub fn from_physical(
        physical_position: PhysicalPosition<f64>,
        window_size: PhysicalSize<u32>,
    ) -> PercentagePosition {
        let percentage_x = physical_position.x / window_size.width as f64;
        let percentage_y = physical_position.y / window_size.height as f64;

        PercentagePosition {
            x: percentage_x,
            y: percentage_y,
        }
    }

    pub fn to_physical(&self, window_size: PhysicalSize<u32>) -> PhysicalPosition<f64> {
        let x = self.x * window_size.width as f64;
        let y = self.y * window_size.height as f64;

        PhysicalPosition { x, y }
    }
}

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

    pub fn draw(&mut self, cursor_position: PercentagePosition) {
        let cursor_position = cursor_position.to_physical(self.window_size());

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

    pub fn window_size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }
}
