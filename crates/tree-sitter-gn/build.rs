fn main() {
    cc::Build::new()
        .include("src")
        .file("src/parser.c")
        .file("src/scanner.c")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .compile("tree-sitter-gn");
    for path in ["src/parser.c", "src/scanner.c", "src/tree_sitter/parser.h"] {
        println!("cargo:rerun-if-changed={path}");
    }
}
