mod decoder;
mod png_decoder;
mod registry;
mod heif_decoder;
mod jpeg_decoder;
mod gif_decoder;
mod webp_decoder;
mod exif;

use file_format::FileFormat;
use std::fmt;
use std::io::{Error, ErrorKind, Result};

fn get_format(file_path: &str) -> Result<FileFormat> {
    FileFormat::from_file(file_path)
}

pub fn read(file_path: &str) -> Result<ImageMeta> {
    let format = get_format(file_path)?;
    let mime_type = format.media_type();
    match registry::REGISTRY.get(mime_type.to_string()) {
        None => {
            Err(Error::new(ErrorKind::Other, "no decoder"))
        }
        Some(e) => {
            e.decode(file_path.to_string(), mime_type.to_string())
        }
    }
}


pub struct ImageMeta {
    pub width: u32,
    pub height: u32,
    pub num_frames: u32,
    pub orientation: u8,
    pub mime_type: String,
}

impl fmt::Display for ImageMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ImageMeta(width={}, height={}, mime_type={}, num_frames={}, orientation={})", self.width, self.height, self.mime_type, self.num_frames, self.orientation)
    }
}




