use smartcropper::SmartCropper;

mod cli;

fn main() {
    let matches = cli::parse_args();

    let input = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    let size = matches.get_one::<String>("size").unwrap();

    // split size into width and height
    let sizes: Vec<u32> = size
        .split('x')
        .map(|part| part.parse().expect("Size is invalid"))
        .collect();
    let width = sizes[0];
    let height = sizes[1];

    let mut sc = SmartCropper::from_file(input).expect("Failed to open image");
    sc.smart_crop(width, height).expect("Failed to crop image");

    sc.cropped.unwrap().save(output).unwrap_or_else(|_| panic!("Failed to save image to {}", output));
}
