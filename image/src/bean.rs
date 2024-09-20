use std::fmt;

pub struct ImageMeta {
    pub width: u32,
    pub height: u32,
}

impl ImageMeta {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl fmt::Display for ImageMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ImageMeta(width={}, height={})", self.width, self.height)
    }
}