use minifb::{Key, WindowOptions};

use crate::graphics::{color::RGBA, output::RenderOutputter};

//TODO: make it into a [u8] in the future
pub struct TinyRendererWindow {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    window: minifb::Window,
}

impl TinyRendererWindow {
    pub fn new(width: usize, height: usize) -> TinyRendererWindow {
        let minifb_window =
            minifb::Window::new("TinyRenderer", width, height, WindowOptions::default()).unwrap();

        TinyRendererWindow {
            width,
            height,
            buffer: vec![0; width * height],
            window: minifb_window,
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

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }
}

impl RenderOutputter for TinyRendererWindow {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn set(&mut self, x: i32, y: i32, color: &RGBA) {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }

        let rgba_u32 = ((color.a as u32) << 24) | // Alpha
                   ((color.r as u32) << 16) | // Red
                   ((color.g as u32) << 8)  | // Green
                   (color.b as u32); // Blue

        self.buffer[(y * self.width as i32 + x) as usize] = rgba_u32;
    }

    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut flipped_buffer = vec![0u32; self.buffer.len()];

        for y in 0..self.height {
            for x in 0..self.width {
                let src_index = y * self.width + x;
                let dst_index = (self.height - 1 - y) * self.width + x; // Flip Y
                flipped_buffer[dst_index] = self.buffer[src_index];
            }
        }

        self.window
            .update_with_buffer(&flipped_buffer, self.width, self.height)?;

        Ok(())
    }
}
