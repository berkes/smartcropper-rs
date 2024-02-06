use std::path::Path;

use image::{GenericImageView, ImageError, Pixel};

pub struct SmartCropper {
    pub original: image::DynamicImage,
    pub cropped: Option<image::DynamicImage>,
}

#[derive(Debug, PartialEq)]
struct Region {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
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
        let region = self.find_interesting_region(width, height);
        let cropped = img.crop_imm(region.x, region.y, width, height);
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

    fn find_interesting_region(&self, width: u32, height: u32) -> Region {
        // // Determine the amount of pixels to crop by comparing the original size and the target size
        // // and then dividing the difference by N_H and N_W. Do this for both width and height.
        let regions = Self::regions(self.original.dimensions(), (width, height));


        let mut max_entropy = 0.0;
        let mut max_region = Region {
            x: 0,
            y: 0,
            width,
            height,
        };

        // Extract a region of the original image for each possible position
        // and calculate the entropy of the region. The region with the highest entropy is the most
        // interesting
        for region in regions {
            let slice = self.original.view(region.x, region.y, width, height);

            let entropy = Self::entropy(slice);
            if entropy > max_entropy {
                max_entropy = entropy;
                max_region = region;
            }
        }

        max_region
    }

    fn entropy(slice: image::SubImage<&image::DynamicImage>) -> f64 {
        let mut histogram = [0; 256];
        for (_, _, pixel) in slice.pixels() {
            let channels = pixel.channels();
            let r = channels[0] as usize;
            let g = channels[1] as usize;
            let b = channels[2] as usize;
            histogram[r] += 1;
            histogram[g] += 1;
            histogram[b] += 1;
        }

        let total = slice.width() * slice.height() * 3;
        histogram
            .iter()
            .filter(|&&x| x > 0)
            .map(|&x| {
                let p = x as f64 / total as f64;
                -p * p.log2()
            })
            .sum()
    }

    fn regions((ow, oh): (u32, u32), (tw, th): (u32, u32)) -> Vec<Region> {
        // We make a list of potiential regions. Each region overlaps with the previous one by half
        // of the regions width and heihgt. The last region on each row and of each column must be
        // within the original image
        if ow == tw && oh == th {
            return vec![Region { x: 0, y: 0, width: tw, height: th }];
        } else if ow < tw || oh < th {
            return vec![];
        } else {
            (0..ow - tw)
                .step_by((tw / 2) as usize)
                .flat_map(|x| {
                    (0..oh - th)
                        .step_by((th / 2) as usize)
                        .map(move |y| (x, y))
                })
                .filter(|(x, y)| x + tw < ow && y + th < oh)
                .map(|(x, y)| Region {
                    x,
                    y,
                    width: tw,
                    height: th,
                }).collect()
        }
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

    #[test]
    fn test_find_interesting_region() {
        let img = SmartCropper::from_file("tests/fixtures/entropyish.png").unwrap();
        let expected_region = Region {
            x: 100,
            y: 100,
            width: 100,
            height: 100,
        };
        let actual_region = img.find_interesting_region(100, 100);
        assert_eq!(actual_region, expected_region);
    }

    #[test]
    fn test_regions_fit() {
        let expected = vec![
            Region { x: 0, y: 0, width: 100, height: 100 },
            Region { x: 0, y: 50, width: 100, height: 100 },
            Region { x: 50, y: 0, width: 100, height: 100 },
            Region { x: 50, y: 50, width: 100, height: 100 },
        ];
        let actual = SmartCropper::regions((200, 200), (100, 100));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_regions_exact_fit() {
        let expected = vec![
            Region { x: 0, y: 0, width: 100, height: 100 },
        ];
        let actual = SmartCropper::regions((100, 100), (100, 100));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_regions_too_high() {
        let expected = vec![];
        let actual = SmartCropper::regions((100, 100), (100, 200));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_regions_too_wide() {
        let expected = vec![];
        let actual = SmartCropper::regions((100, 100), (200, 100));
        assert_eq!(actual, expected);
    }
}
