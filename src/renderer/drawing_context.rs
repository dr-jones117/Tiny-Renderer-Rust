use crate::{
    algorithms::Algorithms,
    graphics::{PixelPos, RenderTarget, color},
};

pub struct DrawingContext<T: RenderTarget> {
    pub render_output: T,
    pub algorithms: Algorithms<T>,
    pub color: color::RGBA,
}

impl<T: RenderTarget> DrawingContext<T> {
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        (self.algorithms.draw_line_alg)(x0, y0, x1, y1, &self.color, &mut self.render_output)
    }
    pub fn rasterize_triangle(
        &mut self,
        v0: &PixelPos,
        v1: &PixelPos,
        v2: &PixelPos,
    ) {
        (self.algorithms.rasterize_triangle_alg)(v0, v1, v2, &self.color, &mut self.render_output)
    }
}
