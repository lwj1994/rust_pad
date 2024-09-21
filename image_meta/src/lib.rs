mod decoder;
mod png_decoder;
mod registry;
use crate::decoder::Decoder;
use file_format::FileFormat;
use std::fmt;
use std::io::Read;
use std::io::{BufRead, Error, ErrorKind, Result, Seek};

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
    pub mime_type: String,
}

impl fmt::Display for ImageMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ImageMeta(width={}, height={},mime_type={})", self.width, self.height, self.mime_type)
    }
}




