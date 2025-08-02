mod tga;

fn main() {
    let white = tga::Color(255, 255, 255, 255);
    let red = tga::Color(255, 0, 0, 255);
    let green = tga::Color(0, 255, 0, 255);
    let blue = tga::Color(0, 0, 255, 255);
    let color = tga::Color(150, 100, 50, 255);

    let mut image = tga::Image::new(
        100,
        100,
        tga::ImageType::UncompressedTrueColor,
        tga::ColorType::RGB,
    );
    draw_line(13, 20, 80, 40, &mut image, &white);
    draw_line(20, 13, 40, 80, &mut image, &color);
    draw_line(80, 40, 13, 20, &mut image, &red);

    match image.write_to_file("test.tga") {
        Ok(()) => (),
        Err(why) => panic!("Error writing image to file: {}", why),
    }
}

// First iteration of drawing the line
fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut tga::Image, color: &tga::Color) {
    for x in x0..x1 {
        let t = (x - x0) as f32 / (x1 - x0) as f32;
        let y = (y0 as f32 * (1.0 - t)) + (y1 as f32 * t);

        image.set(x as u16, y as u16, color);
    }
}
