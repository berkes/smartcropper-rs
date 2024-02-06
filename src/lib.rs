use std::path::Path;

use image::{GenericImageView, ImageError};

pub struct SmartCropper {
    pub original: image::DynamicImage,
    pub cropped: Option<image::DynamicImage>,
}

impl SmartCropper {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ImageError> {
        let img = image::open(path)?;
        Ok(SmartCropper {
            original: img,
            cropped: None,
        })
    }

    pub fn smart_crop(&mut self, width: u32, height: u32) -> Result<(), ImageError> {
        let img = self.original.clone();
        let (x, y) = self.find_interesting_region(width, height);
        let cropped = img.crop_imm(x, y, width, height);
        self.cropped = Some(cropped);
        Ok(())
    }

    pub fn smart_square(&mut self) -> Result<(), ImageError> {
        let (width, height) = self.original.dimensions();
        let size = std::cmp::min(width, height);
        self.smart_crop(size, size)
    }

    pub fn dimensions(&self) -> (u32, u32) {
        match self.cropped {
            Some(ref img) => img.dimensions(),
            None => (0, 0),
        }
    }

    fn find_interesting_region(&self, width: u32, height: u32) -> (u32, u32) {
        let _ = width;
        let _ = height;
        // This is a placeholder for the actual implementation
        (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimensions_before_cropping() {
        let img = SmartCropper::from_file("tests/fixtures/entropyish.png").unwrap();
        assert_eq!(img.dimensions(), (0, 0));
    }

    #[test]
    fn test_dimensions_after_cropping() {
        let mut img = SmartCropper::from_file("tests/fixtures/entropyish.png").unwrap();
        img.smart_crop(100, 100).unwrap();
        assert_eq!(img.dimensions(), (100, 100));
    }
}
