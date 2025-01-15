#[allow(
    dead_code
,   unused_variables
,   unused_imports
)]

use std::path::Path;

use image::{ DynamicImage, GenericImageView, ImageBuffer, Luma, Pixels
};


const GRADIENT: &[u8] = b" .:!/r(l1Z4H9W8$@";

#[derive(Debug)]
struct Image {
    img: DynamicImage
}

impl Image {
    fn open(path: &Path) -> Self {
        Self { img: image::open(path).expect("Image doesn't exist!") }
    }
}

fn main() {
    let path = Path::new("./anime.jpg");
    let gray_image = Image::open(&path).img.into_luma8();
}