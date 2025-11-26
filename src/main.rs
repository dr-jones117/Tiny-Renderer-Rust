mod algorithms;
mod geometry;
mod graphics;
mod mesh;
mod renderer;

use std::env;
use std::process;

use minifb;
use rand::Rng;

use crate::algorithms::line_alg_with_floats;
use crate::algorithms::triangle_raster::rasterize_triangle_scanline;
use crate::algorithms::{Algorithms, bresenhams_line_alg, rasterize_triangle};

use crate::geometry::Vec4;
use crate::graphics::color;
use crate::graphics::{TinyRendererWindow, tga};

use crate::mesh::{FaceElement, Mesh};

use crate::renderer::{DrawType, TinyRendererBuilder};

static USAGE_STATEMENT: &'static str = "USAGE: tiny_renderer [run_type]";

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
const TARGET_FPS: usize = 120;

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

//TODO: fix the triangle rasterization not working on some obj
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
    // create our window renderer with specific configuration using the builder pattern
    let mut window_renderer = TinyRendererBuilder::new()
        .with_render_output(TinyRendererWindow::new(WIDTH, HEIGHT))
        .with_target_fps(TARGET_FPS)
        .with_color(color::PURPLE)
        .with_algorithms(Algorithms::new(bresenhams_line_alg, rasterize_triangle))
        .build();

    // load our mesh into memory
    let body_mesh = Mesh::from_obj_file("obj/body.obj").unwrap_or_else(|err| {
        eprintln!("Error reading in the mesh: {}", err);
        process::exit(1);
    });

    // add the mesh into the renderer, getting back it's id
    let body_id = window_renderer.add_mesh(body_mesh);
    window_renderer.set_draw_type(body_id, DrawType::Line);
    window_renderer.scale_vertices(body_id, 0.05);
    window_renderer.move_vertices(body_id, 0.0, 1.0);

    // ok, now do it again
    let mesh = Mesh::from_obj_file("obj/head.obj").unwrap_or_else(|err| {
        eprintln!("Error reading in the mesh: {}", err);
        process::exit(1);
    });

    let head_mesh_id = window_renderer.add_mesh(mesh);
    window_renderer.set_draw_type(head_mesh_id, DrawType::Line);
    window_renderer.scale_vertices(head_mesh_id, 0.5);

    while window_renderer.is_open() && !window_renderer.is_key_down(minifb::Key::Escape) {
        window_renderer.move_vertices(body_id, 0.0, -0.04);

        window_renderer.clear();

        window_renderer.draw().unwrap_or_else(|err| {
            eprintln!("Error drawing the window: {}", err);
            process::exit(1);
        });
    }
}

fn render_meshes_to_image() {
    // create a renderer with a tga image output instead of a window
    let mut renderer = TinyRendererBuilder::new()
        .with_render_output(tga::Image::new(
            "tga/img.tga",
            3000,
            3000,
            tga::ImageType::UncompressedTrueColor,
            tga::ColorType::RGB,
        ))
        .with_color(color::GREEN)
        .with_algorithms(Algorithms::new(
            line_alg_with_floats,
            rasterize_triangle_scanline,
        ))
        .build();

    let body_mesh = Mesh::from_obj_file("obj/body.obj").unwrap_or_else(|err| {
        eprintln!("Error reading in the mesh: {}", err);
        process::exit(1);
    });

    let body_mesh_2 = body_mesh.clone();
    let body_mesh_3 = body_mesh.clone();

    // bring in more meshes!
    let body_id = renderer.add_mesh(body_mesh);
    renderer.set_draw_type(body_id, DrawType::Line);
    renderer.scale_vertices(body_id, 0.05);
    renderer.move_vertices(body_id, 0.0, -1.2);

    let body_id_2 = renderer.add_mesh(body_mesh_2);
    renderer.set_draw_type(body_id_2, DrawType::Line);
    renderer.scale_vertices(body_id_2, 0.1);
    renderer.move_vertices(body_id_2, -1.0, -1.0);

    let body_id_3 = renderer.add_mesh(body_mesh_3);
    renderer.set_draw_type(body_id_3, DrawType::Line);
    renderer.scale_vertices(body_id_3, 0.1);
    renderer.move_vertices(body_id_3, 1.0, -1.0);

    // read in a mesh from our obj file
    let mesh = Mesh::from_obj_file("obj/head.obj").unwrap_or_else(|err| {
        eprintln!("Error reading in the mesh: {}", err);
        process::exit(1);
    });

    let head_mesh_id = renderer.add_mesh(mesh);
    renderer.set_draw_type(head_mesh_id, DrawType::Line);
    renderer.scale_vertices(head_mesh_id, 0.5);

    // call our draw function once since it's just a single image
    if let Err(err) = renderer.draw() {
        eprintln!("Error rendering mesh: {}", err);
        process::exit(1);
    }

    render_triangles();
}

