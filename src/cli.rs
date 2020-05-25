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
            Arg::from_usage("-d --delay [DELAY] 'Set the time delay (in 1/100th of a second) to pause after drawing the images that are read in or created after this setting has been defined. If you set this number to 100, the delay time will be 1000ms (1 sec).'")
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
        return Err(String::from(
            "The string can't be converted to number or the number is too big.",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end_with_gif_extension_void_string() {
        assert!(end_with_gif_extension(String::from("")).is_err());
    }

    #[test]
    fn test_end_with_gif_extension_not_end_with_gif_extension() {
        assert!(end_with_gif_extension(String::from("sample")).is_err());
    }

    #[test]
    fn test_end_with_gif_extension_just_gif() {
        assert!(end_with_gif_extension(String::from(".gif")).is_err());
    }

    #[test]
    fn test_end_with_gif_extension_ok() {
        assert!(end_with_gif_extension(String::from("sample.gif")).is_ok());
    }

    #[test]
    fn test_end_with_gif_extension_default() {
        assert!(end_with_gif_extension(String::from("./output.gif")).is_ok());
    }

    #[test]
    fn test_is_numberu16_void() {
        assert!(is_numberu16(String::from("")).is_err());
    }

    #[test]
    fn test_is_numberu16_string() {
        assert!(is_numberu16(String::from("sample")).is_err());
    }

    #[test]
    fn test_is_numberu16_too_big() {
        assert!(is_numberu16(String::from("999999")).is_err());
    }

    #[test]
    fn test_is_numberu16_ok() {
        assert!(is_numberu16(String::from("101")).is_ok());
    }

    #[test]
    fn test_is_numberu16_start_with_zero() {
        assert!(is_numberu16(String::from("01")).is_ok());
    }

}
