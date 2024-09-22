use std::io::Error;
use crate::ImageMeta;


pub trait Decoder: Send + Sync {
    fn decode(&self, file_path: String, mime_type: String) -> Result<ImageMeta, Error>;
}



