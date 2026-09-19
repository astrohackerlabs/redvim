# Bundled language expansion

RedVim compiles AppleScript, Caddyfile, WGSL, GN, Dockerfile, Protocol Buffers, Objective-C, Terraform/HCL, SQL, C, C++, Python, HTML, CSS, Ruby, Zig, Swift, XML/SVG and Make
parsers into the executable and embeds their highlighting queries. Existing
languages, including Nushell, remain available. No Neovim installation, external
parser/query files, trust command or runtime download is needed.

| Language | Files | Default indentation |
| --- | --- | --- |
| Caddyfile | Caddyfile/caddyfile and their dot variants; .caddy/.caddyfile; caddyfile/caddy syntax and fences | tabs, width 4 |
| WGSL | .wgsl (case-insensitive), wgsl syntax and fences | 2 spaces |
| GN | .gn, .gni (case-insensitive), exact .gn filename; gn/gni syntax and fences | 2 spaces |
| Dockerfile | Dockerfile/Containerfile (also lowercase), Dockerfile.*/Containerfile.*, .dockerfile/.containerfile; dockerfile/containerfile syntax and fences | 2 spaces |
| C | .c, .h | 4 spaces |
| Objective-C | .m (case-insensitive); objc/objective-c syntax and fences | 4 spaces |
| Protocol Buffers | .proto (case-insensitive); proto/protobuf syntax and fences | 2 spaces |
| Terraform/HCL | .tf, .tfvars, .hcl; hcl/terraform syntax and fences | 2 spaces |
| SQL | .sql (case-insensitive), explicit sql syntax, Markdown sql fences | 4 spaces |
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
query paths/hashes and license hashes. HCL uses Apache-2.0; WGSL retains W3C and GPU for the Web BSD notices, and generated support headers use MIT. The other expansion crates use MIT. License
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

## Terraform/HCL

The HCL parser is pinned at crate 1.1.0. The separately embedded nvim-treesitter
query is pinned by revision and hashes in the provenance record. Adaptations
map member captures to property and special punctuation to delimiter. Semantic
keyword captures take precedence over generic identifier colors in the opted-in
expansion languages, including configured aliases using those built-in grammars.
Explicit theme styles remain authoritative.

Default indentation is two spaces and line comments use `#`; highlighting also
recognizes `//` and block comments. Templates retain distinct expression colors
inside strings and heredocs. An explicit attribute-access rule preserves property
colors after splats and compound expressions. `.tf.json` and `.tfvars.json` remain JSON.
Standalone `.tftpl` templates, embedded shell/JSON grammars, Terraform execution,
LSP and formatting are outside this integration. All tracked infrastructure
HCL is checked as text; no provider or infrastructure operations are required.

## Objective-C boundary

Objective-C uses the exact `tree-sitter-objc =3.0.2` crate and its published
query, combined with the existing tree-sitter-c 0.24.2 query. The C null-node
pattern becomes the Objective-C grammar's `NULL` token. Query inheritance is
flattened; editor-specific captures are translated, and the unsupported ancestor
predicate becomes structural field-declarator patterns. Additional rules cover
protocol declaration names, pointer properties and no-argument selectors.
The parser crate omits its license notice; the retained MIT notice comes from
the exact upstream revision recorded in provenance.

Specific method, type, field, built-in-variable and parameter captures retain
their colors over generic identifiers. Missing built-in-variable and parameter
theme scopes fall back to keyword and ordinary-variable styles respectively;
explicit theme scopes remain authoritative. Comments use `//`.

`.h` remains C unless explicitly configured or manually set to `objc`.
`.mm` is not registered: the mixed C++ probe (std::vector and a C++ initializer)
produces parser errors with this Objective-C grammar. This integration does not
claim Objective-C++. The repository's WebKitHostingProof.m parses error-free;
its token colors and the full synthetic fixture are checked without execution.

## Protocol Buffers boundary

