mod check_release_version;
mod update_release_version;
mod utils;

use std::env;

use miette::{
    Error,
    Result,
    miette,
};

fn main() -> Result<()> {
    let mut args = env::args();

    let _ = args.next();

    let Some(command) = args.next() else {
        return Err(get_usage_error());
    };

    match command.as_str() {
        "check-release-version" => check_release_version::run(args),
        "update-release-version" => update_release_version::run(args),
        _ => Err(get_usage_error()),
    }
}

fn get_usage_error() -> Error {
    miette!("usage: cargo run -p xtask -- <command> [...args]")
}
