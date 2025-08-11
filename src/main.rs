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
    renderer.set_draw_output(DrawOutput::Tga(config.img_file_path));

    let body_mesh = Mesh::from_obj_file("./obj/FinalBaseMesh.obj").unwrap_or_else(|err| {
        eprintln!("Error reading in the mesh: {}", err);
        process::exit(1);
    });
    let body_mesh_2 = body_mesh.clone();
    let body_mesh_3 = body_mesh.clone();

    let body_id = renderer.add_mesh(body_mesh);
    renderer.set_draw_type(body_id, DrawType::Line);
    renderer.scale_vertices(body_id, 0.05);
    renderer.move_vertices(body_id, 0.0, -1.2);

    let body_id_2 = renderer.add_mesh(body_mesh_2);
    renderer.scale_vertices(body_id_2, 0.1);
    renderer.move_vertices(body_id_2, -1.0, -1.0);

    let body_id_3 = renderer.add_mesh(body_mesh_3);
    renderer.scale_vertices(body_id_3, 0.1);
    renderer.move_vertices(body_id_3, 1.0, -1.0);

    // read in a mesh from our obj file
    let mesh = Mesh::from_obj_file(config.obj_file_path).unwrap_or_else(|err| {
        eprintln!("Error reading in the mesh: {}", err);
        process::exit(1);
    });

    // set our renderers verts and faces
    let head_mesh_id = renderer.add_mesh(mesh);
    renderer.set_draw_type(head_mesh_id, DrawType::Line);
    renderer.scale_vertices(head_mesh_id, 0.5);

    if let Err(err) = renderer.draw() {
        eprintln!("Error rendering mesh: {}", err);
        process::exit(1);
    }
}
