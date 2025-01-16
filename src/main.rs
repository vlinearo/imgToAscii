#[allow(
    dead_code
,   unused_variables
,   unused_imports
)]

use imaget::Imaget;

use std::path::Path;

use image::{
    DynamicImage,
    GenericImageView,
    ImageBuffer,
    Luma
};

use tokio::*;



// #[derive(Debug)]
// struct Image;

// impl Image {
//     fn to_grayscale(image: DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
//         image.into_luma8()
//     }

//     fn map_pixels_to_ascii(image: ImageBuffer<Luma<u8>, Vec<u8>>, width: u32) -> String {
//         let ascii_str: String = image
//             .pixels()
//             .map(|p| {
//                 let ascii_index = p[0] as usize * (GRADIENT.len() - 1) / 255;
//                 GRADIENT[ascii_index] as char
//             })
//             .collect();

//         let mut ascii_lines = Vec::new();
//         for chunk in ascii_str.chars().collect::<Vec<char>>().chunks(width as usize) {
//             ascii_lines.push(chunk.iter().collect::<String>());
//         }

//         ascii_lines.join("\n")
//     }
// }

#[tokio::main]
async  fn main() {
    let path = Path::new("./anime.jpg");
    let width = 150;

    let mut image = Imaget::open(path).await.unwrap();
    image.resize(width).await.unwrap();
}