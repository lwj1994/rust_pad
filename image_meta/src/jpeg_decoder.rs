use crate::decoder::Decoder;
use crate::ImageMeta;
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::io::Error;
use jpeg_decoder::ImageInfo;

pub(crate) struct JpegDecoder {}

impl Decoder for JpegDecoder {
    fn decode(&self, file_path: String, mime_type: String) -> Result<ImageMeta, Error> {
        let file = File::open(file_path).expect("failed to open file");
        let mut decoder = jpeg_decoder::Decoder::new(BufReader::new(file));
        match decoder.read_info() {
            Ok(_) => {
                match decoder.info() {
                    None => {
                        Err(Error::new(ErrorKind::Other, "failed to get info"))
                    }
                    Some(metadata) => {
                        Ok(ImageMeta {
                            width: metadata.width as u32,
                            height: metadata.height as u32,
                            frames: 1,
                            mime_type: mime_type.to_string(),
                        })
                    }
                }
            }
            Err(e) => {
                Err(Error::new(ErrorKind::Other, e))
            }
        }

    }

    fn can_parse(&self, file_path: String, mime_type: String) -> bool {
        str::eq(&mime_type, "image/jpeg")
    }
}