---
title: Daily Workflow
description: The day-to-day loop of capturing notes, fetching activity, and summarizing.
---

1. Add notes manually in Obsidian or from the terminal:

   ```sh
   swelog log "Reviewed the checkout migration plan"
   ```

2. Optionally fetch activity from GitHub or Linear:

   ```sh
   swelog fetch github
   swelog fetch linear
   ```

3. Review and edit `WORK.md`. Integration sections are created only after the
   corresponding integration is used.

4. Generate the daily log:

   ```sh
   swelog summarize
   ```

5. Generate a weekly log when ready:

   ```sh
   swelog summarize week
   ```

## Missed a day?

`WORK.md` is always the scratch pad the summary is built from, and every
day-scoped command accepts `--date MM-DD-YYYY`. See
[Backfilling a Missed Day](/guides/backfilling/).
