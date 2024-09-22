use std::io::{Error, ErrorKind};
use std::fs::File;
use crate::decoder::Decoder;
use crate::ImageMeta;

pub(crate) struct PngDecoder {}

impl Decoder for PngDecoder {
    fn decode(&self, file_path: String, mime_type: String) -> Result<ImageMeta, Error> {
        let decoder = png::Decoder::new(File::open(&file_path)?);
        match decoder.read_info() {
            Ok(e) => {
                let info = e.info();
                let mut frames = 0;
                match info.animation_control {
                    None => {}
                    Some(e) => {
                        frames = e.num_frames
                    }
                }
                Ok(ImageMeta {
                    width: info.width,
                    height: info.height,
                    num_frames: frames,
                    orientation: 0,
                    mime_type: mime_type.to_string(),
                })
            }
            Err(e) => {
                Err(Error::new(ErrorKind::Other, e.to_string()))
            }
        }
    }
}