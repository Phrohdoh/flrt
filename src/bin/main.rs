extern crate clap;
extern crate flrt;

use std::path::PathBuf;
use clap::{App, Arg};

fn main() {
    let matches = App::new("flrt")
        .version("0.0.1")
        .author("Taryn <taryn@phrohdoh.com>")
        .arg(Arg::with_name("package-name")
             .required(true))
        .arg(Arg::with_name("manifest-path")
             .long("manifest-path")
             .takes_value(true)
             .default_value("./pubspec.yaml")
             .required(true))
        .get_matches();

    let package_name = matches.value_of("package-name").unwrap();
    let manifest_path = matches.value_of("manifest-path").unwrap();
    let manifest_path = PathBuf::from(manifest_path);

    let opts = flrt::Options {
        manifest_path,
        package_names: vec![package_name.into()],
    };

    match flrt::run(opts) {
        Ok(()) => {},
        Err(msg) => {
            eprintln!("Error: {}", msg);
            std::process::exit(1);
        }
    };
}