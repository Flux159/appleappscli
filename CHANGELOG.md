# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.1] - 2026-05-16

### Fixed
- Release workflow Cargo.toml version bump silently failed on macOS runners because BSD sed doesn't support the GNU `0,/PATTERN/` address range syntax. Switched to portable awk. Result: prior release binaries (v0.2.0–v0.7.0) reported `aacli 0.1.0` from `--version` despite being correctly built; v0.7.1+ reports the correct tag version.

## [0.7.0] - 2026-05-16

### Added
- `aacli mail send` — send email via Mail.app with `--to`/`--cc`/`--bcc` (repeatable), `--subject`, `--body`, and optional `--from-account` to select the sending account.
- `aacli mail list-mailboxes` — list all `<account>\t<mailbox>` pairs across configured accounts.
- `aacli mail list-recent` — list recent messages from each account's INBOX. Iterates per-account inbox rather than the unified `inbox` keyword (which is unreliable across macOS versions). Output includes account name, message id, date, sender, and subject.
- README: added explicit scope decision for third-party apps (Slack, Discord, Signal, WhatsApp, Chrome, Safari) explaining why they belong in their own tools, not this one.

## [0.6.0] - 2026-05-16

### Added
- `aacli terminal new-window` — open a new Terminal window, optionally running a command. Returns the window id.
- `aacli terminal new-tab` — open a new tab in the front Terminal window (uses System Events Cmd+T since Terminal.app doesn't expose tab creation via AppleScript directly). Falls back to opening a new window if no windows are open.
- `aacli terminal send` — send a command to a specific window/tab (or the front window's selected tab by default).
- `aacli terminal list-windows` — list open Terminal windows with their id, tab count, title, and tty path.

## [0.5.0] - 2026-05-16

### Added
- `aacli photos albums` — list all album names from Photos.app.
- `aacli photos find` — find photos by filename substring (`--name`), 1-based library index (`--index`), or stable photo id (`--id`). Returns id, filename, ISO date, dimensions.
- `aacli photos export` — export a photo to a directory. `--format` accepts `original` (as-is), `png`, or `jpg`. PNG/JPG conversion uses macOS `sips`.
- All Photos AppleScript ops wrapped in a 600s explicit timeout (default osascript timeout is 2 min, which can be exceeded on libraries with 10K+ photos).

## [0.4.0] - 2026-05-16

### Added
- `aacli messages send` — send a message via iMessage (falls back to SMS) using AppleScript.
- `aacli messages list` — list chats sorted by most-recent message (matches Messages.app UI order). Reads `~/Library/Messages/chat.db` directly via rusqlite. Requires Full Disk Access for the terminal.
- `aacli messages read` — read recent messages from a chat by phone/email/guid. Output is local-time formatted with `me|them` direction.
- Added `rusqlite` dependency (linked dynamically to macOS system SQLite — minimal binary size impact).

## [0.3.0] - 2026-05-16

### Added
- `aacli calendar add` — create a calendar event with title, start/end datetime, calendar, optional notes and location.
- `aacli calendar list-day` — list events for a specific date (optionally filtered to one calendar). Output includes UID, calendar name, ISO start/end timestamps, summary.
- `aacli calendar list-calendars` — list all calendar names accessible to Calendar.app.

## [0.2.0] - 2026-05-16

### Added
- `aacli notes append` — append HTML/markdown/stdin content to an existing note by id.
- `aacli notes read` — fetch the HTML body of a note by id.
- `aacli notes move` — move a note to a different folder (creates folder if missing).
- `aacli notes attach` — attach a local image file (PNG/JPG/GIF/HEIC) to an existing note.
- `aacli reminders complete` — mark a reminder as completed by id.
- `aacli reminders delete` — delete a reminder by id.

## [0.1.0] - 2026-05-14

### Added
- Initial release.
- `aacli notes create` — create notes from a markdown file, stdin, or raw HTML body. Auto-creates folder if missing. Derives title from first heading or file stem.
- `aacli notes list` — list notes (optionally filtered by folder).
- `aacli reminders create` — create reminders with optional due date (`YYYY-MM-DD HH:MM`), notes body, and list target.
- `aacli reminders list` — list reminders (optionally include completed).
- Subcommand stubs for `calendar`, `messages`, `photos`, `terminal`.
- AppleScript string escaping (handles `"` and `\` correctly — improvement over naive embedding used by similar tools).
- Markdown to HTML conversion via `pulldown-cmark` with tables, footnotes, strikethrough, task lists, and smart punctuation enabled.

[Unreleased]: https://github.com/Flux159/appleappscli/compare/v0.7.1...HEAD
[0.7.1]: https://github.com/Flux159/appleappscli/releases/tag/v0.7.1
[0.7.0]: https://github.com/Flux159/appleappscli/releases/tag/v0.7.0
[0.6.0]: https://github.com/Flux159/appleappscli/releases/tag/v0.6.0
[0.5.0]: https://github.com/Flux159/appleappscli/releases/tag/v0.5.0
[0.4.0]: https://github.com/Flux159/appleappscli/releases/tag/v0.4.0
[0.3.0]: https://github.com/Flux159/appleappscli/releases/tag/v0.3.0
[0.2.0]: https://github.com/Flux159/appleappscli/releases/tag/v0.2.0
[0.1.0]: https://github.com/Flux159/appleappscli/releases/tag/v0.1.0
