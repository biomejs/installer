use std::env::{set_var, var};

use assert_cmd::Command;
use predicates::prelude::*;
use predicates::{prelude::predicate, str::contains};

// #[test]
// pub fn it_installs_the_latest_stable_version() {
//     // Create a testing file system
//     let home = assert_fs::TempDir::new().unwrap();

//     // Set the HOME environment variable to the temporary directory
//     unsafe {
//         #[cfg(not(target_os = "windows"))]
//         set_var("HOME", home.path());

//         #[cfg(target_os = "windows")]
//         set_var("USERPROFILE", home.path());
//     }

//     let mut cmd = Command::cargo_bin("biome-installer").unwrap();

//     cmd.assert()
//         .stdout(contains(
//             "No version specified, using latest stable version.",
//         ))
//         .success();

//     let path_exists = predicate::path::exists();

//     #[cfg(not(target_os = "windows"))]
//     assert_eq!(
//         true,
//         path_exists.eval(&home.path().join(".biome").join("bin").join("biome"))
//     );

//     #[cfg(target_os = "windows")]
//     assert_eq!(
//         true,
//         path_exists.eval(&home.path().join(".biome").join("bin").join("biome.exe"))
//     );
// }

#[test]
pub fn it_installs_the_specified_version() {
    // Create a testing file system
    let home = assert_fs::TempDir::new().unwrap();

    // Set the HOME environment variable to the temporary directory
    unsafe {
        #[cfg(not(target_os = "windows"))]
        {
            set_var("HOME", home.path());
            println!("HOME set to: {}", var("HOME").unwrap());
        }

        #[cfg(target_os = "windows")]
        {
            set_var("USERPROFILE", home.path());
            println!("HOME set to: {}", var("USERPROFILE").unwrap());
        }
    }

    let mut cmd = Command::cargo_bin("biome-installer").unwrap();

    let value = cmd
        .arg("2.0.5")
        .assert()
        .stdout(contains("Downloading Biome version 2.0.5"))
        .get_output()
        .clone();

    println!(
        "Command output: {:?}",
        String::from_utf8(value.stdout).unwrap()
    );

    let path_exists = predicate::path::exists();

    #[cfg(not(target_os = "windows"))]
    assert_eq!(
        true,
        path_exists.eval(&home.path().join(".biome").join("bin").join("biome"))
    );

    #[cfg(target_os = "windows")]
    {
        println!("Checking for Windows executable...");
        println!(
            "Expected path: {}",
            home.path()
                .join(".biome")
                .join("bin")
                .join("biome.exe")
                .display()
        );
        println!(
            "Directory contents: {:?}",
            home.path().join(".biome").read_dir()
        );
        assert_eq!(
            true,
            path_exists.eval(&home.path().join(".biome").join("bin").join("biome.exe"))
        );
    }
}
