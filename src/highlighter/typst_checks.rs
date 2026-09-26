//! Standalone checks of Typst syntax and the effective (shortest-range) styles.
use super::*;

pub(super) const SOURCE: &str = include_str!("../queries/highlights/smoke/example.typ");

pub(super) fn assert_clean(node: tree_sitter::Node<'_>) -> anyhow::Result<()> {
    anyhow::ensure!(
        !node.is_error()
            && !node.is_missing()
            && !node.kind().starts_with("malformed_")
            && !node.kind().starts_with("incomplete_"),
        "Typst recovery node {} at {:?}",
        node.kind(),
        node.range()
    );
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        assert_clean(child)?;
    }
    Ok(())
}

pub(super) fn token_style(
    theme: &Theme,
    source: &str,
    spans: &[StyleInfo],
    anchor: &str,
    token: &str,
    scope: &str,
) -> anyhow::Result<()> {
    let start = source.find(anchor).context("missing Typst anchor")?
        + anchor.find(token).context("missing Typst token")?;
    let expected = theme
        .get_style(scope)
        .with_context(|| format!("missing scope {scope}"))?;
    for byte in start..start + token.len() {
        // Same priority as the editor's StyleCursor: shortest range, last tie.
        let actual = spans
            .iter()
            .enumerate()
            .filter(|(_, s)| s.start <= byte && byte < s.end)
            .min_by_key(|(order, s)| (s.end - s.start, std::cmp::Reverse(*order)))
            .map(|(_, s)| &s.style);
        anyhow::ensure!(
            actual == Some(&expected),
            "Typst {token:?} at {byte}: expected {scope}, got {actual:?}"
        );
    }
    Ok(())
}

pub(super) fn check(theme: &Theme) -> anyhow::Result<()> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_typst::LANGUAGE.into())?;
    let tree = parser
        .parse(SOURCE, None)
        .context("Typst parse cancelled")?;
    assert_clean(tree.root_node())?;
    let mut highlighter = Highlighter::new(theme)?;
    anyhow::ensure!(
        highlighter.language_id_for_file(Some("example.TYP")) == Some("typst"),
        "Typst detection"
    );
    let styles = highlighter.highlight("typst", SOURCE)?;
    for (anchor, token, scope) in [
        (
            "// café: let set show are comments",
            "let set show",
            "comment",
        ),
        (
            "/* outer /* nested */ still comment */",
            "still comment",
            "comment",
        ),
        ("= Typst in RedVim", "Typst in RedVim", "heading.1.markdown"),
        ("== Code", "Code", "heading.2.markdown"),
        ("*strong text*", "strong text", "text.strong"),
        ("_emphasized text_", "emphasized text", "text.emphasis"),
        ("`raw let text`", "raw let text", "string"),
        ("https://typst.app", "https://typst.app", "text.uri"),
        ("@intro", "intro", "text.reference"),
        ("#import", "import", "keyword.import"),
        ("#let answer", "let", "keyword"),
        ("answer = 42", "42", "number"),
        ("enabled = true", "true", "boolean"),
        ("greeting(name)", "greeting", "function"),
        ("greeting(name)", "name", "variable.parameter"),
        ("title: \"café\"", "title", "property"),
        ("title: \"café\"", "café", "string"),
        ("#set text", "set", "keyword"),
        ("#show strong", "show", "keyword"),
        ("#greeting(\"world\")", "greeting", "function.call"),
        ("x_1^2", "^", "operator"),
        ("alpha / beta", "alpha", "constant"),
        ("alpha / beta", "/", "operator"),
        ("let total = 73", "73", "number"),
        (
            "let unparsed = \"plain raw text\"",
            "plain raw text",
            "string",
        ),
        ("*still strong*", "still strong", "text.strong"),
    ] {
        token_style(theme, SOURCE, &styles, anchor, token, scope)?;
    }
    Ok(())
}
