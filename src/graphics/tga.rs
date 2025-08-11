use crate::graphics::{
    color::{Color, ColorType},
    output::RenderOutputter,
};
use bytemuck::{Pod, Zeroable};
use std::{fs::File, io::Write, path::Path};

#[allow(dead_code)]
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

impl RenderOutputter for Image {
    fn width(&self) -> u16 {
        self.header.width
    }

    fn height(&self) -> u16 {
        self.header.height
    }

    fn set(&mut self, x: i32, y: i32, color: &Color) {
        let width = self.width() as i32;
        let height = self.height() as i32;

        if self.data.is_empty() || x >= width || y >= height || x < 0 || y < 0 {
            return;
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
    }

    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.write_to_file("test.tga") {
            Err(err) => Err(Box::new(err)),
            Ok(()) => Ok(()),
        }
    }
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
}
