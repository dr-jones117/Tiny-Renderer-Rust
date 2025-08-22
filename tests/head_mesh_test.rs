#[cfg(test)]
mod tests {
    use std::fs;

    use tiny_renderer::{
        algorithms::{Algorithms, bresenhams_line_alg, rasterize_triangle},
        graphics::{
            color,
            tga::{self},
        },
        mesh::Mesh,
        renderer::{DrawType, TinyRendererBuilder},
    };

    const WIDTH: u16 = 5000;
    const HEIGHT: u16 = 5000;

    const HEAD_OBJ: &'static str = "tests/obj/head.obj";
    const BODY_OBJ: &'static str = "tests/obj/body.obj";
    const MESH_CREATED_BY_TEST: &'static str = "tests/output/meshes_created_by_test.tga";
    const MESH_FILE: &'static str = "tests/output/meshes.tga";

    fn create_mesh_image() {
        // create a renderer with a tga image output instead of a window
        let mut renderer = TinyRendererBuilder::new()
            .with_render_output(tga::Image::new(
                MESH_CREATED_BY_TEST,
                WIDTH as u16,
                HEIGHT as u16,
                tga::ImageType::UncompressedTrueColor,
                tga::ColorType::RGB,
            ))
            .with_algorithms(Algorithms::new(bresenhams_line_alg, rasterize_triangle))
            .with_color(color::GREEN)
            .build();

        let body_mesh = Mesh::from_obj_file(BODY_OBJ).unwrap_or_else(|err| {
            panic!("Error reading in the mesh: {}", err);
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
        let mesh = Mesh::from_obj_file(HEAD_OBJ).unwrap_or_else(|err| {
            panic!("Error reading in the mesh: {}", err);
        });

        let head_mesh_id = renderer.add_mesh(mesh);
        renderer.set_draw_type(head_mesh_id, DrawType::Line);
        renderer.scale_vertices(head_mesh_id, 0.5);

        // call our draw function once since it's just a single image
        if let Err(err) = renderer.draw() {
            panic!("Error rendering mesh: {}", err);
        }
    }

    #[test]
    fn head_line_fill_test_success() {
        create_mesh_image();

        let expected = fs::read(MESH_FILE).unwrap_or_else(|err| {
            panic!("Error reading {}: {}", MESH_FILE, err);
        });

        let result = fs::read(MESH_CREATED_BY_TEST).unwrap_or_else(|err| {
            panic!("Error reading {}: {}", MESH_CREATED_BY_TEST, err);
        });

        let test_passed = expected == result;

        assert!(test_passed);
    }
}
