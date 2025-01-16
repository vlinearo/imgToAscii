#[allow(
    dead_code
,   unused_variables
,   unused_imports
)]

use imaget::Imaget;

use std::path::Path;


#[tokio::main]
async  fn main() {
    let path = Path::new("./anime_2.jpg");
    let width = 100;

    let mut image = Imaget::open(path).await.unwrap();
    let ascii_art = image.resize(width).await.expect("Cant resize!").img_to_ascii_convertor().await.unwrap();
    println!("{}", ascii_art)
}