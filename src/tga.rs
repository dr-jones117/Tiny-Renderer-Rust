use bytemuck::{Pod, Zeroable}; //TODO : what does this do?
use std::{fs::File, io::Write, path::Path};

use crate::tga;

pub struct Color(pub u8, pub u8, pub u8, pub u8);

#[derive(Debug)]
pub enum ColorType {
    RGB,
    GrayScale,
    RGBA,
}

#[derive(Debug)]
pub enum ImageType {
    UncompressedTrueColor,
    UncompressedGrayScale,
    RleTrueColor,
    RleGrayScale,
}

impl ImageType {
    fn get_value(&self) -> u8 {
        match self {
            ImageType::UncompressedTrueColor => 2,
            ImageType::UncompressedGrayScale => 3,
            ImageType::RleTrueColor => 10,
            ImageType::RleGrayScale => 11,
        }
    }
}

impl ColorType {
    fn bytes_per_pixel(&self) -> u8 {
        match self {
            ColorType::GrayScale => 1,
            ColorType::RGB => 3,
            ColorType::RGBA => 4,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Header {
    id_length: u8,
    color_map_type: u8,
    image_type: u8,
    cm_first_entry_index: u16,
    cm_length: u16,
    cm_entry_size: u8,
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    bits_per_pixel: u8,
    image_descriptor: u8,
}

impl Header {
    pub fn new(width: u16, height: u16, image_type: &ImageType, color_type: &ColorType) -> Header {
        Header {
            id_length: 0,
            color_map_type: 0,
            image_type: image_type.get_value(),
            cm_first_entry_index: 0,
            cm_length: 0,
            cm_entry_size: 0,
            x_origin: 0,
            y_origin: 0,
            width,
            height,
            bits_per_pixel: color_type.bytes_per_pixel() * 8,
            image_descriptor: 0,
        }
    }
}

#[derive(Debug)]
pub struct Image {
    color_type: ColorType,
    header: Header,
    data: Vec<u8>,
}

impl Image {
    pub fn new(width: u16, height: u16, image_type: ImageType, color_type: ColorType) -> Image {
        let data_length =
            (width as u32 * height as u32 * color_type.bytes_per_pixel() as u32) as usize;
        Image {
            header: Header::new(width, height, &image_type, &color_type),
            color_type,
            data: vec![0; data_length],
        }
    }

    pub fn set(&mut self, x: u16, y: u16, color: &Color) -> bool {
        let width = self.width();
        let height = self.height();

        if self.data.is_empty() || x >= width || y >= height {
            return false;
        }

        let bpp = self.color_type.bytes_per_pixel() as usize;
        let start = (x as u32 + y as u32 * width as u32) as usize * bpp;

        match self.color_type {
            ColorType::GrayScale => {
                panic!("unimplemented GrayScale in tga image set");
            }
            ColorType::RGB => {
                self.data[start] = color.2;
                self.data[start + 1] = color.1;
                self.data[start + 2] = color.0;
            }
            ColorType::RGBA => {
                panic!("unimplemented RGBA in tga image set");
            }
        }

        true
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: &tga::Color) {
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
                self.set(y as u16, x as u16, color);
            } else {
                self.set(x as u16, y as u16, color);
            }

            error += derror;

            if error > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error -= dx * 2;
            }
        }
    }

    pub fn write_to_file(&self, name: &str) -> std::io::Result<()> {
        let path = Path::new(name);
        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(_) => panic!("Error opening file"),
        };

        let header: &[u8] = bytemuck::bytes_of(&self.header); // TODO: How does bytemuck work?
        file.write_all(header)?;
        file.write_all(&self.data[..])?;
        Ok(())
    }

    fn width(&self) -> u16 {
        self.header.width
    }

    fn height(&self) -> u16 {
        self.header.height
    }
}
