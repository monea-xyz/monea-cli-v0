pub fn get_latest_version() -> Result<semver::Version, Box<dyn std::error::Error>> {
    use reqwest::blocking::get;
    use semver::Version;
    use serde_json::Value;

    // Replace these URLs with the actual endpoints for your package versions
    let urls = vec![
        "https://crates.io/api/v1/crates/monea-cli",
        "https://registry.npmjs.org/monea-cli",
        "https://formulae.brew.sh/api/formula/monea.json",
        "https://api.github.com/repos/yourusername/monea/releases/latest",
    ];

    for url in urls {
        let response = get(url)?;
        let json: Value = response.json()?;
        if let Some(version) = json["version"].as_str() {
            return Ok(Version::parse(version)?);
        }
    }

    Err("Could not fetch the latest version from any source".into())
}
