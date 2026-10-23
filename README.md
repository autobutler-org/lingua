# Template Rust Repo

Template for new Rust projects.

## How to rename

`grep` for `template`, you will find every string to change for your project.

## Usage

```sh
make setup   # Install rustup and the toolchain
make build   # Build the binary
make run     # Build and run the binary (PROFILE=debug for a debug build)
make test    # Run tests
make check   # Lint (clippy, warnings denied)
make format  # Format code
```

Run `make` to see every target.

## Automation Included

* [Check](./.github/workflows/check.yml)
  * Format check
  * Lint
* [CI](./.github/workflows/ci.yml)
  * Build
  * Run tests
