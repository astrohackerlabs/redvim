//! Parse real Protocol Buffers schemas and assert token colors without protoc.
use anyhow::Context;
use red::{highlighter::Highlighter, theme::parse_vscode_theme_contents};

fn main() -> anyhow::Result<()> {
    let theme = parse_vscode_theme_contents(include_str!("../themes/mocha.json"))?;
    let mut highlighter = Highlighter::new(&theme)?;
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_proto::LANGUAGE.into())?;
    let files = std::env::args().skip(1).collect::<Vec<_>>();
    anyhow::ensure!(!files.is_empty(), "supply tracked .proto schemas");
    for file in files {
        let source = std::fs::read_to_string(&file)?;
        anyhow::ensure!(
            highlighter.language_id_for_file(Some(&file)) == Some("proto"),
            "wrong detection: {file}"
        );
        let tree = parser.parse(&source, None).context("no tree")?;
        anyhow::ensure!(
            !tree.root_node().has_error(),
            "{file}: {}",
            tree.root_node().to_sexp()
        );
        let spans = highlighter.highlight("proto", &source)?;
        let mut cursor = tree.walk();
        let mut checked = 0;
        loop {
            let node = cursor.node();
            let scope = match node.kind() {
                "int_lit" => Some("number"),
                "float_lit" => Some("number.float"),
                "true" | "false" => Some("boolean"),
                "comment" => Some("comment"),
                "message_name" | "enum_name" | "service_name" | "message_or_enum_type" => {
                    Some("type")
                }
                "type"
                    if node
                        .named_child(0)
                        .is_some_and(|child| child.kind() == "message_or_enum_type") =>
                {
                    Some("type")
                }
                "type" | "key_type" => Some("type.builtin"),
                "rpc_name" => Some("function.method"),
                "string" if node.is_named() => Some("string"),
                "identifier" => match node.parent().map(|n| n.kind()) {
                    Some("field" | "map_field" | "oneof_field" | "field_option") => {
                        Some("property")
                    }
                    Some("enum_field") => Some("constant"),
                    Some("oneof") => Some("type"),
                    _ => None,
                },
                "syntax" if !node.is_named() => Some("keyword.directive"),
                "message" | "oneof" | "service" | "enum" if !node.is_named() => {
                    Some("keyword.type")
                }
                "stream" | "repeated" | "optional" => Some("keyword.modifier"),
                "rpc" if !node.is_named() => Some("keyword.function"),
                "returns" => Some("keyword.return"),
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
