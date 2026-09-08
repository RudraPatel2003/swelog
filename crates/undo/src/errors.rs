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
