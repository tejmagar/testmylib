use std::io::Cursor;

use image::{ImageBuffer, Rgba};


pub fn raw_pixels_from_padded_buffer(
    width: usize,
    height: usize,
    channels: usize,
    row_stride: usize,
    buffer: &mut [u8]
) -> Result<Vec<u8>, String> {
    let mut raw_pixels = Vec::with_capacity(width * height * channels);

    for row in 0..height {
        let start = row * row_stride;
        let end = start + width * channels;
        raw_pixels.extend_from_slice(&buffer[start..end]);
    }

    Ok(raw_pixels)
}

pub fn bytes_from_raw(
    width: usize,
    height: usize,
    channels: usize,
    row_stride: usize,
    buffer: &mut [u8],
    convert_to: String,
) -> Result<Vec<u8>, String> {
        let raw_pixels =
            raw_pixels_from_padded_buffer(width, height, channels, row_stride, buffer)?;

        match convert_to.to_lowercase().as_str() {
            "rgb" => {
                return get_png_from_raw_pixels(width, height, channels, raw_pixels);
            }
            _ => {
                return Err(format!("Format unavailable: {}", convert_to));
            }
        }
}

pub fn get_png_from_raw_pixels(
    width: usize,
    height: usize,
    channels: usize,
    raw_pixels: Vec<u8>
) -> Result<Vec<u8>, String> {
    let image: ImageBuffer<Rgba<u8>, _>  = match ImageBuffer::from_raw(width as u32, height as u32, raw_pixels) {
        Some(image) => image,
        None => {
            return Err("Something wrong with image.".to_string());
        }
    };

    let mut cursor = Cursor::new(Vec::new());
    if let Err(error) = image.write_to(&mut cursor, image::ImageFormat::Png) {
        return Err(error.to_string());
    };

    Ok(cursor.into_inner())
}
