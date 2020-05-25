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

        // TODO: 全ての画像の大きさが一致するか確認する。

        let mut image = File::create(output).unwrap();
        // TODO: 高さと幅がマジックナンバーになっている
        let mut encoder = Encoder::new(&mut image, 200, 200, &[]).unwrap();
        encoder.set(Repeat::Infinite).unwrap();

        for entry in glob(&directory_format).unwrap() {
            match entry {
                Ok(path) => {
                    // TODO: 失敗する可能性あり
                    let image = image::open(&path).unwrap();
                    let height = image.to_rgb().height();
                    let height: u16 = TryFrom::try_from(height).unwrap();
                    let width = image.to_rgb().width();
                    let width: u16 = TryFrom::try_from(width).unwrap();
                    let mut pixels: Vec<u8> = image.to_bytes();
                    let mut frame = Frame::from_rgb(width, height, &mut *pixels);
                    frame.delay = delay;
                    println!("adding {:?}", &path.display());
                    encoder.write_frame(&frame).unwrap();
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
