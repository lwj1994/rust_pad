use std::io::{Error, ErrorKind};
use std::fs::File;
use png::FrameControl;
use crate::decoder::Decoder;
use crate::ImageMeta;

pub(crate) struct PngDecoder {}

impl Decoder for PngDecoder {
    fn decode(&self, file_path: String, mime_type: String) -> Result<ImageMeta, Error> {
        let decoder = png::Decoder::new(File::open(&file_path)?);
        match decoder.read_info() {
            Ok(e) => {
                let info = e.info();
                let mut  frames = 1;
                match info.animation_control {
                    None => {
                    }
                    Some(e) => {
                        frames = e.num_frames
                    }
                }
                Ok(ImageMeta {
                    width: info.width,
                    height: info.height,
                    frames,
                    mime_type: mime_type.to_string(),
                })
            }
            Err(e) => {
                Err(Error::new(ErrorKind::Other, e.to_string()))
            }
        }
    }

    fn can_parse(&self, file_path: String, mime_type: String) -> bool {
        str::eq(&mime_type, "image/png")
    }
}