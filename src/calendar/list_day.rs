use super::datetime::{applescript_date, parse_datetime};
use crate::applescript::{escape_string, run};
use anyhow::{Context, Result};

/// List events on a specific date. Optionally filter to a single calendar.
/// Output per line: `<uid>\t<calendar>\t<start ISO>\t<end ISO>\t<summary>`
pub fn list_day(date: &str, calendar: Option<&str>) -> Result<Vec<String>> {
    let d = parse_datetime(date).context("parsing --date")?;
    // start = midnight of given day; end = midnight of next day.
    let start_setup = applescript_date("startDate", d);

    let cal_filter = match calendar {
        Some(name) => format!(
            "set calList to (every calendar whose name is \"{}\")",
            escape_string(name)
        ),
        None => "set calList to calendars".to_string(),
    };

    let script = format!(
        r#"
tell application "Calendar"
    {start_setup}
    set endDate to startDate + (1 * days)
    {cal_filter}
    set out to ""
    repeat with c in calList
        repeat with e in ((events of c) whose start date is greater than or equal to startDate and start date is less than endDate)
            set s to start date of e
            set f to end date of e
            set out to out & (uid of e) & tab & (name of c) & tab & (s as «class isot» as string) & tab & (f as «class isot» as string) & tab & (summary of e) & linefeed
        end repeat
    end repeat
    return out
end tell
"#
    );

    let stdout = run(&script).context("listing day events")?;
    Ok(stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.to_string())
        .collect())
}
