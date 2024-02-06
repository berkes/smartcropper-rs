use clap::{Arg, ArgMatches, Command};

pub(crate) fn parse_args() -> ArgMatches {
    Command::new("smartcrop")
        .about("Content aware image cropping")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("Sets the input file to use")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Sets the output file to use")
                .required(true),
        )
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .help("Sets the size of the output image. Example: 800x600. Use 'square' to crop to a square")
                .required(true),
        )
        .get_matches()
}
