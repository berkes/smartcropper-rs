extern crate smartcropper;

use image_compare::{Algorithm};
use std::path::Path;

use smartcropper::SmartCropper;

fn fixture_path(name: &str) -> Box<Path> {
    // Use the relative path to the fixtures directory but make it absolute
    let mut path = std::env::current_dir().unwrap();
    path.push("tests/fixtures");
    path.push(name);
    path.into_boxed_path()
}

#[test]
fn create_from_image_file() {
    let cropper = SmartCropper::from_file(fixture_path("entropyish.png"));
    assert!(cropper.is_ok());
}

#[test]
#[should_panic(expected = "Unsupported")]
fn fail_on_text_file() {
    SmartCropper::from_file(fixture_path("entropyish.txt")).unwrap();
}

#[test]
fn crop_to_100x100() {
    let img_path = fixture_path("entropyish.png");
    let mut img = SmartCropper::from_file(img_path).unwrap();

    img.smart_crop(100, 100).unwrap();

    assert_eq!(img.dimensions(), (100, 100));
}

#[test]
fn square_image() {
    let img_path = fixture_path("entropyish.png");
    let mut img = SmartCropper::from_file(img_path).unwrap();

    img.smart_square().unwrap();

    let (width, height) = img.dimensions();
    assert_eq!(width, height);
}

#[test]
fn crop_to_100x100_and_compare() {
    let img_path = fixture_path("entropyish.png");
    let mut img = SmartCropper::from_file(img_path).unwrap();

    img.smart_crop(100, 100).unwrap();

    let expected_path = fixture_path("entropyish_cropped.png");
    let expected = SmartCropper::from_file(expected_path).unwrap();

    let score = image_compare::rgb_similarity_structure(
        &Algorithm::RootMeanSquared,
        &img.cropped.unwrap().into_rgb8(),
        &expected.original.into_rgb8(),
    )
    .unwrap()
    .score;

    assert_eq!(score, 1.0);
}

#[test]
fn write_cropped_image() {
    let img_path = fixture_path("entropyish.png");
    let mut img = SmartCropper::from_file(img_path).unwrap();

    img.smart_crop(100, 100).unwrap();
    img.cropped.unwrap().save("/tmp/cropped.png").unwrap();
}
