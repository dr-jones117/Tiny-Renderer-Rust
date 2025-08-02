mod tga;

fn main() {
    let white = tga::Color(255, 255, 255, 255);
    let red = tga::Color(255, 0, 0, 255);
    let green = tga::Color(0, 255, 0, 255);
    let blue = tga::Color(0, 0, 255, 255);
    let color = tga::Color(150, 100, 50, 255);

    let mut image = tga::Image::new(
        1280,
        720,
        tga::ImageType::UncompressedTrueColor,
        tga::ColorType::RGB,
    );

    draw_line(50, 20, 100, 40, &mut image, &white);
    draw_line(400, 600, 300, 0, &mut image, &red);
    draw_line(50, 2000, 100, 40, &mut image, &green);
    draw_line(0, 0, 600, 500, &mut image, &blue);
    draw_line(1280, 720, 0, 500, &mut image, &color);

    match image.write_to_file("test.tga") {
        Ok(()) => (),
        Err(why) => panic!("Error writing image to file: {}", why),
    }
}

// First iteration of drawing the line
fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut tga::Image, color: &tga::Color) {
    let mut t: f32 = 0.0;

    while t < 1.0 {
        let x = x0 + ((x1 - x0) as f32 * t) as i32;
        let y = y0 + ((y1 - y0) as f32 * t) as i32;

        image.set(x as u16, y as u16, color);
        t += 0.01;
    }
}
