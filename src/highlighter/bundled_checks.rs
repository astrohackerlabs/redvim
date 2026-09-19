//! Release checks exercise the same registry, queries and theme as normal rendering.
use super::*;

struct Fixture {
    language: &'static str,
    filename: &'static str,
    source: &'static str,
    tokens: &'static [(&'static str, &'static str)],
}

const FIXTURES: &[Fixture] = &[
    Fixture {
        language: "c",
        filename: "example.c",
        source: include_str!("../queries/highlights/smoke/example.c"),
        tokens: &[
            ("// café", "comment"),
            ("return", "keyword"),
            ("\"hello\"", "string"),
            ("42", "number"),
        ],
    },
    Fixture {
        language: "cpp",
        filename: "example.cpp",
        source: include_str!("../queries/highlights/smoke/example.cpp"),
        tokens: &[
            ("// café", "comment"),
            ("template", "keyword"),
            ("\"hello\"", "string"),
            ("21", "number"),
        ],
    },
    Fixture {
        language: "python",
        filename: "example.py",
        source: include_str!("../queries/highlights/smoke/example.py"),
        tokens: &[
            ("# café", "comment"),
            ("def", "keyword"),
            ("greet", "function"),
            ("\"world\"", "string"),
        ],
    },
    Fixture {
        language: "html",
        filename: "example.html",
        source: include_str!("../queries/highlights/smoke/example.html"),
        tokens: &[
            ("<!-- café -->", "comment"),
            ("h1", "tag"),
            ("class", "attribute"),
            ("greeting", "string"),
        ],
    },
    Fixture {
        language: "css",
        filename: "example.css",
        source: include_str!("../queries/highlights/smoke/example.css"),
        tokens: &[
            ("/* café */", "comment"),
            ("@media", "keyword"),
            ("\"hello\"", "string"),
            ("42", "number"),
        ],
    },
    Fixture {
        language: "ruby",
        filename: "example.rb",
        source: include_str!("../queries/highlights/smoke/example.rb"),
        tokens: &[
            ("# café", "comment"),
            ("def", "keyword"),
            ("greet", "function.method"),
            ("hello", "string"),
            ("message", "variable"),
        ],
    },
    Fixture {
        language: "zig",
        filename: "example.zig",
        source: include_str!("../queries/highlights/smoke/example.zig"),
        tokens: &[
            ("// café", "comment"),
            ("pub", "keyword"),
            ("main", "function"),
            ("hello", "string"),
            ("42", "number"),
            ("count", "variable"),
        ],
    },
    Fixture {
        language: "swift",
        filename: "example.swift",
        source: include_str!("../queries/highlights/smoke/example.swift"),
        tokens: &[
            ("// café", "comment"),
            ("let", "keyword"),
            ("greet()", "function.method"),
            ("hello", "string"),
        ],
    },
    Fixture {
        language: "xml",
        filename: "example.xml",
        source: include_str!("../queries/highlights/smoke/example.xml"),
        tokens: &[
            ("<!-- café -->", "comment"),
            ("greeting", "tag"),
            ("world", "string"),
        ],
    },
    Fixture {
        language: "xml",
        filename: "example.svg",
        source: include_str!("../queries/highlights/smoke/example.svg"),
        tokens: &[
            ("<!-- café -->", "comment"),
            ("circle", "tag"),
            ("cx", "property"),
            ("red", "string"),
        ],
    },
    Fixture {
        language: "make",
        filename: "Makefile",
        source: include_str!("../queries/highlights/smoke/Makefile"),
        tokens: &[
            ("# café", "comment"),
            ("GREETING", "constant"),
            (":=", "operator"),
            ("hello", "string"),
        ],
    },
    Fixture {
        language: "nu",
        filename: "example.nu",
        source: include_str!("../queries/highlights/nu-smoke.nu"),
        tokens: &[
            ("def", "keyword.function"),
            ("# café", "comment"),
            ("Hello", "string"),
        ],
    },
];

