use chrono::NaiveDate;

pub fn get_daily_log_prompt(
    work_file_content: &str,
    context_file_content: &str,
    log_date: &NaiveDate,
) -> String {
    let formatted_date = log_date.format("%m-%d-%Y").to_string();

    format!(
        "
You are generating a daily engineering work log.

You will be given:
1. Long-term engineer context
2. Raw work notes
3. Potentially activity data from git, GitHub, PRs, reviews, issues, and related metadata

Your job is to generate a concise but specific daily summary.

Goals:
- Preserve important engineering details
- Highlight impact, decisions, debugging, and collaboration
- Organize scattered work into coherent themes
- Include mentoring, reviews, meetings, and operational work
- Avoid exaggeration or corporate filler
- Prefer concrete technical language
- Write in a professional tone suitable for future performance reviews

The output should be Markdown.

The resulting output should be a valid Markdown document that can be directly copied and pasted into the daily log file.
Do not include any extraneous text or formatting in the output.

Use the following structure:

```markdown
# Daily Log - {formatted_date}

## Summary

2-5 sentence overview of the day.

## Wins

Highlight meaningful progress, shipped work, resolved issues, or impactful contributions. Structure these as if they were resume bullet points.
```

CONTEXT:

work file content:

{work_file_content}

context file content:

{context_file_content}
"
    )
}
