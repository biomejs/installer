> [!WARNING] This repository is a work in progress, do not use it yet.

# Biome Installer

This repository contains the source code for the cross-platform installer for
Biome.

> [!TIP] If you are looking to install Biome, run the following command:
>
> ```bash
> curl -fsSL https://biomejs.org/install.sh | bash
> ```

## Usage

```bash
biome-installer [OPTIONS] [version]

Arguments:
  [version]  The version of Biome to download

Options:
  -d, --install-dir <DIR>  The directory in which to install Biome
  -h, --help               Print help

Flags:
  -n, --no-update-path  Do not update the PATH environment variable
```

## Building

To build the installer, run the following command.

```bash
cargo build
```

## Testing

To run the tests, use the following command.

```bash
cargo test
```

## License

This project is licensed under the [MIT License](LICENSE).
