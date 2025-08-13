#[allow(dead_code)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ColorType {
    RGB,
    GrayScale,
    RGBA,
}

impl ColorType {
    pub fn bytes_per_pixel(&self) -> u8 {
        match self {
            ColorType::GrayScale => 1,
            ColorType::RGB => 3,
            ColorType::RGBA => 4,
        }
    }
}
