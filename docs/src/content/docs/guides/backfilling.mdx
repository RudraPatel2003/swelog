---
title: Backfilling a Missed Day
description: Reconstruct a day you missed using the --date and --week-of flags.
---

Every command that is tied to a day accepts `--date MM-DD-YYYY`, so a day you
missed can be reconstructed later. `WORK.md` is always the scratch pad the
summary is built from; the date flags only decide which day is fetched and which
file the summary is written to.

## Backfilling Monday, 08-17-2026

1. Reset the work file so the day starts clean:

   ```sh
   swelog reset
   ```

2. Write Monday's notes in `WORK.md`, or add them from the terminal:

   ```sh
   swelog log "Debugged the retry storm in the payments worker"
   ```

3. Fetch Monday's integration activity:

   ```sh
   swelog fetch github --date 08-17-2026
   swelog fetch linear --date 08-17-2026
   ```

4. Summarize into Monday's daily log:

   ```sh
   swelog summarize day --date 08-17-2026
   ```

   This writes `Daily/08-17-2026.md` and resets `WORK.md`, so the next missed day
   can be backfilled straight away. Add `--force` to replace a daily log that
   already exists, or `--keep` to leave `WORK.md` alone.

5. Once the week's daily logs exist, summarize the week:

   ```sh
   swelog summarize week --week-of 08-17-2026
   ```

## Backfill flags

| Command                 | Backfill flag          |
| ----------------------- | ---------------------- |
| `swelog fetch github`   | `--date MM-DD-YYYY`    |
| `swelog fetch linear`   | `--date MM-DD-YYYY`    |
| `swelog summarize day`  | `--date MM-DD-YYYY`    |
| `swelog summarize week` | `--week-of MM-DD-YYYY` |

`swelog log` and `swelog reset` need no date; they act on `WORK.md`, which holds
whichever day you are currently assembling.

:::caution[Linear backfill is best-effort]
Linear does not expose issue history, so what `--date` can recover depends on
which timestamps Linear stores on the issue. See
[What `--date` can and cannot recover](/integrations/linear/#what---date-can-and-cannot-recover).
:::
