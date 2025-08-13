use crate::graphics::{
    color::RGBA,
    output::{RenderOutputCoords, RenderOutputter},
};

pub struct Algorithms<T: RenderOutputter> {
    pub draw_line_alg: fn(i32, i32, i32, i32, &RGBA, &mut T),
    pub rasterize_triangle_alg: fn(
        &RenderOutputCoords,
        &RenderOutputCoords,
        &RenderOutputCoords,
        &RGBA,
        &mut T,
        fn(i32, i32, i32, i32, &RGBA, &mut T),
    ),
}

impl<T: RenderOutputter> Algorithms<T> {
    pub fn new(
        draw_line_alg: fn(i32, i32, i32, i32, &RGBA, &mut T),
        rasterize_triangle_alg: fn(
            &RenderOutputCoords,
            &RenderOutputCoords,
            &RenderOutputCoords,
            &RGBA,
            &mut T,
            fn(i32, i32, i32, i32, &RGBA, &mut T),
        ),
    ) -> Algorithms<T> {
        Algorithms {
            draw_line_alg,
            rasterize_triangle_alg,
        }
    }
}
