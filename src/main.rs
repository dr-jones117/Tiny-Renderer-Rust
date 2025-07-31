use std::fs::File;
use std::io::{Read, Result};

#[derive(Debug)]
struct TgaHeader {
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

fn main() -> Result<()> {
    let mut file = File::open("earth.tga")?;
    let mut buf = [0u8; 18]; // TGA header is exactly 18 bytes
    file.read_exact(&mut buf)?;

    let header = TgaHeader {
        id_length: buf[0],
        color_map_type: buf[1],
        image_type: buf[2],
        cm_first_entry_index: u16::from_le_bytes([buf[3], buf[4]]),
        cm_length: u16::from_le_bytes([buf[5], buf[6]]),
        cm_entry_size: buf[7],
        x_origin: u16::from_le_bytes([buf[8], buf[9]]),
        y_origin: u16::from_le_bytes([buf[10], buf[11]]),
        width: u16::from_le_bytes([buf[12], buf[13]]),
        height: u16::from_le_bytes([buf[14], buf[15]]),
        bits_per_pixel: buf[16],
        image_descriptor: buf[17],
    };
    let bytes_left: u32 =
        header.width as u32 * header.height as u32 * (header.bits_per_pixel / 8) as u32;

    println!("bytesLeft: {bytes_left}");
    println!("{:#?}", header);
    println!("Image dimensions: {} x {}", header.width, header.height);

    Ok(())
}

