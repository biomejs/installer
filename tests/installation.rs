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

#[test]
pub fn it_prepends_the_installation_directory_to_the_path_with_bash() {
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
        .env("SHELL", "/bin/bash")
        .assert();

    // Read $HOME/.bashrc
    let bashrc_path = home.path().join(".bashrc");
    let bashrc_content = std::fs::read_to_string(&bashrc_path).unwrap_or_default();

    assert!(bashrc_content.contains(&format!(
            "export PATH=\"{}:$PATH\"",
            home.path()
                .join(".biome")
                .join("bin")
                .canonicalize()
                .unwrap()
                .display()
        )));
}

#[test]
pub fn it_does_not_prepend_the_installation_directory_to_the_path_with_bash() {
    // Create a testing file system
    let home = assert_fs::TempDir::new().unwrap();

    // Create the command
    let mut cmd = Command::cargo_bin("biome-installer").unwrap();

    // Configure the command
    cmd.arg("2.0.5")
        .arg("--no-update-path")
        .env(
            if cfg!(target_os = "windows") {
                "USERPROFILE"
            } else {
                "HOME"
            },
            home.path(),
        )
        .env("SHELL", "/bin/bash")
        .assert();

    // Read $HOME/.bashrc
    let bashrc_path = home.path().join(".bashrc");
    let bashrc_content = std::fs::read_to_string(&bashrc_path).unwrap_or_default();

    assert!(!bashrc_content.contains(&format!(
            "export PATH=\"{}:$PATH\"",
            home.path()
                .join(".biome")
                .join("bin")
                .canonicalize()
                .unwrap()
                .display()
        )));
}

#[test]
pub fn it_prepends_the_installation_directory_to_the_path_with_zsh() {
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
        .env("SHELL", "/bin/zsh")
        .assert();

    // Read $HOME/.zshrc
    let zshrc_path = home.path().join(".zshrc");
    let zshrc_content = std::fs::read_to_string(&zshrc_path).unwrap_or_default();

    assert!(zshrc_content.contains(&format!(
            "export PATH=\"{}:$PATH\"",
            home.path()
                .join(".biome")
                .join("bin")
                .canonicalize()
                .unwrap()
                .display()
        )));
}

#[test]
pub fn it_does_not_prepend_the_installation_directory_to_the_path_with_zsh() {
    // Create a testing file system
    let home = assert_fs::TempDir::new().unwrap();

    // Create the command
    let mut cmd = Command::cargo_bin("biome-installer").unwrap();

    // Configure the command
    cmd.arg("2.0.5")
        .arg("--no-update-path")
        .env(
            if cfg!(target_os = "windows") {
                "USERPROFILE"
            } else {
                "HOME"
            },
            home.path(),
        )
        .env("SHELL", "/bin/zsh")
        .assert();

    // Read $HOME/.zshrc
    let zshrc_path = home.path().join(".zshrc");
    let zshrc_content = std::fs::read_to_string(&zshrc_path).unwrap_or_default();

    assert!(!zshrc_content.contains(&format!(
            "export PATH=\"{}:$PATH\"",
            home.path()
                .join(".biome")
                .join("bin")
                .canonicalize()
                .unwrap()
                .display()
        )));
}

#[test]
pub fn it_prepends_the_installation_directory_to_the_path_with_fish() {
    // Create a testing file system
    let home = assert_fs::TempDir::new().unwrap();

    // Create .config/fish directory
    std::fs::create_dir_all(home.path().join(".config/fish")).unwrap();

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
        .env("SHELL", "/usr/bin/fish")
        .assert();

    // Read $HOME/.config/fish/config.fish
    let fish_config_path = home.path().join(".config/fish/config.fish");
    let fish_config_content =
        std::fs::read_to_string(&fish_config_path).unwrap_or("ok".to_string());

    assert!(fish_config_content.contains(&format!(
            "set -gx PATH \"{}\" $PATH",
            home.path()
                .join(".biome")
                .join("bin")
                .canonicalize()
                .unwrap()
                .display()
        )));
}

#[test]
pub fn it_does_not_prepend_the_installation_directory_to_the_path_with_fish() {
    // Create a testing file system
    let home = assert_fs::TempDir::new().unwrap();

    // Create .config/fish directory
    std::fs::create_dir_all(home.path().join(".config/fish")).unwrap();

    // Create the command
    let mut cmd = Command::cargo_bin("biome-installer").unwrap();

    // Configure the command
    cmd.arg("2.0.5")
        .arg("--no-update-path")
        .env(
            if cfg!(target_os = "windows") {
                "USERPROFILE"
            } else {
                "HOME"
            },
            home.path(),
        )
        .env("SHELL", "/usr/bin/fish")
        .assert();

    // Read $HOME/.config/fish/config.fish
    let fish_config_path = home.path().join(".config/fish/config.fish");
    let fish_config_content =
        std::fs::read_to_string(&fish_config_path).unwrap_or("ok".to_string());

    assert!(!fish_config_content.contains(&format!(
            "set -gx PATH \"{}\" $PATH",
            home.path()
                .join(".biome")
                .join("bin")
                .canonicalize()
                .unwrap()
                .display()
        )));
}

#[test]
pub fn it_does_not_prepend_the_installation_directory_if_its_already_in_path() {
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
        .env("SHELL", "/bin/bash")
        .env(
            "PATH",
            format!(
                "{}:{}",
                home.path().join(".biome").join("bin").display(),
                std::env::var("PATH").unwrap_or_default()
            ),
        )
        .assert();

    // Read $HOME/.bashrc
    let bashrc_path = home.path().join(".bashrc");
    let bashrc_content = std::fs::read_to_string(&bashrc_path).unwrap_or_default();

    assert!(!bashrc_content.contains(&format!(
            "export PATH=\"{}:$PATH\"",
            home.path()
                .join(".biome")
                .join("bin")
                .canonicalize()
                .unwrap()
                .display()
        )));
}
