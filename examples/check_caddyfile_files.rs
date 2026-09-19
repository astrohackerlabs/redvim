//! Validate Caddyfile syntax/colors without executing or reloading configuration.
use anyhow::Context;
use red::{highlighter::Highlighter, theme::parse_vscode_theme_contents};
use std::collections::BTreeMap;

fn main() -> anyhow::Result<()> {
    let theme = parse_vscode_theme_contents(include_str!("../themes/mocha.json"))?;
    let mut highlighter = Highlighter::new(&theme)?;
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_caddyfile::LANGUAGE.into())?;
    let files = std::env::args().skip(1).collect::<Vec<_>>();
    anyhow::ensure!(!files.is_empty(), "supply tracked Caddyfiles");
    for file in files {
        let source = std::fs::read_to_string(&file)?;
        anyhow::ensure!(
            highlighter.language_id_for_file(Some(&file)) == Some("caddyfile"),
            "wrong detection: {file}"
        );
        let tree = parser.parse(&source, None).context("no tree")?;
        anyhow::ensure!(
            !tree.root_node().has_error(),
            "{file}: {}",
            tree.root_node().to_sexp()
        );
        let spans = highlighter.highlight("caddyfile", &source)?;
        let mut cursor = tree.walk();
        let mut checked = BTreeMap::<&str, usize>::new();
        loop {
            let node = cursor.node();
            let scope = match node.kind() {
                "comment" => Some("comment"),
                "site_address" => Some("keyword"),
                "directive_name" => Some("property"),
                "network_address" | "path" | "path_matcher" => Some("type"),
                "argument" => Some("string"),
                "matcher_identifier" => Some("function.macro"),
                "matcher_directive_name" => Some("function.method"),
                _ => None,
            };
            if let Some(scope) = scope {
                let expected = theme.get_style(scope).context("missing scope")?;
                anyhow::ensure!(
                    expected.fg.is_some() && expected.fg != theme.style.fg,
                    "invisible {scope}"
                );
                for byte in node.start_byte()..node.end_byte() {
                    let actual = spans
                        .iter()
                        .rev()
                        .find(|s| s.start <= byte && byte < s.end)
                        .map(|s| &s.style);
                    anyhow::ensure!(
                        actual == Some(&expected),
                        "{file}: {} {:?} at {byte}: expected {scope}, got {actual:?}",
                        node.kind(),
                        node.utf8_text(source.as_bytes())?
                    );
                }
                *checked.entry(scope).or_default() += 1;
            }
            if cursor.goto_first_child() {
                continue;
            }
            while !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    break;
                }
            }
            if cursor.node() == tree.root_node() {
                break;
            }
        }
        for scope in [
            "comment",
            "keyword",
            "property",
            "type",
            "function.macro",
            "function.method",
        ] {
            anyhow::ensure!(
                checked.get(scope).copied().unwrap_or_default() > 0,
                "{file}: missing {scope} coverage"
            );
        }
        println!(
            "{file}: error-free, {} whole-token checks: {checked:?}",
            checked.values().sum::<usize>()
        );
    }
    Ok(())
}
