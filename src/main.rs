mod algorithms;
mod geometry;
mod graphics;
mod mesh;
mod renderer;

use std::env;
use std::process;

use minifb;

use crate::algorithms::{Algorithms, bresenhams_line_alg, rasterize_triangle};
use crate::graphics::color;
use crate::graphics::{TinyRendererWindow, tga};
use crate::mesh::Mesh;
use crate::renderer::{DrawType, TinyRendererBuilder};

static USAGE_STATEMENT: &'static str = "USAGE: tiny_renderer [run_type]";

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const TARGET_FPS: usize = 60;

enum RunType {
    Window,
    Image,
}

struct Config {
    run_type: RunType,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Config {
        if args.len() != 2 {
            panic!("{}", USAGE_STATEMENT);
        }

        let run_type;

        if args[1] == "image" {
            run_type = RunType::Image;
        } else if args[1] == "window" {
            run_type = RunType::Window;
        } else {
            panic!("{}", USAGE_STATEMENT);
        }

        Config { run_type }
    }
}

//TODO: Read over rasterize_triangle for a better understanding
//TODO: fix the triangle rasterization not working on some obj meshes
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);

    match config.run_type {
        RunType::Window => {
            render_window();
        }
        RunType::Image => {
            render_meshes_to_image();
        }
    }
}

fn render_window() {
    let mut window_renderer = TinyRendererBuilder::new()
        .with_render_output(TinyRendererWindow::new(WIDTH, HEIGHT))
        .with_target_fps(TARGET_FPS)
        .with_color(color::RGBA {
            r: 24,
            g: 250,
            b: 20,
            a: 255,
        })
        .with_algorithms(Algorithms::new(bresenhams_line_alg, rasterize_triangle))
        .build();

    let body_mesh = Mesh::from_obj_file("./obj/body.obj").unwrap_or_else(|err| {
        eprintln!("Error reading in the mesh: {}", err);
        process::exit(1);
    });

    let body_id = window_renderer.add_mesh(body_mesh);
    window_renderer.set_draw_type(body_id, DrawType::Fill);
    window_renderer.scale_vertices(body_id, 0.05);
    window_renderer.move_vertices(body_id, 0.0, 1.0);

    // read in a mesh from our obj file
    let mesh = Mesh::from_obj_file("./obj/head.obj").unwrap_or_else(|err| {
        eprintln!("Error reading in the mesh: {}", err);
        process::exit(1);
    });

    let head_mesh_id = window_renderer.add_mesh(mesh);
    window_renderer.set_draw_type(head_mesh_id, DrawType::Line);
    window_renderer.scale_vertices(head_mesh_id, 0.5);

    while window_renderer.is_open() && !window_renderer.is_key_down(minifb::Key::Escape) {
        window_renderer.move_vertices(body_id, 0.0, -0.01);

        window_renderer.clear();
        window_renderer.draw().unwrap_or_else(|err| {
            eprintln!("Error drawing the window: {}", err);
            process::exit(1);
        });
    }
}

fn render_meshes_to_image() {
    let mut renderer = TinyRendererBuilder::new()
        .with_render_output(tga::Image::new(
            "tga/img2.tga",
            WIDTH as u16,
            HEIGHT as u16,
            tga::ImageType::UncompressedTrueColor,
            tga::ColorType::RGB,
        ))
        .with_algorithms(Algorithms::new(bresenhams_line_alg, rasterize_triangle))
        .build();

    let body_mesh = Mesh::from_obj_file("./obj/body.obj").unwrap_or_else(|err| {
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
    let mesh = Mesh::from_obj_file("./obj/head.obj").unwrap_or_else(|err| {
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
