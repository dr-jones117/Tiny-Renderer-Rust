mod tga;

fn main() {
    let color = tga::Color(150, 100, 50, 255);

    let mut image = tga::Image::new(
        100,
        100,
        tga::ImageType::UncompressedTrueColor,
        tga::ColorType::RGB,
    );

    draw_line(80, 40, 13, 20, &mut image, &color);

    match image.write_to_file("test.tga") {
        Ok(()) => (),
        Err(why) => panic!("Error writing image to file: {}", why),
    }
}

fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut tga::Image, color: &tga::Color) {
    let steep = (x1 - x0).abs() < (y1 - y0).abs();

    // transpose it if it's steep
    let (x0, y0, x1, y1) = if steep {
        (y0, x0, y1, x1)
    } else {
        (x0, y0, x1, y1)
    };

    // if going right to left, we need to swap the points to go left to right
    let (x0, y0, x1, y1) = if x0 > x1 {
        (x1, y1, x0, y0)
    } else {
        (x0, y0, x1, y1)
    };

    let dx = x1 - x0;
    let dy = y1 - y0;

    let derror = (dy * 2).abs();
    let mut error = 0;
    let mut y = y0;

    for x in x0..=x1 {
        if steep {
            // detranspose the image
            image.set(y as u16, x as u16, color);
        } else {
            image.set(x as u16, y as u16, color);
        }

        error += derror;

        if error > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error -= dx * 2;
        }
    }
}
