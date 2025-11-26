use std::error::Error;

use minifb;

use crate::algorithms::Algorithms;
use crate::geometry::Vec4;
use crate::graphics::{PixelPos, RenderTarget, TinyRendererWindow, color};
use crate::mesh::Mesh;
use crate::renderer::DrawingContext;

#[derive(Debug)]
pub enum DrawType {
    Fill,
    Line,
}

pub struct TinyRenderer<T: RenderTarget> {
    meshes: Vec<Mesh>,
    draw_types: Vec<DrawType>,
    drawing_ctx: DrawingContext<T>,
}

impl<T: RenderTarget> TinyRenderer<T> {
    pub fn new(render_output: T, algorithms: Algorithms<T>, color: color::RGBA) -> TinyRenderer<T> {
        TinyRenderer {
            draw_types: Vec::new(),
            meshes: Vec::new(),
            drawing_ctx: DrawingContext {
                render_output,
                algorithms,
                color,
            },
        }
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

    pub fn set_render_output(&mut self, render_output: T) {
        self.drawing_ctx.render_output = render_output;
    }

    pub fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        for (i, mesh) in self.meshes.iter_mut().enumerate() {
            let mut screen_space_coordinates: Vec<PixelPos> = Vec::new();

            for vertice in mesh.vertices.iter() {
                screen_space_coordinates.push(
                    world_to_screen_space(
                        self.drawing_ctx.render_output.width(),
                        self.drawing_ctx.render_output.height(),
                        vertice,
                    )
                )
            }

            for face in &mesh.faces {
                let get_screen_space_coordinate = |idx: Option<i32>| -> Result<&PixelPos, Box<dyn Error>> {
                    idx.ok_or("Face missing vertex index")?
                        .try_into()
                        .ok()
                        .and_then(|i: usize| screen_space_coordinates.get(i))
                        .ok_or_else(|| "Invalid vertex index".into())
                };

                let v0 = get_screen_space_coordinate(face[0].vertex_index)?;
                let v1 = get_screen_space_coordinate(face[1].vertex_index)?;
                let v2 = get_screen_space_coordinate(face[2].vertex_index)?;

                match &self.draw_types[i] {
                    DrawType::Fill => self.drawing_ctx.rasterize_triangle(v0, v1, v2),
                    DrawType::Line => {
                        self.drawing_ctx.draw_line(v0.x, v0.y, v1.x, v1.y);
                        self.drawing_ctx.draw_line(v1.x, v1.y, v2.x, v2.y);
                        self.drawing_ctx.draw_line(v2.x, v2.y, v0.x, v0.y);
                    }
                }
            }
        }

        self.drawing_ctx.render_output.render()?;
        Ok(())
    }

    fn check_mesh_range(&self, id: &usize) {
        if *id > self.meshes.len() - 1 {
            panic!("Error In Renderer: Referencing an invalid mesh.")
        }
    }
}

impl TinyRenderer<TinyRendererWindow> {
    pub fn clear(&mut self) {
        self.drawing_ctx.render_output.clear();
    }

    pub fn is_open(&self) -> bool {
        self.drawing_ctx.render_output.is_open()
    }

    pub fn is_key_down(&self, key: minifb::Key) -> bool {
        self.drawing_ctx.render_output.is_key_down(key)
    }
}

fn world_to_screen_space(
    width: usize,
    height: usize,
    world_coordinates: &Vec4<f32>,
) -> PixelPos {
    PixelPos {
        x: ((world_coordinates.x + 1.0) * width as f32 / 2.0) as i32,
        y: ((world_coordinates.y + 1.0) * height as f32 / 2.0) as i32,
    }
}
