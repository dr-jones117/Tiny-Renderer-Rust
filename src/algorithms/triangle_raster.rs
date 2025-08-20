use std::collections::HashMap;

use crate::{
    algorithms::line_raster::bresenhams_line_map,
    graphics::{RenderOutputCoords, RenderOutputter, color},
};

pub fn rasterize_triangle<T>(
    v0: &RenderOutputCoords,
    v1: &RenderOutputCoords,
    v2: &RenderOutputCoords,
    color: &color::RGBA,
    render_output: &mut T,
    draw_line_alg: fn(i32, i32, i32, i32, &color::RGBA, &mut T),
) where
    T: RenderOutputter,
{
    // Sort vertices by y coordinate (v0 has smallest y, v2 has largest y)
    let (v0, v1, v2) = if v1.y < v0.y {
        (v1, v0, v2)
    } else {
        (v0, v1, v2)
    };

    let (v0, v1, v2) = if v2.y < v1.y {
        (v0, v2, v1)
    } else {
        (v0, v1, v2)
    };

    // Get image dimensions
    let width = render_output.width() as i32;
    let height = render_output.height() as i32;

    // Find bounding box of the triangle
    let min_x = (v0.x.min(v1.x).min(v2.x)).max(0);
    let max_x = (v0.x.max(v1.x).max(v2.x)).min(width - 1);
    let min_y = (v0.y.min(v1.y).min(v2.y)).max(0);
    let max_y = (v0.y.max(v1.y).max(v2.y)).min(height - 1);

    // Rasterize using barycentric coordinates
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = RenderOutputCoords { x, y };

            // Calculate barycentric coordinates
            let (w0, w1, w2) = barycentric_coords(&p, v0, v1, v2);

            // Check if point is inside triangle (all barycentric coordinates >= 0)
            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                // Draw the pixel
                draw_line_alg(x, y, x, y, &color, render_output);
            }
        }
    }
}

fn barycentric_coords(
    p: &RenderOutputCoords,
    v0: &RenderOutputCoords,
    v1: &RenderOutputCoords,
    v2: &RenderOutputCoords,
) -> (f32, f32, f32) {
    let denom = ((v1.y - v2.y) * (v0.x - v2.x) + (v2.x - v1.x) * (v0.y - v2.y)) as f32;

    // Handle degenerate triangle
    if denom.abs() < f32::EPSILON {
        return (0.0, 0.0, 0.0);
    }

    let w0 = ((v1.y - v2.y) * (p.x - v2.x) + (v2.x - v1.x) * (p.y - v2.y)) as f32 / denom;
    let w1 = ((v2.y - v0.y) * (p.x - v2.x) + (v0.x - v2.x) * (p.y - v2.y)) as f32 / denom;
    let w2 = 1.0 - w0 - w1;

    (w0, w1, w2)
}

pub fn rasterize_triangle_scanline<T>(
    v0: &RenderOutputCoords,
    v1: &RenderOutputCoords,
    v2: &RenderOutputCoords,
    color: &color::RGBA,
    render_output: &mut T,
    draw_line_alg: fn(i32, i32, i32, i32, &color::RGBA, &mut T),
) where
    T: RenderOutputter,
{
    let mut y_to_xs: HashMap<i32, Vec<i32>> = HashMap::new();
    // 'draw' all three triangles.
    // instead of drawing them, just save every single y and x coord using bresenhams
    // does it make sense to reuse bresenhams line alg from here?
    bresenhams_line_map(v0.x, v0.y, v1.x, v1.y, &mut y_to_xs);
    bresenhams_line_map(v1.x, v1.y, v2.x, v2.y, &mut y_to_xs);
    bresenhams_line_map(v2.x, v2.y, v0.x, v0.y, &mut y_to_xs);

    for (y, vec) in y_to_xs.iter() {
        draw_line_alg(vec[0], *y, vec[vec.len() - 1], *y, color, render_output);
    }
}
