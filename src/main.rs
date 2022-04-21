use clap::Parser;
use cargo_user::*;


/// The warning given when the program is run directly.
const STANDALONE_WARN: &str = "{bin} {version}
This program is intended to be invoked as a Cargo subcommand:
        cargo user [...]

For exhausting technical reasons, in order to run it alone, it needs to
be run with the `user` subcommand:
        cargo-user \x1B[4muser\x1B[m [...]";


#[derive(Debug, Parser)]
#[clap(
bin_name = "cargo",
disable_help_subcommand(true),
disable_version_flag(true),
help_template(STANDALONE_WARN),
propagate_version(true),
version,
)]
enum Cargo {
    #[clap(arg_required_else_help(true))]
    User {
        #[clap(subcommand)]
        operation: Option<Op>,
        // profile: Option<String>,
    }
}


/// Save and load Cargo credentials with named profiles.
#[derive(Debug, Parser)]
enum Op {
    /// Store the current Cargo credentials as a named profile.
    #[clap(alias = "store")]
    Save {
        /// Overwrite the profile if it already exists.
        #[clap(long, short)]
        force: bool,
        /// A name for the new profile.
        name: String,
    },
    /// Load Cargo credentials from a stored profile.
    #[clap(alias = "switch")]
    Load {
        /// The name of the profile to be loaded.
        profile: String,
    },
    /// Clear the currently active credentials.
    //  Equivalent to `cargo logout`?
    Clear,
    /// Delete a stored credential profile.
    #[clap(alias = "del", alias = "remove")]
    Delete {
        /// Do not prompt for confirmation; delete immediately.
        #[clap(long)]
        noconfirm: bool,
        /// The name of the profile to be deleted.
        #[clap(required(true))]
        profile: Vec<String>,
    },
}


fn main() {
    let Cargo::User { operation/*, profile*/ } = Cargo::parse();

    // dbg!(&operation);
    // dbg!(&profile);
    //
    // let operation = match operation {
    //     Some(op) => op,
    //     None => Op::Load { profile: profile.expect("No profile specified") },
    // };

    dbg!(&operation);

    match operation.unwrap() {
        Op::Save { force, name } => profile_save(name, force),
        Op::Load { profile } => profile_load(profile),
        Op::Clear => profile_clear(),
        Op::Delete { noconfirm, profile } => profile_remove(profile, !noconfirm),
    }
}