Protocol Buffers uses the exact `tree-sitter-proto =0.6.0` crate, its generated
ABI-15 parser and matching published query. Package captures map to
`type.namespace` and member captures to `property`; the parser and query need
no runtime downloads or trust action. The MIT notice and original/adapted hashes
are retained in provenance. Defaults are two spaces and `//` comments.

Complete proto2/proto3 fixtures cover maps, oneofs, services/streaming RPC,
options/defaults, groups/extensions and reserved fields. An edition-2023 fixture
also parses and highlights. This is syntax support, not protobuf semantic
validation or a promise of complete edition conformance. All three repository
schemas parse without errors and pass token-color checks without protoc or
service execution. User grammar, theme, indentation and comment overrides remain
available.

## Caddyfile boundary

Caddyfile uses the official caddyserver grammar at the revision recorded in
provenance, with its unchanged generated ABI-15 parser. A complete local crate
retains original scanner/query inputs and compiles with LanguageFn/cc. The
scanner correction distinguishes complete heredoc delimiters from prefix body
lines, preserves lookahead on failed delimiter probes, and resets empty
deserialization state. LF/CRLF, indented markers, sequential heredocs and real
incremental delimiter deletion/reinsertion have tree and color regressions.
Native heredoc markers retain the upstream limit of 31 ASCII letters/digits/
underscores; this is not a Caddy configuration validator.

The adapted matched query colors arguments, paths, escapes and separate
heredoc content/delimiters. Standalone parsed placeholders/environment tokens
retain distinct colors. Quoted/raw literal and compound argument contents stay
host-colored where the grammar exposes no child nodes; CEL/regex/HTML or other
embedded grammars are not injected. No Caddy process or configuration reload is
needed for syntax checks. The pinned MIT notice ships with the binary.

Defaults use tabs of width four and `#` comments. An explicit `indent_width`
selects spaces. Exact configured filenames take precedence over Caddyfile dot
variants; custom grammar, filename, extension, theme and comment overrides remain
available.

## WGSL boundary

WGSL uses the official GPUWeb grammar at the revision recorded in provenance.
Its generated ABI-15 parser and scanner compile in a local LanguageFn/cc crate;
normal builds do not invoke Node or Tree-sitter generation. Tree-sitter CLI
0.25.8 generated the retained sources reproducibly. The original grammar,
scanner, metadata, query and license are retained alongside the adaptations.

Comments, comparison/shift operators and template delimiters are visible nodes.
The scanner skips comments while classifying generic delimiters, including
comments between a type name and `<` and misleading `>` inside line comments.
Nested comment scanning consumes delimiter boundaries without dropping adjacent
characters. Regression fixtures cover the original failures and division/shift
expressions in generic arguments. Queries target individual names and tokens,
preserving independent argument and initializer colors instead of broad upstream
type/declaration captures. Defaults are two spaces and `//` comments.

All three repository shaders are checked as text. This provides syntax colors,
not WGSL semantic validation, compilation or GPU execution. W3C, GPU for the Web
BSD and Tree-sitter generator notices are shipped with exact provenance.

## GN boundary

GN uses the generated ABI-14 parser and native scanner from the checksummed
tree-sitter-gn 1.0.0 crate in a local `crates/tree-sitter-gn` workspace package.
LanguageFn bindings replace its old Tree-sitter 0.20 dependency; RedVim keeps
Tree-sitter 0.25. Builds require neither grammar generation nor external files.
The MIT license comes from the exact upstream revision because the published
crate omitted it. Per-input hashes and wrapper/scanner adaptations are recorded
in provenance; the original scanner and query are retained for comparison.

The scanner's expansion lookahead additionally recognizes `0`, exposing GN
`$0x41` byte escapes to the existing `expansion(hex)` grammar. The generated
parser/header/grammar are unchanged. The adapted query maps fields and special
punctuation to supported scopes, gives interpolation identifiers explicit
parameter captures, and colors byte escapes and assignment names. Tests check
both parse shape and whole-token/custom-theme escape colors so ordinary string
color cannot mask a missing expansion. Defaults are two spaces and `#` comments.

This is GN syntax highlighting, not build evaluation or semantic validation.
No GN, Ninja, Chromium or embedded command is run by the corpus checker.

