use crate::decoder::Decoder;
use std::fs::File;
use std::io::{BufReader, Error};
use crate::{ImageMeta};

pub(crate) struct WebpDecoder {}

impl Decoder for WebpDecoder {
    fn decode(&self, file_path: String, mime_type: String) -> Result<ImageMeta, Error> {
        let file = File::open(file_path).expect("failed to open file");
        let decoder = image_webp::WebPDecoder::new(BufReader::new(file)).expect("WebPDecoder error");
        let size = decoder.dimensions();
        Ok(ImageMeta {
            width: size.0,
            height: size.1,
            num_frames: decoder.num_frames(),
            orientation: 0,
            mime_type: mime_type.to_string(),
        })
    }
}