use core::f32;
use std::error::Error;
use std::fs;

use super::geometry::{Vec3, Vec4};

#[derive(Debug)]
pub struct TriangleMesh {
    pub vertices: Vec<Vec4<f32>>,
    pub vertex_normals: Vec<Vec3<f32>>,
    pub texture_coordinates: Vec<Vec3<f32>>,
    pub faces: Vec<Vec<i32>>,
}

impl TriangleMesh {
    pub fn from_obj_file(obj_file_path: &str) -> Result<TriangleMesh, Box<dyn Error>> {
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

        Ok(TriangleMesh {
            vertices,
            vertex_normals,
            texture_coordinates,
            faces,
        })
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
