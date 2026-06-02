use chrono::NaiveDate;

pub fn get_daily_log_prompt(
    work_file_content: &str,
    context_file_content: &str,
    log_date: &NaiveDate,
) -> String {
    let formatted_date = log_date.format("%m-%d-%Y").to_string();

    format!(
        r#"
You are generating a daily engineering work log.

You will be given:
1. Long-term engineer context
2. Raw daily work notes, often written as terse quick-capture bullets
3. Potentially activity data from git, GitHub, PRs, reviews, issues, meetings, and related metadata

Your job is to generate a concise, specific, and useful daily summary.

Goals:
- Preserve important engineering details
- Group scattered notes into coherent engineering themes
- Highlight impact, decisions, debugging, collaboration, and operational work
- Include coding, reviews, mentoring, meetings, design discussions, investigations, and support work when relevant
- Prefer concrete technical language over generic summaries
- Write in a professional tone suitable for future performance reviews
- Avoid exaggeration, speculation, corporate filler, or unsupported claims
- Distinguish clearly between completed work, work in progress, investigations, planning, and blockers

How to reason about the inputs:
- Treat the work file as a busy-engineer capture surface; notes may be terse, incomplete, or organized into optional sections
- Ignore instructional HTML comments from the default templates; they are guidance for the user, not accomplishments
- Use long-term engineer context only to clarify systems, ownership, priorities, collaborators, and impact areas
- Do not treat long-term context as evidence that work happened today
- Treat commits, PRs, reviews, issues, and activity metadata as supporting evidence, not as the primary narrative
- Prefer explaining the engineering outcome over listing repository actions
- Do not imply that work was shipped, resolved, or delivered unless the notes support it
- If the notes are sparse, produce a short but still useful log
- If the notes are dense, compress them into the most meaningful themes
- Prefer density over verbosity
- Every bullet should communicate a meaningful engineering outcome, decision, learning, or contribution

Impact lens:
When possible, describe why the work mattered in terms of:
- reliability
- performance
- developer productivity
- customer impact
- risk reduction
- maintainability
- operational efficiency
- team enablement

Avoid:
- Generic statements like "worked on", "continued work", or "made progress"
- Commit-by-commit narration
- Repeating the same information across sections
- Including template comments, placeholder text, or empty headings from the source files
- Inventing impact that is not supported by the notes
- Overstating tentative or exploratory work

The output should be Markdown.

The resulting output should be a valid Markdown document that can be directly copied and pasted into the daily log file.
Do not include any extraneous text or formatting outside the Markdown document.

Use the following structure:

```markdown
# Daily Log - {formatted_date}

## Summary

3-5 sentence overview of the day's primary engineering themes, outcomes, and notable context.

## Technical Highlights

- Key technical work, investigations, implementations, debugging efforts, design decisions, or operational activities.
- Include specific systems, components, bugs, workflows, or technical tradeoffs when supported by the notes.

## Wins

- Resume-style bullets focused on meaningful outcomes, shipped work, resolved issues, risk reduction, productivity improvements, or impactful contributions.
- Do not include routine activity unless it had clear impact.

## Collaboration

- Reviews, mentoring, pairing, design discussions, meetings, cross-team coordination, stakeholder communication, or support work.
- Omit this section if there was no meaningful collaboration in the notes.

## Follow-ups

- Open questions, blockers, unfinished work, next steps, or risks.
- Omit this section if there are no clear follow-ups.
```

CONTEXT:

work file content:

{work_file_content}

context file content:

{context_file_content}
"#,
        formatted_date = formatted_date,
        work_file_content = work_file_content,
        context_file_content = context_file_content,
    )
}

pub fn get_weekly_log_prompt(
    daily_logs: &[String],
    context_file_content: &str,
    log_date: &NaiveDate,
) -> String {
    let formatted_date = log_date.format("%m-%d-%Y").to_string();

    let combined_daily_logs = daily_logs.join("\n\n--- DAILY LOG ---\n\n");

    format!(
        r#"
You are generating a weekly engineering work log.

You will be given:
1. Long-term engineer context
2. A collection of daily engineering logs covering a week of work

Your job is to generate a concise, specific, and useful weekly summary.

Goals:
- Identify the most important engineering themes across the week
- Consolidate related work that appears across multiple days
- Highlight meaningful outcomes, decisions, debugging, investigations, design work, reviews, operational work, and collaboration
- Emphasize impact and progress rather than daily activity
- Write in a professional tone suitable for future performance reviews
- Avoid exaggeration, speculation, corporate filler, or unsupported claims
- Clearly distinguish completed work, ongoing efforts, investigations, and follow-up items

How to reason about the inputs:
- Daily logs may contain overlap; consolidate repeated themes rather than repeating them
- Prefer describing engineering outcomes over enumerating tasks
- Use long-term engineer context only to clarify systems, ownership, collaborators, and impact areas
- Do not treat long-term context as evidence that work occurred this week
- If a topic appears repeatedly throughout the week, treat it as a significant theme
- Compress low-signal routine work into broader themes when appropriate
- Do not invent accomplishments or impact not supported by the logs
- Prefer density over verbosity

Impact lens:
When possible, describe why the work mattered in terms of:
- reliability
- performance
- developer productivity
- customer impact
- risk reduction
- maintainability
- operational efficiency
- team enablement

Avoid:
- Day-by-day narration
- Repeating the same information across sections
- Generic statements like "worked on" or "made progress"
- Excessive implementation detail when a higher-level summary is more useful
- Inventing shipped outcomes that are not supported by the logs
- Overstating exploratory or unfinished work

The output should be Markdown.

The resulting output should be a valid Markdown document that can be directly copied and pasted into the weekly log file.
Do not include any extraneous text or formatting outside the Markdown document.

Use the following structure:

```markdown
# Weekly Log - {formatted_date}

## Summary

1-2 paragraphs summarizing the week's primary engineering themes, major outcomes, notable investigations, and overall progress.

## Major Technical Work

- Significant implementations, debugging efforts, architecture/design work, operational improvements, investigations, or technical decisions.
- Group related work into coherent themes.

## Key Outcomes

- Resume-style bullets highlighting meaningful accomplishments, risk reduction, productivity gains, reliability improvements, customer impact, or delivered value.

## Collaboration

- Reviews, mentoring, pairing, design discussions, stakeholder communication, cross-team coordination, and support work.
- Omit this section if collaboration was not a meaningful theme.

## Ongoing Work / Follow-ups

- Open investigations, unfinished work, blockers, risks, and next steps.
- Omit this section if there are no clear follow-ups.
```
CONTEXT:

weekly daily logs:

{combined_daily_logs}

context file content:

{context_file_content}
"#,
        formatted_date = formatted_date,
        combined_daily_logs = combined_daily_logs,
        context_file_content = context_file_content,
    )
}
