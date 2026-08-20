mod check_release_version;
mod update_release_version;
mod utils;

use std::env;

use miette::{
    Error,
    Result,
    miette,
};

use crate::{
    check_release_version::run_check_release_version,
    update_release_version::run_update_release_version,
};

fn main() -> Result<()> {
    let mut args = env::args();

    let _ = args.next();

    let Some(command) = args.next() else {
        return Err(get_usage_error());
    };

    match command.as_str() {
        "check-release-version" => run_check_release_version(args),
        "update-release-version" => run_update_release_version(args),
        _ => Err(get_usage_error()),
    }
}

fn get_usage_error() -> Error {
    miette!("usage: cargo run -p xtask -- <command> [...args]")
}
