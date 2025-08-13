use crate::graphics::{RenderOutputCoords, RenderOutputter, color};

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
    // Early return if triangle is degenerate (all points have same x coordinate)
    if v0.0 == v1.0 && v0.0 == v2.0 {
        return;
    }

    // Sort vertices by y coordinate (v0 has smallest y, v2 has largest y)
    let (v0, v1, v2) = if v1.1 < v0.1 {
        (v1, v0, v2)
    } else {
        (v0, v1, v2)
    };

    let (v0, v1, v2) = if v2.1 < v1.1 {
        (v0, v2, v1)
    } else {
        (v0, v1, v2)
    };

    // Get image dimensions
    let width = render_output.width() as i32;
    let height = render_output.height() as i32;

    // Find bounding box of the triangle
    let min_x = (v0.0.min(v1.0).min(v2.0)).max(0);
    let max_x = (v0.0.max(v1.0).max(v2.0)).min(width - 1);
    let min_y = (v0.1.min(v1.1).min(v2.1)).max(0);
    let max_y = (v0.1.max(v1.1).max(v2.1)).min(height - 1);

    // Rasterize using barycentric coordinates
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = RenderOutputCoords(x, y);

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
    let denom = ((v1.1 - v2.1) * (v0.0 - v2.0) + (v2.0 - v1.0) * (v0.1 - v2.1)) as f32;

    // Handle degenerate triangle
    if denom.abs() < f32::EPSILON {
        return (0.0, 0.0, 0.0);
    }

    let w0 = ((v1.1 - v2.1) * (p.0 - v2.0) + (v2.0 - v1.0) * (p.1 - v2.1)) as f32 / denom;
    let w1 = ((v2.1 - v0.1) * (p.0 - v2.0) + (v0.0 - v2.0) * (p.1 - v2.1)) as f32 / denom;
    let w2 = 1.0 - w0 - w1;

    (w0, w1, w2)
}
