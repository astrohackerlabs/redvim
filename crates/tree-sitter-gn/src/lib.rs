//! GN 1.0.0 native parser with LanguageFn bindings for RedVim's Tree-sitter runtime.
use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_gn() -> *const ();
}

/// Statically linked GN grammar. Source and scanner adaptation provenance lives
/// in RedVim's docs/languages-provenance.json.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_gn) };
