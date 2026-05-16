use crate::applescript::{escape_string, run};
use anyhow::{Context, Result};

pub struct SendOptions {
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub body: String,
    /// Account display name to send from. If None, uses Mail's default.
    pub from_account: Option<String>,
}

/// Send an email via Mail.app.
pub fn send(opts: &SendOptions) -> Result<()> {
    let subject_lit = escape_string(&opts.subject);
    let body_lit = escape_string(&opts.body);

    let mut recipients_block = String::new();
    for addr in &opts.to {
        recipients_block.push_str(&format!(
            "        make new to recipient at end of to recipients with properties {{address:\"{}\"}}\n",
            escape_string(addr)
        ));
    }
    for addr in &opts.cc {
        recipients_block.push_str(&format!(
            "        make new cc recipient at end of cc recipients with properties {{address:\"{}\"}}\n",
            escape_string(addr)
        ));
    }
    for addr in &opts.bcc {
        recipients_block.push_str(&format!(
            "        make new bcc recipient at end of bcc recipients with properties {{address:\"{}\"}}\n",
            escape_string(addr)
        ));
    }

    let from_clause = match &opts.from_account {
        Some(account) => {
            let acc_lit = escape_string(account);
            format!(
                "set sender of newMessage to (email addresses of (first account whose name is \"{acc_lit}\")) as string\n    "
            )
        }
        None => String::new(),
    };

    let script = format!(
        r#"
tell application "Mail"
    set newMessage to make new outgoing message with properties {{subject:"{subject_lit}", content:"{body_lit}", visible:false}}
    {from_clause}tell newMessage
{recipients_block}    end tell
    send newMessage
end tell
"#
    );

    run(&script).context("sending email")?;
    Ok(())
}
