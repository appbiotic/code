# appbiotic-code

A collection of software to build services and apps in a particular style.

Applications:

- [`appbiotic`](https://crates.io/crates/appbiotic), a Rust CLI tool
- [`appbiotic-code-error`](https://crates.io/crates/appbiotic-code-error), error types with well known metadata

## Conventions

Guiding principles, strive for as much as possible:

- code readable
- tools inspectable
- builds hermetic

Meaning:

- avoid macros
- avoid non-hermetic generators such as Rust's `build.rs`

## Getting Started

### Rust

In order to develop and test against all features, select the `full` feature in your `rust-analyzer` settings. This
project's workspace `.vscode/settings.json` already has this setting.

## Contributing

Please read the [contributing guide](CONTRIBUTING.md) for the latest contributing guidelines.
