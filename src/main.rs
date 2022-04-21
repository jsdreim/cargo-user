use clap::Parser;
use cargo_user::*;


/// The warning given when the program is run directly.
const STANDALONE_WARN: &str = "{bin} {version}
This program is intended to be invoked as a Cargo subcommand:
        cargo user [...]

For exhausting technical reasons, in order to run it alone, it needs to
be run with the `user` subcommand:
        cargo-user \x1B[4muser\x1B[m [...]";


#[derive(Parser)]
#[clap(
bin_name = "cargo",
disable_help_subcommand(true),
disable_version_flag(true),
help_template(STANDALONE_WARN),
version,
)]
enum Cargo {
    #[clap(version)]
    User {}
}


fn main() {
    let Cargo::User { .. } = Cargo::parse();
}