fn render_triangles() {
    let mut renderer = TinyRendererBuilder::new()
        .with_render_output(tga::Image::new(
            "tga/triangles.tga",
            WIDTH as u16,
            HEIGHT as u16,
            tga::ImageType::UncompressedTrueColor,
            tga::ColorType::RGB,
        ))
        .with_algorithms(Algorithms::new(
            line_alg_with_floats,
            rasterize_triangle_scanline,
        ))
        .with_color(color::PURPLE)
        .build();

    // Create array to store all triangles
    let mut triangles: Vec<Mesh> = Vec::new();
    let mut rng = rand::rng();

    // Grid parameters
    let grid_size = 4;
    let triangle_size = 0.15; // Base size of each triangle
    let spacing = 0.05; // Spacing between triangles
    let total_step = triangle_size * 2.0 + spacing; // Total step between triangle centers

    // Calculate starting position to center the grid
    let start_offset = -((grid_size as f32 - 1.0) * total_step) / 2.0;

    // Create 4x4 grid of triangles
    for row in 0..grid_size {
        for col in 0..grid_size {
            let mut triangle: Mesh = Mesh::new();

            // Calculate center position for this triangle
            let center_x = start_offset + (col as f32) * total_step;
            let center_y = start_offset + (row as f32) * total_step;

            // Random rotation angle
            let rotation = rng.random_range(0.0..std::f32::consts::PI * 2.0);
            let cos_rot = rotation.cos();
            let sin_rot = rotation.sin();

            // Random scale factors for different triangle shapes
            let scale_x = rng.random_range(0.8..1.4);
            let scale_y = rng.random_range(0.8..1.4);

            // Generate random triangle vertices with different shapes
            let vertex1_local = (
                rng.random_range(-0.5..0.5) * triangle_size * scale_x,
                rng.random_range(-0.8..-0.2) * triangle_size * scale_y,
            );
            let vertex2_local = (
                rng.random_range(0.3..0.8) * triangle_size * scale_x,
                rng.random_range(0.2..0.8) * triangle_size * scale_y,
            );
            let vertex3_local = (
                rng.random_range(-0.8..-0.3) * triangle_size * scale_x,
                rng.random_range(0.2..0.8) * triangle_size * scale_y,
            );

            // Apply rotation and translation to vertices
            let rotate_and_translate = |local_x: f32, local_y: f32| -> (f32, f32) {
                let rotated_x = local_x * cos_rot - local_y * sin_rot;
                let rotated_y = local_x * sin_rot + local_y * cos_rot;
                (center_x + rotated_x, center_y + rotated_y)
            };

            let (v1_x, v1_y) = rotate_and_translate(vertex1_local.0, vertex1_local.1);
            let (v2_x, v2_y) = rotate_and_translate(vertex2_local.0, vertex2_local.1);
            let (v3_x, v3_y) = rotate_and_translate(vertex3_local.0, vertex3_local.1);

            // Define triangle vertices with random positions and orientations
            triangle.vertices = vec![
                Vec4 {
                    x: v1_x,
                    y: v1_y,
                    z: 0.0,
                    w: 1.0,
                },
                Vec4 {
                    x: v2_x,
                    y: v2_y,
                    z: 0.0,
                    w: 1.0,
                },
                Vec4 {
                    x: v3_x,
                    y: v3_y,
                    z: 0.0,
                    w: 1.0,
                },
            ];

            // Same faces array for all triangles
            triangle.faces = vec![vec![
                FaceElement {
                    vertex_index: Some(0),
                    texture_index: Some(0),
                    normal_index: Some(0),
                },
                FaceElement {
                    vertex_index: Some(1),
                    texture_index: Some(0),
                    normal_index: Some(0),
                },
                FaceElement {
                    vertex_index: Some(2),
                    texture_index: Some(0),
                    normal_index: Some(0),
                },
            ]];

            triangles.push(triangle);
        }
    }

    // Add all triangles to renderer and draw them
    let mut triangle_ids: Vec<usize> = Vec::new();
    for triangle in triangles {
        let tri_id: usize = renderer.add_mesh(triangle);
        triangle_ids.push(tri_id);
    }

    // Set draw type and draw all triangles
    for tri_id in triangle_ids {
        renderer.set_draw_type(tri_id, DrawType::Line);
    }

    renderer
        .draw()
        .unwrap_or_else(|err| panic!("Error drawing triangle: {}", err))
}
