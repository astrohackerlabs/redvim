//! Regression contract for the embedded product default, independent of Mocha fixtures.
use super::*;
use crate::{assets, config::Config, highlighter::Highlighter};

fn theme() -> Theme {
    parse_vscode_theme_contents(assets::bundled_theme(assets::DEFAULT_THEME_FILENAME).unwrap())
        .unwrap()
}

fn rgb(hex: &str) -> Color {
    crate::color::parse_rgb(hex).unwrap()
}

#[test]
fn default_config_and_recovery_select_austin_night() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    let missing = Config::load_user_file(&path, &[]).unwrap();
    assert_eq!(missing.config.theme, assets::DEFAULT_THEME_FILENAME);
    for source in [
        "",
        "relative_line_numbers = true",
        &assets::starter_config(),
        "invalid = [",
    ] {
        std::fs::write(&path, source).unwrap();
        let loaded = Config::load_user_file(&path, &[]).unwrap();
        assert_eq!(loaded.config.theme, assets::DEFAULT_THEME_FILENAME);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), source);
        assert_eq!(loaded.diagnostics.is_empty(), source != "invalid = [");
    }
    for name in ["red.json", "mocha.json", "tokyo-night.json"] {
        let source = format!("theme = {name:?}");
        let loaded = Config::load_user_toml(&source, &path, &[]).unwrap();
        assert_eq!(loaded.config.theme, name);
        let selected = parse_vscode_theme_contents(assets::bundled_theme(name).unwrap()).unwrap();
        assert_ne!(selected.style.bg, theme().style.bg);
    }
}

#[test]
fn palette_scope_coverage_and_effective_contrast() {
    let theme = theme();
    assert_eq!(theme.name, "Austin Night");
    assert_eq!(theme.style.bg, Some(rgb("#111219")));
    assert_eq!(theme.style.fg, Some(rgb("#c0caf5")));
    let palette = [
        "#111219", "#060608", "#292e42", "#c0caf5", "#a9b1d6", "#9aa5ce", "#3b4261", "#7aa2f7",
        "#bb9af7", "#7dcfff", "#9ece6a", "#e8a84a", "#f7768e", "#33467c", "#15161e", "#414868",
    ]
    .map(rgb);
    for color in theme.colors.values().copied().chain(
        theme
            .token_styles
            .iter()
            .flat_map(|s| [s.style.fg, s.style.bg].into_iter().flatten()),
    ) {
        let opaque = match color {
            Color::Rgba { r, g, b, a: 128 } => Color::Rgb { r, g, b },
            _ => color,
        };
        assert!(palette.contains(&opaque), "off-palette {color:?}");
    }
    let base = parse_vscode_theme_contents(include_str!("../../themes/tokyo-night.json")).unwrap();
    assert_eq!(base.token_styles.len(), 114);
    for (a, b) in base
        .token_styles
        .iter()
        .zip(theme.token_styles.iter().skip(9))
    {
        assert_eq!(a.scope, b.scope);
        assert_eq!(
            (a.style.bold, a.style.italic, a.style.underline),
            (b.style.bold, b.style.italic, b.style.underline)
        );
    }
    for (scope, color) in [
        ("comment", "#9aa5ce"),
        ("keyword", "#bb9af7"),
        ("function", "#7aa2f7"),
        ("type", "#7dcfff"),
        ("operator", "#7dcfff"),
        ("number", "#e8a84a"),
        ("string", "#9ece6a"),
        ("variable.parameter", "#e8a84a"),
    ] {
        let style = theme.get_style(scope).unwrap();
        assert_eq!(style.fg, Some(rgb(color)), "{scope}");
        for bg in [
            theme.style.bg.unwrap(),
            theme.line_highlight_style.as_ref().unwrap().bg.unwrap(),
        ] {
            assert!(
                contrast_ratio(style.fg.unwrap(), bg) >= 4.5,
                "{scope} on {bg:?}"
            );
        }
    }
    for style in [
        &theme.style,
        &theme.gutter_style,
        &theme.ui_style.popup,
        &theme.ui_style.dialog,
        &theme.ui_style.muted,
        &theme.ui_style.picker_selected_item,
        &theme.statusline_style.inner_style,
        &theme.statusline_style.outer_style,
        theme.selection_style.as_ref().unwrap(),
    ] {
        assert!(
            contrast_ratio(style.fg.unwrap(), style.bg.unwrap()) >= 4.5,
            "{style:?}"
        );
    }
    for bg in [
        theme.find_match_style.as_ref().unwrap().bg.unwrap(),
        theme
            .find_match_highlight_style
            .as_ref()
            .unwrap()
            .bg
            .unwrap(),
    ] {
        // Search overlays replace only the background; they do not adjust text.
        let rendered_bg = blend_color(bg, theme.style.bg.unwrap());
        for scope in [
            "comment", "keyword", "function", "type", "number", "string", "variable",
        ] {
            let fg = theme.get_style(scope).unwrap().fg.unwrap();
            assert!(
                contrast_ratio(fg, rendered_bg) >= 4.5,
                "search {scope}: {rendered_bg:?}"
            );
        }
    }
    assert!(
        contrast_ratio(
            theme.colors["breadcrumb.foreground"],
            theme.colors["breadcrumb.background"]
        ) >= 4.5
    );
}

#[test]
fn actual_language_content_uses_austin_night_colors() {
    let theme = theme();
    let mut highlighter = Highlighter::new(&theme).unwrap();
    for (language, source, tokens) in [
        (
            "nu",
            "# Austin\nlet count = 42\nprint \"hello\"\n",
            vec![
                ("# Austin", "#9aa5ce"),
                ("let", "#bb9af7"),
                ("42", "#e8a84a"),
                ("hello", "#9ece6a"),
            ],
        ),
        (
            "sql",
            "-- Austin\nSELECT 42, 'hello' FROM users;\n",
            vec![
                ("-- Austin", "#9aa5ce"),
                ("SELECT", "#bb9af7"),
                ("42", "#e8a84a"),
                ("hello", "#9ece6a"),
            ],
        ),
        (
            "rust",
            "// Austin\nfn answer() -> u32 { 42 }\n",
            vec![
                ("// Austin", "#9aa5ce"),
                ("fn", "#bb9af7"),
                ("answer", "#7aa2f7"),
                ("u32", "#7dcfff"),
                ("42", "#e8a84a"),
            ],
        ),
        (
            "c",
            "// Austin\nint answer(void) { return 42; }\n",
            vec![
                ("// Austin", "#9aa5ce"),
                ("int", "#7dcfff"),
                ("answer", "#7aa2f7"),
                ("return", "#bb9af7"),
                ("42", "#e8a84a"),
            ],
        ),
        (
            "wgsl",
            "// Austin\nfn answer() -> u32 { return 42u; }\n",
            vec![
                ("// Austin", "#9aa5ce"),
                ("fn", "#bb9af7"),
                ("answer", "#7aa2f7"),
                ("u32", "#7dcfff"),
                ("42u", "#e8a84a"),
            ],
        ),
    ] {
        let spans = highlighter.highlight(language, source).unwrap();
        for (token, color) in tokens {
            let byte = source.find(token).unwrap();
            let actual = spans
                .iter()
                .rev()
                .find(|s| s.start <= byte && byte < s.end)
                .and_then(|s| s.style.fg);
            assert_eq!(actual, Some(rgb(color)), "{language}: {token}");
        }
    }
}
