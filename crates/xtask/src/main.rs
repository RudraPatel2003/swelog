mod check_release_version;
mod crate_versions;
mod package_json;
mod release_tag;
mod update_release_version;

use std::env;

use miette::{
    Error,
    Result,
    miette,
};

use crate::{
    check_release_version::run_check_release_version,
    release_tag::run_list_release_tag,
    update_release_version::run_update_release_version,
};

fn main() -> Result<()> {
    let mut args = env::args();

    let _ = args.next();

    let Some(command) = args.next() else {
        let usage_error = get_usage_error();

        return Err(usage_error);
    };

    match command.as_str() {
        "check-release-version" => run_check_release_version(),

        "update-release-version" => run_update_release_version(args),

        "list-release-tag" => run_list_release_tag(),

        _ => {
            let usage_error = get_usage_error();

            Err(usage_error)
        }
    }
}

fn get_usage_error() -> Error {
    miette!("usage: cargo run -p xtask -- <command> [...args]")
}
