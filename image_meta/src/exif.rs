use std::{fmt, io};
use std::fs::File;


pub(crate) fn get_exif(file_path: &str) -> io::Result<ExifInfo> {
    let file = File::open(file_path).expect("failed to open file");
    let mut bufreader = std::io::BufReader::new(&file);
    match exif::Reader::new().read_from_container(&mut bufreader) {
        Ok(e) => {
            let mut width = 0;
            let mut height = 0;

            // width
            match e.get_field(exif::Tag::PixelXDimension, exif::In::THUMBNAIL) {
                Some(field) => {
                    if let Some(w) = field.value.get_uint(0) {
                        width = w;
                        println!("width = {width}")
                    }
                }
                None => {}
            }

            // height
            match e.get_field(exif::Tag::PixelYDimension, exif::In::THUMBNAIL) {
                Some(field) => {
                    if let Some(h) = field.value.get_uint(0) {
                        height = h;
                    }
                }
                None => {}
            }


            let mut orientation: u8 = 0;
            match e.get_field(exif::Tag::Orientation, exif::In::THUMBNAIL) {
                None => {}

                Some(e) => {
                    match e.value.get_uint(0) {
                        None => {}
                        Some(e) => {
                            println!("orientation = {e}");
                            if (0..=8).contains(&e) {
                                orientation = e as u8;
                                if [5, 6, 7, 8].contains(&e) {
                                    std::mem::swap(&mut width, &mut height);
                                }
                            }
                        }
                    }
                }
            }

            Ok(ExifInfo {
                width,
                height,
                orientation,
            })
        }
        Err(e) => {
            Err(io::Error::new(io::ErrorKind::Other, e))
        }
    }
}


pub struct ExifInfo {
    pub width: u32,
    pub height: u32,
    pub orientation: u8,
}

impl fmt::Display for ExifInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExifInfo(width={}, height={}, orientation={})", self.width, self.height, self.orientation, )
    }
}
