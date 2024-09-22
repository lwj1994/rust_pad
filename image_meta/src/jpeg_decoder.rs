use crate::decoder::Decoder;
use crate::ImageMeta;
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::io;
use crate::exif::get_exif;

pub(crate) struct JpegDecoder {}

impl Decoder for JpegDecoder {
    fn decode(&self, file_path: String, mime_type: String) -> io::Result<ImageMeta> {
        let file = File::open(&file_path).expect("failed to open file");
        let mut decoder = jpeg_decoder::Decoder::new(BufReader::new(&file));
        match decoder.read_info() {
            Ok(_) => {
                match decoder.info() {
                    None => {
                        Err(io::Error::new(ErrorKind::Other, "failed to get info"))
                    }
                    Some(metadata) => {
                        let mut width = metadata.width as u32;
                        let mut height = metadata.height as u32;
                        let mut orientation = 0;
                        match  get_exif(&file_path) {
                            Ok(e) => {
                                width = e.width;
                                height = e.height;
                                orientation = e.orientation;
                            }
                            Err(_) => {}
                        }

                        Ok(ImageMeta {
                            width,
                            height,
                            num_frames: 0,
                            orientation: orientation as u8,
                            mime_type: mime_type.to_string(),
                        })
                    }
                }
            }
            Err(e) => {
                Err(io::Error::new(ErrorKind::Other, e))
            }
        }
    }
}