use super::*;
use crate::theme::parse_vscode_theme_contents;

#[test]
fn typst_standalone_colors_in_both_bundled_themes() {
    for source in [
        include_str!("../../themes/austin-night.json"),
        include_str!("../../themes/mocha.json"),
    ] {
        typst_checks::check(&parse_vscode_theme_contents(source).unwrap()).unwrap();
    }
}

#[test]
fn typst_pinned_grammar_loads_with_existing_runtime() {
    let language = tree_sitter_typst::LANGUAGE.into();
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();
    let tree = parser
        .parse("= Hello\n#let answer = 42\n$ x^2 $\n", None)
        .unwrap();
    assert!(
        !tree.root_node().has_error(),
        "{}",
        tree.root_node().to_sexp()
    );
    Query::new(&language, tree_sitter_typst::HIGHLIGHTS_QUERY).unwrap();
    Query::new(&language, tree_sitter_typst::INJECTIONS_QUERY).unwrap();
}

#[test]
fn typst_queries_use_supported_predicates_and_theme_scopes() {
    let language = tree_sitter_typst::LANGUAGE.into();
    let theme =
        parse_vscode_theme_contents(include_str!("../../themes/austin-night.json")).unwrap();
    for source in [
        include_str!("../queries/highlights/typst.scm"),
        include_str!("../queries/injections/typst.scm"),
    ] {
        let query = Query::new(&language, source).unwrap();
        for pattern in 0..query.pattern_count() {
            assert!(
                query.general_predicates(pattern).is_empty(),
                "unsupported predicate {pattern}"
            );
        }
        for name in query.capture_names() {
            if !name.starts_with('_') && !name.starts_with("injection.") {
                assert!(theme.get_style(name).is_some(), "unmapped capture {name}");
            }
        }
    }
}

#[test]
fn typst_detection_fences_and_raw_language_injections() {
    let theme =
        parse_vscode_theme_contents(include_str!("../../themes/austin-night.json")).unwrap();
    let mut highlighter = Highlighter::new(&theme).unwrap();
    for file in ["main.typ", "main.TYP"] {
        assert_eq!(highlighter.language_id_for_file(Some(file)), Some("typst"));
    }
    for alias in ["typ", "typst"] {
        assert_eq!(highlighter.language_id_for_name(alias), Some("typst"));
        let source = format!("```{alias}\n#let answer = 42\n```\n");
        for host in ["markdown", "typst"] {
            let styles = highlighter.highlight(host, &source).unwrap();
            typst_checks::token_style(&theme, &source, &styles, "42", "42", "number").unwrap();
        }
    }
    for alias in ["rust", "rs", "nu", "nushell"] {
        let source = format!("```{alias}\nlet answer = 42\n```\n");
        let styles = highlighter.highlight("typst", &source).unwrap();
        typst_checks::token_style(&theme, &source, &styles, "42", "42", "number").unwrap();
    }
    for alias in ["not-installed", "typc", "typm"] {
        let source = format!("```{alias}\nlet answer = 42\n```\n");
        let styles = highlighter.highlight("typst", &source).unwrap();
        typst_checks::token_style(
            &theme,
            &source,
            &styles,
            "let answer = 42",
            "let answer = 42",
            "string",
        )
        .unwrap();
    }
    let mut nested = "#let answer = 42\n".to_owned();
    for n in 0..MAX_INJECTION_DEPTH + 3 {
        let fence = "`".repeat(n + 3);
        nested = format!("{fence}typst\n{nested}{fence}\n");
    }
    let spans = highlighter.highlight("typst", &nested).unwrap();
    assert!(!spans.is_empty());
    assert!(spans.len() < 200);
}

#[test]
fn typst_incremental_edits_match_fresh_trees_and_styles() {
    let theme =
        parse_vscode_theme_contents(include_str!("../../themes/austin-night.json")).unwrap();
    let mut highlighter = Highlighter::new(&theme).unwrap();
    let original = typst_checks::SOURCE;
    let mut versions = vec![original.to_owned()];
    for (from, to) in [
        ("#let answer = 42", "#let answer = 123"),
        ("/* nested */", "/* nested"),
        ("*strong text*", "*strong café*"),
        ("_emphasized text_", "_unfinished"),
        ("$x_1^2 + alpha / beta$", "$x_1^2 + alpha / beta"),
        ("```rust", "``rust"),
        ("\"café\"", "\"café"),
        ("#greeting(\"world\")", "#greeting[world]"),
        ("#let values = (1, 2, 3)", "#let values = (1, 2,"),
    ] {
        versions.push(original.replace(from, to));
        versions.push(original.to_owned());
    }
    let shape = |spans: Vec<StyleInfo>| {
        spans
            .into_iter()
            .map(|s| (s.start, s.end, s.style))
            .collect::<Vec<_>>()
    };
    for source in versions {
        let actual = highlighter.highlight("typst", &source).unwrap();
        for span in &actual {
            assert!(
                span.start <= span.end
                    && source.is_char_boundary(span.start)
                    && source.is_char_boundary(span.end)
            );
        }
        let mut fresh = Highlighter::new(&theme).unwrap();
        assert_eq!(
            shape(actual),
            shape(fresh.highlight("typst", &source).unwrap())
        );
        let incremental = highlighter.highlighters["typst"]
            .cached_tree
            .as_ref()
            .unwrap();
        let rebuilt = fresh.highlighters["typst"].cached_tree.as_ref().unwrap();
        assert_eq!(
            incremental.tree.root_node().to_sexp(),
            rebuilt.tree.root_node().to_sexp()
        );
    }
    // Independent expected colors and clean complete-form tree after recovery.
    typst_checks::check(&theme).unwrap();
    let spans = highlighter.highlight("typst", original).unwrap();
    typst_checks::token_style(
        &theme,
        original,
        &spans,
        "*still strong*",
        "still strong",
        "text.strong",
    )
    .unwrap();
}

#[test]
fn typst_user_routing_and_theme_overrides_remain_authoritative() {
    let directory = tempfile::tempdir().unwrap();
    let config = HashMap::from([(
        "typst".to_owned(),
        LanguageConfig {
            extensions: vec!["document".into()],
            ..LanguageConfig::default()
        },
    )]);
    let registry = Arc::new(LanguageRegistry::from_config(&config, directory.path()).unwrap());
    let mut theme =
        parse_vscode_theme_contents(include_str!("../../themes/austin-night.json")).unwrap();
    let mut custom = theme.get_style("function").unwrap();
    custom.bold = !custom.bold;
    theme.token_styles.insert(
        0,
        crate::theme::TokenStyle {
            name: None,
            scope: vec!["function".into()],
            style: custom,
        },
    );
    let mut highlighter = Highlighter::with_registry(&theme, registry).unwrap();
    assert_eq!(
        highlighter.language_id_for_file(Some("main.document")),
        Some("typst")
    );
    let source = "#let greet(name) = [Hello #name]\n";
    let styles = highlighter.highlight("typst", source).unwrap();
    typst_checks::token_style(&theme, source, &styles, "greet", "greet", "function").unwrap();
}
