use std::io;
use imagesize;
use crate::decoder::Decoder;
use crate::ImageMeta;

pub(crate) struct HeifDecoder {}

impl Decoder for HeifDecoder {
    fn decode(&self, file_path: String, mime_type: String) -> io::Result<ImageMeta> {
        match imagesize::size(&file_path) {
            Ok(e) => {
                Ok(ImageMeta {
                    width: e.width as u32,
                    height: e.height as u32,
                    num_frames: 0,
                    orientation: 0,
                    mime_type: mime_type.to_string(),
                })
            }
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e))
        }
    }
}