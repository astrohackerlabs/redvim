//! Validate AppleScript syntax/colors without executing or reloading configuration.
use anyhow::Context;
use red::{highlighter::Highlighter, theme::parse_vscode_theme_contents};
use std::collections::BTreeMap;

fn main() -> anyhow::Result<()> {
    let theme = parse_vscode_theme_contents(include_str!("../themes/mocha.json"))?;
    let mut highlighter = Highlighter::new(&theme)?;
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_applescript::LANGUAGE.into())?;
    let files = std::env::args().skip(1).collect::<Vec<_>>();
    anyhow::ensure!(!files.is_empty(), "supply tracked AppleScripts");
    for file in files {
        let source = std::fs::read_to_string(&file)?;
        anyhow::ensure!(
            highlighter.language_id_for_file(Some(&file)) == Some("applescript"),
            "wrong detection: {file}"
        );
        let tree = parser.parse(&source, None).context("no tree")?;
        anyhow::ensure!(
            !tree.root_node().has_error(),
            "{file}: {}",
            tree.root_node().to_sexp()
        );
        let spans = highlighter.highlight("applescript", &source)?;
        let mut cursor = tree.walk();
        let mut checked = BTreeMap::<&str, usize>::new();
        loop {
            let node = cursor.node();
            let scope = match node.kind() {
                "comment" | "block_comment" => Some("comment"),
                "command_name"
                    if node
                        .parent()
                        .is_some_and(|p| p.kind() == "handler_definition") =>
                {
                    Some("function")
                }
                "command_name" => Some("function.builtin"),
                "identifier"
                    if node.parent().is_some_and(|p| {
                        p.kind() == "error_parameters" || p.kind() == "handler_definition"
                    }) =>
                {
                    Some("variable.parameter")
                }
                "parameter_name" => Some("variable.parameter"),
                "string" => Some("string"),
                "number" => Some("number"),
                "boolean" => Some("boolean"),
                "comparison_operator" | "&" => Some("operator"),
                kind if kind.starts_with("keyword_") => Some("keyword"),
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
            "function",
            "function.builtin",
            "variable.parameter",
            "string",
            "number",
            "boolean",
            "operator",
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
