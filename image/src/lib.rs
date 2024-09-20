use image::{DynamicImage, GenericImageView};
mod bean;

pub fn getImageMeta(filePath: &str) ->  bean::ImageMeta{
    let rect1 = bean::ImageMeta{ width: 30, height: 40 };
    print!("rect1: {}",rect1);
    return  rect1;
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}



