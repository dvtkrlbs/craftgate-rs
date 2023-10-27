# Craftgate Rust Client

[//]: # ([<img alt="Rust" src="https://img.shields.io/badge/rust-2021-a72145?logo=rust&style=flat" />]&#40;https://www.rust-lang.org&#41;)
[//]: # ([<img alt="GraphQL" src="https://img.shields.io/badge/graphql-e10098?logo=graphql&style=flat" />]&#40;https://graphql.org&#41;)
[//]: # ([<img alt="Tokio" src="https://img.shields.io/badge/tokio-463103?logo=rust&style=flat" />]&#40;https://tokio.rs&#41;)
[//]: # ([<img alt="Axum" src="https://img.shields.io/badge/axum-7b5312?logo=rust&style=flat" />]&#40;https://crates.io/crates/axum&#41;)
[//]: # ([<img alt="SeaORM" src="https://img.shields.io/badge/SeaORM-032846?logo=postgresql&style=flat" />]&#40;https://github.com/SeaQL/sea-orm&#41;)

[//]: # (This is an example app for the upcoming Rust video series by [Brandon Konkle]&#40;https://github.com/bkonkle&#41;. It implements a basic API to support a number of hypothetical frontends for the imaginary "Caster" app, a tool to help podcasters, broadcasters, and streamers coordinate show content with their co-hosts and guests. Limited to just the API to support the front end.)

## Local Development

Install Rust with [rustup](https://rustup.rs/).

### Clippy

For helpful linting rools, install [Clippy](https://github.com/rust-lang/rust-clippy) with `rustup`:

```sh
rustup component add clippy
```

Run it with `cargo`:

```sh
cargo clippy --fix
```

Configure the `rust-analyzer` VS Code plugin to use it (in _settings.json_):

```json
{
    "rust-analyzer.checkOnSave.command": "clippy"
}
```

### libclang

The `cargo-spellcheck` utility depends on [`libclang`](https://clang.llvm.org/doxygen/group__CINDEX.html).

In Ubuntu, the package to install is `libclang-dev`:

```sh
sudo apt install libclang-dev
```

### Cargo Make

To build scripts from the _Makefile.toml_, install Cargo Make:

```sh
cargo install cargo-make
```

Run "setup" to install some tooling dependencies:

```sh
cargo make setup
```

### Environment Variables

For integration test, we have to provide a small `.env` file. Use the `.env.example` as a guide.


### Update Dependencies

First, install the `outdated` command for `cargo`:

```sh
cargo install --locked cargo-outdated
```

Then, update and check for any major dependency changes:

```sh
cargo update
cargo outdated
```

### Running Integration Tests

To integration test, you need to have the Docker Compose stack with Postgres and Redis running locally, or within your CI pipeline.

NOTE: This is destructive, and will wipe out data within your local database. See below for how to use an alternate test database locally.

To run the integration tests:

```sh
cargo make integration
```