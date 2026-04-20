//! Photo loading, metadata, and basic transformations.

use std::io::Cursor;
use std::path::Path;

use bytes::Bytes;
use image::{DynamicImage, ImageFormat, ImageReader};
use serde::{Deserialize, Serialize};

use common::Result;

/// Dimensions and format information for a photo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoInfo {
    pub width: u32,
    pub height: u32,
    pub format: String,
}

/// Load a photo from disk and return basic info about it.
pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<(DynamicImage, PhotoInfo)> {
    let reader = ImageReader::open(path.as_ref())
        .map_err(|e| common::Error::Io(e))?
        .with_guessed_format()
        .map_err(|e| common::Error::Io(e))?;
    let format = reader.format();
    let image = reader
        .decode()
        .map_err(|e| common::Error::Other(anyhow::anyhow!(e)))?;
    let info = PhotoInfo {
        width: image.width(),
        height: image.height(),
        format: format_label(format),
    };
    Ok((image, info))
}

/// Decode a photo from an in-memory byte buffer.
pub fn decode(bytes: &Bytes) -> Result<(DynamicImage, PhotoInfo)> {
    let reader = ImageReader::new(Cursor::new(bytes.clone()))
        .with_guessed_format()
        .map_err(|e| common::Error::Io(e))?;
    let format = reader.format();
    let image = reader
        .decode()
        .map_err(|e| common::Error::Other(anyhow::anyhow!(e)))?;
    let info = PhotoInfo {
        width: image.width(),
        height: image.height(),
        format: format_label(format),
    };
    Ok((image, info))
}

/// Resize an image preserving aspect ratio so it fits in `max_side` pixels.
pub fn thumbnail(image: &DynamicImage, max_side: u32) -> DynamicImage {
    image.thumbnail(max_side, max_side)
}

fn format_label(format: Option<ImageFormat>) -> String {
    match format {
        Some(f) => format!("{:?}", f).to_lowercase(),
        None => "unknown".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thumbnail_shrinks_large_image() {
        let image = DynamicImage::new_rgb8(800, 600);
        let thumb = thumbnail(&image, 100);
        assert!(thumb.width() <= 100);
        assert!(thumb.height() <= 100);
    }
}
