# RedVim

Astrohacker's modal editor, based on [Red](https://github.com/codersauce/red).
Nushell syntax highlighting is built in. No external parser, query file, or
grammar trust command is required.

## Install

Apple Silicon, macOS 26 or newer:

```nu
brew trust astrohackerlabs/astrohacker
brew tap astrohackerlabs/astrohacker
brew install redvim
redvim example.nu
```

Configuration lives in `$XDG_CONFIG_HOME/redvim`, or `~/.config/redvim`.
Existing Red configuration is independent. `REDVIM_RUNTIME` is an optional
development override. Normal installations use embedded runtime assets.
Language servers and optional agent integrations require their own setup;
basic editing and bundled highlighting do not.

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

See PROVENANCE.json and docs/NUSHELL.md for source revisions and attribution.
RedVim retains Red's MIT license and upstream history. Bundled Nushell grammar
is MIT; the Neovim highlighting queries are Apache-2.0. See docs/licenses/.
