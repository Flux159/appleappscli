# appleappscli (`aacli`)

[![CI](https://github.com/Flux159/appleappscli/actions/workflows/ci.yml/badge.svg)](https://github.com/Flux159/appleappscli/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Flux159/appleappscli)](https://github.com/Flux159/appleappscli/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

CLI for scripting macOS apps from the terminal via AppleScript. Built in Rust.

## Status

| Subcommand | Status |
|---|---|
| `aacli notes` | ✅ **Working** — create, list, append, read, move, attach images |
| `aacli reminders` | ✅ **Working** — create (with due dates), list, complete, delete |
| `aacli calendar` | ✅ **Working** — add events, list-day, list-calendars |
| `aacli messages` | ✅ **Working** — send, list (most-recent order), read (requires Full Disk Access for list/read) |
| `aacli photos` | ✅ **Working** — albums, find (by name/index/id), export to PNG/JPG/original (albums/find require Full Disk Access) |
| `aacli terminal` | ✅ **Working** — new-window, new-tab, send command, list-windows |
| `aacli mail` | ✅ **Working** — send (to/cc/bcc/from-account), list-mailboxes, list-recent (per-account inbox) |

Contributions for the stubbed subcommands welcome — the module skeletons are in place; see [Contributing](#contributing).

## Install

### Pre-built binaries (recommended)

Download from the [latest release](https://github.com/Flux159/appleappscli/releases/latest). Pick the archive for your Mac's architecture:

- **Apple Silicon (M1/M2/M3/M4):** `aacli-vX.Y.Z-macos-aarch64.tar.gz`
- **Intel:** `aacli-vX.Y.Z-macos-x86_64.tar.gz`

```bash
# Apple Silicon
curl -L -o aacli.tar.gz https://github.com/Flux159/appleappscli/releases/latest/download/aacli-vX.Y.Z-macos-aarch64.tar.gz
tar -xzf aacli.tar.gz
sudo mv aacli /usr/local/bin/
aacli --version
```

(Replace `vX.Y.Z` with the version on the releases page.)

Each release also includes a `.sha256` file for integrity verification:

```bash
shasum -a 256 -c aacli-vX.Y.Z-macos-aarch64.tar.gz.sha256
```

### From source (Rust toolchain required)

```bash
git clone https://github.com/Flux159/appleappscli
cd appleappscli
cargo install --path .
```

This puts `aacli` on `$PATH` under `~/.cargo/bin`.

For development: `cargo build --release` and use `./target/release/aacli` directly.

## Notes

### Create a note from a markdown file

```bash
aacli notes create --markdown-file path/to/note.md --folder "MyFolder"
```

- The folder is created if it doesn't exist.
- Title is derived from the first `# H1` heading; falls back to the file stem.
- Override with `--title "Custom Title"`.

### Create a note from stdin

```bash
echo "# Hello\n\nWorld" | aacli notes create --stdin --folder "Inbox"
```

### Create a note with explicit HTML

```bash
aacli notes create --html-body "<h1>Hello</h1><p>World</p>" --folder "Inbox"
```

### List notes

```bash
aacli notes list                      # all notes
aacli notes list --folder "MyFolder"  # notes in folder
```

Output is one note per line: `<id>\t<name>`.

### Append to an existing note

```bash
aacli notes append --id "x-coredata://..." --markdown-file extra.md
echo "## Update\nNew section" | aacli notes append --id "x-coredata://..." --stdin
aacli notes append --id "x-coredata://..." --html-body "<p>more</p>"
```

### Read a note's HTML body

```bash
aacli notes read --id "x-coredata://..."
```

Pipe through `pandoc -f html -t markdown` or similar if you want markdown.

### Move a note to a different folder

```bash
aacli notes move --id "x-coredata://..." --folder "Archive"
```

Folder is created if it doesn't exist.

### Attach an image

```bash
aacli notes attach --id "x-coredata://..." --image /path/to/photo.png
aacli notes attach --id "x-coredata://..." --image ~/Downloads/IMG_0595.heic
```

Works with PNG, JPG, GIF, HEIC, and any image type Notes natively accepts.

## Reminders

### Create a reminder

```bash
aacli reminders create "Buy milk"
aacli reminders create "Submit report" --due "2026-05-20 14:30"
aacli reminders create "Call dentist" --due "2026-05-20" --list "Personal"
aacli reminders create "Pay rent" --due "2026-06-01" --notes "Via Zelle"
```

Due format: `YYYY-MM-DD HH:MM` (24-hour) or `YYYY-MM-DD` (defaults to 09:00).

### List reminders

```bash
aacli reminders list                       # default list, incomplete only
aacli reminders list --list "Personal"     # specific list
aacli reminders list --all                 # include completed
```

Output: `<id>\t<name>\t<completed:true|false>`.

### Complete a reminder

```bash
aacli reminders complete --id "x-apple-reminder://..."
```

### Delete a reminder

```bash
aacli reminders delete --id "x-apple-reminder://..."
```

## Calendar

### List your calendars

```bash
aacli calendar list-calendars
```

### List events for a specific day

```bash
aacli calendar list-day --date "2026-05-20"
aacli calendar list-day --date "2026-05-20" --calendar "Work"
```

Output: `<uid>\t<calendar>\t<start ISO>\t<end ISO>\t<summary>`.

### Add a calendar event

```bash
aacli calendar add \
  --calendar "Home" \
  --title "Doctor appointment" \
  --start "2026-05-20 14:00" \
  --end "2026-05-20 15:00" \
  --location "123 Main St" \
  --notes "Bring insurance card"
```

Datetime format: `YYYY-MM-DD HH:MM` (24h) or `YYYY-MM-DD` (defaults to 00:00).

## Messages

### Permissions

- **`send`** uses AppleScript and needs Messages.app Automation permission (granted on first prompt).
- **`list`** and **`read`** query `~/Library/Messages/chat.db` directly, which requires **Full Disk Access** for the terminal you run `aacli` from. Grant it in **System Settings → Privacy & Security → Full Disk Access**.

### Send a message

```bash
aacli messages send --to "+15551234567" --text "Hello"
aacli messages send --to "name@example.com" --text "iMessage to email handle"
```

Uses iMessage if the recipient is on iMessage; falls back to the first available service (SMS) otherwise.

### List chats (most-recent first)

```bash
aacli messages list                  # last 25 chats
aacli messages list --limit 50
```

Output: `<chat_identifier>\t<display_name>\t<last_message_local_time>\t<preview>`. Order matches Messages.app UI (most-recent at top).

### Read recent messages from a chat

```bash
aacli messages read --chat "+15551234567"
aacli messages read --chat "group-chat-name" --limit 100
```

Output (oldest-to-newest within the most-recent N): `<local_time>\t<me|them>\t<sender>\t<text>`.

## Photos

- **`albums`** and **`find`** query the Photos library SQLite database at `~/Pictures/Photos Library.photoslibrary/database/Photos.sqlite` directly. This is much faster than driving Photos.app via AppleScript (a 31K-photo library: `find --index 28000` runs in ~50ms instead of timing out past 10 min). It requires **Full Disk Access** for the terminal you run `aacli` from — grant it in **System Settings → Privacy & Security → Full Disk Access**.
- **`export`** still uses Photos.app via AppleScript so iCloud-only assets are downloaded on demand. UUIDs returned by `find` are accepted as `--id`.

### List albums

```bash
aacli photos albums
```

### Find a photo

Three lookup modes — choose one:

```bash
# By filename substring (e.g. IMG_0595 matches IMG_0595.HEIC, IMG_0595_edited.jpg)
aacli photos find --name "IMG_0595"
aacli photos find --name "IMG_0595" --limit 5

# By library index (1-based, oldest-first by capture date — matches Photos "All Photos" order)
aacli photos find --index 28301

# By stable photo id (from a prior find call)
aacli photos find --id "F1D2D3E4-..."
```

Output: `<photo_id>\t<filename>\t<iso_date>\t<width>x<height>`.

### Export a photo (to PNG, JPG, or original)

```bash
aacli photos export --id "F1D2D3E4-..." --output-dir ~/Pictures/exports --format png
aacli photos export --name "IMG_0595" --output-dir ./out --format jpg
aacli photos export --index 28301 --output-dir ./out --format original
```

- `--format original` exports the original file as-is (HEIC, JPG, PNG, etc.).
- `--format png` and `--format jpg` use macOS's built-in `sips` to convert from the original.
- The output directory is created if missing.

## Terminal

### Open a new window (optionally running a command)

```bash
aacli terminal new-window
aacli terminal new-window --command "ssh user@host"
```

Returns the new window's id on stdout.

### Open a new tab in the front window

```bash
aacli terminal new-tab
aacli terminal new-tab --command "htop"
```

`new-tab` uses System Events to send Cmd+T (Terminal.app doesn't expose tab creation via AppleScript directly). Falls back to opening a new window if no Terminal windows are open.

### Send a command to an existing window/tab

```bash
aacli terminal send --command "ls -la"                  # front window's selected tab
aacli terminal send --window 12345 --command "pwd"       # specific window
aacli terminal send --window 12345 --tab 2 --command "git status"
```

### List open Terminal windows

```bash
aacli terminal list-windows
```

Output: `<window_id>\t<tab_count>\t<title>\t<tty>`. The window id can be used with `terminal send --window`.

## Mail

### Send an email

```bash
aacli mail send \
  --to "alice@example.com" --to "bob@example.com" \
  --cc "boss@example.com" \
  --subject "Status update" \
  --body "Hi team, here is the weekly status..."

aacli mail send \
  --to "alice@example.com" \
  --subject "From specific account" \
  --body "Body" \
  --from-account "Work"
```

`--to`, `--cc`, `--bcc` accept multiple values (repeat the flag). If `--from-account` is omitted, Mail.app uses the default account.

### List mailboxes

```bash
aacli mail list-mailboxes
```

Output: `<account>\t<mailbox>`. Useful for discovering account names (the `--from-account` value matches the first column).

### List recent inbox messages

```bash
aacli mail list-recent --limit 25
```

Iterates each account's INBOX (more reliable across macOS versions than the unified `inbox` keyword). Output: `<account>\t<message_id>\t<date>\t<sender>\t<subject>`.

## Why not just use osascript?

- **Quote-safe**: HTML bodies routinely contain `"` in attribute values, which breaks naive AppleScript embedding. `aacli` escapes them.
- **Markdown native**: pulldown-cmark handles tables, footnotes, task lists, strikethrough, and smart punctuation.
- **Scriptable from any language**: returns IDs on stdout, exits non-zero on failure. Easy to compose in shell pipelines or other tools.

## Size & performance

| | `aacli` |
|---|---|
| Distributed binary size | **1.2 MB** (stripped) |
| Required runtime | None — self-contained |
| Cold-start time | <10 ms |

A self-contained static binary means a single download, no package-manager install steps, no language runtime to provision, and no virtual-environment management. Cold start is fast enough to use from shell aliases, hooks, and agents without noticeable lag.

## Scope: Apple apps only

This CLI is intentionally scoped to **first-party Apple apps that ship with macOS**. We've considered popular third-party Mac apps and decided to keep them out of scope. Here's the reasoning so you don't have to relitigate it:

| App | Decision | Why |
|---|---|---|
| **Slack** | Out of scope | Has a [first-class HTTP API](https://api.slack.com/) and the `slack-cli` (official) plus many community CLIs. AppleScript surface is minimal. Use the API, not AppleScript |
| **Discord** | Out of scope | Official REST/Gateway APIs + many SDKs. No real AppleScript dictionary |
| **Signal** | Out of scope | No AppleScript dictionary at all. Use [signal-cli](https://github.com/AsamK/signal-cli) (links to the same account via Signal's official linking protocol) |
| **WhatsApp** | Out of scope | No AppleScript dictionary. Official approach is the WhatsApp Business API (very limited for personal use) or browser automation |
| **Chrome / Safari** | Out of scope | Browser automation is better served by [Playwright](https://playwright.dev/), [Puppeteer](https://pptr.dev/), or browser-agent frameworks. AppleScript browser APIs are limited to URL navigation and tab listing |

**The principle:** `aacli` is for apps where AppleScript is the *native* automation interface (the first-party Apple apps). For apps where a documented HTTP API or purpose-built CLI exists, use that — it's more reliable, doesn't require the app to be running, and has better cross-platform support.

If you want a unified CLI that wraps both first-party AppleScript apps and third-party HTTP APIs, that's a separate tool (likely with an MCP-style plugin architecture) — happy to see one built, but it's not this one.

## Roadmap

- [x] Notes: create, list
- [x] Reminders: create, list
- [x] Notes: append (add content to existing note by id)
- [x] Notes: read (fetch HTML body by id)
- [x] Notes: move (between folders)
- [x] Notes: attach (add image file to existing note)
- [x] Reminders: complete (mark done by id)
- [x] Reminders: delete
- [x] Calendar: add, list-day, list-calendars
- [x] Messages: list (most-recent order), read, send
- [x] Photos: albums, find by name/id/index, export PNG/JPG/original
- [x] Terminal: new-tab, new-window, send-command, list-windows
- [x] Mail: send, list-mailboxes, list-recent

## Architecture

```
src/
├── main.rs            CLI entry, subcommand dispatch
├── lib.rs             Module exports
├── applescript.rs     osascript invocation + string escaping
├── markdown.rs        Markdown → HTML (pulldown-cmark with tables/footnotes/etc)
├── notes/
│   ├── mod.rs         Subcommand types
│   ├── create.rs      `notes create`
│   ├── list.rs        `notes list`
│   ├── append.rs      `notes append`
│   ├── read.rs        `notes read`
│   ├── move_note.rs   `notes move`
│   └── attach.rs      `notes attach`
├── reminders/
│   ├── mod.rs
│   ├── create.rs      `reminders create` (with due-date parsing)
│   ├── list.rs        `reminders list`
│   ├── complete.rs    `reminders complete`
│   └── delete.rs      `reminders delete`
├── calendar/
│   ├── mod.rs
│   ├── datetime.rs    parse YYYY-MM-DD HH:MM, AppleScript date construction
│   ├── add.rs         `calendar add`
│   ├── list_day.rs    `calendar list-day`
│   └── list_calendars.rs   `calendar list-calendars`
├── messages/
│   ├── mod.rs
│   ├── chatdb.rs      read-only chat.db queries (rusqlite, system sqlite)
│   ├── send.rs        `messages send` (AppleScript)
│   ├── list.rs        `messages list`
│   └── read.rs        `messages read`
├── photos/
│   ├── mod.rs
│   ├── photosdb.rs    Photos.sqlite read-only access (rusqlite, immutable mode)
│   ├── albums.rs      `photos albums` (SQLite)
│   ├── find.rs        `photos find` name/index/id (SQLite)
│   └── export.rs      `photos export` (AppleScript + sips for PNG/JPG conversion)
├── terminal/
│   ├── mod.rs
│   ├── new_window.rs    `terminal new-window`
│   ├── new_tab.rs       `terminal new-tab` (uses System Events for Cmd+T)
│   ├── send.rs          `terminal send`
│   └── list_windows.rs  `terminal list-windows`
├── mail/
│   ├── mod.rs
│   ├── send.rs          `mail send` (to/cc/bcc/from-account)
│   ├── mailboxes.rs     `mail list-mailboxes`
│   └── list_recent.rs   `mail list-recent` (per-account INBOX)
```

Each app gets its own module. Adding a new app = new module + clap subcommand + module dispatch.

## Tests

```bash
cargo test
```

Unit tests cover markdown→HTML conversion, title extraction, AppleScript escaping, and due-date parsing. Integration with actual macOS apps requires running on macOS with the relevant apps installed; tests don't hit real Notes/Reminders.

## Contributing

Contributions are welcome — especially for the stubbed subcommands (`calendar`, `messages`, `photos`, `terminal`).

### Pattern to follow when implementing a stubbed app

Use `notes` or `reminders` as the template. Each implemented app has the same shape:

```
src/<app>/
├── mod.rs        clap Subcommand enum + run() dispatcher
├── create.rs     (or similar) command implementations
└── list.rs
```

The `mod.rs` defines a `Subcommand` enum, a `run(&Cmd)` dispatcher, and `pub use`s for any types that callers need. Then in `src/main.rs`, the `Command::<App>` variant is wired up to call `<app>::run`.

To add a new operation to an existing app (e.g., `notes append`):
1. Add a variant to the app's `Subcommand` enum
2. Add a match arm in the app's `run()` function
3. Implement the function in a new file under the app's module
4. Add unit tests for any pure logic (parsing, escaping, etc.)

### Development workflow

```bash
# Run all tests
cargo test

# Format
cargo fmt

# Lint (CI gates on this)
cargo clippy --all-targets -- -D warnings

# Build release binary
cargo build --release
```

CI runs `fmt --check`, `clippy -D warnings`, `cargo test`, and `cargo build --release` on every push to `main` and PR. Please run all four locally before opening a PR.

### Releasing (maintainers)

```bash
git tag v0.2.0
git push origin v0.2.0
```

The release workflow auto-bumps `Cargo.toml`, commits back to `main`, builds Apple Silicon + Intel binaries, attaches them (with SHA256 sidecars) to the GitHub Release, and publishes it.

### Reporting issues

Open a GitHub issue with:
- macOS version (`sw_vers`)
- `aacli --version`
- The exact command run and the error output
- The AppleScript error message if present

### Code of conduct

Be kind. Assume good intent.

## License

[MIT](LICENSE) © 2026 Suyog Sonwalkar
