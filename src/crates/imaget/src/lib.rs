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
    img: DynamicImage
}

impl Image {
    async fn open(path_i: impl Into<PathBuf>) -> Result<Self> {
        let path = path_i.into();
        let img = task::spawn_blocking(
            move || image::open(path).map_err(|err| eyre::eyre!("Error: {}", err)))
            .await?
            .map_err(|err| eyre::eyre!("Error: {}", err))?;
        Ok(Self { img })
    }
}
