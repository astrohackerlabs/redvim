//! Inspect SQL recovery nodes and actual rendered colors without executing SQL.
use red::{highlighter::Highlighter, theme::parse_vscode_theme_contents};

fn errors(node: tree_sitter::Node<'_>, source: &str) {
    if node.is_error() || node.is_missing() {
        println!(
            "  recovery {:?}: {:?}: {}",
            node.range(),
            &source[node.byte_range()],
            node.to_sexp()
        );
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        errors(child, source);
    }
}

fn main() -> anyhow::Result<()> {
    let theme = parse_vscode_theme_contents(include_str!("../themes/mocha.json"))?;
    let mut highlighter = Highlighter::new(&theme)?;
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_sequel::LANGUAGE.into())?;
    for file in std::env::args().skip(1) {
        let source = std::fs::read_to_string(&file)?;
        let tree = parser.parse(&source, None).unwrap();
        println!("{file}: parse_errors={}", tree.root_node().has_error());
        errors(tree.root_node(), &source);
        let spans = highlighter.highlight("sql", &source)?;
        let style_at = |byte| {
            spans
                .iter()
                .rev()
                .find(|s| s.start <= byte && byte < s.end)
                .map(|s| &s.style)
        };
        let mut checked = 0;
        for (token, scope) in [
            ("CREATE", "keyword"),
            ("SELECT", "keyword"),
            ("ALTER", "keyword"),
            ("PREPARE", "keyword"),
            ("EXECUTE", "keyword"),
            ("DEALLOCATE", "keyword"),
            ("AUTOINCREMENT", "keyword"),
            ("UNSIGNED", "attribute"),
            ("AUTO_INCREMENT", "attribute"),
            ("DATETIME", "type.builtin"),
            ("ENGINE", "attribute"),
            ("COLLATE", "attribute"),
            ("UNIQUE", "type.qualifier"),
            ("KEY", "keyword"),
            ("DEFAULT", "attribute"),
            ("BOOLEAN", "type.builtin"),
        ] {
            for (byte, _) in source.match_indices(token) {
                let word = |c: char| c.is_alphanumeric() || c == '_';
                if source[..byte].chars().next_back().is_some_and(word)
                    || source[byte + token.len()..]
                        .chars()
                        .next()
                        .is_some_and(word)
                {
                    continue;
                }
                let actual = style_at(byte);
                if actual == theme.get_style("comment").as_ref()
                    || actual == theme.get_style("string").as_ref()
                {
                    continue;
                }
                anyhow::ensure!(
                    actual == theme.get_style(scope).as_ref(),
                    "{file}: {token} at {byte} expected {scope}, got {actual:?}"
                );
                checked += 1;
            }
        }
        anyhow::ensure!(checked > 0, "no SQL tokens checked: {file}");
        println!(
            "  verified {checked} dialect keyword/type colors in {} spans",
            spans.len()
        );
    }
    Ok(())
}
