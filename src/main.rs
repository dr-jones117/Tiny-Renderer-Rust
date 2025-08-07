mod geometry;
mod mesh;
mod tga;

use mesh::Mesh;
use std::env;
use std::process;

struct Config<'a> {
    obj_file_path: &'a str,
    img_file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn build(args: &Vec<String>) -> Config {
        let mut obj_file_path = "./obj/african_head.obj";
        let mut img_file_path = "./tga/img.tga";

        if args.len() >= 2 {
            obj_file_path = args[1].as_str();
        }
        if args.len() >= 3 {
            img_file_path = args[2].as_str();
        }

        Config {
            obj_file_path,
            img_file_path,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);

    let mesh = Mesh::from_obj_file(config.obj_file_path).unwrap_or_else(|err| {
        eprintln!("Error creating the mesh: {}", err);
        process::exit(1);
    });

    if let Err(err) = mesh.write_to_tga_image(config.img_file_path) {
        eprintln!("Error creating image from mesh: {}", err);
        process::exit(1);
    }
}
