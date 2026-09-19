//! Validate tracked GN build configuration as text, without running GN or Ninja.
use anyhow::Context;
use red::{highlighter::Highlighter, theme::parse_vscode_theme_contents};

fn main() -> anyhow::Result<()> {
    let theme = parse_vscode_theme_contents(include_str!("../themes/mocha.json"))?;
    let mut highlighter = Highlighter::new(&theme)?;
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_gn::LANGUAGE.into())?;
    let files = std::env::args().skip(1).collect::<Vec<_>>();
    anyhow::ensure!(!files.is_empty(), "supply tracked GN files");
    for file in files {
        let source = std::fs::read_to_string(&file)?;
        anyhow::ensure!(
            highlighter.language_id_for_file(Some(&file)) == Some("gn"),
            "wrong detection: {file}"
        );
        let tree = parser.parse(&source, None).context("no tree")?;
        anyhow::ensure!(
            !tree.root_node().has_error(),
            "{file}: {}",
            tree.root_node().to_sexp()
        );
        let spans = highlighter.highlight("gn", &source)?;
        let mut cursor = tree.walk();
        let mut checked = 0;
        loop {
            let node = cursor.node();
            let scope = match node.kind() {
                "comment" => Some("comment"),
                "boolean" => Some("boolean"),
                "integer" => Some("number"),
                "string" => Some("string"),
                "=" | "+=" | "-=" => Some("operator"),
                "identifier"
                    if node.parent().is_some_and(|p| {
                        p.kind() == "assignment_statement" && p.child(0) == Some(node)
                    }) =>
                {
                    Some("property")
                }
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
                        "{file}: {} at {byte}: expected {scope}, got {actual:?}",
                        node.kind()
                    );
                }
                checked += 1;
            }
            if cursor.goto_first_child() {
                continue;
            }
            while !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    anyhow::ensure!(checked > 0, "no colors checked");
                    println!("{file}: error-free, {checked} whole-token colors checked");
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
