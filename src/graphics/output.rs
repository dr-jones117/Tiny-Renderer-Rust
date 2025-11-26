use std::error::Error;

use crate::graphics::color;

#[derive(Debug)]
pub struct PixelPos {
    pub x: i32,
    pub y: i32,
}

pub trait RenderOutputter {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn set(&mut self, x: i32, y: i32, color: &color::RGBA);
    fn render(&mut self) -> Result<(), Box<dyn Error>>;
}
