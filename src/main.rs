mod tga;

fn main() {
    let white = tga::Color(255, 255, 255, 255);

    let mut image = tga::Image::new(
        100,
        100,
        tga::ImageType::UncompressedTrueColor,
        tga::ColorType::RGB,
    );

    draw_line(50, 20, 100, 40, &mut image, &white);

    match image.write_to_file("test.tga") {
        Ok(()) => (),
        Err(why) => panic!("Error writing image to file: {}", why),
    }
}

// First iteration of drawing the line
fn draw_line(x0: u16, y0: u16, x1: u16, y1: u16, image: &mut tga::Image, color: &tga::Color) {
    let mut t: f32 = 0.0;

    while t < 1.0 {
        let x = x0 + ((x1 - x0) as f32 * t) as u16;
        let y = y0 + ((y1 - y0) as f32 * t) as u16;

        image.set(x, y, color);
        t += 0.01;
    }
}
