use assert_cmd::Command;
use predicates::prelude::predicate;
use predicates::prelude::*;

#[test]
pub fn it_installs_the_latest_stable_version() {
    // Create a testing file system
    let home = assert_fs::TempDir::new().unwrap();

    // Create the command
    let mut cmd = Command::cargo_bin("biome-installer").unwrap();

    // Configure the command
    cmd.arg("2.0.5")
        .env(
            if cfg!(target_os = "windows") {
                "USERPROFILE"
            } else {
                "HOME"
            },
            home.path(),
        )
        .assert();

    let path_exists = predicate::path::exists();

    assert_eq!(
        true,
        path_exists.eval(&home.path().join(".biome").join("bin").join(
            if cfg!(target_os = "windows") {
                "biome.exe"
            } else {
                "biome"
            }
        ))
    );
}

#[test]
pub fn it_installs_the_specified_version() {
    // Create a testing file system
    let home = assert_fs::TempDir::new().unwrap();

    // Create the command
    let mut cmd = Command::cargo_bin("biome-installer").unwrap();

    // Configure the command
    cmd.arg("2.0.5")
        .env(
            if cfg!(target_os = "windows") {
                "USERPROFILE"
            } else {
                "HOME"
            },
            home.path(),
        )
        .assert();

    let path_exists = predicate::path::exists();

    assert_eq!(
        true,
        path_exists.eval(&home.path().join(".biome").join("bin").join(
            if cfg!(target_os = "windows") {
                "biome.exe"
            } else {
                "biome"
            }
        ))
    );
}
