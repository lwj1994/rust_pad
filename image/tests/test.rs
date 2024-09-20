extern crate image_pad;

use image_pad::add;
use image_pad::getImageMeta;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn get_imageMeta() {
        getImageMeta("qweq");
    }
}
