use tree_sitter::{InputEdit, Parser, Point, Tree};

const SOURCE: &str = "example.test {\n    respond <<END\n    first café body\n    END_suffix is body\n    EN partial prefix\n    END 200\n    respond <<NEXT\n    NEXT_suffix\n    NEXT\n    header X-After \"still colored\"\n}\n";

fn parser() -> Parser {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_caddyfile::LANGUAGE.into())
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
    for token in ["first café body", "END_suffix", "EN partial", "NEXT_suffix"] {
        let byte = source.find(token).unwrap();
        let node = tree
            .root_node()
            .descendant_for_byte_range(byte, byte + token.len())
            .unwrap();
        assert_eq!(node.kind(), "heredoc_body", "{token}: {}", node.to_sexp());
    }
    let byte = source.find("header X-After").unwrap();
    assert_eq!(
        tree.root_node()
            .descendant_for_byte_range(byte, byte + 6)
            .unwrap()
            .kind(),
        "directive_name"
    );
    let ends = shape(tree)
        .into_iter()
        .filter(|(kind, _, _)| kind == "heredoc_end")
        .map(|(_, start, end)| source[start..end].trim().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(ends, ["END", "NEXT"]);
}

#[test]
fn exact_heredoc_boundaries_survive_real_incremental_edits() {
    for source in [SOURCE.to_string(), SOURCE.replace('\n', "\r\n")] {
        let mut parser = parser();
        let mut old = source.clone();
        let mut tree = parser.parse(&old, None).unwrap();
        assert_complete(&tree, &old);
        for new in [
            source.replace("<<END", "END"),
            source.clone(),
            source.replace("    END 200", "    EN 200"),
            source.clone(),
            source.replace("<<NEXT", "NEXT"),
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
fn empty_deserialization_clears_previous_delimiter() {
    use std::ffi::c_void;
    extern "C" {
        fn tree_sitter_caddyfile_external_scanner_create() -> *mut c_void;
        fn tree_sitter_caddyfile_external_scanner_destroy(scanner: *mut c_void);
        fn tree_sitter_caddyfile_external_scanner_deserialize(
            scanner: *mut c_void,
            buffer: *const u8,
            length: u32,
        );
        fn tree_sitter_caddyfile_external_scanner_serialize(
            scanner: *mut c_void,
            buffer: *mut u8,
        ) -> u32;
    }
    // Use the native scanner's public Tree-sitter serialization contract.
    unsafe {
        let scanner = tree_sitter_caddyfile_external_scanner_create();
        assert!(!scanner.is_null());
        let active = [1, b'E', b'N', b'D'];
        tree_sitter_caddyfile_external_scanner_deserialize(scanner, active.as_ptr(), 4);
        let mut buffer = [0u8; 1024];
        assert_eq!(
            tree_sitter_caddyfile_external_scanner_serialize(scanner, buffer.as_mut_ptr()),
            4
        );
        assert_eq!(buffer[..4], active);
        tree_sitter_caddyfile_external_scanner_deserialize(scanner, std::ptr::null(), 0);
        assert_eq!(
            tree_sitter_caddyfile_external_scanner_serialize(scanner, buffer.as_mut_ptr()),
            1
        );
        assert_eq!(buffer[0], 0);
        tree_sitter_caddyfile_external_scanner_destroy(scanner);
    }
}