/// Validate bundled parsers, filename detection and effective token colors without
/// reading user configuration, loading external grammars or entering raw mode.
pub fn check_bundled_languages(theme: &Theme) -> anyhow::Result<Vec<&'static str>> {
    let registry = Arc::new(LanguageRegistry::bundled());
    let mut highlighter = Highlighter::with_registry(theme, Arc::clone(&registry))?;
    let mut checked = Vec::new();
    for fixture in FIXTURES {
        let id = fixture.language;
        anyhow::ensure!(
            highlighter.language_id_for_file(Some(fixture.filename)) == Some(id),
            "{}: wrong bundled language detection",
            fixture.filename
        );
        let Some(GrammarSource::Bundled(grammar)) = registry.languages[id].grammar else {
            anyhow::bail!("{id}: parser is not bundled");
        };
        let mut parser = Parser::new();
        parser.set_language(&grammar())?;
        let tree = parser
            .parse(fixture.source, None)
            .context("parser returned no tree")?;
        anyhow::ensure!(
            !tree.root_node().has_error(),
            "{id}: fixture parse error: {}",
            tree.root_node().to_sexp()
        );
        let styles = highlighter
            .highlight(id, fixture.source)
            .with_context(|| format!("{id}: bundled highlighting"))?;
        for &(token, scope) in fixture.tokens {
            let byte = fixture
                .source
                .find(token)
                .context("missing fixture token")?;
            let expected = theme
                .get_style(scope)
                .with_context(|| format!("{id}: missing theme scope {scope}"))?;
            let actual = styles
                .iter()
                .rev()
                .find(|s| s.start <= byte && byte < s.end)
                .map(|s| &s.style);
            anyhow::ensure!(actual == Some(&expected), "{id}: {token:?} should have {scope} style; actual={actual:?}, expected={expected:?}");
            if scope != "variable" {
                anyhow::ensure!(
                    expected.fg.is_some() && expected.fg != theme.style.fg,
                    "{id}: {scope} must have visible color"
                );
            }
        }
        if !checked.contains(&id) {
            checked.push(id);
        }
    }
    Ok(checked)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::parse_vscode_theme_contents;

    #[test]
    fn common_languages_parse_and_render_with_embedded_mocha() {
        let theme = parse_vscode_theme_contents(include_str!("../../themes/mocha.json")).unwrap();
        let checked = check_bundled_languages(&theme).unwrap();
        assert_eq!(checked.len(), 11);
    }

    #[test]
    fn common_language_detection_and_overrides_are_deterministic() {
        let directory = tempfile::tempdir().unwrap();
        let mut registry =
            LanguageRegistry::from_config(&HashMap::new(), directory.path()).unwrap();
        let theme = Theme::default();
        let highlighter = Highlighter::with_registry(&theme, Arc::new(registry.clone())).unwrap();
        for definition in language_definitions().iter().take(10) {
            for extension in definition.extensions {
                assert_eq!(
                    highlighter.language_id_for_file(Some(&format!("example.{extension}"))),
                    Some(definition.id)
                );
            }
            for filename in definition.filenames {
                assert_eq!(
                    highlighter.language_id_for_file(Some(filename)),
                    Some(definition.id)
                );
            }
        }
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 0);
        // Explicit C++ header routing remains available through supported config.
        registry
            .insert_configured(
                "cpp",
                &LanguageConfig {
                    extensions: vec!["h".into()],
                    ..LanguageConfig::default()
                },
                directory.path(),
            )
            .unwrap();
        let highlighter = Highlighter::with_registry(&theme, Arc::new(registry)).unwrap();
        assert_eq!(
            highlighter.language_id_for_file(Some("example.h")),
            Some("cpp")
        );
        assert_eq!(
            highlighter.language_id_for_file(Some("example.c")),
            Some("c")
        );
    }

    #[test]
    fn configured_builtin_alias_preserves_semantic_colors() {
        let directory = tempfile::tempdir().unwrap();
        let config = HashMap::from([(
            "custom-python".to_owned(),
            LanguageConfig {
                grammar: Some(LanguageGrammarConfig {
                    builtin: Some("python".into()),
                    ..LanguageGrammarConfig::default()
                }),
                ..LanguageConfig::default()
            },
        )]);
        let registry = LanguageRegistry::from_config(&config, directory.path()).unwrap();
        let theme = parse_vscode_theme_contents(include_str!("../../themes/mocha.json")).unwrap();
        let mut highlighter = Highlighter::with_registry(&theme, Arc::new(registry)).unwrap();
        let code = "def greet():\n    return 42\n";
        let shape = |styles: Vec<StyleInfo>| {
            styles
                .into_iter()
                .map(|s| (s.start, s.end, s.style))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            shape(highlighter.highlight("python", code).unwrap()),
            shape(highlighter.highlight("custom-python", code).unwrap())
        );
    }
}
