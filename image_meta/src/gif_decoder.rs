use crate::decoder::Decoder;
use std::fs::File;
use std::io::Error;
use crate::ImageMeta;

pub(crate) struct GifDecoder {}

impl Decoder for GifDecoder {
    fn decode(&self, file_path: String, mime_type: String) -> Result<ImageMeta, Error> {
        let file = File::open(file_path).expect("failed to open file");
        // let mut decoder = gif::Decoder::new(BufReader::new(file))?;
        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::Indexed);
        let  decoder = options.read_info(file).expect("failed to read info");
        let  frames_num = 0;
        // while let Some(frame) = decoder.read_next_frame().expect("read_next_frame") {
        //     println!("buffer len {} kb",frame.buffer.len()/1024,);
        //     frames_num += 1;
        // }

        Ok(ImageMeta {
            width: decoder.width() as u32,
            height: decoder.height() as u32,
            num_frames: frames_num,
            orientation: 0,
            mime_type: mime_type.to_string(),
        })
    }
}