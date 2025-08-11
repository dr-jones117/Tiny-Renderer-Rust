use std::error::Error;

use crate::geometry::Vec4;
use crate::graphics::color::Color;
use crate::graphics::output::{RenderOutputCoords, RenderOutputter};
use crate::mesh::Mesh;
use rand::Rng;

#[allow(dead_code)]
#[derive(Debug)]
pub enum DrawType {
    Fill,
    Line,
}

#[allow(dead_code)]
pub enum DrawOutput {
    Tga {
        file_path: String,
        width: u16,
        height: u16,
    },
    Window,
}

pub struct TinyRenderer<T: RenderOutputter> {
    meshes: Vec<Mesh>,
    draw_types: Vec<DrawType>,
    config: TinyRendererConfig,
    render_output: T,
}

pub struct TinyRendererConfig {
    draw_output: DrawOutput,
}

impl TinyRendererConfig {
    fn new() -> TinyRendererConfig {
        TinyRendererConfig {
            draw_output: DrawOutput::Tga {
                file_path: String::from(""),
                width: 800,
                height: 800,
            },
        }
    }
}

impl<T: RenderOutputter> TinyRenderer<T> {
    pub fn new(render_output: T) -> TinyRenderer<T> {
        TinyRenderer {
            config: TinyRendererConfig::new(),
            draw_types: Vec::new(),
            meshes: Vec::new(),
            render_output,
        }
    }

    pub fn set_draw_output(&mut self, draw_output: DrawOutput) {
        self.config.draw_output = draw_output;
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> usize {
        self.meshes.push(mesh);
        self.draw_types.push(DrawType::Fill);
        self.meshes.len() - 1
    }

    pub fn scale_vertices(&mut self, id: usize, scale: f32) {
        self.check_mesh_range(&id);
        if id > self.meshes.len() - 1 {
            panic!("referencing an invalid mesh");
        }
        for vertice in self.meshes[id].vertices.iter_mut() {
            vertice.x = scale * vertice.x;
            vertice.y = scale * vertice.y;
            vertice.z = scale * vertice.z;
        }
    }

    pub fn move_vertices(&mut self, id: usize, x: f32, y: f32) {
        self.check_mesh_range(&id);
        for vertice in self.meshes[id].vertices.iter_mut() {
            vertice.x = x + vertice.x;
            vertice.y = y + vertice.y;
        }
    }

    pub fn set_draw_type(&mut self, id: usize, draw_type: DrawType) {
        self.check_mesh_range(&id);
        self.draw_types[id] = draw_type;
    }

    pub fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        //TODO: make sure to new up the outputter when creating the renderer

        let color = Color(20, 200, 50, 255);

        for (i, mesh) in self.meshes.iter_mut().enumerate() {
            let mut transformed_coords: Vec<RenderOutputCoords> = Vec::new();

            for vertice in mesh.vertices.iter() {
                transformed_coords.push(world_to_output_coordinates(
                    self.render_output.width(),
                    self.render_output.height(),
                    vertice,
                ))
            }

            for face_index in 0..mesh.faces.len() {
                let face: &Vec<i32> = &mesh.faces[face_index];

                let v0 = &transformed_coords[face[0] as usize];
                let v1 = &transformed_coords[face[3] as usize];
                let v2 = &transformed_coords[face[6] as usize];

                match &self.draw_types[i] {
                    DrawType::Fill => {
                        rasterize_triangle(v0, v1, v2, &color, &mut self.render_output);
                    }
                    DrawType::Line => {
                        draw_line(v0.0, v0.1, v1.0, v1.1, &color, &mut self.render_output);
                        draw_line(v1.0, v1.1, v2.0, v2.1, &color, &mut self.render_output);
                        draw_line(v2.0, v2.1, v0.0, v0.1, &color, &mut self.render_output);
                    }
                }
            }
        }

        self.render_output.render()?;
        Ok(())
    }

    fn check_mesh_range(&self, id: &usize) {
        if *id > self.meshes.len() - 1 {
            panic!("Error In Renderer: Referencing an invalid mesh.")
        }
    }
}

fn world_to_output_coordinates(
    width: u16,
    height: u16,
    world_coords: &Vec4<f32>,
) -> RenderOutputCoords {
    RenderOutputCoords(
        ((world_coords.x + 1.0) * width as f32 / 2.0) as i32,
        ((world_coords.y + 1.0) * height as f32 / 2.0) as i32,
    )
}

fn rasterize_triangle<T>(
    v0: &RenderOutputCoords,
    v1: &RenderOutputCoords,
    v2: &RenderOutputCoords,
    color: &Color,
    render_output: &mut T,
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

    // Generate random color (keeping your original color generation logic)
    let mut rng = rand::rng();
    let red = rng.random_range(0..255);
    let green = rng.random_range(0..255);
    let blue = rng.random_range(0..255);
    let triangle_color = Color(red, green, blue, 255);

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
                draw_line(x, y, x, y, &triangle_color, render_output);
            }
        }
    }
}

// Helper function to calculate barycentric coordinates
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

fn draw_line<T>(x0: i32, y0: i32, x1: i32, y1: i32, color: &Color, render_output: &mut T)
where
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
