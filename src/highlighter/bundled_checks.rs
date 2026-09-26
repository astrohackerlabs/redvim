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
        language: "applescript",
        filename: "example.APPLESCRIPT",
        source: include_str!("../queries/highlights/smoke/example.applescript"),
        tokens: &[
            ("-- café", "comment"),
            ("(* outer", "comment"),
            ("greeting", "property"),
            ("greet(personName)", "function"),
            ("personName)", "variable.parameter"),
            ("set message", "keyword"),
            ("café ", "string"),
            ("true then", "boolean"),
            ("if true", "keyword"),
            ("return message", "keyword"),
            ("else", "keyword"),
            ("missing value", "constant.builtin"),
            ("doubleValue(inputValue)", "function"),
            ("inputValue)", "variable.parameter"),
            ("* 2", "operator"),
            ("42", "number"),
            ("1.5", "number"),
            ("name:", "property"),
            ("repeat with", "keyword"),
            ("false", "boolean"),
            ("tell application", "keyword"),
            ("Finder", "string"),
            ("try", "keyword"),
            ("display dialog", "function.builtin"),
            ("default answer", "variable.parameter"),
            ("on error", "keyword"),
            ("& errNum", "operator"),
            ("greet(\"World\")", "function.call"),
            ("99", "number"),
        ],
    },
    Fixture {
        language: "caddyfile",
        filename: "Caddyfile.dev",
        source: include_str!("../queries/highlights/smoke/Caddyfile"),
        tokens: &[
            ("# café", "comment"),
            ("skip_install_trust", "property"),
            ("email operator", "property"),
            ("auto_https", "property"),
            ("disable_redirects", "string"),
            ("shared", "function"),
            ("www.example.test", "keyword"),
            ("import", "property"),
            ("tls internal", "property"),
            ("internal", "string"),
            ("@api", "function.macro"),
            ("path /api", "function.method"),
            ("/api/*", "type"),
            ("reverse_proxy", "property"),
            ("127.0.0.1:8080", "type"),
            ("[::1]:8081", "type"),
            ("backend.example.test:8082", "type"),
            ("5s", "number"),
            ("/static/*", "type"),
            ("root", "property"),
            ("file_server", "property"),
            ("{uri}", "constant"),
            ("{$CADDY_ENV:development}", "constant"),
            ("\\\"quoted", "string.escape"),
            ("raw {uri}", "string"),
            ("first café body", "string"),
            ("END_suffix", "string"),
            ("EN partial", "string"),
            ("another body", "string"),
            ("still colored", "string"),
            ("healthy", "string"),
            ("200", "number"),
        ],
    },
    Fixture {
        language: "wgsl",
        filename: "example.WGSL",
        source: include_str!("../queries/highlights/smoke/example.wgsl"),
        tokens: &[
            ("// café", "comment"),
            ("enable", "keyword.directive"),
            ("diagnostic", "keyword.directive"),
            ("alias", "keyword"),
            ("Samples", "type"),
            ("/* comment */", "comment"),
            ("// misleading >", "comment"),
            ("Vertex", "type"),
            ("position", "property"),
            ("vec4", "type.builtin"),
            ("f32", "type.builtin"),
            ("location", "keyword.directive"),
            ("const count", "keyword"),
            ("override", "keyword"),
            ("1.5f", "float"),
            ("group", "keyword.directive"),
            ("binding", "keyword.directive"),
            ("const_assert", "keyword.directive"),
            ("/* nested", "comment"),
            ("helper", "function"),
            ("0x2au", "number"),
            ("<<", "operator"),
            (">>", "operator"),
            (">=", "operator"),
            ("&&", "operator"),
            ("!=", "operator"),
            ("for (", "keyword"),
            ("while", "keyword"),
            ("continuing", "keyword"),
            ("break if", "keyword"),
            ("switch", "keyword"),
            ("return", "keyword"),
            ("compute", "keyword.directive"),
            ("workgroup_size", "keyword.directive"),
            ("global_invocation_id", "constant.builtin"),
            ("0x1.fp+2f", "float"),
            ("true", "constant.builtin"),
            ("discard", "keyword"),
            ("after", "function"),
        ],
    },
    Fixture {
        language: "gn",
        filename: "BUILD.GN",
        source: include_str!("../queries/highlights/smoke/example.gn"),
        tokens: &[
            ("# café", "comment"),
            ("import(", "include"),
            ("//build/config.gni", "string"),
            ("declare_args", "function.call"),
            ("enabled =", "property"),
            ("true", "boolean"),
            ("42", "number"),
            ("template(", "function.call"),
            ("component", "string"),
            ("source_set", "function.call"),
            ("sources =", "property"),
            ("main.cc", "string"),
            ("+=", "operator"),
            ("-=", "operator"),
            ("if (", "conditional"),
            ("!is_debug", "operator"),
            ("&&", "operator"),
            ("||", "operator"),
            (">=", "operator"),
            ("!=", "operator"),
            ("else", "conditional"),
            ("false", "boolean"),
            ("foreach(", "repeat"),
            ("print(", "function.call"),
            ("-1", "number"),
            ("< 9", "operator"),
            ("<=", "operator"),
            ("> 1", "operator"),
            ("==", "operator"),
            ("quote", "string"),
            ("\\\"", "string.escape"),
            ("\\$", "string.escape"),
            ("\\\\", "string.escape"),
            ("$0x41", "string.escape"),
            ("$0x42", "string.escape"),
            ("after =", "property"),
            ("still colored", "string"),
            ("{", "punctuation.bracket"),
            ("[", "punctuation.bracket"),
            ("(", "punctuation.bracket"),
            (",", "punctuation.delimiter"),
        ],
    },
    Fixture {
        language: "dockerfile",
        filename: "Dockerfile.dev",
        source: include_str!("../queries/highlights/smoke/Dockerfile"),
        tokens: &[
            ("# syntax", "comment"),
            ("# café FROM", "comment"),
            ("ARG", "keyword"),
            ("BASE=", "property"),
            ("FROM --", "keyword"),
            ("platform", "property"),
            ("${BASE}", "punctuation.delimiter"),
            ("AS builder", "keyword"),
            ("builder", "type"),
            ("ENV TITLE", "keyword"),
            ("TITLE=", "property"),
            ("hello", "string"),
            ("lower=", "property"),
            ("'value'", "string"),
            ("LABEL", "keyword"),
            ("org.example.name", "property"),
            ("WORKDIR", "keyword"),
            ("USER", "keyword"),
            ("RUN --", "keyword"),
            ("mount", "property"),
            ("type=cache", "property"),
            ("target=/cache", "property"),
            ("\\\n", "punctuation.delimiter"),
            ("COPY --chown", "keyword"),
            ("chown", "property"),
            ("ADD", "keyword"),
            ("checksum", "property"),
            ("EXPOSE", "keyword"),
            ("8080", "number"),
            ("SHELL", "keyword"),
            ("python3", "string"),
            ("ONBUILD", "keyword"),
            ("HEALTHCHECK", "keyword"),
            ("interval", "property"),
            ("VOLUME", "keyword"),
            ("STOPSIGNAL", "keyword"),
            ("<<EOF", "constant"),
            ("echo heredoc", "string"),
            ("literal heredoc", "string"),
            ("from alpine", "keyword"),
            ("alpine:3.21 as", "string.special"),
            ("as final", "keyword"),
            ("final", "type"),
            ("AFTER=", "property"),
            ("after heredoc", "string"),
            ("ENTRYPOINT", "keyword"),
            ("/app/start", "string"),
            ("CMD [", "keyword"),
            ("--help", "string"),
            ("\\nvalue", "string.escape"),
        ],
    },
    Fixture {
        language: "proto",
        filename: "example.PROTO",
        source: include_str!("../queries/highlights/smoke/example.proto"),
        tokens: &[
            ("// café", "comment"),
            ("syntax", "keyword.directive"),
            ("\"proto3\"", "string.special.symbol"),
            ("package", "keyword.import"),
            ("demo", "type.namespace"),
            ("import", "keyword.import"),
            ("public", "keyword.modifier"),
            ("other.proto", "string.special.path"),
            ("option", "keyword.directive"),
            ("java_package", "property"),
            ("message", "keyword.type"),
            ("Request", "type"),
            ("Nested", "type"),
            ("reserved", "keyword"),
            ("9", "number"),
            ("to 12", "keyword"),
            ("old_name", "string"),
            ("optional", "keyword.modifier"),
            ("name =", "property"),
            ("repeated", "keyword.modifier"),
            (".demo.api.Request.Nested", "type"),
            ("map", "keyword.type"),
            ("counts", "property"),
            ("oneof", "keyword.type"),
            ("choice", "type"),
            ("data", "property"),
            ("enum", "keyword.type"),
            ("UNKNOWN", "constant"),
            ("deprecated", "property"),
            ("true", "boolean"),
            ("service", "keyword.type"),
            ("Gateway", "type"),
            ("rpc", "keyword.function"),
            ("Send", "function.method"),
            ("stream", "keyword.modifier"),
            ("returns", "keyword.return"),
            ("/* after", "comment"),
            ("After", "type"),
            ("42", "number"),
            ("=", "operator"),
            ("<", "punctuation.bracket"),
            (";", "punctuation.delimiter"),
        ],
    },
    Fixture {
        language: "proto",
        filename: "proto2.proto",
        source: include_str!("../queries/highlights/smoke/proto2.proto"),
        tokens: &[
            ("\"proto2\"", "string.special.symbol"),
            ("required", "keyword.modifier"),
            ("string", "type.builtin"),
            ("label", "property"),
            ("default", "property"),
            ("café", "string"),
            ("\\n", "string.escape"),
            ("3.14", "number.float"),
            ("false", "boolean"),
            ("group", "keyword.type"),
            ("Details", "type"),
            ("note", "property"),
            ("extensions", "keyword.type"),
            ("max", "keyword"),
            ("extend Record", "keyword.type"),
            ("extra", "property"),
            ("100", "number"),
        ],
    },
    Fixture {
        language: "proto",
        filename: "edition.proto",
        source: include_str!("../queries/highlights/smoke/edition.proto"),
        tokens: &[
            ("edition =", "keyword.directive"),
            ("2023", "string"),
            ("edition_demo", "type.namespace"),
            ("Modern", "type"),
            ("string", "type.builtin"),
            ("label", "property"),
            ("2;", "number"),
        ],
    },
    Fixture {
        language: "objc",
        filename: "example.M",
        source: include_str!("../queries/highlights/smoke/example.m"),
        tokens: &[
            ("// café", "comment"),
            ("#import", "keyword.directive"),
            ("Foundation/Foundation.h", "string"),
            ("field", "property"),
            ("twice", "function"),
            ("ordinary", "variable"),
            ("return", "keyword"),
            ("@protocol", "keyword"),
            ("Greeting", "type"),
            ("@interface", "keyword"),
            ("Example", "type"),
            ("NSObject", "type"),
            ("@property", "keyword"),
            ("title", "property"),
            ("self", "variable.builtin"),
            ("super", "variable.builtin"),
            ("^callback", "operator"),
            ("if (ordinary", "keyword"),
            ("@implementation", "keyword"),
            ("@synthesize", "keyword"),
            ("greet", "function.method"),
            ("shared", "function.method"),
            ("alloc", "function.method.call"),
            ("init", "constructor"),
            ("42", "number"),
            ("17", "number"),
            ("hello", "string"),
            ("key", "string"),
            ("NSLog", "function"),
            ("@selector", "keyword"),
            ("@try", "keyword.exception"),
            ("@catch", "keyword.exception"),
            ("@throw", "keyword.exception"),
            ("@finally", "keyword.exception"),
            ("/* a block", "comment"),
            ("uppercaseString", "function.method.call"),
            ("after", "function"),
            ("99", "number"),
        ],
    },
    Fixture {
        language: "hcl",
        filename: "example.auto.tfvars",
        source: include_str!("../queries/highlights/smoke/example.tfvars"),
        tokens: &[
            ("# café", "comment"),
            ("region", "property"),
            ("earth", "string"),
            ("3", "number"),
            ("true", "boolean"),
            ("null", "constant"),
        ],
    },
    Fixture {
        language: "hcl",
        filename: "example.TF",
        source: include_str!("../queries/highlights/smoke/example.tf"),
        tokens: &[
            ("# café", "comment"),
            ("variable", "keyword"),
            ("message", "property"),
            ("hello", "string"),
            ("upper", "function"),
            ("lower", "function"),
            ("not_an_expression", "string"),
            ("true", "boolean"),
            ("null", "constant"),
            ("42", "number"),
            ("for n", "keyword.repeat"),
            ("if", "keyword.conditional"),
            ("endfor", "keyword.repeat"),
            ("endif", "keyword.conditional"),
            ("// line", "comment"),
            ("/* block", "comment"),
            ("after", "property"),
        ],
    },
    Fixture {
        language: "sql",
        filename: "example.SQL",
        source: include_str!("../queries/highlights/smoke/example.sql"),
        tokens: &[
            ("-- café", "comment"),
            ("SELECT", "keyword"),
            ("'hello'", "string"),
            ("42", "number"),
            ("3.14", "float"),
            ("TRUE", "boolean"),
        ],
    },
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
    super::sql_checks::check(theme)?;
    super::typst_checks::check(theme)?;
    let registry = Arc::new(LanguageRegistry::bundled());
    let mut highlighter = Highlighter::with_registry(theme, Arc::clone(&registry))?;
    let mut checked = vec!["typst"];
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
        if id == "applescript" {
            for (anchor, token, scope) in [
                (
                    "-- café: tell return are comments",
                    "tell return",
                    "comment",
                ),
                (
                    "(* outer (* nested *) \"*)\" still comment *)",
                    "(* outer (* nested *) \"*)\" still comment *)",
                    "comment",
                ),
                ("tell -- literal", "tell -- literal", "string"),
                ("greet(personName)", "greet", "function"),
                ("greet(personName)", "personName", "variable.parameter"),
                ("my greet(\"World\")", "greet", "function.call"),
                ("default answer", "default answer", "variable.parameter"),
                ("SeT afterValue to 99", "SeT", "keyword"),
                ("SeT afterValue to 99", "99", "number"),
            ] {
                let start = fixture
                    .source
                    .find(anchor)
                    .context("missing AppleScript anchor")?
                    + anchor.find(token).context("missing AppleScript token")?;
                let expected = theme
                    .get_style(scope)
                    .context("missing AppleScript scope")?;
                for byte in start..start + token.len() {
                    let actual = styles
                        .iter()
                        .rev()
                        .find(|s| s.start <= byte && byte < s.end)
                        .map(|s| &s.style);
                    anyhow::ensure!(
                        actual == Some(&expected),
                        "applescript: {anchor:?} byte {byte}: expected {scope}, got {actual:?}"
                    );
                }
            }
        }
        if id == "caddyfile" {
            for (anchor, token, scope) in [
                ("reverse_proxy @api", "@api", "function.macro"),
                ("header X-URI {uri}", "{uri}", "constant"),
                (
                    "header X-Env {$CADDY_ENV:development}",
                    "{$CADDY_ENV:development}",
                    "constant",
                ),
                ("X-Quoted \"café \\\"quoted\\\" {uri}\"", "{uri}", "string"),
                ("raw {uri} # literal", "{uri} # literal", "string"),
                (
                    "END_suffix remains body",
                    "END_suffix remains body",
                    "string",
                ),
                ("EN partial prefix", "EN partial prefix", "string"),
                ("END 200", "END", "constant"),
                ("END 200", "200", "number"),
                ("header X-After", "header", "property"),
                ("respond /health", "/health", "type"),
                ("still colored", "still colored", "string"),
                (
                    "backend.example.test:8082",
                    "backend.example.test:8082",
                    "type",
                ),
            ] {
                let start = fixture
                    .source
                    .find(anchor)
                    .context("missing Caddyfile anchor")?
                    + anchor.find(token).context("missing Caddyfile token")?;
                let expected = theme.get_style(scope).context("missing Caddyfile scope")?;
                for byte in start..start + token.len() {
                    let actual = styles
                        .iter()
                        .rev()
                        .find(|s| s.start <= byte && byte < s.end)
                        .map(|s| &s.style);
                    anyhow::ensure!(
                        actual == Some(&expected),
                        "caddyfile: {anchor:?} at {byte}: expected {scope}, got {actual:?}"
                    );
                }
            }
        }
        if id == "wgsl" {
            for (anchor, token, scope) in [
                ("i < count", "<", "operator"),
                ("value >= 1u", ">=", "operator"),
                ("value << 1u", "<<", "operator"),
                ("café >>= 1u", ">>=", "operator"),
                ("const count:", "count", "variable"),
                ("override scale:", "scale", "variable"),
                ("var<storage, read_write> data:", "data", "variable"),
                ("@group(0)", "0", "number"),
                ("@binding(1)", "1", "number"),
                ("@workgroup_size(64)", "64", "number"),
                ("array<array<u32, 4>, 2>", "4", "number"),
                ("helper(value:", "value", "variable.parameter"),
                ("let result = helper(index", "helper", "function"),
                ("let result = helper(index", "index", "variable"),
                ("invocation.x", "invocation", "variable"),
                ("invocation.x", "x", "property"),
                ("data[index].position.xy", "position", "property"),
                ("data[index].position.xy", "xy", "property"),
                (
                    "/* nested /* inner */ tail **/",
                    "/* nested /* inner */ tail **/",
                    "comment",
                ),
                ("fn after()", "after", "function"),
            ] {
                let start = fixture.source.find(anchor).context("missing WGSL anchor")?
                    + anchor.find(token).context("missing WGSL token")?;
                let expected = theme.get_style(scope).context("missing WGSL scope")?;
                for byte in start..start + token.len() {
                    let actual = styles
                        .iter()
                        .rev()
                        .find(|s| s.start <= byte && byte < s.end)
                        .map(|s| &s.style);
                    anyhow::ensure!(
                        actual == Some(&expected),
                        "wgsl: {anchor:?} at {byte}: expected {scope}, got {actual:?}"
                    );
                }
            }
        }
        if id == "gn" {
            for token in ["$0x41", "$0x42"] {
                let start = fixture.source.find(token).unwrap();
                let node = tree
                    .root_node()
                    .descendant_for_byte_range(start + 1, start + token.len())
                    .context("missing hex node")?;
                anyhow::ensure!(
                    node.kind() == "hex" && node.parent().is_some_and(|n| n.kind() == "expansion"),
                    "{token}: expected expansion(hex), got {}",
                    node.to_sexp()
                );
            }
            let literal = fixture.source.find("\\$0x43").unwrap();
            let node = tree
                .root_node()
                .descendant_for_byte_range(literal + 2, literal + 6)
                .context("missing literal node")?;
            anyhow::ensure!(node.kind() != "hex", "escaped dollar became byte expansion");
            for (anchor, offset, length, scope) in [
                ("$root_out_dir", 0, 1, "punctuation.delimiter"),
                ("$root_out_dir", 1, 12, "variable.parameter"),
                ("${invoker.name}", 0, 2, "punctuation.delimiter"),
                ("${invoker.name}", 10, 4, "property"),
                ("invoker.flags", 8, 5, "property"),
                ("$0x41", 0, 5, "string.escape"),
                ("$0x42", 0, 5, "string.escape"),
                ("\\$0x43", 0, 2, "string.escape"),
                ("still colored", 0, 13, "string"),
            ] {
                let start = fixture.source.find(anchor).context("missing GN anchor")? + offset;
                let expected = theme.get_style(scope).context("missing GN scope")?;
                for byte in start..start + length {
                    let actual = styles
                        .iter()
                        .rev()
                        .find(|s| s.start <= byte && byte < s.end)
                        .map(|s| &s.style);
                    anyhow::ensure!(
                        actual == Some(&expected),
                        "gn: {anchor:?} at {byte}: expected {scope}, got {actual:?}"
                    );
                }
            }
        }
        if id == "dockerfile" {
            for (anchor, offset, length, scope) in [
                ("${BASE}", 2, 4, "constant"),
                ("${MODE}", 2, 4, "constant"),
                ("$lower", 1, 5, "constant"),
                ("${MODE}", 0, 2, "punctuation.delimiter"),
                ("${MODE}", 6, 1, "punctuation.delimiter"),
                ("\nEOF\n", 1, 3, "constant"),
                ("\nEND\n", 1, 3, "constant"),
                ("<<'END'", 0, 7, "constant"),
                ("from alpine", 0, 4, "keyword"),
                ("ENV AFTER", 0, 3, "keyword"),
                ("# café FROM", 8, 4, "comment"),
            ] {
                let start = fixture
                    .source
                    .find(anchor)
                    .context("missing Dockerfile anchor")?
                    + offset;
                let expected = theme.get_style(scope).context("missing Dockerfile scope")?;
                for byte in start..start + length {
                    let actual = styles
                        .iter()
                        .rev()
                        .find(|s| s.start <= byte && byte < s.end)
                        .map(|s| &s.style);
                    anyhow::ensure!(
                        actual == Some(&expected),
                        "dockerfile: {anchor:?} at {byte}: expected {scope}, got {actual:?}"
                    );
                }
            }
        }
        if id == "objc" {
            for (anchor, offset, length, scope) in [
                ("ordinary = 42", 0, 8, "variable"),
                ("@\"hello\"", 2, 5, "string"),
                ("@\"key\"", 2, 3, "string"),
                ("@\"café\"", 2, 5, "string"),
                ("^{ NSLog", 0, 1, "operator"),
                ("@selector(description)", 10, 11, "function.method"),
                ("[super description]", 1, 5, "variable.builtin"),
                ("[super description]", 7, 11, "function.method.call"),
                ("NSString *title", 10, 5, "property"),
            ] {
                let start = fixture
                    .source
                    .find(anchor)
                    .context("missing Objective-C anchor")?
                    + offset;
                let expected = theme
                    .get_style(scope)
                    .context("missing Objective-C scope")?;
                for byte in start..start + length {
                    let actual = styles
                        .iter()
                        .rev()
                        .find(|s| s.start <= byte && byte < s.end)
                        .map(|s| &s.style);
                    anyhow::ensure!(
                        actual == Some(&expected),
                        "objc: {anchor:?} byte {byte}: expected {scope}, got {actual:?}"
                    );
                }
            }
        }
        if id == "hcl" && fixture.filename == "example.TF" {
            // Anchors disambiguate repeated keywords and identifiers. Check every
            // byte of each target, including expression tokens inside templates.
            for (anchor, offset, length, scope) in [
                ("variable \"region\"", 10, 6, "string"),
                ("var.region", 4, 6, "property"),
                ("var.region", 3, 1, "punctuation.delimiter"),
                ("[1, 2]", 0, 1, "punctuation.bracket"),
                ("[1, 2]", 2, 1, "punctuation.delimiter"),
                ("{ a = 1 }", 0, 1, "punctuation.bracket"),
                ("{ a = 1 }", 2, 1, "property"),
                ("? \"yes\"", 0, 1, "punctuation.delimiter"),
                ("[*].id", 0, 3, "punctuation.delimiter"),
                ("[*].id", 4, 2, "property"),
                ("n * 2", 2, 1, "operator"),
                ("k => v", 2, 2, "punctuation.delimiter"),
                ("${upper", 0, 2, "punctuation.delimiter"),
                ("${upper", 2, 5, "function"),
                ("%{ if true ~}", 0, 2, "punctuation.delimiter"),
                ("%{ if true ~}", 3, 2, "keyword.conditional"),
                ("%{ if true ~}", 6, 4, "boolean"),
                ("%{ if true ~}", 11, 1, "punctuation.delimiter"),
                ("%{ for item", 3, 3, "keyword.repeat"),
                ("%{ else", 3, 4, "keyword.conditional"),
                ("${item}", 2, 4, "variable"),
                ("${item}", 6, 1, "punctuation.delimiter"),
                ("<<EOT", 0, 2, "punctuation.delimiter"),
                ("<<-END", 0, 3, "punctuation.delimiter"),
            ] {
                let start = fixture.source.find(anchor).context("missing HCL anchor")? + offset;
                let expected = theme.get_style(scope).context("missing HCL scope")?;
                for byte in start..start + length {
                    let actual = styles
                        .iter()
                        .rev()
                        .find(|s| s.start <= byte && byte < s.end)
                        .map(|s| &s.style);
                    anyhow::ensure!(actual == Some(&expected),
                        "hcl: {anchor:?} byte {byte} should have {scope}; actual={actual:?}, expected={expected:?}");
                }
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
    fn applescript_routing_overrides_and_incremental_colors() {
        let mut theme =
            parse_vscode_theme_contents(include_str!("../../themes/mocha.json")).unwrap();
        let query = Query::new(
            &tree_sitter_applescript::LANGUAGE.into(),
            include_str!("../queries/highlights/applescript.scm"),
        )
        .unwrap();
        for pattern in 0..query.pattern_count() {
            assert!(query.general_predicates(pattern).is_empty());
        }
        let mut highlighter = Highlighter::new(&theme).unwrap();
        for file in ["test.applescript", "test.APPLESCRIPT"] {
            assert_eq!(
                highlighter.language_id_for_file(Some(file)),
                Some("applescript")
            );
        }
        for file in ["test.scpt", "test.scptd"] {
            assert_ne!(
                highlighter.language_id_for_file(Some(file)),
                Some("applescript")
            );
        }
        assert_eq!(
            highlighter.language_id_for_name("applescript"),
            Some("applescript")
        );
        let fence = "```applescript\nset count to 42\n```\n";
        let spans = highlighter.highlight("markdown", fence).unwrap();
        let byte = fence.find("42").unwrap();
        assert_eq!(
            spans
                .iter()
                .rev()
                .find(|s| s.start <= byte && byte < s.end)
                .map(|s| &s.style),
            theme.get_style("number").as_ref()
        );
        let fixture = include_str!("../queries/highlights/smoke/example.applescript");
        for source in [fixture.to_string(), fixture.replace('\n', "\r\n")] {
            for changed in [
                source.clone(),
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
                let shape = |spans: Vec<StyleInfo>| {
                    spans
                        .into_iter()
                        .map(|s| (s.start, s.end, s.style))
                        .collect::<Vec<_>>()
                };
                assert_eq!(
                    shape(highlighter.highlight("applescript", &changed).unwrap()),
                    shape(
                        Highlighter::new(&theme)
                            .unwrap()
                            .highlight("applescript", &changed)
                            .unwrap()
                    )
                );
            }
        }
        let directory = tempfile::tempdir().unwrap();
        let config = HashMap::from([(
            "automation".into(),
            LanguageConfig {
                extensions: vec!["customapple".into()],
                filenames: vec!["AppleScript.override".into()],
                grammar: Some(LanguageGrammarConfig {
                    builtin: Some("applescript".into()),
                    ..LanguageGrammarConfig::default()
                }),
                ..LanguageConfig::default()
            },
        )]);
        let registry = Arc::new(LanguageRegistry::from_config(&config, directory.path()).unwrap());
        let mut function = theme.get_style("function").unwrap();
        function.bold = !function.bold;
        theme.token_styles.insert(
            0,
            crate::theme::TokenStyle {
                name: None,
                scope: vec!["function".into()],
                style: function.clone(),
            },
        );
        let mut highlighter = Highlighter::with_registry(&theme, registry).unwrap();
        for file in ["test.customapple", "AppleScript.override"] {
            assert_eq!(
                highlighter.language_id_for_file(Some(file)),
                Some("automation")
            );
        }
        for id in ["applescript", "automation"] {
            let source = "on greet(personName)\nreturn personName\nend greet\n";
            let spans = highlighter.highlight(id, source).unwrap();
            for byte in 3..8 {
                assert_eq!(
                    spans
                        .iter()
                        .rev()
                        .find(|s| s.start <= byte && byte < s.end)
                        .map(|s| &s.style),
                    Some(&function)
                );
            }
        }
    }

    #[test]
    fn caddyfile_routing_overrides_and_incremental_heredoc_colors() {
        let mut theme =
            parse_vscode_theme_contents(include_str!("../../themes/mocha.json")).unwrap();
        let query = Query::new(
            &tree_sitter_caddyfile::LANGUAGE.into(),
            include_str!("../queries/highlights/caddyfile.scm"),
        )
        .unwrap();
        for pattern in 0..query.pattern_count() {
            assert!(query.general_predicates(pattern).is_empty());
        }
        let mut highlighter = Highlighter::new(&theme).unwrap();
        for file in [
            "Caddyfile",
            "caddyfile",
            "Caddyfile.dev",
            "caddyfile.local",
            "server.CADDY",
            "server.CADDYFILE",
        ] {
            assert_eq!(
                highlighter.language_id_for_file(Some(file)),
                Some("caddyfile")
            );
        }
        assert_ne!(
            highlighter.language_id_for_file(Some("notCaddyfile.txt")),
            Some("caddyfile")
        );
        for alias in ["caddy", "caddyfile"] {
            assert_eq!(highlighter.language_id_for_name(alias), Some("caddyfile"));
            let source = format!("```{alias}\nexample.test {{\nrespond \"ok\" 200\n}}\n```\n");
            let spans = highlighter.highlight("markdown", &source).unwrap();
            let byte = source.find("200").unwrap();
            assert_eq!(
                spans
                    .iter()
                    .rev()
                    .find(|s| s.start <= byte && byte < s.end)
                    .map(|s| &s.style),
                theme.get_style("number").as_ref()
            );
        }
        for fixture in [
            include_str!("../queries/highlights/smoke/Caddyfile").to_string(),
            include_str!("../queries/highlights/smoke/Caddyfile").replace('\n', "\r\n"),
        ] {
            for source in [
                fixture.clone(),
                fixture.replace("<<END", "END"),
                fixture.clone(),
                fixture.replace("        END 200", "        EN 200"),
                fixture.clone(),
                fixture.replace("<<NEXT", "NEXT"),
                fixture.clone(),
            ] {
                let shape = |spans: Vec<StyleInfo>| {
                    spans
                        .into_iter()
                        .map(|s| (s.start, s.end, s.style))
                        .collect::<Vec<_>>()
                };
                assert_eq!(
                    shape(highlighter.highlight("caddyfile", &source).unwrap()),
                    shape(
                        Highlighter::new(&theme)
                            .unwrap()
                            .highlight("caddyfile", &source)
                            .unwrap()
                    )
                );
            }
        }
        let directory = tempfile::tempdir().unwrap();
        let config = HashMap::from([(
            "proxy".into(),
            LanguageConfig {
                extensions: vec!["customcaddy".into()],
                filenames: vec!["Caddyfile.override".into()],
                grammar: Some(LanguageGrammarConfig {
                    builtin: Some("caddyfile".into()),
                    ..LanguageGrammarConfig::default()
                }),
                ..LanguageConfig::default()
            },
        )]);
        let registry = Arc::new(LanguageRegistry::from_config(&config, directory.path()).unwrap());
        let mut property = theme.get_style("property").unwrap();
        property.bold = !property.bold;
        theme.token_styles.insert(
            0,
            crate::theme::TokenStyle {
                name: None,
                scope: vec!["property".into()],
                style: property.clone(),
            },
        );
        let mut highlighter = Highlighter::with_registry(&theme, registry).unwrap();
        for file in ["site.customcaddy", "Caddyfile.override"] {
            assert_eq!(highlighter.language_id_for_file(Some(file)), Some("proxy"));
        }
        let source = "example.test {\n    respond \"ok\" 200\n}\n";
        let start = source.find("respond").unwrap();
        for id in ["caddyfile", "proxy"] {
            let spans = highlighter.highlight(id, source).unwrap();
            for byte in start..start + 7 {
                assert_eq!(
                    spans
                        .iter()
                        .rev()
                        .find(|s| s.start <= byte && byte < s.end)
                        .map(|s| &s.style),
                    Some(&property)
                );
            }
        }
    }

    #[test]
    fn wgsl_routing_overrides_and_incremental_edits() {
        let mut theme =
            parse_vscode_theme_contents(include_str!("../../themes/mocha.json")).unwrap();
        let query = Query::new(
            &tree_sitter_wgsl::LANGUAGE.into(),
            include_str!("../queries/highlights/wgsl.scm"),
        )
        .unwrap();
        for pattern in 0..query.pattern_count() {
            assert!(query.general_predicates(pattern).is_empty());
        }
        let mut highlighter = Highlighter::new(&theme).unwrap();
        for file in ["shader.wgsl", "shader.WGSL"] {
            assert_eq!(highlighter.language_id_for_file(Some(file)), Some("wgsl"));
        }
        assert_ne!(
            highlighter.language_id_for_file(Some("shader.wgsl.txt")),
            Some("wgsl")
        );
        assert_eq!(highlighter.language_id_for_name("wgsl"), Some("wgsl"));
        let fence = "```wgsl\nconst size = 42u;\n```\n";
        let spans = highlighter.highlight("markdown", fence).unwrap();
        let byte = fence.find("42u").unwrap();
        assert_eq!(
            spans
                .iter()
                .rev()
                .find(|s| s.start <= byte && byte < s.end)
                .map(|s| &s.style),
            theme.get_style("number").as_ref()
        );
        for source in [
            "alias A = array /* unfinished",
            "alias A = array /* fixed */ <vec4<f32>, 4>;",
            "alias A = array<vec4 // >\n <f32>, 4>;",
            "fn café() { let x = 1u <<",
            "fn café() { let x = 1u << 2u; }",
            include_str!("../queries/highlights/smoke/example.wgsl"),
        ] {
            let shape = |spans: Vec<StyleInfo>| {
                spans
                    .into_iter()
                    .map(|s| (s.start, s.end, s.style))
                    .collect::<Vec<_>>()
            };
            assert_eq!(
                shape(highlighter.highlight("wgsl", source).unwrap()),
                shape(
                    Highlighter::new(&theme)
                        .unwrap()
                        .highlight("wgsl", source)
                        .unwrap()
                )
            );
        }
        let directory = tempfile::tempdir().unwrap();
        let config = HashMap::from([(
            "shader".into(),
            LanguageConfig {
                extensions: vec!["customwgsl".into()],
                filenames: vec!["custom-shader".into()],
                grammar: Some(LanguageGrammarConfig {
                    builtin: Some("wgsl".into()),
                    ..LanguageGrammarConfig::default()
                }),
                ..LanguageConfig::default()
            },
        )]);
        let registry = Arc::new(LanguageRegistry::from_config(&config, directory.path()).unwrap());
        let mut property = theme.get_style("property").unwrap();
        property.bold = !property.bold;
        theme.token_styles.insert(
            0,
            crate::theme::TokenStyle {
                name: None,
                scope: vec!["property".into()],
                style: property.clone(),
            },
        );
        let mut highlighter = Highlighter::with_registry(&theme, registry).unwrap();
        for file in ["main.customwgsl", "custom-shader"] {
            assert_eq!(highlighter.language_id_for_file(Some(file)), Some("shader"));
        }
        let source = "fn main() { let v = position.xyz; }";
        let start = source.find("xyz").unwrap();
        for id in ["wgsl", "shader"] {
            let spans = highlighter.highlight(id, source).unwrap();
            for byte in start..start + 3 {
                assert_eq!(
                    spans
                        .iter()
                        .rev()
                        .find(|s| s.start <= byte && byte < s.end)
                        .map(|s| &s.style),
                    Some(&property)
                );
            }
        }
    }

    #[test]
    fn gn_routing_overrides_and_incremental_escape_edits() {
        let mut theme =
            parse_vscode_theme_contents(include_str!("../../themes/mocha.json")).unwrap();
        let query = Query::new(
            &tree_sitter_gn::LANGUAGE.into(),
            include_str!("../queries/highlights/gn.scm"),
        )
        .unwrap();
        for pattern in 0..query.pattern_count() {
            assert!(query.general_predicates(pattern).is_empty());
        }
        let mut highlighter = Highlighter::new(&theme).unwrap();
        for file in ["BUILD.gn", "args.GN", "config.gni", "config.GNI", ".gn"] {
            assert_eq!(highlighter.language_id_for_file(Some(file)), Some("gn"));
        }
        assert_ne!(
            highlighter.language_id_for_file(Some("file.gn.txt")),
            Some("gn")
        );
        for alias in ["gn", "gni"] {
            assert_eq!(highlighter.language_id_for_name(alias), Some("gn"));
            let source = format!("```{alias}\nlevel = 42\n```\n");
            let spans = highlighter.highlight("markdown", &source).unwrap();
            let byte = source.find("42").unwrap();
            assert_eq!(
                spans
                    .iter()
                    .rev()
                    .find(|s| s.start <= byte && byte < s.end)
                    .map(|s| &s.style),
                theme.get_style("number").as_ref()
            );
        }
        for source in [
            "value = \"café $0x",
            "value = \"café $0x41\"\n",
            "value = \"${invoker.",
            "value = \"\\$0x41\"\n",
            "value = \"café\"\n",
        ] {
            let shape = |spans: Vec<StyleInfo>| {
                spans
                    .into_iter()
                    .map(|s| (s.start, s.end, s.style))
                    .collect::<Vec<_>>()
            };
            assert_eq!(
                shape(highlighter.highlight("gn", source).unwrap()),
                shape(
                    Highlighter::new(&theme)
                        .unwrap()
                        .highlight("gn", source)
                        .unwrap()
                )
            );
        }
        let directory = tempfile::tempdir().unwrap();
        let config = HashMap::from([(
            "buildscript".into(),
            LanguageConfig {
                extensions: vec!["customgn".into()],
                filenames: vec!["custom-build".into()],
                grammar: Some(LanguageGrammarConfig {
                    builtin: Some("gn".into()),
                    ..LanguageGrammarConfig::default()
                }),
                ..LanguageConfig::default()
            },
        )]);
        let registry = Arc::new(LanguageRegistry::from_config(&config, directory.path()).unwrap());
        let mut escape = theme.get_style("string.escape").unwrap();
        escape.bold = !escape.bold;
        theme.token_styles.insert(
            0,
            crate::theme::TokenStyle {
                name: None,
                scope: vec!["string.escape".into()],
                style: escape.clone(),
            },
        );
        let mut highlighter = Highlighter::with_registry(&theme, registry).unwrap();
        for file in ["main.customgn", "custom-build"] {
            assert_eq!(
                highlighter.language_id_for_file(Some(file)),
                Some("buildscript")
            );
        }
        let source = "value = \"$0x41 prefix $0x42 literal \\$0x43\"\n";
        for id in ["gn", "buildscript"] {
            let spans = highlighter.highlight(id, source).unwrap();
            for token in ["$0x41", "$0x42", "\\$"] {
                let start = source.find(token).unwrap();
                for byte in start..start + token.len() {
                    assert_eq!(
                        spans
                            .iter()
                            .rev()
                            .find(|s| s.start <= byte && byte < s.end)
                            .map(|s| &s.style),
                        Some(&escape)
                    );
                }
            }
            let byte = source.find("0x43").unwrap();
            assert_ne!(
                spans
                    .iter()
                    .rev()
                    .find(|s| s.start <= byte && byte < s.end)
                    .map(|s| &s.style),
                Some(&escape)
            );
        }
    }

    #[test]
    fn dockerfile_routing_predicates_overrides_and_incremental_edits() {
        let mut theme =
            parse_vscode_theme_contents(include_str!("../../themes/mocha.json")).unwrap();
        let query = Query::new(
            &tree_sitter_containerfile::LANGUAGE.into(),
            include_str!("../queries/highlights/dockerfile.scm"),
        )
        .unwrap();
        for pattern in 0..query.pattern_count() {
            assert!(query.general_predicates(pattern).is_empty());
        }
        let mut highlighter = Highlighter::new(&theme).unwrap();
        for file in [
            "Dockerfile",
            "dockerfile",
            "Containerfile",
            "containerfile",
            "Dockerfile.dev",
            "Containerfile.test",
            "x.DOCKERFILE",
            "x.CONTAINERFILE",
        ] {
            assert_eq!(
                highlighter.language_id_for_file(Some(file)),
                Some("dockerfile")
            );
        }
        assert_ne!(
            highlighter.language_id_for_file(Some("myDockerfile.txt")),
            Some("dockerfile")
        );
        for alias in ["dockerfile", "containerfile"] {
            assert_eq!(highlighter.language_id_for_name(alias), Some("dockerfile"));
            let source = format!("```{alias}\nEXPOSE 8080\n```\n");
            let spans = highlighter.highlight("markdown", &source).unwrap();
            let byte = source.find("8080").unwrap();
            assert_eq!(
                spans
                    .iter()
                    .rev()
                    .find(|s| s.start <= byte && byte < s.end)
                    .map(|s| &s.style),
                theme.get_style("number").as_ref()
            );
        }
        for source in [
            "FROM alpine\n# café\n",
            "FROM ${BA",
            "RUN <<EOF\ncafé\n",
            "RUN <<EOF\ncafé\nEOF\nENV AFTER=ok\n",
        ] {
            let shape = |spans: Vec<StyleInfo>| {
                spans
                    .into_iter()
                    .map(|s| (s.start, s.end, s.style))
                    .collect::<Vec<_>>()
            };
            assert_eq!(
                shape(highlighter.highlight("dockerfile", source).unwrap()),
                shape(
                    Highlighter::new(&theme)
                        .unwrap()
                        .highlight("dockerfile", source)
                        .unwrap()
                )
            );
        }
        let directory = tempfile::tempdir().unwrap();
        let config = HashMap::from([(
            "recipe".into(),
            LanguageConfig {
                extensions: vec!["recipe".into()],
                filenames: vec!["Dockerfile.special".into()],
                grammar: Some(LanguageGrammarConfig {
                    builtin: Some("dockerfile".into()),
                    ..LanguageGrammarConfig::default()
                }),
                ..LanguageConfig::default()
            },
        )]);
        let registry = Arc::new(LanguageRegistry::from_config(&config, directory.path()).unwrap());
        let mut custom = theme.get_style("property").unwrap();
        custom.bold = !custom.bold;
        theme.token_styles.insert(
            0,
            crate::theme::TokenStyle {
                name: None,
                scope: vec!["property".into()],
                style: custom.clone(),
            },
        );
        let mut highlighter = Highlighter::with_registry(&theme, registry).unwrap();
        for file in ["main.recipe", "Dockerfile.special"] {
            assert_eq!(highlighter.language_id_for_file(Some(file)), Some("recipe"));
        }
        for id in ["dockerfile", "recipe"] {
            let spans = highlighter.highlight(id, "ARG MODE=release\n").unwrap();
            assert_eq!(
                spans
                    .iter()
                    .rev()
                    .find(|s| s.start <= 4 && 4 < s.end)
                    .map(|s| &s.style),
                Some(&custom)
            );
        }
    }

    #[test]
    fn proto_routing_predicates_overrides_and_incremental_edits() {
        let mut theme =
            parse_vscode_theme_contents(include_str!("../../themes/mocha.json")).unwrap();
        let query = Query::new(
            &tree_sitter_proto::LANGUAGE.into(),
            include_str!("../queries/highlights/proto.scm"),
        )
        .unwrap();
        for pattern in 0..query.pattern_count() {
            assert!(query.general_predicates(pattern).is_empty());
        }
        let mut highlighter = Highlighter::new(&theme).unwrap();
        for file in ["main.proto", "main.PROTO"] {
            assert_eq!(highlighter.language_id_for_file(Some(file)), Some("proto"));
        }
        for alias in ["proto", "protobuf"] {
            assert_eq!(highlighter.language_id_for_name(alias), Some("proto"));
            let source = format!("```{alias}\nmessage A {{ int32 value = 42; }}\n```\n");
            let spans = highlighter.highlight("markdown", &source).unwrap();
            let byte = source.find("42").unwrap();
            assert_eq!(
                spans
                    .iter()
                    .rev()
                    .find(|s| s.start <= byte && byte < s.end)
                    .map(|s| &s.style),
                theme.get_style("number").as_ref()
            );
        }
        for source in [
            "syntax = \"proto3\"; // café\n",
            "message A { string name =",
            "message A { string name = 1; }\n",
        ] {
            let shape = |spans: Vec<StyleInfo>| {
                spans
                    .into_iter()
                    .map(|s| (s.start, s.end, s.style))
                    .collect::<Vec<_>>()
            };
            assert_eq!(
                shape(highlighter.highlight("proto", source).unwrap()),
                shape(
                    Highlighter::new(&theme)
                        .unwrap()
                        .highlight("proto", source)
                        .unwrap()
                )
            );
        }
        let directory = tempfile::tempdir().unwrap();
        let config = HashMap::from([(
            "schema".into(),
            LanguageConfig {
                extensions: vec!["schema".into()],
                grammar: Some(LanguageGrammarConfig {
                    builtin: Some("proto".into()),
                    ..LanguageGrammarConfig::default()
                }),
                ..LanguageConfig::default()
            },
        )]);
        let registry = Arc::new(LanguageRegistry::from_config(&config, directory.path()).unwrap());
        let mut custom = theme.get_style("property").unwrap();
        custom.bold = !custom.bold;
        theme.token_styles.insert(
            0,
            crate::theme::TokenStyle {
                name: None,
                scope: vec!["property".into()],
                style: custom.clone(),
            },
        );
        let mut highlighter = Highlighter::with_registry(&theme, registry).unwrap();
        assert_eq!(
            highlighter.language_id_for_file(Some("main.schema")),
            Some("schema")
        );
        let source = "message A { string name = 1; }";
        for id in ["proto", "schema"] {
            let spans = highlighter.highlight(id, source).unwrap();
            let byte = source.find("name").unwrap();
            assert_eq!(
                spans
                    .iter()
                    .rev()
                    .find(|s| s.start <= byte && byte < s.end)
                    .map(|s| &s.style),
                Some(&custom)
            );
        }
    }

    #[test]
    fn objc_routing_predicates_overrides_and_incremental_edits() {
        let mut theme =
            parse_vscode_theme_contents(include_str!("../../themes/mocha.json")).unwrap();
        let query = Query::new(
            &tree_sitter_objc::LANGUAGE.into(),
            include_str!("../queries/highlights/objc.scm"),
        )
        .unwrap();
        for pattern in 0..query.pattern_count() {
            assert!(
                query.general_predicates(pattern).is_empty(),
                "unsupported predicate at pattern {pattern}"
            );
        }
        let mut highlighter = Highlighter::new(&theme).unwrap();
        for file in ["main.m", "main.M"] {
            assert_eq!(highlighter.language_id_for_file(Some(file)), Some("objc"));
        }
        assert_eq!(highlighter.language_id_for_file(Some("main.h")), Some("c"));
        assert_eq!(highlighter.language_id_for_file(Some("main.mm")), None);
        for alias in ["objc", "objective-c"] {
            assert_eq!(highlighter.language_id_for_name(alias), Some("objc"));
            let source = format!("```{alias}\nint value = 42;\n```\n");
            let spans = highlighter.highlight("markdown", &source).unwrap();
            let byte = source.find("42").unwrap();
            assert_eq!(
                spans
                    .iter()
                    .rev()
                    .find(|s| s.start <= byte && byte < s.end)
                    .map(|s| &s.style),
                theme.get_style("number").as_ref()
            );
        }
        for source in [
            "NSString *value = @\"café\";\n",
            "@interface A : NSObject\n",
            "int value = 42;\n",
        ] {
            let shape = |spans: Vec<StyleInfo>| {
                spans
                    .into_iter()
                    .map(|s| (s.start, s.end, s.style))
                    .collect::<Vec<_>>()
            };
            assert_eq!(
                shape(highlighter.highlight("objc", source).unwrap()),
                shape(
                    Highlighter::new(&theme)
                        .unwrap()
                        .highlight("objc", source)
                        .unwrap()
                )
            );
        }
        let directory = tempfile::tempdir().unwrap();
        let config = HashMap::from([(
            "objc".into(),
            LanguageConfig {
                extensions: vec!["h".into()],
                ..LanguageConfig::default()
            },
        )]);
        let registry = LanguageRegistry::from_config(&config, directory.path()).unwrap();
        assert_eq!(
            Highlighter::with_registry(&theme, Arc::new(registry))
                .unwrap()
                .language_id_for_file(Some("custom.h")),
            Some("objc")
        );
        let mut custom = theme.get_style("function").unwrap();
        custom.bold = !custom.bold;
        theme.token_styles.insert(
            0,
            crate::theme::TokenStyle {
                name: None,
                scope: vec!["function.method".into()],
                style: custom.clone(),
            },
        );
        let source = "void f(void) { [object greet]; }";
        let spans = Highlighter::new(&theme)
            .unwrap()
            .highlight("objc", source)
            .unwrap();
        let byte = source.find("greet").unwrap();
        assert_eq!(
            spans
                .iter()
                .rev()
                .find(|s| s.start <= byte && byte < s.end)
                .map(|s| &s.style),
            Some(&custom)
        );
    }

    #[test]
    fn objc_builtin_and_parameter_theme_overrides_remain_authoritative() {
        let mut theme =
            parse_vscode_theme_contents(include_str!("../../themes/mocha.json")).unwrap();
        let source = "@implementation A\n- (void)greet:(id)person { [self greet:person]; }\n@end\n";
        for (scope, token) in [
            ("variable.builtin", "self"),
            ("variable.parameter", "person"),
        ] {
            let mut custom = theme.get_style("variable").unwrap();
            custom.bold = !custom.bold;
            theme.token_styles.insert(
                0,
                crate::theme::TokenStyle {
                    name: None,
                    scope: vec![scope.into()],
                    style: custom.clone(),
                },
            );
            let spans = Highlighter::new(&theme)
                .unwrap()
                .highlight("objc", source)
                .unwrap();
            let byte = source.find(token).unwrap();
            assert_eq!(
                spans
                    .iter()
                    .rev()
                    .find(|s| s.start <= byte && byte < s.end)
                    .map(|s| &s.style),
                Some(&custom)
            );
        }
    }

    #[test]
    fn objc_cpp_is_not_claimed_without_a_cpp_grammar() {
        let source = "#include <vector>\n@interface A : NSObject\n@end\nvoid f() { std::vector<int> values{1, 2}; [A new]; }\n";
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_objc::LANGUAGE.into())
            .unwrap();
        assert!(parser.parse(source, None).unwrap().root_node().has_error());
    }

    #[test]
    fn hcl_routing_fences_and_incremental_edits() {
        let theme = parse_vscode_theme_contents(include_str!("../../themes/mocha.json")).unwrap();
        let mut highlighter = Highlighter::new(&theme).unwrap();
        for file in [
            "main.TF",
            "main.hcl",
            "terraform.tfvars",
            "example.auto.tfvars",
            ".terraform.lock.hcl",
        ] {
            assert_eq!(highlighter.language_id_for_file(Some(file)), Some("hcl"));
        }
        for file in ["main.tf.json", "example.tfvars.json"] {
            assert_eq!(highlighter.language_id_for_file(Some(file)), Some("json"));
        }
        for alias in ["hcl", "terraform"] {
            assert_eq!(highlighter.language_id_for_name(alias), Some("hcl"));
            let source = format!("```{alias}\nvalue = 42\n```\n");
            let spans = highlighter.highlight("markdown", &source).unwrap();
            let byte = source.find("42").unwrap();
            assert_eq!(
                spans
                    .iter()
                    .rev()
                    .find(|s| s.start <= byte && byte < s.end)
                    .map(|s| &s.style),
                theme.get_style("number").as_ref()
            );
        }
        for code in [
            "value = \"café ${upper(var.name)}\"\n",
            "value = \"${",
            "text = <<-EOT\n${var.name}\nEOT\n",
            "value = 42\n",
        ] {
            let shape = |spans: Vec<StyleInfo>| {
                spans
                    .into_iter()
                    .map(|s| (s.start, s.end, s.style))
                    .collect::<Vec<_>>()
            };
            assert_eq!(
                shape(highlighter.highlight("hcl", code).unwrap()),
                shape(
                    Highlighter::new(&theme)
                        .unwrap()
                        .highlight("hcl", code)
                        .unwrap()
                )
            );
        }
    }

    #[test]
    fn common_languages_parse_and_render_with_embedded_mocha() {
        let theme = parse_vscode_theme_contents(include_str!("../../themes/mocha.json")).unwrap();
        let checked = check_bundled_languages(&theme).unwrap();
        assert_eq!(checked.len(), 21);
    }

    #[test]
    fn common_language_detection_and_overrides_are_deterministic() {
        let directory = tempfile::tempdir().unwrap();
        let mut registry =
            LanguageRegistry::from_config(&HashMap::new(), directory.path()).unwrap();
        let theme = Theme::default();
        let highlighter = Highlighter::with_registry(&theme, Arc::new(registry.clone())).unwrap();
        for definition in language_definitions().iter().take(19) {
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
