use anyhow::{Context, Result, anyhow};

/// Parsed date/time components for AppleScript construction.
#[derive(Debug, Clone, Copy)]
pub struct DateTimeParts {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
}

/// Parse `YYYY-MM-DD HH:MM` (24h, local) or `YYYY-MM-DD` (defaults to 00:00).
pub fn parse_datetime(s: &str) -> Result<DateTimeParts> {
    let (date, time) = match s.split_once(' ') {
        Some((d, t)) => (d, Some(t)),
        None => (s, None),
    };
    let mut date_parts = date.split('-');
    let year: i32 = date_parts
        .next()
        .ok_or_else(|| anyhow!("invalid date: {s}"))?
        .parse()
        .with_context(|| format!("parsing year in {s}"))?;
    let month: u32 = date_parts
        .next()
        .ok_or_else(|| anyhow!("invalid date: {s}"))?
        .parse()
        .with_context(|| format!("parsing month in {s}"))?;
    let day: u32 = date_parts
        .next()
        .ok_or_else(|| anyhow!("invalid date: {s}"))?
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
        None => (0, 0),
    };
    Ok(DateTimeParts {
        year,
        month,
        day,
        hour,
        minute,
    })
}

/// AppleScript date construction snippet writing to a variable name.
/// Caller decides the variable name (e.g. "startDate").
pub fn applescript_date(var: &str, d: DateTimeParts) -> String {
    format!(
        r#"set {var} to current date
    set year of {var} to {y}
    set month of {var} to {m}
    set day of {var} to {dd}
    set hours of {var} to {h}
    set minutes of {var} to {mm}
    set seconds of {var} to 0
"#,
        y = d.year,
        m = d.month,
        dd = d.day,
        h = d.hour,
        mm = d.minute
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_with_time() {
        let d = parse_datetime("2026-05-20 14:30").unwrap();
        assert_eq!(
            (d.year, d.month, d.day, d.hour, d.minute),
            (2026, 5, 20, 14, 30)
        );
    }

    #[test]
    fn parses_date_only_defaults_midnight() {
        let d = parse_datetime("2026-05-20").unwrap();
        assert_eq!((d.hour, d.minute), (0, 0));
    }

    #[test]
    fn rejects_bad_input() {
        assert!(parse_datetime("not-a-date").is_err());
        assert!(parse_datetime("2026").is_err());
    }
}
