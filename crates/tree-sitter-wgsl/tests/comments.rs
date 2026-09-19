use tree_sitter::Parser;

#[test]
fn comments_do_not_change_template_pairing() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_wgsl::LANGUAGE.into())
        .unwrap();
    for source in [
        "alias A = array /* comment */ <u32, 4>;",
        "alias A = array<vec4 // misleading >\n <f32>, 4>;",
        "alias A = array< // misleading >\n vec4<f32>, 4>;",
        "alias A = array /* outer /* nested */ **/ <u32, 4>;",
        "alias A = array<u32, (8 / 2)>;",
        "alias A = array<u32, (8 / /* adjacent */ 2)>;",
        "alias A = array<u32, (8 >> 1)>;",
        "alias A = array<u32, select(1, 2, 3 < 4)>;",
    ] {
        let source = format!("{source}\nfn after() -> u32 {{ return 7u; }}");
        let tree = parser.parse(&source, None).unwrap();
        assert!(
            !tree.root_node().has_error(),
            "{source}\n{}",
            tree.root_node().to_sexp()
        );
        assert_eq!(
            tree.root_node()
                .named_child(tree.root_node().named_child_count() - 1)
                .unwrap()
                .kind(),
            "function_decl"
        );
    }
}
