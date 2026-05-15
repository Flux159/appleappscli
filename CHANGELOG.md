# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/Flux159/appleappscli/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Flux159/appleappscli/releases/tag/v0.1.0
