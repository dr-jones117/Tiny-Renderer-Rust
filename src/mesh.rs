use core::f32;
use std::error::Error;
use std::fs;

use super::geometry::{Vec3, Vec4};
use crate::tga;

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vec4<f32>>,
    vertex_normals: Vec<Vec3<f32>>,
    texture_coordinates: Vec<Vec3<f32>>,
    faces: Vec<Vec<i32>>,
}

impl Mesh {
    pub fn len_faces(&self) -> usize {
        self.faces.len()
    }

    pub fn face_at(&self, index: usize) -> &Vec<i32> {
        &self.faces[index]
    }

    pub fn len_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn vertex_at(&self, index: usize) -> &Vec4<f32> {
        &self.vertices[index]
    }

    pub fn from_obj_file(obj_file_path: &str) -> Result<Mesh, Box<dyn Error>> {
        let obj_content = fs::read_to_string(obj_file_path)?;
        let mut vertices: Vec<Vec4<f32>> = Vec::new();
        let mut texture_coordinates: Vec<Vec3<f32>> = Vec::new();
        let mut vertex_normals: Vec<Vec3<f32>> = Vec::new();
        let mut faces: Vec<Vec<i32>> = Vec::new();

        for line in obj_content.lines() {
            if line.starts_with("v ") {
                vertices.push(parse_vertex(line)?);
            } else if line.starts_with("f ") {
                faces.push(parse_face(line)?);
            } else if line.starts_with("vn ") {
                vertex_normals.push(parse_vertex_normal(line)?);
            } else if line.starts_with("vt ") {
                texture_coordinates.push(parse_texture_coordinate(line)?);
            }
        }

        Ok(Mesh {
            vertices,
            vertex_normals,
            texture_coordinates,
            faces,
        })
    }

    pub fn write_to_tga_image(&self, img_path: &str) -> Result<(), Box<dyn Error>> {
        let width = 1000;
        let height = 1000;

        let color = tga::Color(150, 100, 50, 255);

        let mut image = tga::Image::new(
            width,
            height,
            tga::ImageType::UncompressedTrueColor,
            tga::ColorType::RGB,
        );

        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for i in 0..self.len_vertices() {
            let v = self.vertex_at(i);
            min_x = min_x.min(v.x);
            max_x = max_x.max(v.x);
            min_y = min_y.min(v.y);
            max_y = max_y.max(v.y);
        }

        let range_x = max_x - min_x;
        let range_y = max_y - min_y;
        let max_range = range_x.max(range_y);
        let scale = if max_range > 0.0 {
            1.8 / max_range
        } else {
            1.0
        };

        let center_x = (min_x + max_x) / 2.0;
        let center_y = (min_y + max_y) / 2.0;

        for i in 0..self.len_faces() {
            let face: &Vec<i32> = self.face_at(i);

            for j in 0..3 {
                let v0: &Vec4<f32> = self.vertex_at(face[j * 3] as usize);
                let v1: &Vec4<f32> = self.vertex_at(face[((j + 1) * 3) % face.len()] as usize);

                // Center and scale the coordinates
                let scaled_x0 = (v0.x - center_x) * scale;
                let scaled_y0 = (v0.y - center_y) * scale;
                let scaled_x1 = (v1.x - center_x) * scale;
                let scaled_y1 = (v1.y - center_y) * scale;

                let x0: i32 = ((scaled_x0 + 1.0) * width as f32 / 2.0) as i32;
                let y0: i32 = ((scaled_y0 + 1.0) * height as f32 / 2.0) as i32;
                let x1: i32 = ((scaled_x1 + 1.0) * width as f32 / 2.0) as i32;
                let y1: i32 = ((scaled_y1 + 1.0) * height as f32 / 2.0) as i32;

                image.draw_line(x0, y0, x1, y1, &color);
            }
        }
        image.write_to_file(img_path)?;
        Ok(())
    }
}

fn parse_face(line: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    let mut face_vertices: Vec<i32> = Vec::new();

    for token in tokens {
        if token.starts_with("f") {
            continue;
        }
        if token.starts_with("#") {
            break;
        }

        let indice: Vec<&str> = token.split("/").collect();
        for value in indice {
            if let Ok(mut idx) = value.parse::<i32>() {
                idx -= 1;
                face_vertices.push(idx);
            } else {
                face_vertices.push(0);
            }
        }
    }

    Ok(face_vertices)
}

fn parse_vertex_normal(line: &str) -> Result<Vec3<f32>, Box<dyn Error>> {
    let tokens: Vec<&str> = line.split_whitespace().collect();

    let x = tokens[1].parse()?;
    let y = tokens[2].parse()?;
    let z = tokens[3].parse()?;

    Ok(Vec3 { x, y, z })
}

fn parse_texture_coordinate(line: &str) -> Result<Vec3<f32>, Box<dyn Error>> {
    let tokens: Vec<&str> = line.split_whitespace().collect();

    let u = tokens[1].parse()?;
    let mut v: f32 = 0.0;
    let mut w: f32 = 0.0;

    if tokens.len() > 2 {
        v = tokens[2].parse()?;
    }

    if tokens.len() > 3 {
        w = tokens[3].parse()?;
    }

    Ok(Vec3 { x: u, y: v, z: w })
}

fn parse_vertex(line: &str) -> Result<Vec4<f32>, Box<dyn Error>> {
    let tokens: Vec<&str> = line.split_whitespace().collect();

    let x: f32 = tokens[1].parse()?;
    let y: f32 = tokens[2].parse()?;
    let z: f32 = tokens[3].parse()?;
    let mut a = 1.0;

    if tokens.len() > 4 {
        a = tokens[4].parse()?;
    }

    Ok(Vec4 { x, y, z, a })
}
