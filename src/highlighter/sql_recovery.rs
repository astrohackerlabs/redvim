//! Narrow lexical colors for known dialect gaps in tree-sitter-sequel 0.3.11.
//! This does not repair the parse tree or provide SQL validation. Only uncovered
//! tokens inside ERROR nodes receive colors; strings/comments/quoted identifiers
//! are skipped so SQL words inside them cannot become accidental keywords.
use super::*;

pub(super) fn fill_gaps(
    code: &str,
    root: tree_sitter::Node<'_>,
    theme: &Theme,
    colors: &mut Vec<StyleInfo>,
) {
    let mut ranges = Vec::new();
    let mut cursor = root.walk();
    loop {
        let node = cursor.node();
        if node.is_error() {
            ranges.push(node.byte_range());
        } else if node.has_error() && cursor.goto_first_child() {
            continue;
        }
        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                add_tokens(code, &ranges, theme, colors);
                return;
            }
        }
    }
}

fn add_tokens(code: &str, errors: &[Range<usize>], theme: &Theme, colors: &mut Vec<StyleInfo>) {
    let bytes = code.as_bytes();
    let mut i = 0;
    let original = colors.len();
    while i < bytes.len() {
        let start = i;
        match bytes[i] {
            b'\'' | b'"' | b'`' => {
                let quote = bytes[i];
                i += 1;
                while i < bytes.len() {
                    if bytes[i] == b'\\' {
                        i = (i + 2).min(bytes.len());
                    } else if bytes[i] == quote {
                        i += 1;
                        if bytes.get(i) == Some(&quote) {
                            i += 1;
                        } else {
                            break;
                        }
                    } else {
                        i += 1;
                    }
                }
                continue;
            }
            b'-' if bytes.get(i + 1) == Some(&b'-') => {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            b'/' if bytes.get(i + 1) == Some(&b'*') => {
                i += 2;
                while i < bytes.len() && !bytes[i..].starts_with(b"*/") {
                    i += 1;
                }
                i = (i + 2).min(bytes.len());
                continue;
            }
            b'@' => {
                i += 1;
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
            }
            b if b.is_ascii_alphabetic() || b == b'_' || b >= 128 => {
                i += 1;
                while i < bytes.len()
                    && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] >= 128)
                {
                    i += 1;
                }
            }
            _ => {
                i += 1;
                continue;
            }
        }
        let token = &code[start..i];
        let scope = if token.starts_with('@') && token.len() > 1 {
            "variable.parameter"
        } else if token.eq_ignore_ascii_case("UNIQUE") {
            "type.qualifier"
        } else if ["PREPARE", "DEALLOCATE", "AUTOINCREMENT"]
            .iter()
            .any(|keyword| token.eq_ignore_ascii_case(keyword))
        {
            "keyword"
        } else {
            continue;
        };
        if errors
            .iter()
            .any(|range| range.start <= start && i <= range.end)
            && !colors[..original]
                .iter()
                .any(|span| span.start < i && start < span.end)
        {
            if let Some(style) = theme.get_style(scope) {
                colors.push(StyleInfo {
                    start,
                    end: i,
                    style,
                });
            }
        }
    }
}
