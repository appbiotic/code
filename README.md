# appbiotic-code

A collection of software to build services and apps in a particular style.

Applications:

- [`appbiotic`](https://crates.io/crates/appbiotic), a Rust CLI tool
- [`appbiotic-code-error`](https://crates.io/crates/appbiotic-code-error), error types with well known metadata

## Conventions

Guiding principles, strive for as much as possible:

- code readable
- tools inspectable
- builds hermetic with minimal dependencies

Meaning:

- avoid macros
- avoid non-hermetic generators such as Rust's `build.rs`
- avoid tooling not installed by default in CI runner or any additional runtime

## Getting Started

### Rust

In order to develop and test against all features, select the `full` feature in your `rust-analyzer` settings. This
project's workspace `.vscode/settings.json` already has this setting.

#### Releasing

Releasing uses `cargo-release`:

    cargo install --version 0.24.11 cargo-release

Bump version:

    cargo-release release version patch --workspace --execute

Push and merge to main. Then tag:

    git tag -s "cargo read-manifest | jq -r '"v" + .version'"

Publish:

    cargo-release release publish --workspace --execute

## Contributing

Please read the [contributing guide](CONTRIBUTING.md) for the latest contributing guidelines.
