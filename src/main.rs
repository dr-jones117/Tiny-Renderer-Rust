mod geometry;
mod graphics;
mod mesh;
mod renderer;

use std::env;
use std::process;

use graphics::tga;
use mesh::Mesh;
use minifb::Key;
use renderer::{DrawType, TinyRenderer};

use crate::graphics::window::TinyRendererWindow;

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
    //render_meshes_to_image();

    let mut window = TinyRendererWindow::new(800, 800);
    window.set_target_fps(60);

    let mut renderer = TinyRenderer::new(&window);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        renderer.draw();
    }
}

fn render_meshes_to_image() {
    // build our configuration
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);
    let render_output = tga::Image::new(
        800,
        800,
        tga::ImageType::UncompressedTrueColor,
        graphics::ColorType::RGB,
    );

    // setup our renderer
    let mut renderer = TinyRenderer::new(render_output);

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
