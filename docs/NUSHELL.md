# Bundled Nushell support

The Astrohacker fork bundles Nushell syntax highlighting. Files ending in `.nu`
are detected automatically. `:syntax nu`, the `nushell` alias, Markdown fences
using either name, and `nu` shebangs use the bundled parser. Comments use `#`
and indentation defaults to two spaces. User language settings can override
these editing defaults.

No user configuration, external parser library, query installation, trust
command, or runtime download is required. Cargo compiles the grammar into the
executable and `include_str!` embeds the highlights query. External grammar
libraries retain the normal explicit trust requirement. This integration does
not provide a Nushell language server or Nushell-specific structural queries.

## Source and attribution

The parser and query are the pair qualified using Ryan's installed Neovim in
Astrohacker Issue 26091823140848 Experiment 3, then bundled in Experiment 4.

- Grammar: [nushell/tree-sitter-nu](https://github.com/nushell/tree-sitter-nu/tree/bb3f533e5792260291945e1f329e1f0a779def6e),
  commit `bb3f533e5792260291945e1f329e1f0a779def6e`, pinned in Cargo.toml and
  Cargo.lock. Copyright 2019–2022 The Nushell Project Developers, MIT;
  full notice in [licenses/tree-sitter-nu.txt](licenses/tree-sitter-nu.txt).
- Query: [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter/blob/4d9466677a5ceadef104eaa0fe08d60d91c4e9a7/runtime/queries/nu/highlights.scm),
  commit `4d9466677a5ceadef104eaa0fe08d60d91c4e9a7`,
  `runtime/queries/nu/highlights.scm`, copied unchanged to
  `src/queries/highlights/nu.scm`. SHA-256:
  `6c3acab09f5707e2db70d46d3bac7e578692f1148e3053efedd0f865cfb79323`.
  nvim-treesitter contributors, Apache-2.0; full license in
  [licenses/nvim-treesitter.txt](licenses/nvim-treesitter.txt).

Retain these notices and licenses when distributing the build. Updating either
dependency requires qualifying the parser and query together. To bundle another
language, pin its grammar source, embed compatible licensed queries, register
it in the existing language/default tables, and verify a standalone build with
empty configuration. Do not depend on the developer's editor installation.
