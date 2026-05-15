use crate::applescript::{escape_string, run};
use anyhow::{Context, Result, anyhow};

pub struct CreateOptions {
    pub title: String,
    /// Due date/time in `YYYY-MM-DD HH:MM` format (24h, local time).
    pub due: Option<String>,
    pub notes: Option<String>,
    pub list: Option<String>,
}

#[derive(Debug, Clone, Copy)]
struct DueParts {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

fn parse_due(s: &str) -> Result<DueParts> {
    // Accept `YYYY-MM-DD HH:MM` and also `YYYY-MM-DD` (default 09:00).
    let (date, time) = match s.split_once(' ') {
        Some((d, t)) => (d, Some(t)),
        None => (s, None),
    };
    let mut date_parts = date.split('-');
    let year: i32 = date_parts
        .next()
        .ok_or_else(|| anyhow!("invalid due date: {s}"))?
        .parse()
        .with_context(|| format!("parsing year in {s}"))?;
    let month: u32 = date_parts
        .next()
        .ok_or_else(|| anyhow!("invalid due date: {s}"))?
        .parse()
        .with_context(|| format!("parsing month in {s}"))?;
    let day: u32 = date_parts
        .next()
        .ok_or_else(|| anyhow!("invalid due date: {s}"))?
        .parse()
        .with_context(|| format!("parsing day in {s}"))?;
    let (hour, minute) = match time {
        Some(t) => {
            let mut parts = t.split(':');
            let h: u32 = parts
                .next()
                .ok_or_else(|| anyhow!("invalid time: {t}"))?
                .parse()
                .with_context(|| format!("parsing hour in {t}"))?;
            let m: u32 = parts
                .next()
                .ok_or_else(|| anyhow!("invalid time: {t}"))?
                .parse()
                .with_context(|| format!("parsing minute in {t}"))?;
            (h, m)
        }
        None => (9, 0),
    };
    Ok(DueParts {
        year,
        month,
        day,
        hour,
        minute,
    })
}

pub fn create_reminder(opts: &CreateOptions) -> Result<String> {
    let title_lit = escape_string(&opts.title);
    let notes_clause = match &opts.notes {
        Some(n) => format!(", body:\"{}\"", escape_string(n)),
        None => String::new(),
    };

    let (date_setup, due_clause) = match &opts.due {
        Some(s) => {
            let d = parse_due(s)?;
            let setup = format!(
                r#"
    set theDate to current date
    set year of theDate to {y}
    set month of theDate to {m}
    set day of theDate to {dd}
    set time of theDate to ({h} * hours + {mm} * minutes)
"#,
                y = d.year,
                m = d.month,
                dd = d.day,
                h = d.hour,
                mm = d.minute
            );
            (setup, ", due date:theDate".to_string())
        }
        None => (String::new(), String::new()),
    };

    let target_clause = match &opts.list {
        Some(name) => format!(
            "set targetList to first list whose name is \"{}\"",
            escape_string(name)
        ),
        None => "set targetList to default list".to_string(),
    };

    let script = format!(
        r#"
tell application "Reminders"
    {target_clause}
    {date_setup}
    tell targetList
        set newRem to make new reminder with properties {{name:"{title_lit}"{notes_clause}{due_clause}}}
    end tell
    return id of newRem
end tell
"#
    );

    run(&script).context("creating reminder")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_due() {
        let d = parse_due("2026-05-20 14:30").unwrap();
        assert_eq!(d.year, 2026);
        assert_eq!(d.month, 5);
        assert_eq!(d.day, 20);
        assert_eq!(d.hour, 14);
        assert_eq!(d.minute, 30);
    }

    #[test]
    fn parses_date_only_defaults_to_9am() {
        let d = parse_due("2026-05-20").unwrap();
        assert_eq!(d.hour, 9);
        assert_eq!(d.minute, 0);
    }

    #[test]
    fn rejects_invalid_due() {
        assert!(parse_due("not-a-date").is_err());
    }
}
