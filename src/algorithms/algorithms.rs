use crate::graphics::{PixelPos, RenderTarget, color};

pub struct Algorithms<T: RenderTarget> {
    pub draw_line_alg: fn(i32, i32, i32, i32, &color::RGBA, &mut T),
    pub rasterize_triangle_alg:
        fn(&PixelPos, &PixelPos, &PixelPos, &color::RGBA, &mut T),
}

impl<T: RenderTarget> Algorithms<T> {
    pub fn new(
        draw_line_alg: fn(i32, i32, i32, i32, &color::RGBA, &mut T),
        rasterize_triangle_alg: fn(
            &PixelPos,
            &PixelPos,
            &PixelPos,
            &color::RGBA,
            &mut T,
        ),
    ) -> Algorithms<T> {
        Algorithms {
            draw_line_alg,
            rasterize_triangle_alg,
        }
    }
}
