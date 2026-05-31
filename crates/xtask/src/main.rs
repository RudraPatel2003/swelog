mod check_release_version;

use std::env;

use miette::{
    Result,
    miette,
};

fn main() -> Result<()> {
    let mut args = env::args();

    let _ = args.next();

    let Some(command) = args.next() else {
        return Err(usage_error());
    };

    match command.as_str() {
        "check-release-version" => check_release_version::run(args),
        _ => Err(usage_error()),
    }
}

fn usage_error() -> miette::Error {
    miette!("usage: cargo run -p xtask -- <command> [...args]")
}
