#[allow(
    unused_variables,
    unused_imports,
    dead_code
)]

use std::path::Path;
use std::path::PathBuf;

use image::{
    DynamicImage,
    GenericImageView,
    ImageBuffer,
    Luma
};

use tokio::task;

use color_eyre::{
    Result,
    eyre::{self, Ok}
};

#[derive(Debug)]
pub struct Image {
    img: Option<DynamicImage>
}

impl Image {
    async fn open(path_i: impl Into<PathBuf>) -> Result<Self> {
        let path = path_i.into();
        let img = task::spawn_blocking(
            move || image::open(path).map_err(|err| eyre::eyre!("Error: {}", err)))
            .await?
            .map_err(|err| eyre::eyre!("Error: {}", err))?;
        Ok(Self { img: Some(img) })
    }

    async fn resize(&mut self, nwidth: u32) -> Result<Self> {
        if let Some( img ) = &self.img {
            let img = img.clone();
            let resized_img = task::spawn_blocking(move || {
                let (width, height) = img.dimensions();
                let nheight = (nwidth as f32 * height as f32 / width as f32 * 0.55) as u32;
                img.resize_exact(nwidth, nheight, image::imageops::FilterType::Nearest)
            })
            .await?;

            Ok(Self { img: Some(resized_img)})
        } else {
            Err(eyre::eyre!("File does not exist!\nNothing to resize."))
        }
    }
}
