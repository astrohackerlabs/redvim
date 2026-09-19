//! Check the repository's WebKit Objective-C proof as text, without executing it.
use anyhow::Context;
use red::{highlighter::Highlighter, theme::parse_vscode_theme_contents};

fn main() -> anyhow::Result<()> {
    let files = std::env::args().skip(1).collect::<Vec<_>>();
    anyhow::ensure!(!files.is_empty(), "supply WebKitHostingProof.m");
    let theme = parse_vscode_theme_contents(include_str!("../themes/mocha.json"))?;
    let mut highlighter = Highlighter::new(&theme)?;
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_objc::LANGUAGE.into())?;
    for file in files {
        let source = std::fs::read_to_string(&file)?;
        anyhow::ensure!(
            highlighter.language_id_for_file(Some(&file)) == Some("objc"),
            "wrong detection"
        );
        let tree = parser.parse(&source, None).context("no tree")?;
        anyhow::ensure!(
            !tree.root_node().has_error(),
            "{file}: {}",
            tree.root_node().to_sexp()
        );
        let spans = highlighter.highlight("objc", &source)?;
        let mut checked = 0;
        for (token, scope) in [
            ("#import", "keyword.directive"),
            ("CAContext", "type"),
            ("remoteContextWithOptions", "function.method"),
            ("contextId", "property"),
            ("NSObject", "type"),
            ("proofDirectory", "function"),
            ("test-content/index.html", "string"),
            ("return", "keyword"),
            ("NSMakeRect", "function"),
            (
                "applicationShouldTerminateAfterLastWindowClosed",
                "function.method",
            ),
            ("strtoul", "function"),
            ("NULL", "constant"),
            ("--stress", "string"),
            ("NSLog", "function"),
            ("420", "number"),
        ] {
            let start = source
                .find(token)
                .with_context(|| format!("missing {token}"))?;
            let expected = theme.get_style(scope).context("missing scope")?;
            for byte in start..start + token.len() {
                let actual = spans
                    .iter()
                    .rev()
                    .find(|s| s.start <= byte && byte < s.end)
                    .map(|s| &s.style);
                anyhow::ensure!(
                    actual == Some(&expected),
                    "{token} byte {byte}: expected {scope}, got {actual:?}"
                );
            }
            checked += 1;
        }
        println!("{file}: error-free, {checked} exact token-color checks; no comments present");
    }
    Ok(())
}
