use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::decoder::Decoder;
use crate::gif_decoder::GifDecoder;
use crate::heif_decoder::HeifDecoder;
use crate::jpeg_decoder::JpegDecoder;
use crate::png_decoder::PngDecoder;
use crate::webp_decoder::WebpDecoder;

pub struct DecoderRegistry {
    decoders: HashMap<String, Box<dyn Decoder>>,
}

impl DecoderRegistry {
    pub(crate)fn new() -> Self {
        DecoderRegistry {
            decoders: HashMap::new(),
        }
    }

    pub fn register(&mut self, mime_type: String, parser: Box<dyn Decoder>) {
        self.decoders.insert(mime_type, parser);
    }

    pub(crate) fn get(&self, mime_type: String) -> Option<&Box<dyn Decoder>> {
        self.decoders.get(&mime_type)
    }
}

lazy_static! {
     pub static ref REGISTRY: DecoderRegistry = {
        let mut registry = DecoderRegistry::new();
        registry.register("image/png".to_string(),Box::new(PngDecoder{}));
        registry.register("image/apng".to_string(),Box::new(PngDecoder{}));
        registry.register("image/heif".to_string(),Box::new(HeifDecoder{}));
        registry.register("image/heic".to_string(),Box::new(HeifDecoder{}));
        registry.register("image/jpeg".to_string(),Box::new(JpegDecoder{}));
        registry.register("image/gif".to_string(),Box::new(GifDecoder{}));
        registry.register("image/webp".to_string(),Box::new(WebpDecoder{}));
        registry
    };
}
