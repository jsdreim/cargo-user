use std::process::exit;
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
        operation: Op,
    }
}


/// Save and load Cargo credentials with named profiles.
#[derive(Debug, Parser)]
enum Op {
    /// Print the names of all available profiles.
    #[clap(alias = "ls")]
    List,
    /// Print the name of the currently active profile.
    #[clap(alias = "whoami")]
    Current,
    /// Print the file path to stored profiles.
    #[clap(alias = "path")]
    Find {
        /// The name of a specific profile to be shown.
        profile: Option<String>,
    },
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
    /// Change the name of a profile.
    #[clap(alias = "mv")]
    Rename {
        /// Overwrite the destination if it already exists.
        #[clap(long, short)]
        force: bool,
        /// The current name of the profile to be renamed.
        #[clap(name = "PROFILE")]
        name_old: String,
        /// A new name for the profile.
        #[clap(name = "NAME")]
        name_new: String,
    },
    /// Clear the currently active credentials.
    //  Equivalent to `cargo logout`?
    Clear,
    /// Delete a stored credential profile.
    #[clap(alias = "del", alias = "remove", alias = "rm")]
    Delete {
        // /// Do not prompt for confirmation; delete immediately.
        // #[clap(long)]
        // noconfirm: bool,
        /// One or more names of profiles to be deleted.
        #[clap(required(true))]
        profile: Vec<String>,
    },
}


fn main() {
    let Cargo::User { operation } = Cargo::parse();

    let status = match operation {
        Op::List => profile_list(),
        Op::Current => profile_current(),
        Op::Find { profile } => profile_find(profile),
        Op::Save { force, name } => profile_save(name, force),
        Op::Load { profile } => profile_load(profile),
        Op::Clear => profile_clear(),
        Op::Rename { force, name_old, name_new } => profile_rename(
            name_old, name_new, force,
        ),
        Op::Delete { profile } => profile_remove(profile),
    };

    match status {
        Ok(success) => match success {
            Success::List(mut profiles) => {
                if !profiles.is_empty() {
                    profiles.sort_unstable();

                    for profile in profiles {
                        println!("{}", profile.name());
                    }
                } else {
                    //  NOTE: `eprintln` here, not `println`, in case stdout is
                    //      being read by a machine that is not paying attention
                    //      to exit status. No profiles should mean no output.
                    eprintln!("No profiles found.");
                    exit(1);
                }
            }
            Success::Current(mut profiles) if !profiles.is_empty() => {
                profiles.sort_unstable();

                for profile in profiles {
                    println!("{}", profile.name());
                }
            }
            Success::Current(_) => eprintln!("The current credentials are not \
            saved as a profile."),
            Success::CurrentNone => eprintln!("No active credentials found."),
            Success::Found(path) => println!("{}", path.display()),
            Success::Cleared => println!("Cleared Cargo credentials."),
            Success::Saved(p) => println!("Saved profile {:?}.", p.name()),
            Success::Loaded(p) => println!("Loaded profile {:?}.", p.name()),

            Success::Renamed(old, new) => println!(
                "Profile {:?} renamed to {:?}.",
                old.name(), new.name(),
            ),

            Success::Removed { removed, errors } => {
                for err in &errors {
                    match err {
                        Error::ProfileNotFound(profile) => eprintln!(
                            "Error: Cannot remove profile {:?}: Not found.",
                            profile.name(),
                        ),
                        Error::ProfileCannotRemove(profile, e) => eprintln!(
                            "Error: Cannot remove profile {:?}: {}",
                            profile.name(), e,
                        ),
                        _ => unreachable!(),
                    }
                }

                match (removed.len(), errors.len()) {
                    (0, 0) => println!("No profiles removed."),
                    (0, _) => {
                        println!("Failed to remove any profiles.");
                        exit(1);
                    }
                    (n_del, 0) => println!(
                        "Removed {} {}.",
                        n_del, if n_del == 1 { "profile" } else { "profiles" },
                    ),
                    (n_del, n_err) => println!(
                        "Removed {} {} with {} {}.",
                        n_del, if n_del == 1 { "profile" } else { "profiles" },
                        n_err, if n_err == 1 { "error" } else { "errors" },
                    ),
                }
            }
        }
        Err(error) => {
            match error {
                Error::CannotLoad(profile, err_io) => println!(
                    "Error: Cannot load profile {:?}: {}",
                    profile.name(), err_io,
                ),
                Error::CannotSave(profile, err_io) => println!(
                    "Error: Cannot save profile {:?}: {}",
                    profile.name(), err_io,
                ),

                Error::CredentialsNoPath => println!(
                    "Error: Cannot find Cargo directory.",
                ),
                Error::CredentialsNotFound => println!(
                    "Error: Credentials file does not exist.",
                ),
                Error::CredentialsCannotRead(err_io) => println!(
                    "Error: Cannot read Credentials file: {}",
                    err_io,
                ),
                Error::CredentialsCannotRemove(err_io) => println!(
                    "Error: Cannot remove Credentials file: {}",
                    err_io,
                ),

                Error::ProfileExists(profile) => println!(
                    "Error: Profile {:?} already exists.",
                    profile.name(),
                ),
                Error::ProfileNotFound(profile) => println!(
                    "Error: Profile {:?} does not exist.",
                    profile.name(),
                ),
                Error::ProfileCannotRead(profile, err_io) => println!(
                    "Error: Cannot read profile {:?}: {}",
                    profile.name(), err_io,
                ),
                Error::ProfileCannotRemove(profile, err_io) => println!(
                    "Error: Cannot remove profile {:?}: {}",
                    profile.name(), err_io,
                ),
                Error::ProfileCannotRename(old, new, err_io) => println!(
                    "Error: Cannot rename profile {:?} to {:?}: {}",
                    old.name(), new.name(), err_io,
                ),

                Error::Storage(ErrorStorage::NoPath) => println!(
                    "Error: Cannot find a path for the profile directory.",
                ),
                Error::Storage(ErrorStorage::NotDir) => println!(
                    "Error: The profile storage path is not a directory.",
                ),
                Error::Storage(ErrorStorage::CannotCreate(err_io)) => println!(
                    "Error: Cannot set up the profile directory: {}",
                    err_io,
                ),
            }

            exit(1);
        }
    }
}
