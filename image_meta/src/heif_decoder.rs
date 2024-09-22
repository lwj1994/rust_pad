use std::io::{Error, ErrorKind};
use libheif_rs::HeifContext;
use crate::decoder::Decoder;
use crate::ImageMeta;

pub(crate) struct HeifDecoder {}

impl Decoder for HeifDecoder {
    fn decode(&self, file_path: String, mime_type: String) -> Result<ImageMeta, Error> {
        let ctx = HeifContext::read_from_file(&file_path);
        match ctx {
            Ok(e) => {
                match e.primary_image_handle() {
                    Ok(e) => {
                        Ok(ImageMeta {
                            width: e.width(),
                            height: e.height(),
                            frames: 1,
                            mime_type: mime_type.to_string(),
                        })
                    }
                    Err(e) => {
                        Err(Error::new(ErrorKind::Other, e))
                    }
                }
            }
            Err(e) => {
                Err(Error::new(ErrorKind::Other, e))
            }
        }
    }

    fn can_parse(&self, file_path: String, mime_type: String) -> bool {
        str::eq(&mime_type, "image/heic") || str::eq(&mime_type, "image/heif")
    }
}