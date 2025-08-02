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

    draw_line(&mut 13, &mut 20, &mut 80, &mut 40, &mut image, &white);
    draw_line(&mut 20, &mut 13, &mut 40, &mut 80, &mut image, &red);
    draw_line(&mut 80, &mut 40, &mut 13, &mut 20, &mut image, &color);

    match image.write_to_file("test.tga") {
        Ok(()) => (),
        Err(why) => panic!("Error writing image to file: {}", why),
    }
}

// First iteration of drawing the line
fn draw_line(
    x0: &mut i32,
    y0: &mut i32,
    x1: &mut i32,
    y1: &mut i32,
    image: &mut tga::Image,
    color: &tga::Color,
) {
    let mut steep = false;
    if (*x1 - *x0).abs() < (*y1 - *y0).abs() {
        //if our line is steep, ie dx < dy, we can transpose the image
        std::mem::swap(x0, y0);
        std::mem::swap(x1, y1);
        steep = true;
    }
    if *x0 > *x1 {
        std::mem::swap(x0, x1);
        std::mem::swap(y0, y1);
    }
    for x in *x0..*x1 {
        let t = (x - *x0) as f32 / (*x1 - *x0) as f32;
        let y = (*y0 as f32 * (1.0 - t)) + (*y1 as f32 * t);
        if steep {
            // detranspose the image
            image.set(y as u16, x as u16, color);
        } else {
            image.set(x as u16, y as u16, color);
        }
    }
}
