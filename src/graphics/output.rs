use std::error::Error;

use crate::graphics::color::Color;

#[derive(Debug)]
pub struct RenderOutputCoords(pub i32, pub i32);

pub trait RenderOutputter {
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn set(&mut self, x: i32, y: i32, color: &Color);
    fn render(&self) -> Result<(), Box<dyn Error>>;
}
