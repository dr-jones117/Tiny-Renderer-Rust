use crate::graphics::{
    color::Color,
    output::{RenderOutputCoords, RenderOutputter},
};

pub struct Algorithms<T: RenderOutputter> {
    pub draw_line_alg: fn(i32, i32, i32, i32, &Color, &mut T),
    pub rasterize_triangle_alg: fn(
        &RenderOutputCoords,
        &RenderOutputCoords,
        &RenderOutputCoords,
        &Color,
        &mut T,
        fn(i32, i32, i32, i32, &Color, &mut T),
    ),
}

impl<T: RenderOutputter> Algorithms<T> {
    pub fn new(
        draw_line_alg: fn(i32, i32, i32, i32, &Color, &mut T),
        rasterize_triangle_alg: fn(
            &RenderOutputCoords,
            &RenderOutputCoords,
            &RenderOutputCoords,
            &Color,
            &mut T,
            fn(i32, i32, i32, i32, &Color, &mut T),
        ),
    ) -> Algorithms<T> {
        Algorithms {
            draw_line_alg,
            rasterize_triangle_alg,
        }
    }
}
