#[macro_use]
extern crate clap;
mod cli;

use gif::{Encoder, Frame, Repeat, SetParameter};
use glob::glob;
use std::convert::TryFrom;
use std::fs::File;

fn main() {
    let matches = cli::build_cli().get_matches();

    // output file name
    let output = matches.value_of("output").unwrap_or("./output.gif");
    println!("output file name: {}", output);

    // delay
    let delay = matches.value_of("delay").unwrap_or("10");
    let delay: u16 = delay.parse::<u16>().unwrap();
    println!("delay: {} ms", delay * 10);

    if let Some(directory) = matches.value_of("directory") {
        let directory_format = format!("{}/*", directory);

        let mut output_file = File::create(output).unwrap();
        let mut encoder = Encoder::new(&mut output_file, 1, 1, &[]).unwrap();
        encoder.set(Repeat::Infinite).unwrap();

        for filepath in glob(&directory_format).unwrap() {
            match filepath {
                Ok(path) => {
                    if image::open(&path).is_err() {
                        println!("{:?} can't open file.", &path.display());
                    } else {
                        let image = image::open(&path).unwrap();
                        let height = image.to_rgb().height();
                        let height: u16 = TryFrom::try_from(height).unwrap();
                        let width = image.to_rgb().width();
                        let width: u16 = TryFrom::try_from(width).unwrap();
                        let pixels: Vec<u8> = image.to_bytes();
                        let mut frame = Frame::from_rgb(width, height, &pixels);
                        frame.delay = delay;
                        encoder.write_frame(&frame).unwrap();
                        println!("{:?} added.", &path.display());
                    }
                }
                Err(e) => println!("error occured. : {:?}", e),
            }
        }
    }
}
