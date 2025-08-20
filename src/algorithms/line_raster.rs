use std::collections::HashMap;

use crate::graphics::{color, output::RenderOutputter};

pub fn bresenhams_line_alg<T>(
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: &color::RGBA,
    render_output: &mut T,
) where
    T: RenderOutputter,
{
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
            // detranspose the image
            render_output.set(y, x, color);
        } else {
            render_output.set(x, y, color);
        }

        error += derror;

        if error > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error -= dx * 2;
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

pub fn line_alg_with_floats<T>(
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: &color::RGBA,
    render_output: &mut T,
) where
    T: RenderOutputter,
{
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

    for x in x0..x1 {
        let t: f64 = (x - x0) as f64 / (x1 - x0) as f64;
        let y = (y0 as f64 + (t * (y1 - y0) as f64)) as i32;

        if steep {
            render_output.set(y, x, color);
        } else {
            render_output.set(x, y, color);
        }
    }
}
