use clap::{App, AppSettings, Arg};

pub fn build_cli() -> App<'static, 'static> {
    app_from_crate!()
        // Cargo.tomlから情報を取得
        .setting(AppSettings::DeriveDisplayOrder)
        // ディレクトリ
        .arg(Arg::from_usage(
            "<directory> '画像ファイルが存在するディレクトリ'",
        ))
        // 出力先
        .arg(
            Arg::from_usage("-o --output [OUTPUT] '出力ファイル名'")
                .default_value("./output.gif")
                .validator(end_with_gif),
        )
        // 間隔
        .arg(
            Arg::from_usage("-d --delay [DELAY] '画像間の間隔'")
                .default_value("10")
                .validator(is_number),
        )
}

fn end_with_gif(output: String) -> Result<(), String> {
    if !output.ends_with(".gif") || output.starts_with(".") {
        return Err(String::from(".gifで終わるファイル名にしてください。"));
    }
    Ok(())
}

fn is_number(delay: String) -> Result<(), String> {
    if let Err(_) = delay.parse::<u16>() {
        return Err(String::from("数字に変換出来ませんでした。"));
    }
    Ok(())
}

#[test]
fn test_end_with_gif() {
    let result = end_with_gif(String::from(""));
    match result {
        Ok(_) => {
            panic!("空文字はエラーになる");
        }
        Err(_) => {}
    }

    let result = end_with_gif(String::from("sample"));
    match result {
        Ok(_) => {
            panic!(".gifで終わってなければエラー");
        }
        Err(_) => {}
    }

    let result = end_with_gif(String::from(".gif"));
    match result {
        Ok(_) => {
            panic!(".gifだけだとエラー");
        }
        Err(_) => {}
    }

    let result = end_with_gif(String::from("sample.gif"));
    match result {
        Ok(_) => {}
        Err(_) => {
            panic!("正常系");
        }
    }
}

#[test]
fn test_is_number() {
    let result = is_number(String::from(""));
    match result {
        Ok(_) => {
            panic!("空文字はエラーになる");
        }
        Err(_) => {}
    }

    let result = is_number(String::from("sample"));
    match result {
        Ok(_) => {
            panic!("文字列はエラーになる");
        }
        Err(_) => {}
    }

    let result = is_number(String::from("101"));
    match result {
        Ok(_) => {}
        Err(_) => {
            panic!("正常系！");
        }
    }

    let result = is_number(String::from("999999"));
    match result {
        Ok(_) => {
            panic!("値が大きすぎる");
        }
        Err(_) => {}
    }

    let result = is_number(String::from("01"));
    match result {
        Ok(_) => {}
        Err(_) => {
            panic!("正常系！");
        }
    }
}
