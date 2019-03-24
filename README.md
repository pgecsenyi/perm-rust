# Perm Rust

Perm Rust is a Rust implementation of a cross-platform command line utility for measuring the performance of programs.

## Installation and usage

Use cargo directly or _Visual Studio Code_ to build the application. This will provide you an executable in one of the subfolders of the _target_ directory.

The program accepts the following command line arguments.

  * `config_path`: The path of the configuration file. This is a JSON file that describes the commands to execute and defines their order.
  * `output_path`: The path of the output file.
  * `display_output` (optional): a boolean value indicating whether the output of the executed command should be displayed.
  * `generate_sample_config` (optional): a boolean value indicating whether a sample configuration file should be generated at `config_path` instead of measuring performance.

## Development

After configuring the Rust development environment, the application can be built for debugging using the following command.

    cargo build

The resulting binary can be found under the `target/debug` directory. In order to build a release version, `cargo build --release` has to be invoked, which will emit the binary to the `target/release` folder.

_Visual Studio Code_ configuration files are also provided so it can be also used to build and debug the application.

### Environment

  * Ubuntu 18.10
  * Rust 1.32.0
  * Visual Studio Code 1.30.2
    * Extension: Rust (rls) 0.5.4
