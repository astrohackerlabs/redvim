fn main() {
    cc::Build::new()
        .std("c11")
        .include("src")
        .file("src/parser.c")
        .file("src/scanner.c")
        .flag_if_supported("-Wno-unused-parameter")
        .compile("tree-sitter-applescript");
    for file in ["src/parser.c", "src/scanner.c", "src/tree_sitter/parser.h"] {
        println!("cargo:rerun-if-changed={file}");
    }
}
