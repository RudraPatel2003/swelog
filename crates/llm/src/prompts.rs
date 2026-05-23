pub fn get_daily_log_prompt(work_file_content: &str, context_file_content: &str) -> String {
    format!(
        "
You are generating a daily engineering work log.

You will be given:
1. Long-term engineer context from context.md
2. Raw work notes from work.md
3. Activity data from git, GitHub, PRs, reviews, issues, and related metadata

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

Use the following structure:

# Daily Log - YYYY-MM-DD

## Summary
2-5 sentence overview of the day.

## Project Work
Group work by project or initiative.

For each project:
- what was worked on
- important findings or decisions
- blockers or follow-ups
- technical details if relevant

## Collaboration
Include:
- PR reviews
- mentoring
- cross-team discussions
- design conversations
- operational coordination

## Wins
Highlight meaningful progress, shipped work, resolved issues, or impactful contributions.

## Follow Ups
List unresolved items, risks, or next steps.

Guidelines:
- Be concise but information-dense
- Avoid repeating raw notes verbatim
- Avoid generic phrases like “worked on various tasks”
- Prefer specific technical descriptions
- Infer intent and impact when possible

CONTEXT:

work.md content:

{work_file_content}

context.md content:

{context_file_content}
"
    )
}
