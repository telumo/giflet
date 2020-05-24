#[macro_use]
extern crate clap;
mod cli;

use gif::{Encoder, Frame, Repeat, SetParameter};
use glob::glob;
use std::fs::File;
use std::convert::TryFrom;

fn main() {
    let matches = cli::build_cli().get_matches();

    let output = matches.value_of("output").unwrap();
    println!("output: {}", output);

    // 間隔
    let delay = matches.value_of("delay").unwrap();
    let delay: u16 = delay.parse::<u16>().unwrap();

    if let Some(directory) = matches.value_of("directory") {
        let directory_format = format!("{}/*", directory);

        let mut image = File::create(output).unwrap();
        let mut encoder = Encoder::new(&mut image, 200, 200, &[]).unwrap();
        encoder.set(Repeat::Infinite).unwrap();

        for entry in glob(&directory_format).unwrap() {
            match entry {
                Ok(path) => {
                    // TODO: 失敗する可能性あり
                    let image = image::open(&path).unwrap();
                    let height = image.to_rgb().height();
                    let height:u16 = TryFrom::try_from(height).unwrap();
                    let width = image.to_rgb().width();
                    let width:u16 = TryFrom::try_from(width).unwrap();
                    let mut pixels: Vec<u8> = image.to_bytes();
                    let mut frame = Frame::from_rgb(width, height, &mut *pixels);
                    frame.delay = delay;
                    println!("adding {:?}", &path.display());
                    encoder.write_frame(&frame).unwrap();
                },
                Err(e) => println!("{:?}", e),
            }
        }
    }
}