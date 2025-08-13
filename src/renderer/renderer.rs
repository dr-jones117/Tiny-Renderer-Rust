use std::error::Error;

use crate::algorithms::algorithms::Algorithms;
use crate::geometry::Vec4;
use crate::graphics::color;
use crate::graphics::output::{RenderOutputCoords, RenderOutputter};
use crate::graphics::window::TinyRendererWindow;
use crate::mesh::Mesh;
use minifb::Key;

#[derive(Debug)]
pub enum DrawType {
    Fill,
    Line,
}

pub struct TinyRenderer<T: RenderOutputter> {
    meshes: Vec<Mesh>,
    draw_types: Vec<DrawType>,
    render_output: T,
    algorithms: Algorithms<T>,
}

impl<T: RenderOutputter> TinyRenderer<T> {
    pub fn new(render_output: T, algorithms: Algorithms<T>) -> TinyRenderer<T> {
        TinyRenderer {
            draw_types: Vec::new(),
            meshes: Vec::new(),
            render_output,
            algorithms,
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

    pub fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        //TODO: set the color in the config for line renders
        let color = color::RGBA {
            r: 20,
            g: 200,
            b: 50,
            a: 255,
        };

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
                    DrawType::Fill => (self.algorithms.rasterize_triangle_alg)(
                        v0,
                        v1,
                        v2,
                        &color,
                        &mut self.render_output,
                        self.algorithms.draw_line_alg,
                    ),
                    DrawType::Line => {
                        (self.algorithms.draw_line_alg)(
                            v0.0,
                            v0.1,
                            v1.0,
                            v1.1,
                            &color,
                            &mut self.render_output,
                        );
                        (self.algorithms.draw_line_alg)(
                            v1.0,
                            v1.1,
                            v2.0,
                            v2.1,
                            &color,
                            &mut self.render_output,
                        );
                        (self.algorithms.draw_line_alg)(
                            v2.0,
                            v2.1,
                            v0.0,
                            v0.1,
                            &color,
                            &mut self.render_output,
                        );
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

impl TinyRenderer<TinyRendererWindow> {
    pub fn clear(&mut self) {
        self.render_output.clear();
    }

    pub fn is_open(&self) -> bool {
        self.render_output.is_open()
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.render_output.is_key_down(key)
    }
}

fn world_to_output_coordinates(
    width: usize,
    height: usize,
    world_coords: &Vec4<f32>,
) -> RenderOutputCoords {
    RenderOutputCoords(
        ((world_coords.x + 1.0) * width as f32 / 2.0) as i32,
        ((world_coords.y + 1.0) * height as f32 / 2.0) as i32,
    )
}
