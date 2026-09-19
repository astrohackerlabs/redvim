# Bundled language expansion

RedVim compiles C, C++, Python, HTML, CSS, Ruby, Zig, Swift, XML/SVG and Make
parsers into the executable and embeds their highlighting queries. Existing
languages, including Nushell, remain available. No Neovim installation, external
parser/query files, trust command or runtime download is needed.

| Language | Files | Default indentation |
| --- | --- | --- |
| C | .c, .h | 4 spaces |
| C++ | .cc, .cpp, .cxx, .hpp, .hxx, .hh | 4 spaces |
| Python | .py, .pyi; python/python3 shebang | 4 spaces |
| HTML | .html, .htm | 2 spaces |
| CSS | .css | 2 spaces |
| Ruby | .rb, Gemfile, Rakefile, formula.rb.in; ruby shebang | 2 spaces |
| Zig | .zig | 4 spaces |
| Swift | .swift | 4 spaces |
| XML | .xml, .svg, textual XML .plist | 2 spaces |
| Make | .mk, Makefile, makefile, GNUmakefile | tabs, width 8 |

C owns .h by default. For a C++ project, use the existing language override or
add `[languages.cpp]` with `extensions = ["h", "cc", "cpp", "cxx", "hpp", "hxx", "hh"]`
to config.toml. Comment commands use the language's usual comment delimiter.
The existing indent_width override selects spaces; omit it for Make recipe tabs.

This expansion provides host-language syntax highlighting. Embedded JS/CSS in
HTML and shell syntax in Make recipes are not enabled here. DTD files, binary
plists, Objective-C++, SCSS, LSP completion and semantic diagnostics are not
claimed by these mappings. Syntax highlighting tolerates incomplete edits; it
is not a compiler or dialect conformance check.

## Reproducible inputs

Cargo.toml pins exact crate versions and Cargo.lock pins archive checksums.
`languages-provenance.json` records crate checksums, upstream source revisions,
query paths/hashes and license hashes. All ten crates declare MIT; their license
texts are in docs/licenses and distributed in the binary archive. When the crate
omits its license file, the recorded source URL identifies the exact upstream
revision used to retrieve it.

C++ combines the C and C++ queries. Most queries are embedded directly from
their crate. Zig's copied query replaces its three Lua patterns with equivalent
Rust regex predicates and removes the editor-specific priority directive.
Ruby's copied query removes the generic identifier-as-method rule guarded by
`#is-not? local`: Red does not evaluate local-scope predicates, so leaving it
would incorrectly color local variables as methods. Explicit method/call
patterns remain. Adapted query hashes are recorded separately.

For the ten added languages, semantic captures such as function/type/constant
take precedence over generic captures covering the same bytes. Existing language
precedence is preserved. Theme compatibility fallbacks map legacy constant,
boolean/float and control-flow captures to existing styles when an explicit
theme scope is absent.

`redvim --self-check` parses embedded fixtures and checks filename recognition
and effective Mocha token colors for each new language, SVG and Nushell. It uses
built-ins without reading external grammar configuration. Tests additionally
cover detection overrides, default comments/indentation and Make tabs.

Nushell's independently pinned provenance remains in docs/NUSHELL.md.
