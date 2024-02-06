# smartcropper

Content aware cropping.

Crops images based on entropy: leaving the most interesting part intact.

Don't expect this to be a replacement for human cropping or AI cropping, it is
an algorithm and not an extremely smart one at that. But it performs a lot 
better than humans or AI.

## Usage

Use as CLI:

TODO

Use as library:

```rust
let mut img = SmartCropper::from_file(img_path).unwrap();

img.smart_crop(100, 100).unwrap();

// do stuff to or with img.cropped
```

```rust
let mut img = SmartCropper::from_file(img_path).unwrap();

img.smart_square().unwrap();

// do stuff to or with img.cropped
```

## Development

Tests are ran with `cargo test --all`

Linter with `cargo fmt --all`
