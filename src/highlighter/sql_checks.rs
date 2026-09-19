//! Dialect highlighting checks, distinct from the general grammar's parse validity.
use super::*;

pub(super) fn check(theme: &Theme) -> anyhow::Result<()> {
    let mut highlighter = Highlighter::new(theme)?;
    for (code, tokens) in [
        (
            include_str!("../queries/highlights/smoke/mysql.sql"),
            &[
                ("-- café", "comment"),
                ("CREATE", "keyword"),
                ("UNSIGNED", "attribute"),
                ("AUTO_INCREMENT", "attribute"),
                ("DATETIME", "type.builtin"),
                ("UNIQUE", "type.qualifier"),
                ("KEY", "keyword"),
                ("ENGINE", "attribute"),
                ("COLLATE", "attribute"),
                ("ALTER", "keyword"),
                ("SET", "keyword"),
                ("SELECT", "keyword"),
                ("PREPARE", "keyword"),
                ("EXECUTE", "keyword"),
                ("DEALLOCATE", "keyword"),
                ("'café'", "string"),
                ("42", "number"),
                ("UNIQUE(`name`)", "type.qualifier"),
            ][..],
        ),
        (
            include_str!("../queries/highlights/smoke/sqlite.sql"),
            &[
                ("/* café", "comment"),
                ("INTEGER", "type.builtin"),
                ("AUTOINCREMENT", "keyword"),
                ("CREATE INDEX", "keyword"),
                ("TRUE", "boolean"),
                ("INSERT", "keyword"),
                ("'it''s café'", "string"),
                ("select", "keyword"),
                ("JOIN", "keyword"),
                ("WHERE", "keyword"),
                ("UPDATE", "keyword"),
                ("DELETE", "keyword"),
                ("42", "number"),
            ][..],
        ),
    ] {
        let spans = highlighter.highlight("sql", code)?;
        for &(token, scope) in tokens {
            let byte = code.find(token).context("SQL fixture token missing")?;
            let expected = theme.get_style(scope).context("SQL theme scope missing")?;
            let actual = spans.iter().rev().find(|s| s.start <= byte && byte < s.end);
            anyhow::ensure!(
                actual.map(|s| &s.style) == Some(&expected),
                "SQL dialect token {token:?}: expected {scope}, got {actual:?}"
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::parse_vscode_theme_contents;

    #[test]
    fn sql_dialect_colors_and_recovery_are_checked() {
        check(&parse_vscode_theme_contents(include_str!("../../themes/mocha.json")).unwrap())
            .unwrap();
    }

    #[test]
    fn sql_configuration_fences_and_edits_preserve_highlighting() {
        let theme = parse_vscode_theme_contents(include_str!("../../themes/mocha.json")).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let config = HashMap::from([(
            "sql".to_owned(),
            LanguageConfig {
                extensions: vec!["query".into()],
                ..LanguageConfig::default()
            },
        )]);
        let registry = LanguageRegistry::from_config(&config, directory.path()).unwrap();
        let mut highlighter = Highlighter::with_registry(&theme, Arc::new(registry)).unwrap();
        assert_eq!(
            highlighter.language_id_for_file(Some("example.query")),
            Some("sql")
        );
        assert_eq!(highlighter.language_id_for_name("sql"), Some("sql"));
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 0);
        let fenced = "```sql\nSELECT 42;\n```\n";
        let spans = highlighter.highlight("markdown", fenced).unwrap();
        let byte = fenced.find("SELECT").unwrap();
        assert_eq!(
            spans
                .iter()
                .rev()
                .find(|s| s.start <= byte && byte < s.end)
                .map(|s| &s.style),
            theme.get_style("keyword").as_ref()
        );
        for code in [
            "SELECT 'café';",
            "SELECT '",
            "PREPARE x FROM @ddl;",
            "SELECT 'café';",
        ] {
            let spans = highlighter.highlight("sql", code).unwrap();
            let fresh = Highlighter::new(&theme)
                .unwrap()
                .highlight("sql", code)
                .unwrap();
            let shape = |spans: Vec<StyleInfo>| {
                spans
                    .into_iter()
                    .map(|s| (s.start, s.end, s.style))
                    .collect::<Vec<_>>()
            };
            assert_eq!(shape(spans), shape(fresh), "{code}");
        }
    }

    #[test]
    fn sql_recovery_never_colors_keywords_in_quotes_comments_or_identifiers() {
        let theme = parse_vscode_theme_contents(include_str!("../../themes/mocha.json")).unwrap();
        let code = "'PREPARE' \"DEALLOCATE\" `AUTOINCREMENT` 'it''s PREPARE' -- PREPARE\n/* DEALLOCATE */ caféPREPARE PREPARE_suffix prepare";
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_sequel::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(code, None).unwrap();
        let mut spans = Vec::new();
        sql_recovery::fill_gaps(code, tree.root_node(), &theme, &mut spans);
        assert_eq!(
            spans
                .iter()
                .map(|s| &code[s.start..s.end])
                .collect::<Vec<_>>(),
            ["prepare"]
        );
    }
}
