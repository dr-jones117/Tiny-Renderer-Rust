mod geometry;
mod mesh;
mod renderer;
mod tga;

use std::env;
use std::process;

use mesh::Mesh;
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
    renderer.set_draw_type(DrawType::Fill);
    renderer.set_draw_output(DrawOutput::Tga(config.img_file_path));

    let bodyMesh = Mesh::from_obj_file("./obj/FinalBaseMesh.obj").unwrap_or_else(|err| {
        eprintln!("Error reading in the mesh: {}", err);
        process::exit(1);
    });

    renderer.set_vertices(0, bodyMesh.vertices.clone());
    renderer.set_faces(0, bodyMesh.faces.clone());
    renderer.scale_vertices(0, 0.05);
    renderer.move_vertices(0, 0.0, -1.2);


    renderer.set_vertices(3, bodyMesh.vertices.clone());
    renderer.set_faces(3, bodyMesh.faces.clone());
    renderer.scale_vertices(3, 0.1);
    renderer.move_vertices(3, -1.0, -1.0);


    renderer.set_vertices(4, bodyMesh.vertices.clone());
    renderer.set_faces(4, bodyMesh.faces.clone());
    renderer.scale_vertices(4, 0.1);
    renderer.move_vertices(4, 1.0, -1.0);

    // read in a mesh from our obj file
    let mesh = Mesh::from_obj_file(config.obj_file_path).unwrap_or_else(|err| {
        eprintln!("Error reading in the mesh: {}", err);
        process::exit(1);
    });

    // set our renderers verts and faces
    renderer.set_vertices(1, mesh.vertices);
    renderer.set_faces(1, mesh.faces);
    renderer.scale_vertices(1, 0.5);


    if let Err(err) = renderer.draw() {
        eprintln!("Error rendering mesh: {}", err);
        process::exit(1);
    }
}
