use clap::*;
use clap_complete::{generate_to, shells};

include!("src/cli.rs");

const BIN_NAME: &str = "xxx"; // something easy and unique for this example

fn main() {
    let mut app = Args::into_app();
    app.set_bin_name(BIN_NAME);

    let outdir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completions/");
    generate_to(shells::Bash, &mut app, BIN_NAME, outdir.clone()).unwrap();
    generate_to(shells::Fish, &mut app, BIN_NAME, outdir.clone()).unwrap();
    generate_to(shells::Zsh, &mut app, BIN_NAME, outdir.clone()).unwrap();
    generate_to(shells::PowerShell, &mut app, BIN_NAME, outdir.clone()).unwrap();
    generate_to(shells::Elvish, &mut app, BIN_NAME, outdir).unwrap();
}
