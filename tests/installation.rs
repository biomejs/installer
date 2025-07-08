use std::env::set_var;

use assert_cmd::Command;
use predicates::prelude::*;
use predicates::{prelude::predicate, str::contains};

#[test]
pub fn it_installs_the_latest_stable_version() {
    // Create a testing file system
    let home = assert_fs::TempDir::new().unwrap();

    // Set the HOME environment variable to the temporary directory
    unsafe {
        set_var("HOME", home.path());
    }

    let mut cmd = Command::cargo_bin("biome-installer").unwrap();

    cmd.assert()
        .stdout(contains(
            "No version specified, using latest stable version.",
        ))
        .success();

    let path_exists = predicate::path::exists();

    #[cfg(not(target_os = "windows"))]
    assert_eq!(
        true,
        path_exists.eval(&home.path().join(".biome").join("bin").join("biome"))
    );

    #[cfg(target_os = "windows")]
    assert_eq!(
        true,
        path_exists.eval(&home.path().join(".biome").join("bin").join("biome.exe"))
    );
}

#[test]
pub fn it_installs_the_specified_version() {
    // Create a testing file system
    let home = assert_fs::TempDir::new().unwrap();

    // Set the HOME environment variable to the temporary directory
    unsafe {
        set_var("HOME", home.path());
    }

    let mut cmd = Command::cargo_bin("biome-installer").unwrap();

    cmd.arg("2.0.5")
        .assert()
        .stdout(contains("Downloading Biome version 2.0.5"))
        .success();

    let path_exists = predicate::path::exists();

    #[cfg(not(target_os = "windows"))]
    assert_eq!(
        true,
        path_exists.eval(&home.path().join(".biome").join("bin").join("biome"))
    );

    #[cfg(target_os = "windows")]
    assert_eq!(
        true,
        path_exists.eval(&home.path().join(".biome").join("bin").join("biome.exe"))
    );
}
