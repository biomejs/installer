use std::process::Command;

use anyhow::Result;
use semver::Version;

/// Retrieves the latest stable version of Biome
///
/// This function retrieve the latest stable version of Biome from Biome's
/// version API at https://biomejs.dev/api/versions/latest.txt.
pub fn get_latest_stable_version() -> Result<Version> {
    let response = reqwest::blocking::get("https://biomejs.dev/api/versions/latest.txt");

    match response {
        Ok(resp) => Ok(Version::parse(&resp.text()?)?),
        Err(_) => Err(anyhow::anyhow!(
            "Failed to retrieve the latest stable version of Biome"
        )),
    }
}

/// Detects if the current platform is musl-based
pub fn is_musl() -> bool {
    if !cfg!(target_os = "linux") {
        return false;
    }

    // Run ldd and check the output
    let output = Command::new("ldd").arg("--version").output();

    return output
        .ok()
        .and_then(|output| {
            String::from_utf8(output.stdout)
                .ok()
                .map(|stdout| stdout.contains("musl"))
        })
        .unwrap_or(false);
}
