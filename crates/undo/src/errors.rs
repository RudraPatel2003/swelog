use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("nothing to undo")]
#[diagnostic(
    code(swelog::undo::no_undo_snapshot),
    help(
        "`swelog undo` reverses the last `swelog log`, `swelog summarize day`, or `swelog reset`"
    )
)]
pub struct NoUndoSnapshot;

#[derive(Debug, Diagnostic, Error)]
#[error("unable to determine the cache directory")]
#[diagnostic(
    code(swelog::undo::unavailable_cache_directory),
    help("set a cache directory for your operating system, such as XDG_CACHE_HOME on Linux")
)]
pub(crate) struct UnavailableCacheDirectory;
