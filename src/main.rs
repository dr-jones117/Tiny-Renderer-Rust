mod geometry;
mod mesh;
mod renderer;
mod tga;

use std::env;
use std::process;

use geometry::Vec4;
use mesh::TriangleMesh;
use renderer::{DrawOutput, DrawType, TinyRenderer};

struct Config<'a> {
    obj_file_path: &'a str,
    img_file_path: String,
}

impl<'a> Config<'a> {
    pub fn build(args: &Vec<String>) -> Config {
        let mut obj_file_path = "./obj/african_head.obj";
        let mut img_file_path = String::from("./tga/img.tga");

        if args.len() >= 2 {
            obj_file_path = args[1].as_str();
        }
        if args.len() >= 3 {
            img_file_path = args[2].clone();
        }

        Config {
            obj_file_path,
            img_file_path,
        }
    }
}

fn main() {
    // build our configuration
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);

    // setup our renderer
    let mut renderer = TinyRenderer::new();
    renderer.set_draw_type(DrawType::Line);
    renderer.set_draw_output(DrawOutput::Tga(config.img_file_path));

    // read in a mesh from our obj file
    let mesh = TriangleMesh::from_obj_file(config.obj_file_path).unwrap_or_else(|err| {
        eprintln!("Error reading in the mesh: {}", err);
        process::exit(1);
    });

    // set our renderers verts and faces
    renderer.set_vertices(mesh.vertices);
    renderer.set_faces(mesh.faces);

    // draw our render
    if let Err(err) = renderer.draw() {
        eprintln!("Error rendering mesh: {}", err);
        process::exit(1);
    }

    renderer.set_vertices(vec![
        Vec4 {
            x: -0.9,
            y: 0.0,
            z: 0.0,
            a: 1.0,
        },
        Vec4 {
            x: 0.9,
            y: 0.9,
            z: 0.0,
            a: 1.0,
        },
        Vec4 {
            x: 0.9,
            y: 0.0,
            z: 0.0,
            a: 1.0,
        },
    ]);
    renderer.set_faces(vec![vec![0, 0, 0, 1, 0, 0, 2, 0, 0]]);

    renderer.set_draw_output(DrawOutput::Tga(String::from("./tga/triangle.tga")));

    if let Err(err) = renderer.draw() {
        eprintln!("Error rendering mesh: {}", err);
        process::exit(1);
    }
}
