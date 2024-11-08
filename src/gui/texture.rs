//! Utilities for constructing SDL2 textures from images.
//!
//! Fir uses the [`image`] crate for loading images, but SDL2 for displaying them. So, we need some
//! glue code to help map between the two.

use std::fmt::Display;

use derive_more::derive::From;
use image::DynamicImage;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureCreator, TextureValueError};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

// --- TextureFactory --- //////////////////////////////////

/// Factory type for constructing SDL2 textures from image files.
///
/// Fir uses the [`image`] crate for loading images, but SDL2 for displaying them. This struct acts
/// as glue between the two.
pub struct TextureFactory {
    texture_creator: TextureCreator<WindowContext>,
}

impl TextureFactory {
    /// Construct a new TextureFactory using the given [`sdl2::render::TextureCreator`].
    pub fn new(texture_creator: TextureCreator<WindowContext>) -> Self {
        Self { texture_creator }
    }

    /// Construct an [`sdl2::render::Texture`] from some image data.
    pub fn construct_from_image(
        &self,
        image: &DynamicImage,
    ) -> Result<Texture, TextureCreationError> {
        let mut data = image.to_rgba8().into_flat_samples();
        let width = data.layout.width;
        let height = data.layout.height;
        let pitch = data.layout.height_stride as u32;
        let surf = Surface::from_data(
            data.as_mut_slice(),
            width,
            height,
            pitch,
            PixelFormatEnum::RGBA32,
        )?;
        Ok(surf.as_texture(&self.texture_creator)?)
    }
}

// --- TextureCreationError  --- ///////////////////////////

/// Possible failure modes for [`TextureFactory::construct_from_image`].
///
/// NOTE: The function constructs an intermediary [sdl2::surface::Surface]. This is reason for the
/// `SurfaceCreationFailed` variant.
#[derive(Clone, Debug, From)]
pub enum TextureCreationError {
    /// An error occured when creating an [`sdl2::surface::Surface`] from the data.
    SurfaceCreationFailed(String),
    /// An error occured when creating an [`sdl2::render::Texture`] from the surface.
    TextureCreationFailed(TextureValueError),
}

impl Display for TextureCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SurfaceCreationFailed(s) => writeln!(f, "surface creation failed: {s}"),
            Self::TextureCreationFailed(e) => writeln!(f, "{e}"),
        }
    }
}

impl std::error::Error for TextureCreationError {}
