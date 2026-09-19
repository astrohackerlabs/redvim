use tree_sitter::{InputEdit, Parser, Point, Tree};

const SOURCE: &str = include_str!("../../../src/queries/highlights/smoke/example.applescript");

fn parser() -> Parser {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_applescript::LANGUAGE.into())
        .unwrap();
    parser
}

fn shape(tree: &Tree) -> Vec<(String, usize, usize)> {
    let mut result = Vec::new();
    let mut cursor = tree.walk();
    loop {
        let node = cursor.node();
        result.push((node.kind().into(), node.start_byte(), node.end_byte()));
        if cursor.goto_first_child() {
            continue;
        }
        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return result;
            }
        }
    }
}

fn point(text: &str, byte: usize) -> Point {
    let prefix = &text[..byte];
    Point::new(
        prefix.bytes().filter(|&b| b == b'\n').count(),
        prefix.rsplit('\n').next().unwrap().len(),
    )
}

fn edit(tree: &mut Tree, old: &str, new: &str) {
    let mut start = old
        .bytes()
        .zip(new.bytes())
        .take_while(|(a, b)| a == b)
        .count();
    while !old.is_char_boundary(start) || !new.is_char_boundary(start) {
        start -= 1;
    }
    let mut suffix = old[start..]
        .bytes()
        .rev()
        .zip(new[start..].bytes().rev())
        .take_while(|(a, b)| a == b)
        .count();
    while !old.is_char_boundary(old.len() - suffix) || !new.is_char_boundary(new.len() - suffix) {
        suffix -= 1;
    }
    let old_end = old.len() - suffix;
    let new_end = new.len() - suffix;
    tree.edit(&InputEdit {
        start_byte: start,
        old_end_byte: old_end,
        new_end_byte: new_end,
        start_position: point(old, start),
        old_end_position: point(old, old_end),
        new_end_position: point(new, new_end),
    });
}

fn assert_complete(tree: &Tree, source: &str) {
    assert!(
        !tree.root_node().has_error(),
        "{}",
        tree.root_node().to_sexp()
    );
    for (token, kind) in [
        (
            "(* outer (* nested *) \"*)\" still comment *)",
            "block_comment",
        ),
        ("|odd name|", "piped_identifier"),
        ("alias \"Macintosh", "alias_prefix"),
        ("SeT afterValue", "keyword_set"),
    ] {
        let start = source.find(token).unwrap();
        let length = match kind {
            "alias_prefix" => 5,
            "keyword_set" => 3,
            _ => token.len(),
        };
        assert_eq!(
            tree.root_node()
                .descendant_for_byte_range(start, start + length)
                .unwrap()
                .kind(),
            kind
        );
    }
    let nodes = shape(tree);
    assert!(nodes.iter().any(|(kind, _, _)| kind == "if_block"));
    assert!(nodes
        .iter()
        .any(|(kind, _, _)| kind == "if_simple_statement"));
}

#[test]
fn scanner_boundaries_survive_incremental_edits() {
    for source in [SOURCE.to_string(), SOURCE.replace('\n', "\r\n")] {
        let mut parser = parser();
        let mut old = source.clone();
        let mut tree = parser.parse(&old, None).unwrap();
        assert_complete(&tree, &old);
        for new in [
            source.replace("still comment *)", "still comment *"),
            source.clone(),
            source.replace("(* outer", "( outer"),
            source.clone(),
            source.replace("\"Hello\"", "\"Hello"),
            source.clone(),
            source.replace("café", "東京"),
            source.clone(),
            source.replace("|odd name|", "|odd name"),
            source.clone(),
            source.replace("if i > 1 then return false", "if i > 1 then"),
            source.clone(),
        ] {
            edit(&mut tree, &old, &new);
            tree = parser.parse(&new, Some(&tree)).unwrap();
            let fresh = self::parser().parse(&new, None).unwrap();
            assert_eq!(shape(&tree), shape(&fresh), "incremental mismatch: {new}");
            if new == source {
                assert_complete(&tree, &new);
            }
            old = new;
        }
    }
}

#[test]
fn indented_to_handler_limit_is_explicit() {
    let source = "to doubleValue(inputValue)\nreturn inputValue * 2\nend doubleValue\n";
    assert!(!parser()
        .parse(source, None)
        .unwrap()
        .root_node()
        .has_error());
    let indented = format!("    {source}");
    for (text, expects_handler) in [(source.to_string(), true), (indented, false)] {
        let tree = parser().parse(&text, None).unwrap();
        assert!(
            !tree.root_node().has_error(),
            "{}",
            tree.root_node().to_sexp()
        );
        assert_eq!(
            shape(&tree)
                .iter()
                .any(|(kind, _, _)| kind == "handler_definition"),
            expects_handler,
            "{text}: {}",
            tree.root_node().to_sexp()
        );
    }
}
