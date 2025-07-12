use assert_cmd::Command;
use predicates::prelude::*;

#[test]
#[cfg(target_os = "macos")]
pub fn it_installs_the_latest_stable_version() {
    let home = assert_fs::TempDir::new().unwrap();

    Command::cargo_bin("biome-installer")
        .unwrap()
        .arg("install")
        .env("HOME", home.path())
        .assert();

    assert_eq!(
        true,
        predicate::path::exists().eval(&home.path().join(".biome/bin/biome"))
    );
}

#[test]
#[cfg(target_os = "macos")]
pub fn it_installs_the_specified_version() {
    let home = assert_fs::TempDir::new().unwrap();

    Command::cargo_bin("biome-installer")
        .unwrap()
        .arg("install")
        .arg("--version")
        .arg("2.0.6")
        .env("HOME", home.path())
        .assert();

    assert_eq!(
        true,
        predicate::path::exists().eval(&home.path().join(".biome/bin/biome"))
    );

    assert!(
        String::from_utf8(
            Command::new(&home.path().join(".biome/bin/biome"))
                .arg("--version")
                .unwrap()
                .stdout
        )
        .unwrap()
        .contains("2.0.6")
    );
}

#[test]
#[cfg(target_os = "macos")]
pub fn it_adds_the_installation_directory_to_the_path_in_bash_profile() {
    use assert_fs::prelude::{FileTouch, PathChild};

    let home = assert_fs::TempDir::new().unwrap();

    let shell_config = home.child(".bash_profile");
    shell_config.touch().unwrap();

    Command::cargo_bin("biome-installer")
        .unwrap()
        .arg("install")
        .env("HOME", home.path())
        .env("SHELL", "/bin/bash")
        .env_remove("PATH")
        .assert();

    let config_content = std::fs::read_to_string(&shell_config).unwrap();

    assert!(config_content.contains(&format!(
        "export PATH=\"{}:$PATH\"",
        home.path().join(".biome/bin").display()
    )));
}

#[test]
#[cfg(target_os = "macos")]
pub fn it_does_not_add_the_installation_directory_to_the_path_in_bash_profile_if_explicitly_requested()
 {
    use assert_fs::prelude::{FileTouch, PathChild};

    let home = assert_fs::TempDir::new().unwrap();

    let shell_config = home.child(".bash_profile");
    shell_config.touch().unwrap();

    Command::cargo_bin("biome-installer")
        .unwrap()
        .arg("install")
        .arg("--no-prepend-path")
        .env("HOME", home.path())
        .env("SHELL", "/bin/bash")
        .env_remove("PATH")
        .assert();

    let config_content = std::fs::read_to_string(&shell_config).unwrap();

    assert!(!config_content.contains(&format!(
        "export PATH=\"{}:$PATH\"",
        home.path().join(".biome/bin").display()
    )));
}

#[test]
#[cfg(target_os = "macos")]
pub fn it_adds_the_installation_directory_to_the_path_in_zshrc() {
    use assert_fs::prelude::{FileTouch, PathChild};

    let home = assert_fs::TempDir::new().unwrap();

    let shell_config = home.child(".zshrc");
    shell_config.touch().unwrap();

    Command::cargo_bin("biome-installer")
        .unwrap()
        .arg("install")
        .env("HOME", home.path())
        .env("SHELL", "/bin/zsh")
        .env_remove("PATH")
        .assert();

    let config_content = std::fs::read_to_string(&shell_config).unwrap();

    assert!(config_content.contains(&format!(
        "export PATH=\"{}:$PATH\"",
        home.path().join(".biome/bin").display()
    )));
}

#[test]
#[cfg(target_os = "macos")]
pub fn it_does_not_add_the_installation_directory_to_the_path_in_zshrc_if_explicitly_requested() {
    use assert_fs::prelude::{FileTouch, PathChild};

    let home = assert_fs::TempDir::new().unwrap();

    let shell_config = home.child(".zshrc");
    shell_config.touch().unwrap();

    Command::cargo_bin("biome-installer")
        .unwrap()
        .arg("install")
        .arg("--no-prepend-path")
        .env("HOME", home.path())
        .env("SHELL", "/bin/zsh")
        .env_remove("PATH")
        .assert();

    let config_content = std::fs::read_to_string(&shell_config).unwrap();

    assert!(!config_content.contains(&format!(
        "export PATH=\"{}:$PATH\"",
        home.path().join(".biome/bin").display()
    )));
}

#[test]
#[cfg(target_os = "macos")]
pub fn it_adds_the_installation_directory_to_the_path_in_fish_config() {
    use assert_fs::prelude::{FileTouch, PathChild};

    let home = assert_fs::TempDir::new().unwrap();

    let shell_config = home.child(".config/fish/config.fish");
    std::fs::create_dir_all(home.path().join(".config/fish")).unwrap();
    shell_config.touch().unwrap();

    let output = Command::cargo_bin("biome-installer")
        .unwrap()
        .arg("install")
        .env("HOME", home.path())
        .env("SHELL", "/bin/fish")
        .env_remove("PATH")
        .assert()
        .get_output()
        .clone();

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    let config_content = std::fs::read_to_string(&shell_config).unwrap_or_else(|_| String::new());

    println!("config: {}", config_content);
    println!(
        "expect: {}",
        format!(
            "set -gx PATH \"{}\" $PATH",
            home.path().join(".biome/bin").display()
        )
    );

    assert!(config_content.contains(&format!(
        "set -gx PATH \"{}\" $PATH",
        home.path().join(".biome/bin").display()
    )));
}

#[test]
#[cfg(target_os = "macos")]
pub fn it_does_not_add_the_installation_directory_to_the_path_in_fish_config_if_explicitly_requested()
 {
    use assert_fs::prelude::{FileTouch, PathChild};

    let home = assert_fs::TempDir::new().unwrap();

    let shell_config = home.child(".config/fish/config.fish");
    std::fs::create_dir_all(home.path().join(".config/fish")).unwrap();
    shell_config.touch().unwrap();

    Command::cargo_bin("biome-installer")
        .unwrap()
        .arg("install")
        .arg("--no-prepend-path")
        .env("HOME", home.path())
        .env("SHELL", "/bin/fish")
        .env_remove("PATH")
        .assert();

    let config_content = std::fs::read_to_string(&shell_config).unwrap_or_else(|_| String::new());

    assert!(!config_content.contains(&format!(
        "set -gx PATH \"{}\" $PATH",
        home.path().join(".biome/bin").display()
    )));
}
