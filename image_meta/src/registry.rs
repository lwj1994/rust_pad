use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::decoder::Decoder;
use crate::png_decoder::PngDecoder;

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
        registry
    };
}
