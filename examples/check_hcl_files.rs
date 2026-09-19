//! Parse tracked infrastructure as text and check effective token colors.
use anyhow::Context;
use red::{highlighter::Highlighter, theme::parse_vscode_theme_contents};

fn main() -> anyhow::Result<()> {
    let theme = parse_vscode_theme_contents(include_str!("../themes/mocha.json"))?;
    let mut highlighter = Highlighter::new(&theme)?;
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_hcl::LANGUAGE.into())?;
    let files = std::env::args().skip(1).collect::<Vec<_>>();
    anyhow::ensure!(!files.is_empty(), "supply tracked HCL files");
    for file in files {
        let source = std::fs::read_to_string(&file)?;
        anyhow::ensure!(
            highlighter.language_id_for_file(Some(&file)) == Some("hcl"),
            "{file}: detection"
        );
        let tree = parser.parse(&source, None).context("no tree")?;
        anyhow::ensure!(
            !tree.root_node().has_error(),
            "{file}: {}",
            tree.root_node().to_sexp()
        );
        let spans = highlighter.highlight("hcl", &source)?;
        let mut cursor = tree.walk();
        let mut checked = 0;
        loop {
            let node = cursor.node();
            let scope = match node.kind() {
                "numeric_lit" => Some("number"),
                "bool_lit" => Some("boolean"),
                "null_lit" => Some("constant"),
                "comment" => Some("comment"),
                "template_literal" => Some("string"),
                "identifier" => match node.parent().map(|n| n.kind()) {
                    Some("attribute") => Some("property"),
                    Some("function_call") => Some("function"),
                    _ => None,
                },
                _ => None,
            };
            if let Some(scope) = scope {
                let byte = node.start_byte();
                let actual = spans
                    .iter()
                    .rev()
                    .find(|s| s.start <= byte && byte < s.end)
                    .map(|s| &s.style);
                anyhow::ensure!(
                    actual == theme.get_style(scope).as_ref(),
                    "{file}: {} at {byte}: expected {scope}, got {actual:?}",
                    node.kind()
                );
                checked += 1;
            }
            if cursor.goto_first_child() {
                continue;
            }
            while !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    anyhow::ensure!(checked > 0, "no tokens checked: {file}");
                    println!("{file}: error-free, {checked} token colors checked");
                    break;
                }
            }
            if cursor.node() == tree.root_node() {
                break;
            }
        }
    }
    Ok(())
}
