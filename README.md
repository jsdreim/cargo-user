# Cargo-User

[
![Crates.io](https://img.shields.io/crates/v/cargo-user?logo=rust)
](https://crates.io/crates/cargo-user)
[
![docs.rs](https://docs.rs/cargo-user/badge.svg)
](https://docs.rs/cargo-user)


## Installation

With [Cargo](https://github.com/rust-lang/cargo) installed, the following command will build and install `cargo-user` from [crates.io](https://crates.io) automatically:

```bash
cargo install cargo-user
```

As long as `$HOME/.cargo/bin/` is included in `$PATH`, the subcommand should be available immediately.


## Usage

Assuming you have already run `cargo login`, you should first run `cargo user save` to save your credentials as a profile. You can then clear your credentials with `cargo logout` or `cargo user clear`, log in again with different credentials, and save those too.

```bash
cargo user save first
# Saved profile "first".

cargo user clear
# Cleared Cargo credentials.

cargo login
# [...]

cargo user save second
# Saved profile "second".
```

You can then invoke `cargo user current` to print the name of the currently loaded profile, `cargo user list` to view all available profiles, and `cargo user load` to switch to another.

```bash
cargo user list
# first
# second

cargo user current
# second

cargo user load first
# Loaded profile "first".

cargo user current
# first
```

A complete list of capabilities can be accessed with `cargo help user` or `cargo user --help`.


## Details

User credentials for publishing to [crates.io](https://crates.io) are saved in `$CARGO_HOME/credentials`. This plugin saves a copy of that file, with a user-specified name, as a credentials "profile" in `$XDG_CONFIG_HOME/cargo-user/profiles/`.

For example, a profile named "third", saved with `cargo user save third`, would be saved at `$XDG_CONFIG_HOME/cargo-user/profiles/credentials-third`. When that profile is later loaded with `cargo user load third`, that file is copied back to `$CARGO_HOME/credentials`, and is immediately accessible to be read by `cargo publish`.

The exact path to the saved profile directory, or to a specific profile, can be printed with `cargo user find`.
