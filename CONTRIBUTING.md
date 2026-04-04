# Contributing to embers

Thanks for your interest! This project is early-stage and contributions are welcome.

## Getting started

1. Install [Rust](https://rustup.rs) (1.85+) and [Mojo](https://docs.modular.com/mojo/manual/get-started) via pixi
2. Clone the repo and run the examples:

```sh
git clone https://github.com/honeyspoon/embers
cd embers
make test
```

## Development

```sh
cargo build --workspace --features embers/max  # build everything
cargo clippy --workspace --features embers/max  # lint
cargo fmt --all                                  # format
make test                                        # run all 7 examples
```

## Pull requests

- Keep PRs small and focused
- Run `cargo clippy` and `cargo fmt` before submitting
- Add or update examples if you're changing the public API
- Update CHANGELOG.md under `[Unreleased]`

## License

By contributing, you agree that your contributions will be licensed under the MIT license.
