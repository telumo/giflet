#[macro_use]
extern crate clap;

use clap::Shell;

include!("cli.rs");

fn main() {
    let mut app = build_cli();
    app.gen_completions("giflet", Shell::Bash, "./");
}
