use std::collections::HashMap;

use crate::graphics::{PixelPos, RenderTarget, color};

pub fn rasterize_triangle<T>(
    v0: &PixelPos,
    v1: &PixelPos,
    v2: &PixelPos,
    color: &color::RGBA,
    render_output: &mut T,
) where
    T: RenderTarget,
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
            let p = PixelPos { x, y };

            // Calculate barycentric coordinates
            let (w0, w1, w2) = barycentric_coords(&p, v0, v1, v2);

            // Check if point is inside triangle (all barycentric coordinates >= 0)
            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                // Draw the pixel
                render_output.set(x, y, color);
            }
        }
    }
}

fn barycentric_coords(
    p: &PixelPos,
    v0: &PixelPos,
    v1: &PixelPos,
    v2: &PixelPos,
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
    v0: &PixelPos,
    v1: &PixelPos,
    v2: &PixelPos,
    color: &color::RGBA,
    render_output: &mut T,
) where
    T: RenderTarget,
{
    let mut y_to_xs: HashMap<i32, Vec<i32>> = HashMap::new();

    bresenhams_line_map(v0.x, v0.y, v1.x, v1.y, &mut y_to_xs);
    bresenhams_line_map(v1.x, v1.y, v2.x, v2.y, &mut y_to_xs);
    bresenhams_line_map(v2.x, v2.y, v0.x, v0.y, &mut y_to_xs);

    for (y, vec) in y_to_xs.iter() {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;

        for &x in vec {
            min_x = min_x.min(x);
            max_x = max_x.max(x);
        }

        for x in min_x..max_x {
            render_output.set(x, *y, color);
        }
    }
}

pub fn bresenhams_line_map(
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    y_to_xs: &mut HashMap<i32, Vec<i32>>,
) {
    let steep = (x1 - x0).abs() < (y1 - y0).abs();

    // transpose it if it's steep
    let (x0, y0, x1, y1) = if steep {
        (y0, x0, y1, x1)
    } else {
        (x0, y0, x1, y1)
    };

    // if going right to left, we need to swap the points to go left to right
    let (x0, y0, x1, y1) = if x0 > x1 {
        (x1, y1, x0, y0)
    } else {
        (x0, y0, x1, y1)
    };

    let dx = x1 - x0;
    let dy = y1 - y0;

    let derror = (dy * 2).abs();
    let mut error = 0;
    let mut y = y0;

    for x in x0..=x1 {
        if steep {
            y_to_xs
                .entry(x)
                .and_modify(|x: &mut Vec<i32>| x.push(y))
                .or_insert(vec![y]);
        } else {
            y_to_xs
                .entry(y)
                .and_modify(|y: &mut Vec<i32>| y.push(x))
                .or_insert(vec![x]);
        }

        error += derror;

        if error > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error -= dx * 2;
        }
    }
}