## AppleScript boundary

AppleScript uses HelgeSverre/tree-sitter-applescript revision
`1676f5fe99eee6b6532ab6d13559323c48d26190`, with its unchanged generated ABI-14
parser and external scanner. A local LanguageFn/cc wrapper compiles both native
files; the upstream Rust build omits the required scanner. Builds do not generate
the grammar. Source/query/license hashes and the complete MIT notice are retained.

Plain `.applescript` files are detected case-insensitively; `applescript` also
works for manual syntax selection and Markdown fences. Compiled `.scpt` files
and `.scptd` bundles are not registered as source. Defaults are four-column tabs
and `--` comments, with normal configuration overrides.

The adapted query maps Helix captures to supported scopes, preserves specialized
handler/parameter colors over generic identifiers, and handles command-shaped
handler names, bare handler parameters and error parameters. Tests use effective
whole-token colors, LF/CRLF incremental edits, nested comments, strings, Unicode,
alias forms, piped names and line continuations. The real repository script is
parsed and color-checked without running its desktop automation.

This grammar does not resolve application dictionaries: for example, `click`
and `keystroke` in the repository UI script remain generic identifiers. Some
dictionary-heavy scripts, chained one-line tells and one-line `if` command tails
have upstream grammar gaps. An indented `to` handler can produce an error-free
tree without a handler-definition node, so its handler-name styling is not
claimed; column-zero `to` and ordinary `on` handlers are qualified. Error-free
parsing alone is not semantic validation or proof of complete AppleScript support.

## Dockerfile boundary

Dockerfile uses exact `tree-sitter-containerfile =0.9.2`, including its generated
parser, native scanner and matching query. The checksummed crate is the exact
source authority: its published VCS metadata reports a dirty tree. Both the
WharfLab and Camden Cheek MIT notices are retained.

The copied query translates special punctuation, removes the spell annotation,
colors generic flag nodes (their names are hidden tokens in the generated parser),
matches anonymous mount names, and adds image names, stage aliases and line
continuations. Variables use the constant color regardless of letter case.
Heredoc string captures cover the content node, leaving delimiters distinct
with a constant capture instead of the unsupported label theme scope.
Exact configured filenames take precedence over Dockerfile/Containerfile prefix
variants. Comments use `#`. Configuration and theme overrides remain supported.

This is host-language highlighting, including BuildKit flags and heredocs.
Shell command text and arbitrary heredoc content have no Bash/JSON/Python
injections; a custom SHELL is not assumed to be Bash. No container build or
embedded command is executed by the corpus checker.

## SQL dialect boundary

SQL uses the checksummed `tree-sitter-sequel =0.3.11` crate. Its published VCS
metadata reports a dirty source tree; the crate checksum, not its Git revision
alone, is the exact parser authority. The copied query translates Lua numeric
patterns to Rust regexes and maps field/parameter/storageclass captures to
property/variable.parameter/keyword. Normal configuration and theme overrides
remain supported; comments use `--` and indentation defaults to four spaces.

The general grammar recovers with ERROR nodes on MySQL DATETIME precision,
charset/collation assignments, session-variable SET statements, prepared
statements and some Drizzle named UNIQUE constraints, and SQLite AUTOINCREMENT.
The tree is preserved, including those errors. A narrow supplemental pass colors
uncovered PREPARE, DEALLOCATE, AUTOINCREMENT, UNIQUE and ASCII-named @session variables
inside recovery regions. It skips quoted strings/identifiers and comments and
does not override query colors. It applies only to the bundled SQL grammar and
its default query, including configured aliases of that built-in grammar.

The self-check verifies concrete MySQL/SQLite token colors, including statements
after recovery, alongside an error-free basic SQL fixture. Tests cover overrides,
fences, Unicode, incomplete edits, undo, and quoted/commented lookalikes. The
developer example `check_sql_files` reports recovery nodes and asserts token
colors against real migration files without executing them. Syntax highlighting
is provided; dialect conformance, SQL execution, formatting, language servers
and SQL embedded inside other host languages are not part of this support.
