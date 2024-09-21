extern crate image_meta;

use image_meta::read;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_meta() {
        // test_get_format("tests/images/2.jpg");
        // return;
        let images_dir = fs::read_dir("tests/images").unwrap();
        images_dir.for_each(|e| {
            test_get_format(e.unwrap().path().to_str().unwrap());
            println!("\n");
        });
    }


    fn test_get_format(file_path: &str) {
        println!("path {}", file_path);
        let image_mate_res = read(file_path);
        match image_mate_res {
            Ok(e) => println!("res {}", e),
            Err(e) => {
                println!("error {}", e)
            }
        }
    }
}
