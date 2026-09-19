//! Official GPUWeb WGSL grammar, generated and adapted for RedVim.
use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_wgsl() -> *const ();
}

/// Statically linked WGSL parser/scanner. Input hashes and adaptations are in
/// RedVim's docs/languages-provenance.json.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_wgsl) };
