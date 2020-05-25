use clap::{App, AppSettings, Arg};
use regex::Regex;

pub fn build_cli() -> App<'static, 'static> {
    app_from_crate!()
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(Arg::from_usage(
            "<directory> 'The directory existing target image files.'",
        ))
        .arg(
            Arg::from_usage("-o --output [OUTPUT] 'The output file name.'")
                .default_value("./output.gif")
                .validator(end_with_gif_extension),
        )
        .arg(
            Arg::from_usage("-d --delay [DELAY] 'Set the time delay (in 1/100th of a second) to pause after drawing the images that are read in or created after this setting has been defined.'")
                .default_value("10")
                .validator(is_numberu16),
        )
}

fn end_with_gif_extension(output: String) -> Result<(), String> {
    let re = Regex::new(r"^.+\.gif$").unwrap();
    if !re.is_match(&output) {
        return Err(String::from("The file name must be ended with '.gif'."));
    }
    Ok(())
}

fn is_numberu16(delay: String) -> Result<(), String> {
    if let Err(_) = delay.parse::<u16>() {
        return Err(String::from("The string can't be converted to number or the number is too big."));
    }
    Ok(())
}

#[test]
fn test_end_with_gif() {
    let result = end_with_gif_extension(String::from(""));
    match result {
        Ok(_) => {
            panic!("空文字はエラーになる");
        }
        Err(_) => {}
    }

    let result = end_with_gif_extension(String::from("sample"));
    match result {
        Ok(_) => {
            panic!(".gifで終わってなければエラー");
        }
        Err(_) => {}
    }

    let result = end_with_gif_extension(String::from(".gif"));
    match result {
        Ok(_) => {
            panic!(".gifだけだとエラー");
        }
        Err(_) => {}
    }

    let result = end_with_gif_extension(String::from("sample.gif"));
    match result {
        Ok(_) => {}
        Err(_) => {
            panic!("正常系");
        }
    }

    let result = end_with_gif_extension(String::from("./output.gif"));
    match result {
        Ok(_) => {}
        Err(_) => {
            panic!("正常系");
        }
    }
}

#[test]
fn test_is_number() {
    let result = is_numberu16(String::from(""));
    match result {
        Ok(_) => {
            panic!("空文字はエラーになる");
        }
        Err(_) => {}
    }

    let result = is_numberu16(String::from("sample"));
    match result {
        Ok(_) => {
            panic!("文字列はエラーになる");
        }
        Err(_) => {}
    }

    let result = is_numberu16(String::from("101"));
    match result {
        Ok(_) => {}
        Err(_) => {
            panic!("正常系！");
        }
    }

    let result = is_numberu16(String::from("999999"));
    match result {
        Ok(_) => {
            panic!("値が大きすぎる");
        }
        Err(_) => {}
    }

    let result = is_numberu16(String::from("01"));
    match result {
        Ok(_) => {}
        Err(_) => {
            panic!("正常系！");
        }
    }
}
