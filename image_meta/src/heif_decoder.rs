use std::io::{Error, ErrorKind};
use libheif_rs::HeifContext;
use crate::decoder::Decoder;
use crate::exif::get_exif;
use crate::ImageMeta;

pub(crate) struct HeifDecoder {}

impl Decoder for HeifDecoder {
    fn decode(&self, file_path: String, mime_type: String) -> Result<ImageMeta, Error> {
        let ctx = HeifContext::read_from_file(&file_path);
        match ctx {
            Ok(e) => {
                match e.primary_image_handle() {
                    Ok(e) => {
                        let mut width = e.width();
                        let mut height = e.height();
                        let mut orientation = 0;
                        match get_exif(&file_path) {
                            Ok(e) => {
                                width = e.width;
                                height = e.height;
                                orientation = e.orientation;
                            }
                            Err(_) => {}
                        };
                        Ok(ImageMeta {
                            width,
                            height,
                            orientation: orientation as u8,
                            num_frames: 0,
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
}