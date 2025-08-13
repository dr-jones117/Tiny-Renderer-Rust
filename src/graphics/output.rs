use std::error::Error;

use crate::graphics::color::RGBA;

#[derive(Debug)]
pub struct RenderOutputCoords(pub i32, pub i32);

pub trait RenderOutputter {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn set(&mut self, x: i32, y: i32, color: &RGBA);
    fn render(&mut self) -> Result<(), Box<dyn Error>>;
}
