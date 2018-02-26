pub extern crate reqwest;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
#[macro_use] extern crate serde_derive;

mod models;
use models::Package;

use std::fs::File;
use std::path::PathBuf;

pub struct Options {
    pub manifest_path: PathBuf,
    pub package_names: Vec<String>,
}

pub fn run(options: Options) -> Result<(), String> {
    let mut f = File::open(&options.manifest_path)
        .map_err(|e| format!("Failed to open manifest at {} (consider providing the `--manifest-path` option): {}", options.manifest_path.display(), e))?;

    let client = reqwest::Client::new();
    for pkg_name in options.package_names {
        let json = match Api::get_package_json(&client, &pkg_name) {
            Ok(v) => v,
            Err(msg) => {
                eprintln!("Error: {}", msg);
                std::process::exit(1);
            },
        };

        let package = Package::from_json(&json)?;

        if let Some(version) = package.latest.version {
            println!("dependencies/{}/{}", pkg_name, version);
        } else {
            eprintln!("Response pubspec did not contain a version");
            std::process::exit(3);
        }
    }

    Ok(())
}

struct Api;

impl Api {
    const URL_PACKAGE: &'static str = "https://pub.dartlang.org/api/packages/";

    fn get_package_json(client: &reqwest::Client, name: &str) -> Result<String, String> {
        let url = Self::URL_PACKAGE.to_string() + name;
        let mut resp = client.get(&url).send()
            .map_err(|e| format!("{}", e))?;

        if resp.status() != reqwest::StatusCode::Ok {
            return Err("Got non-ok response status".into());
        }

        let body = resp.text().unwrap();
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
