#[cfg(test)]
mod tests {
    use std::fs;

    use tiny_renderer::{
        algorithms::{Algorithms, bresenhams_line_alg, rasterize_triangle_scanline},
        geometry::Vec4,
        graphics::{
            color,
            tga::{self, ColorType, ImageType},
        },
        mesh::Mesh,
        renderer::{DrawType, TinyRendererBuilder},
    };

    const TRIANGLE: &'static str = "./tests/output/triangle.tga";
    const TRIANGLE_FILL: &'static str = "./tests/output/triangle_fill.tga";

    const TRIANGLE_CREATED_BY_TEST: &'static str = "./tests/output/triangle_created_by_test.tga";
    const TRIANGLE_FILL_CREATED_BY_TEST: &'static str =
        "./tests/output/triangle_fill_created_by_test.tga";

    const WIDTH: u16 = 5000;
    const HEIGHT: u16 = 5000;

    fn create_triangle_images() {
        let mut renderer = TinyRendererBuilder::new()
            .with_render_output(tga::Image::new(
                TRIANGLE_CREATED_BY_TEST,
                WIDTH as u16,
                HEIGHT as u16,
                ImageType::UncompressedTrueColor,
                ColorType::RGB,
            ))
            .with_algorithms(Algorithms::new(
                bresenhams_line_alg,
                rasterize_triangle_scanline,
            ))
            .with_color(color::GREEN)
            .build();

        let mut triangle_meshes = vec![Mesh::new(); 4];

        triangle_meshes[0].vertices = vec![
            Vec4 {
                x: -0.8,
                y: -0.9,
                z: 0.0,
                w: 1.0,
            },
            Vec4 {
                x: -0.6,
                y: -0.1,
                z: 0.0,
                w: 1.0,
            },
            Vec4 {
                x: -0.4,
                y: -0.9,
                z: 0.0,
                w: 1.0,
            },
        ];

        triangle_meshes[1].vertices = vec![
            Vec4 {
                x: 0.2,
                y: -0.8,
                z: 0.0,
                w: 1.0,
            },
            Vec4 {
                x: 0.8,
                y: -0.6,
                z: 0.0,
                w: 1.0,
            },
            Vec4 {
                x: 0.5,
                y: -0.6,
                z: 0.0,
                w: 1.0,
            },
        ];

        triangle_meshes[2].vertices = vec![
            Vec4 {
                x: -0.9,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            Vec4 {
                x: -0.3,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            Vec4 {
                x: -0.9,
                y: 0.6,
                z: 0.0,
                w: 1.0,
            },
        ];

        triangle_meshes[3].vertices = vec![
            Vec4 {
                x: 0.2,
                y: 0.2,
                z: 0.0,
                w: 1.0,
            },
            Vec4 {
                x: 0.8,
                y: 0.2,
                z: 0.0,
                w: 1.0,
            },
            Vec4 {
                x: 0.5,
                y: 0.8,
                z: 0.0,
                w: 1.0,
            },
        ];

        let mut mesh_ids = Vec::new();
        for mut mesh in triangle_meshes {
            mesh.faces = vec![vec![0, 0, 0, 1, 0, 0, 2, 0, 0]];

            let id = renderer.add_mesh(mesh);
            mesh_ids.push(id);
            renderer.set_draw_type(id, DrawType::Line);
        }

        renderer.draw().unwrap_or_else(|err| {
            panic!("unable to draw triangle: {}", err);
        });

        renderer.set_render_output(tga::Image::new(
            TRIANGLE_FILL_CREATED_BY_TEST,
            WIDTH as u16,
            HEIGHT as u16,
            ImageType::UncompressedTrueColor,
            ColorType::RGB,
        ));

        for id in mesh_ids {
            renderer.set_draw_type(id, DrawType::Fill);
        }

        renderer.draw().unwrap_or_else(|err| {
            panic!("unable to draw filled triangles: {}", err);
        });
    }

    #[test]
    fn triangle_line_fill_test_success() {
        create_triangle_images();

        let expected = fs::read(TRIANGLE).unwrap_or_else(|err| {
            panic!("Error reading {}: {}", TRIANGLE, err);
        });

        let result = fs::read(TRIANGLE_CREATED_BY_TEST).unwrap_or_else(|err| {
            panic!("Error reading {}: {}", TRIANGLE_CREATED_BY_TEST, err);
        });

        assert_eq!(expected, result);

        let expected = fs::read(TRIANGLE_FILL).unwrap_or_else(|err| {
            panic!("Error reading {}: {}", TRIANGLE, err);
        });

        let result = fs::read(TRIANGLE_FILL_CREATED_BY_TEST).unwrap_or_else(|err| {
            panic!("Error reading {}: {}", TRIANGLE_CREATED_BY_TEST, err);
        });

        assert_eq!(expected, result);
    }
}
