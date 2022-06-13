use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event_loop::EventLoop,
    monitor::MonitorHandle,
    window::{Fullscreen, Window, WindowBuilder, WindowId},
};

#[derive(Copy, Clone)]
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
    last_position: PhysicalPosition<f64>,
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

        Ok(MonitorCanvas {
            window,
            pixels,
            last_position: PhysicalPosition { x: -1.0, y: -1.0 },
        })
    }

    fn draw_x(&mut self, x: i16, rgba: [u8; 4]) {
        let width = self.window.inner_size().width as usize;

        if x < 0 || x as usize > width {
            return;
        }

        let frame = self.pixels.get_frame();

        for pixel in frame.chunks_exact_mut(4).skip(x as usize).step_by(width) {
            pixel.copy_from_slice(&rgba);
        }
    }

    fn draw_y(&mut self, y: i16, rgba: [u8; 4]) {
        let height = self.window.inner_size().height as usize;

        if y < 0 || y as usize > height {
            return;
        }

        let width = self.window.inner_size().width as usize;
        let frame = self.pixels.get_frame();

        for pixel in frame
            .chunks_exact_mut(4)
            .skip(y as usize * width)
            .take(width)
        {
            pixel.copy_from_slice(&rgba);
        }
    }

    pub fn draw(&mut self, cursor_position: PercentagePosition) {
        let cursor_position = cursor_position.to_physical(self.window_size());

        let cursor_x = cursor_position.x as i16;
        let cursor_y = cursor_position.y as i16;
        let last_x = self.last_position.x as i16;
        let last_y = self.last_position.y as i16;
        self.last_position = cursor_position;

        let red_color: [u8; 4] = [0xff, 0x00, 0x00, 0xff];
        let transparent_color: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

        self.draw_x(last_x, transparent_color);
        self.draw_x(cursor_x, red_color);

        self.draw_y(last_y, transparent_color);
        self.draw_y(cursor_y, red_color);
    }

    pub fn render(&self) -> Result<(), pixels::Error> {
        self.pixels.render()
    }

    pub fn window_size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn id(&self) -> WindowId {
        self.window.id()
    }
}
