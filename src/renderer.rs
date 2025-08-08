use std::error::Error;

use crate::geometry::Vec4;
use crate::mesh::Mesh;
use crate::tga::{self, ImageCoords};

static MESH_CAPACITY: usize = 50;

#[allow(dead_code)]
#[derive(Debug)]
pub enum DrawType {
    Fill,
    Line,
}

#[allow(dead_code)]
pub enum DrawOutput {
    Tga(String),
    Screen,
}

pub struct TinyRenderer {
    meshes: Vec<Mesh>,
    config: TinyRendererConfig,
}

pub struct TinyRendererConfig {
    draw_type: DrawType,
    draw_output: DrawOutput,
}

impl TinyRendererConfig {
    fn new() -> TinyRendererConfig {
        TinyRendererConfig {
            draw_type: DrawType::Line,
            draw_output: DrawOutput::Tga(String::from("")),
        }
    }
}

impl TinyRenderer {
    pub fn new() -> TinyRenderer {
        TinyRenderer {
            config: TinyRendererConfig::new(),
            meshes: vec![Mesh::new(); MESH_CAPACITY],
        }
    }

    pub fn set_draw_type(&mut self, draw_type: DrawType) {
        self.config.draw_type = draw_type;
    }

    pub fn set_draw_output(&mut self, draw_output: DrawOutput) {
        self.config.draw_output = draw_output;
    }

    pub fn set_vertices(&mut self, id: usize, vertices: Vec<Vec4<f32>>) {
        if id > MESH_CAPACITY - 1 {
            panic!("Out of bounds!");
        }
        self.meshes[id].vertices = vertices;
    }

    pub fn scale_vertices(&mut self, id: usize, scale: f32) {
        if id > MESH_CAPACITY - 1 {
            panic!("Out of bounds!");
        }
        for vertice in self.meshes[id].vertices.iter_mut() {
            vertice.x = scale * vertice.x;
            vertice.y = scale * vertice.y;
            vertice.z = scale * vertice.z;
        }
    }

    pub fn set_faces(&mut self, id: usize, faces: Vec<Vec<i32>>) {
        if id > MESH_CAPACITY - 1 {
            panic!("Out of bounds!");
        }
        self.meshes[id].faces = faces;
    }

    pub fn draw(&self) -> Result<(), Box<dyn Error>> {
        match self.config.draw_output {
            DrawOutput::Tga(ref str) => {
                self.write_to_tga_image(str.as_str())?;
            }
            DrawOutput::Screen => {
                panic!("outputting to screen not implemented!");
            }
        }
        Ok(())
    }

    fn write_to_tga_image(&self, img_path: &str) -> Result<(), Box<dyn Error>> {
        let width = 1000;
        let height = 1000;

        let color = tga::Color(20, 200, 50, 255);

        let mut image = tga::Image::new(
            width,
            height,
            tga::ImageType::UncompressedTrueColor,
            tga::ColorType::RGB,
        );

        for mesh in self.meshes.iter() {
            for face_index in 0..mesh.faces.len() {
                let face: &Vec<i32> = &mesh.faces[face_index];

                let v0 = &mesh.vertices[face[0] as usize];
                let v1 = &mesh.vertices[face[3] as usize];
                let v2 = &mesh.vertices[face[6] as usize];

                let v0 = world_to_image_coords(v0, &image);
                let v1 = world_to_image_coords(v1, &image);
                let v2 = world_to_image_coords(v2, &image);

                match self.config.draw_type {
                    DrawType::Fill => {
                        panic! {"not implemented"};
                    }
                    DrawType::Line => {
                        image.draw_line(v0.0, v0.1, v1.0, v1.1, &color);
                        image.draw_line(v1.0, v1.1, v2.0, v2.1, &color);
                        image.draw_line(v2.0, v2.1, v0.0, v0.1, &color);
                    }
                }
            }
        }

        image.write_to_file(img_path)?;
        Ok(())
    }
}

fn world_to_image_coords(world_coords: &Vec4<f32>, image: &tga::Image) -> ImageCoords {
    ImageCoords(
        ((world_coords.x + 1.0) * image.width() as f32 / 2.0) as i32,
        ((world_coords.y + 1.0) * image.height() as f32 / 2.0) as i32,
    )
}
