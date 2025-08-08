use std::error::Error;

use crate::geometry::{Vec3, Vec4};
use crate::tga::{self, ImageCoords};

#[derive(Debug)]
pub enum DrawType {
    Fill,
    Line,
}

pub enum DrawOutput {
    Tga(String),
    Screen,
}

pub struct TinyRenderer {
    vertices: Vec<Vec4<f32>>,
    vertex_normals: Vec<Vec3<f32>>,
    texture_coordinates: Vec<Vec3<f32>>,
    faces: Vec<Vec<i32>>,
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
            vertices: Vec::new(),
            vertex_normals: Vec::new(),
            texture_coordinates: Vec::new(),
            faces: Vec::new(),
            config: TinyRendererConfig::new(),
        }
    }

    pub fn set_draw_type(&mut self, draw_type: DrawType) {
        self.config.draw_type = draw_type;
    }

    pub fn set_draw_output(&mut self, draw_output: DrawOutput) {
        self.config.draw_output = draw_output;
    }

    pub fn set_vertices(&mut self, vertices: Vec<Vec4<f32>>) {
        self.vertices = vertices;
    }

    pub fn set_faces(&mut self, faces: Vec<Vec<i32>>) {
        self.faces = faces;
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

        for i in 0..self.faces.len() {
            let face: &Vec<i32> = &self.faces[i];

            let v0 = &self.vertices[face[0] as usize];
            let v1 = &self.vertices[face[3] as usize];
            let v2 = &self.vertices[face[6] as usize];

            image.draw_triangle(
                world_to_image_coords(v0, &image),
                world_to_image_coords(v1, &image),
                world_to_image_coords(v2, &image),
                &color,
                &self.config.draw_type,
            );
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
