# RedVim

Astrohacker's modal editor, based on [Red](https://github.com/codersauce/red).
Typst, AppleScript, Caddyfile, WGSL, GN, Dockerfile, Protocol Buffers, Objective-C, Terraform/HCL, SQL, Nushell, C, C++, Python, HTML, CSS, Ruby, Zig, Swift, XML/SVG and Makefile syntax
highlighting are built in alongside Red's existing languages. No external parser, query file, or
grammar trust command is required.

## Install

Apple Silicon, macOS 26 or newer:

```nu
brew trust astrohackerlabs/astrohacker
brew tap astrohackerlabs/astrohacker
brew install redvim
redvim example.nu
```

Configuration lives in `$XDG_CONFIG_HOME/astrohacker/redvim`, or `~/.config/astrohacker/redvim`.

The main file is `config.toml`. Themes, plugins, preferences, trust records,
session state, caches and `redvim.log` share that root. Existing `~/.config/redvim`
and `~/.config/red` directories are not imported automatically. To reuse an old
configuration, close the editor and copy `config.toml` to the new directory
without overwriting an existing file. Custom themes and plugins can be copied
selectively; leave sockets, PID files, recovery state and caches behind.
Re-trust external native grammars with `redvim language trust /full/path/to/parser.so`.
Bundled Nushell highlighting needs no trust command.
Existing Red configuration is independent. `REDVIM_RUNTIME` is an optional
development override. Normal installations use embedded runtime assets.
Language servers and optional agent integrations require their own setup;
basic editing and bundled highlighting do not.

## Markdown formatting

Homebrew installs Prettier (and its Node dependency) alongside RedVim. Press
Space f in normal mode to format Markdown, or save to use the default automatic
formatting. Paragraphs wrap to an 80-column target; long unbreakable words or
URLs can exceed it. No personal formatter configuration is required.

The embedded settings are:

```toml
[languages.markdown.formatter]
name = "Prettier"
command = "prettier"
args = ["--stdin-filepath", "{file}", "--print-width", "80", "--prose-wrap", "always"]
root_markers = ["package.json", ".git"]
```

Project-local Prettier takes precedence over PATH. These CLI wrapping flags
override project Prettier wrapping options. Override the formatter args in your
config.toml to choose another width or follow project settings. Copy the complete
formatter table above before editing it: explicit language definitions replace
the embedded language entry. For manual-only
formatting, add:

```toml
[formatting]
on_save = false
```

User settings are preserved on upgrade. Source/archive users must install
Prettier separately if they want this formatter.

## Build

Use Rust with edition 2024 support and Apple's C/C++ command line tools.
The lockfile pins dependencies; Cargo fetches the pinned Nushell grammar Git
revision during the build. The native parser is linked into the executable.

```nu
with-env {MACOSX_DEPLOYMENT_TARGET: '26.0'} {
  cargo build --locked --release --bin redvim
}
./target/release/redvim --self-check
cargo test --locked --workspace --all-targets --all-features
```

See PROVENANCE.json, docs/NUSHELL.md and docs/BUNDLED_LANGUAGES.md for source
revisions, language mappings, limitations and attribution.
RedVim retains Red's MIT license and upstream history. Bundled Nushell grammar
is MIT; the Neovim highlighting queries are Apache-2.0. WGSL retains its W3C
Software and Document License and GPU for the Web BSD notices. See docs/licenses/.
