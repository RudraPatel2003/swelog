pub const DEFAULT_CONTEXT_FILE_CONTENT: &str = "# Engineer Context

## Role and Team
<!-- Example: Senior backend engineer on the Payments Platform team. -->

## Systems Owned
<!-- Example: Checkout APIs, billing event pipeline, payout reconciliation jobs. -->

## Current Priorities
<!-- Example: Improve billing reliability, reduce support escalations, and make deployments safer. -->
";

pub const DEFAULT_WORK_FILE_CONTENT: &str = "# Today's Work

## Focus
<!-- Optional: one or two priorities for the day. -->

## Log
<!-- Quick capture. Use short bullets; include systems, outcomes, reviews, debugging, meetings, or support work when useful. -->
";

pub const DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS: &str = "# Today's Work

## Focus

## Log
";

#[must_use]
pub fn is_default_work_file_content(work_file_content: &str) -> bool {
    work_file_content == DEFAULT_WORK_FILE_CONTENT
        || work_file_content == DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS
}

#[cfg(test)]
mod tests;
