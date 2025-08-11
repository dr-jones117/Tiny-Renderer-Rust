use minifb::{Key, WindowOptions};

use crate::graphics::output::RenderOutputter;

pub struct TinyRendererWindow {
    width: u16,
    height: u16,
    buffer: Vec<u32>,
    window: minifb::Window,
}

impl TinyRendererWindow {
    pub fn new(width: u16, height: u16) -> TinyRendererWindow {
        TinyRendererWindow {
            width,
            height,
            buffer: vec![0; (width * height) as usize],
            window: minifb::Window::new(
                "",
                width as usize,
                height as usize,
                WindowOptions::default(),
            )
            .unwrap_or_else(|err| {
                panic!("Error creating window: {}", err);
            }),
        }
    }

    pub fn set_target_fps(&mut self, fps: usize) {
        self.window.set_target_fps(fps);
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }
}

impl RenderOutputter for TinyRendererWindow {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn set(&mut self, x: i32, y: i32, color: &super::color::Color) {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }

        let rgba_u32 = ((color.3 as u32) << 24) | // Alpha
                   ((color.0 as u32) << 16) | // Red
                   ((color.1 as u32) << 8)  | // Green
                   (color.2 as u32); // Blue

        self.buffer[(y * self.width as i32 + x) as usize] = rgba_u32;
    }

    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.window
            .update_with_buffer(&self.buffer, self.width as usize, self.height as usize)?;

        Ok(())
    }
}
