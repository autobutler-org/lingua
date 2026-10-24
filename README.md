# Lingua

A programming language for code that a language model writes and a checker accepts or rejects.

The normative design, grammar, and sample programs are in [docs/lingua1.md](docs/lingua1.md). The compiler is this Rust crate. It is not implemented yet. The binary is still the placeholder greeting.

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
