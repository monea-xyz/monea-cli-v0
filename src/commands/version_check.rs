use crate::utils::get_latest_version;
use semver::Version;
use std::env;

pub fn version_check() {
    let current_version =
        Version::parse(env!("CARGO_PKG_VERSION")).expect("Invalid current version");

    let latest_version = get_latest_version().unwrap_or_else(|err| {
        eprintln!("Failed to fetch the latest version: {}", err);
        std::process::exit(1);
    });

    if latest_version > current_version {
        println!(
            "A new version of monea-cli is available: {}. You are using {}.",
            latest_version, current_version
        );
        println!("To update, run the following command based on how you installed the CLI:");

        println!("\nVia npm:");
        println!("npm install -g monea-cli");

        println!("\nVia Homebrew:");
        println!("brew upgrade monea");

        println!("\nVia Scoop:");
        println!("scoop update monea");

        println!("\nVia Cargo:");
        println!("cargo install monea-cli");

        println!("\nFrom GitHub Releases:");
        println!("Download the latest binary from https://github.com/monea-xyz/monea-cli/releases and replace your current executable.");
    }
}
