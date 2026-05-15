# appleappscli (`aacli`)

[![CI](https://github.com/Flux159/appleappscli/actions/workflows/ci.yml/badge.svg)](https://github.com/Flux159/appleappscli/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Flux159/appleappscli)](https://github.com/Flux159/appleappscli/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

CLI for scripting macOS apps from the terminal via AppleScript. Built in Rust.

## Status

| Subcommand | Status |
|---|---|
| `aacli notes` | ✅ **Working** — create from markdown / stdin / HTML, list, folder targeting |
| `aacli reminders` | ✅ **Working** — create with due dates, list (filtered by list name, with/without completed) |
| `aacli calendar` | 🚧 **Stub** — exits with "not implemented yet". Future: add/list events |
| `aacli messages` | 🚧 **Stub** — Future: send messages, list recent threads |
| `aacli photos` | 🚧 **Stub** — Future: import, list albums, export |
| `aacli terminal` | 🚧 **Stub** — Future: open new tab/window with command |

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

## Roadmap

- [x] Notes: create, list
- [x] Reminders: create, list
- [ ] Notes: append (add content to existing note by id)
- [ ] Notes: read (fetch HTML body by id)
- [ ] Notes: move (between folders)
- [ ] Reminders: complete (mark done by id)
- [ ] Reminders: delete
- [ ] Calendar: add, list
- [ ] Messages: send
- [ ] Photos: import, albums, export
- [ ] Terminal: new-tab, new-window

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
│   └── list.rs        `notes list`
├── reminders/
│   ├── mod.rs
│   ├── create.rs      `reminders create` (with due-date parsing)
│   └── list.rs        `reminders list`
├── calendar/ messages/ photos/ terminal/   stubs
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
