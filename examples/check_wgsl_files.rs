//! Check shader syntax and visible token colors without dispatching GPU work.
use anyhow::Context;
use red::{highlighter::Highlighter, theme::parse_vscode_theme_contents};
use std::collections::BTreeMap;

fn main() -> anyhow::Result<()> {
    let theme = parse_vscode_theme_contents(include_str!("../themes/mocha.json"))?;
    let mut highlighter = Highlighter::new(&theme)?;
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_wgsl::LANGUAGE.into())?;
    let builtins = regex::Regex::new("^(bool|i32|u32|f32|f16|vec[234][fhiu]?|mat[234]x[234][fh]?|array|atomic|ptr|sampler|sampler_comparison|texture_[a-z0-9_]+)$")?;
    let files = std::env::args().skip(1).collect::<Vec<_>>();
    anyhow::ensure!(!files.is_empty(), "supply tracked WGSL shaders");
    for file in files {
        let source = std::fs::read_to_string(&file)?;
        anyhow::ensure!(
            highlighter.language_id_for_file(Some(&file)) == Some("wgsl"),
            "wrong detection: {file}"
        );
        let tree = parser.parse(&source, None).context("no tree")?;
        anyhow::ensure!(
            !tree.root_node().has_error(),
            "{file}: {}",
            tree.root_node().to_sexp()
        );
        let spans = highlighter.highlight("wgsl", &source)?;
        let mut cursor = tree.walk();
        let mut checked = BTreeMap::<&str, usize>::new();
        loop {
            let node = cursor.node();
            let scope = match node.kind() {
                "line_comment" | "block_comment" => Some("comment"),
                "int_literal" => Some("number"),
                "float_literal" => Some("float"),
                "bool_literal" | "builtin_value_name" => Some("constant.builtin"),
                "member_ident" | "swizzle_name" => Some("property"),
                "ident" if builtins.is_match(node.utf8_text(source.as_bytes())?) => {
                    Some("type.builtin")
                }
                "ident" => match node.parent().map(|p| p.kind()) {
                    Some("function_header") => Some("function"),
                    Some("struct_decl" | "type_alias_decl") => Some("type"),
                    Some("template_elaborated_ident") => {
                        match node.parent().and_then(|p| p.parent()).map(|p| p.kind()) {
                            Some("call_phrase") => Some("function"),
                            Some("type_specifier" | "function_header") => Some("type"),
                            _ => None,
                        }
                    }
                    _ => None,
                },
                "@" | "compute" | "workgroup_size" | "binding" | "group" | "builtin" => {
                    Some("keyword.directive")
                }
                "const" | "let" | "var" | "fn" | "struct" | "if" | "else" | "for" | "while"
                | "return" => Some("keyword"),
                "less_than" | "less_than_equal" | "greater_than" | "greater_than_equal"
                | "shift_left" | "shift_right" | "shift_left_assign" | "shift_right_assign"
                | "+" | "-" | "*" | "/" | "%" | "=" | "+=" | "^" | "&" | "|" | "==" | "!=" => {
                    Some("operator")
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
            "number",
            "function",
            "type.builtin",
            "keyword.directive",
            "keyword",
            "operator",
        ] {
            anyhow::ensure!(
                checked.get(scope).copied().unwrap_or_default() > 0,
                "{file}: missing corpus coverage for {scope}"
            );
        }
        println!(
            "{file}: error-free, {} whole-token checks: {checked:?}",
            checked.values().sum::<usize>()
        );
    }
    Ok(())
}
