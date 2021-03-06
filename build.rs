use std::fs;
use structopt::clap::Shell;

include!("src/cli.rs");

const BIN_NAME: &str = "zellij";

fn main() {
    // Generate Shell Completions
    let mut clap_app = CliArgs::clap();
    println!("cargo:rerun-if-changed=src/cli.rs");
    let mut out_dir = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
    out_dir.push("/assets/completions");

    println!(
        "Completion files will to added to this location: {:?}",
        out_dir
    );
    fs::create_dir_all(&out_dir).unwrap();
    clap_app.gen_completions(BIN_NAME, Shell::Bash, &out_dir);
    clap_app.gen_completions(BIN_NAME, Shell::Zsh, &out_dir);
    clap_app.gen_completions(BIN_NAME, Shell::Fish, &out_dir);
}
