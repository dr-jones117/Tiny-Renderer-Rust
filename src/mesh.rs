use std::error::Error;
use std::fs;

use crate::geometry::{Vec3, Vec4};

#[derive(Debug, Clone)]
pub struct FaceElement {
    pub vertex_index: Option<i32>,
    pub texture_index: Option<i32>,
    pub normal_index: Option<i32>,
}

impl FaceElement {
    pub fn new(vertex_index_str: &str, texture_index_str: &str, normal_index_str: &str) -> FaceElement {
        let parse_index = |s: &str| s.parse::<i32>().ok().map(|idx| idx - 1);

        FaceElement {
            vertex_index: parse_index(vertex_index_str),
            texture_index: parse_index(texture_index_str),
            normal_index: parse_index(normal_index_str),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vec4<f32>>,
    pub vertex_normals: Vec<Vec3<f32>>,
    pub texture_coordinates: Vec<Vec3<f32>>,
    pub faces: Vec<Vec<FaceElement>>,
}

impl Mesh {
    #[allow(dead_code)]
    pub fn new() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            vertex_normals: Vec::new(),
            texture_coordinates: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn from_obj_file(obj_file_path: &str) -> Result<Mesh, Box<dyn Error>> {
        let obj_content = fs::read_to_string(obj_file_path)?;
        let mut vertices: Vec<Vec4<f32>> = Vec::new();
        let mut texture_coordinates: Vec<Vec3<f32>> = Vec::new();
        let mut vertex_normals: Vec<Vec3<f32>> = Vec::new();
        let mut faces: Vec<Vec<FaceElement>> = Vec::new();

        for line in obj_content.lines() {
            if line.starts_with("v ") {
                vertices.push(Mesh::parse_vertex(line)?);
            } else if line.starts_with("f ") {
                faces.push(Mesh::parse_face(line)?);
            } else if line.starts_with("vn ") {
                vertex_normals.push(Mesh::parse_vertex_normal(line)?);
            } else if line.starts_with("vt ") {
                texture_coordinates.push(Mesh::parse_texture_coordinate(line)?);
            }
        }

        Ok(Mesh {
            vertices,
            vertex_normals,
            texture_coordinates,
            faces,
        })
    }

    fn parse_face(line: &str) -> Result<Vec<FaceElement>, Box<dyn Error>> {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let mut face_vertices: Vec<FaceElement> = Vec::new();

        for token in tokens {
            if token.starts_with("f") {
                continue;
            }
            if token.starts_with("#") {
                break;
            }

            let indice: Vec<&str> = token.split("/").collect();
            
            // OBJ format supports: v, v/vt, v/vt/vn, v//vn
            if indice.is_empty() || indice.len() > 3 {
                return Err(format!("Invalid face format: expected 1-3 indices, got {}", indice.len()).into());
            }

            let vertex_idx = indice.get(0).copied().unwrap_or("");
            let texture_idx = indice.get(1).copied().unwrap_or("");
            let normal_idx = indice.get(2).copied().unwrap_or("");

            face_vertices.push(FaceElement::new(vertex_idx, texture_idx, normal_idx));
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

        Ok(Vec4 { x, y, z, w: a })
    }
}
