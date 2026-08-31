use std::env::Args;

use miette::{
    Result,
    miette,
};

pub fn get_release_tag_from_args(args: &mut Args, command: &str) -> Result<String> {
    let Some(release_tag) = args.next() else {
        return Err(miette!("usage: cargo run -p xtask -- {command} <release-tag>"));
    };

    if let Some(extra_arg) = args.next() {
        return Err(miette!("unexpected argument: {extra_arg}"));
    }

    Ok(release_tag)
}

pub fn get_release_version_from_tag(release_tag: &str) -> Result<&str> {
    let Some(release_version) = release_tag.strip_prefix('v') else {
        return Err(miette!("release tag must look like vX.Y.Z"));
    };

    Ok(release_version)
}
