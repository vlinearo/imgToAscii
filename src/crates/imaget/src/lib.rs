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
};

use tokio::task;

use color_eyre::{
    Result,
    eyre::{self, Ok}
};

const GRADIENT: &[u8] = b" .:!/r(l1Z4H9W8$@";

#[derive(Debug)]
pub struct Imaget {
    img: Option<DynamicImage>,
    size: Option<(u32, u32)>
}

impl Imaget {
    pub async fn open(path_i: impl Into<PathBuf>) -> Result<Self> {
        let path = path_i.into();
        let img = task::spawn_blocking(
            move || image::open(path).map_err(|err| eyre::eyre!("Error: {}", err)))
            .await?
            .map_err(|err| eyre::eyre!("Error: {}", err))?;
        Ok(Self { img: Some(img), size: None })
    }

    pub async fn resize(&mut self, nwidth: u32) -> Result<Self> {
        if let Some( img ) = &self.img {
            let img = img.clone();
            let (width, height) = img.dimensions();
            let nheight = (nwidth as f32 * height as f32 / width as f32 * 0.55) as u32;
            let resized_img = task::spawn_blocking(move || {
                img.resize_exact(nwidth, nheight, image::imageops::FilterType::Nearest)
            })
            .await?;

            Ok(Self { img: Some(resized_img), size: Some((nwidth, nheight)) })
        } else {
            Err(eyre::eyre!("File does not exist!\nNothing to resize."))
        }
    }

    pub async fn save(&self, path: impl Into<PathBuf>) -> Result<()> {
        if let Some( img ) = &self.img {
            let img = img.clone();
            let path = path.into();
            task::spawn_blocking(move || { img.save(path) }).await?.unwrap();
            Ok(())
        } else {
            Err(eyre::eyre!("File does not exist!\nNothing to save."))
        }
    }

    pub async fn img_to_ascii_convertor(&self) -> Result<String, eyre::Error> {
        if let Some(img) = &self.img {
            let img = img.clone();
            let size = self.size.ok_or_else(|| eyre::eyre!("Image size not specified"))?;
            let ascii_art = task::spawn_blocking(move || -> Result<String, eyre::Error> {
                let ascii_str: String = img
                    .pixels()
                    .map(|p| {
                        let brightness = 0.2126 * p.2[0] as f32
                                            + 0.7152 * p.2[1] as f32
                                            + 0.0722 * p.2[2] as f32;

                        let ascii_index = (brightness * (GRADIENT.len() - 1) as f32 / 255.0) as usize;
                        GRADIENT[ascii_index] as char
                    })
                    .collect();

                let mut ascii_lines = Vec::new();
                for chunk in ascii_str.chars().collect::<Vec<char>>().chunks(size.0 as usize) {
                    ascii_lines.push(chunk.iter().collect::<String>());
                }

                Ok(ascii_lines.join("\n"))
            })
            .await??;

            Ok(ascii_art)
        } else {
            Err(eyre::eyre!("File does not exist!\nNothing to convert."))
        }
    }
}
