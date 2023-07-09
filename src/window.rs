use pixels::{Pixels, SurfaceTexture, Error};
use winit::{window::{Window, WindowBuilder}, event_loop::EventLoop, dpi::LogicalSize};

use crate::{WIDTH, HEIGHT};

pub struct GameWindow {
    pub window: Window,
    pub size: (u32, u32),
    pub pixels: Pixels,
}

impl GameWindow {
    pub fn new(title: &str, event_loop: &EventLoop<()>) -> Result<Self, Error> {
        let width;
        let height;
        unsafe { width = WIDTH as u32; height = HEIGHT as u32; }

        let size = LogicalSize::new(width, height);
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .build(event_loop)
            .unwrap();

        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(window_size.width, window_size.height, surface_texture)?;

        Ok(Self {
            window,
            size: (window_size.width, window_size.height),
            pixels,
        })
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        unsafe {
            WIDTH = new_size.0 as u32;
            HEIGHT = new_size.1 as u32;
        }
        self.pixels.resize_buffer(new_size.0, new_size.1).unwrap();
        self.size = new_size;
    }
}
