use super::datetime::{applescript_date, parse_datetime};
use crate::applescript::{escape_string, run};
use anyhow::{Context, Result};

pub struct AddOptions {
    pub calendar: String,
    pub title: String,
    pub start: String,
    pub end: String,
    pub notes: Option<String>,
    pub location: Option<String>,
}

/// Create a calendar event. Returns the new event's id.
pub fn add_event(opts: &AddOptions) -> Result<String> {
    let start = parse_datetime(&opts.start).context("parsing --start")?;
    let end = parse_datetime(&opts.end).context("parsing --end")?;

    let cal_lit = escape_string(&opts.calendar);
    let title_lit = escape_string(&opts.title);

    let mut extra_props = String::new();
    if let Some(notes) = &opts.notes {
        extra_props.push_str(&format!(", description:\"{}\"", escape_string(notes)));
    }
    if let Some(location) = &opts.location {
        extra_props.push_str(&format!(", location:\"{}\"", escape_string(location)));
    }

    let start_setup = applescript_date("startDate", start);
    let end_setup = applescript_date("endDate", end);

    let script = format!(
        r#"
tell application "Calendar"
    {start_setup}
    {end_setup}
    tell calendar "{cal_lit}"
        set newEvent to make new event with properties {{summary:"{title_lit}", start date:startDate, end date:endDate{extra_props}}}
        return uid of newEvent
    end tell
end tell
"#
    );

    run(&script).context("creating event")
}
