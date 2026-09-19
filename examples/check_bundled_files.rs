//! Developer probe: render real source files using only bundled grammars/themes.
use anyhow::Context;
use red::{highlighter::Highlighter, theme::parse_vscode_theme_contents};

fn main() -> anyhow::Result<()> {
    let theme = parse_vscode_theme_contents(include_str!("../themes/mocha.json"))?;
    let mut highlighter = Highlighter::new(&theme)?;
    let files = std::env::args().skip(1).collect::<Vec<_>>();
    anyhow::ensure!(!files.is_empty(), "supply source files to check");
    for file in files {
        let source = std::fs::read_to_string(&file).with_context(|| file.clone())?;
        let language = highlighter
            .language_id_for_file(Some(&file))
            .with_context(|| format!("no bundled language for {file}"))?
            .to_owned();
        let spans = highlighter.highlight(&language, &source)?;
        anyhow::ensure!(
            spans
                .iter()
                .any(|span| span.style.fg.is_some() && span.style.fg != theme.style.fg),
            "{file}: no visible syntax colors"
        );
        println!("{language}: {} spans: {file}", spans.len());
    }
    Ok(())
}
