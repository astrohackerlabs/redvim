/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

// AppleScript grammar with block structure support
// AppleScript's English-like syntax is inherently ambiguous for LR parsing,
// so we use error recovery and loose matching for expressions.

// Helper for case-insensitive keywords
const ci = (word) => {
  return new RegExp(
    word
      .split("")
      .map((char) => {
        if (/[a-zA-Z]/.test(char)) {
          return `[${char.toLowerCase()}${char.toUpperCase()}]`;
        }
        return char;
      })
      .join("")
  );
};

module.exports = grammar({
  name: "applescript",

  // External scanner (src/scanner.c) supplies two tokens that pure-grammar
  // lexing can't represent:
  //   - block_comment: `(* ... *)` that respects strings and nests.
  //   - alias_prefix: `alias` only when NOT followed by `of`. Distinguishes
  //     `copy alias "X" to y` (prefix) from `alias of theItem` (property).
  externals: ($) => [
    $.block_comment,
    $.alias_prefix,
    $.piped_identifier,
    $.keyword_handler_to,
    $.inline_marker,
  ],

  // Treat `identifier` as the canonical "word" rule so every `ci(...)` keyword
  // token only matches as a whole word. Without this, the lexer happily
  // tokenizes `me` inside `home`, `of` inside `office`, etc.
  word: ($) => $.identifier,

  // Note: `¬` (U+00AC) is AppleScript's line-continuation glyph, not logical
  // NOT (that's the keyword `not`). Treat as whitespace so a trailing `¬`
  // transparently joins the next line.
  extras: ($) => [/\s/, /¬/, $.comment, $.block_comment],

  conflicts: ($) => [
    [$.record, $.list],
    // compound_name's optional 2nd/3rd identifier creates internal ambiguity
    // (does `current view` stop at `current` or eat `view` too?). GLR keeps
    // both alive; whichever extends into a valid property_reference wins.
    [$.compound_name],
    [$._expression, $.compound_name],
    [$._expression],
    [$.object_specifier, $.property_reference],
    [$.else_clause],
    [$.else_if_clause],
    [$.if_block, $.if_simple_statement],
    // `file type of x` should be property_reference(compound_name(file type), x),
    // not index_expression(file, property_reference(type, x)).
    [$.index_expression, $.property_reference],
    [$.handler_definition, $._expression, $.compound_name],
    [$.handler_definition, $._expression],
    [$.objc_handler_definition, $._expression],
    [$.bare_objc_call, $._expression],
    [$.objc_handler_definition, $._expression, $.compound_name],
    [$.bare_objc_call, $._expression, $.compound_name],
    // `with transaction <expr>` — the optional session expression is
    // ambiguous with the start of the body; let GLR keep both interpretations.
    [$.transaction_block, $._item],
    // A bare `piped_identifier` can be either an `_expression` (e.g. as the
    // first word of a statement) or a `_name_ref` (the first word of a
    // multi-word `compound_name`). GLR keeps both alive until the next token
    // disambiguates, mirroring the existing $.identifier path.
    [$._expression, $._name_ref],
    // Inside a Folder-Action handler header, the bare identifier after the
    // event may be the folder-name parameter or the start of a compound_name
    // body item — same shape as the existing handler_definition vs _expression
    // conflict, but with _name_ref now in the mix.
    [$.handler_definition, $._expression, $._name_ref],
    // Same ambiguity as above for ObjC-bridge handlers: the tail identifier
    // after `:arg` can be another selector word or the start of the body.
    [$.objc_handler_definition, $._expression, $._name_ref],
  ],

  rules: {
    // `implicit_run_end` is only valid at the top level (orphan `end run`
    // without a matching `on run` handler). Inside any block, `end run` is
    // either a real handler terminator or an error.
    source_file: ($) => repeat(choice($._item, $.implicit_run_end)),

    _item: ($) =>
      choice(
        $.handler_definition,
        $.script_block,
        $.tell_block,
        $.tell_simple_statement,
        $.if_block,
        $.if_simple_statement,
        $.repeat_block,
        $.try_block,
        $.considering_block,
        $.ignoring_block,
        $.timeout_block,
        $.transaction_block,
        $.using_terms_block,
        $.use_statement,
        $.property_declaration,
        $.global_declaration,
        $.local_declaration,
        $.set_statement,
        $.copy_statement,
        $.return_statement,
        $.error_statement,
        $.exit_statement,
        $.continue_statement,
        $.log_statement,
        $.bare_objc_call,
        $.command_call,
        $._expression
      ),

    // Bare ObjC selector call at statement level: `sortList:myList`,
    // `splitString:s byDelim:d`. Distinct from `objc_selector_call` (which
    // requires a `receiver's` prefix) so this form doesn't compete with
    // record-entry syntax inside `{}`. Only valid as a top-level item.
    bare_objc_call: ($) =>
      prec.left(
        seq(
          $.identifier,
          ":",
          choice($._expression, $.command_call),
          repeat(seq($.identifier, ":", choice($._expression, $.command_call)))
        )
      ),

    // `end run` at the bottom of a script with no matching `on run` —
    // AppleScript wraps top-level statements in an implicit run handler, and
    // many real scripts put `end run` at the bottom for clarity. Use a single
    // multi-word token so the bare `keyword_end` rule (used by every other
    // block) isn't perturbed. Lower precedence than the handler terminator so
    // a real `on run … end run` consumes `end run` as the closer first.
    implicit_run_end: ($) => prec(-1, token(seq(ci("end"), /[ \t]+/, ci("run")))),

    // ==================== HANDLERS ====================

    // Handler definition:
    //   `on greet(name) … end greet`
    //   `on opening folder fld … end opening folder`
    //   `on adding folder items to fld after receiving items … end …`
    //   `on sortList:theList … end sortList:`
    //   `on splitString:s byDelim:d … end splitString:byDelim:`
    //
    // Three header shapes:
    //   1. Regular: identifier + parenthesized params (or bare identifier param)
    //   2. Folder-Action: multi-word event + folder + prepositional clauses
    //   3. ObjC-bridge: identifier `:` ident (identifier `:` ident)+
    handler_definition: ($) =>
      prec.right(
        choice(
          // Regular and Folder-Action shapes. Parameters can be:
          //   • a parenthesized list: `(a, b)`
          //   • a bare identifier (Folder-Action style): `on open theItems`
          //   • a list pattern: `on run {}`, `on open {a, b}` for droplets
          // The terminator can be the bare `end` keyword, optionally followed
          // by the handler name; or the single-token `end run` form for the
          // run handler specifically. The handler name may be either a plain
          // identifier or a command-name token (`on open …`, `on quit …`).
          seq(
            field("keyword", $.keyword_function),
            field("name", choice($.identifier, $.folder_action_event, $.command_name)),
            optional(choice($.parameter_list, $.identifier, $.list)),
            repeat($.folder_action_param),
            optional($.given_clause),
            repeat($._item),
            choice(
              seq($.keyword_end, optional(choice($.identifier, $.folder_action_event, $.command_name))),
              $.implicit_run_end
            )
          ),
          // ObjC-style selector handler: each selector word is followed by
          // `:identifier`. The end clause repeats the selector words with
          // trailing colons but no parameters.
          $.objc_handler_definition
        )
      ),


    objc_handler_definition: ($) =>
      prec.right(seq(
        field("keyword", $.keyword_function),
        $.identifier,
        ":",
        $.identifier,
        repeat(seq($.identifier, ":", $.identifier)),
        repeat($._item),
        $.keyword_end,
        optional(seq($.identifier, ":", repeat(seq($.identifier, ":"))))
      )),

    // Multi-word Folder Action event names. Single-token so the rest of the
    // header doesn't have to peek ahead at individual words.
    folder_action_event: ($) =>
      token(
        choice(
          seq(ci("adding"), /[ \t]+/, ci("folder"), /[ \t]+/, ci("items"), /[ \t]+/, ci("to")),
          seq(ci("removing"), /[ \t]+/, ci("folder"), /[ \t]+/, ci("items"), /[ \t]+/, ci("from")),
          seq(ci("moving"), /[ \t]+/, ci("folder"), /[ \t]+/, ci("window"), /[ \t]+/, ci("for")),
          seq(ci("closing"), /[ \t]+/, ci("folder"), /[ \t]+/, ci("window"), /[ \t]+/, ci("for")),
          seq(ci("opening"), /[ \t]+/, ci("folder"), /[ \t]+/, ci("window")),
          seq(ci("opening"), /[ \t]+/, ci("folder")),
          seq(ci("closing"), /[ \t]+/, ci("folder")),
          seq(ci("adding"), /[ \t]+/, ci("folder"), /[ \t]+/, ci("items")),
          seq(ci("removing"), /[ \t]+/, ci("folder"), /[ \t]+/, ci("items"))
        )
      ),

    // Prepositional argument used by Folder Action handlers and similar:
    // `after receiving items`, `from rect`, the bare folder identifier.
    folder_action_param: ($) =>
      seq(
        token(
          choice(
            seq(ci("after"), /[ \t]+/, ci("receiving")),
            ci("from"),
            ci("for")
          )
        ),
        $.identifier
      ),

    keyword_on: ($) => token(ci("on")),
    keyword_function: ($) => choice($.keyword_on, $.keyword_handler_to),

    keyword_end: ($) => token(ci("end")),

    // Forced move: once `piped_identifier` reaches `$._expression`, the
    // header `on greet(name, age)` becomes GLR-ambiguous — `(name, age)`
    // could also parse as a `parenthesized_expression` (or comma-separated
    // expression list) since `name` and `age` are valid expressions. The
    // dynamic precedence resolves in favour of `parameter_list` whenever
    // the surrounding context is a handler header.
    parameter_list: ($) =>
      prec.dynamic(
        10,
        prec(
          2,
          seq(
            "(",
            optional(seq($._name_ref, repeat(seq(",", $._name_ref)))),
            ")"
          )
        )
      ),

    // Labeled parameters: given name:paramName, age:paramAge
    given_clause: ($) =>
      seq(
        token(ci("given")),
        $.labeled_parameter,
        repeat(seq(",", $.labeled_parameter))
      ),

    labeled_parameter: ($) =>
      seq(
        field("label", $.identifier),
        ":",
        field("name", $._name_ref)
      ),

    // ==================== SCRIPT OBJECTS ====================

    // Script block: script [name] ... end script
    script_block: ($) =>
      prec.right(
        1,
        seq(
          field("keyword", $.keyword_script),
          optional(field("name", $.identifier)),
          optional($.parent_clause),
          repeat($._item),
          $.keyword_end,
          optional(token(ci("script")))
        )
      ),

    keyword_script: ($) => token(ci("script")),

    // Parent inheritance: script MyScript parent MyParent
    parent_clause: ($) =>
      seq(
        token(ci("parent")),
        $._expression
      ),

    // ==================== TELL BLOCK ====================

    // Tell block: tell target ... end tell
    tell_block: ($) =>
      prec.right(
        seq(
          field("keyword", $.keyword_tell),
          field("target", $._expression),
          repeat($._item),
          $.keyword_end,
          optional(token(ci("tell")))
        )
      ),

    // One-line tell: tell application "Finder" to activate
    tell_simple_statement: ($) =>
      prec.right(
        10,
        seq(
          field("keyword", $.keyword_tell),
          field("target", $.reference),
          $.keyword_to,
          field("action", choice(
            $.command_call,
            $.set_statement,
            $.return_statement,
            $._expression
          ))
        )
      ),

    keyword_to: ($) => token(ci("to")),

    keyword_tell: ($) => token(ci("tell")),

    // ==================== IF BLOCK ====================

    // If block: if condition then ... [else if ... then ...] [else ...] end [if]
    // Uses `prec.dynamic` so that when both `if_block` and `if_simple_statement`
    // can match the same prefix, the parser actively prefers the multi-line
    // form whenever it sees a matching `end if` ahead.
    if_block: ($) =>
      prec.dynamic(
        2,
        prec.right(
          3,
          seq(
            field("keyword", $.keyword_if),
            field("condition", $._expression),
            $.keyword_then,
            repeat($._item),
            repeat($.else_if_clause),
            optional($.else_clause),
            $.keyword_end,
            optional(token(ci("if")))
          )
        )
      ),

    // One-line if: `if x then return y`. Allows a small set of items as the
    // single tail action — deliberately restricted so a multi-line if-block
    // body (which often starts with `set_statement` / `command_call`) doesn't
    // accidentally match here and orphan the `end if`. The trade-off is that
    // a true one-liner like `if x then set y to 5` or `if x then say "hi"`
    // parses as an `if_block` with a missing `end if`. Confirmed: adding
    // `set_statement`/`command_call`/`copy_statement` here causes ~14 files
    // in the realworld corpus to regress (multi-line `if … then\ncommand\n…
    // end if` blocks commit to the one-line form before the parser reaches
    // `end if`, even with `prec.dynamic` on `if_block`). One-liners using
    // command-call tails should be quarantined or rewritten in tests.
    // The `then` between condition and the tail is required to be
    // followed by an external `inline_marker` token — emitted by the
    // scanner only when the next non-extras token is on the SAME logical
    // line as `then` (either physical-same-row, or reached through one
    // or more `¬` continuations). This prevents the parser from matching
    // multi-line `if … then\n  body\nend if` as an `if_simple_statement`
    // (the bare newline case rejects the marker, so if_block wins). The
    // regular `keyword_then` (no inline_marker requirement) is used by
    // `if_block` and `else_if_clause`.
    //
    // The tail is restricted to a SET of explicit single-statement
    // shapes. Originally only the 5 atomic ones (return/exit/continue/
    // error/log) were allowed; the same-line constraint from
    // `inline_marker` lets us also accept set/copy/command_call/
    // tell_simple_statement without re-introducing the "multi-line body
    // greedy match" regression that bit the prior session. We do NOT
    // widen this to all of `_item` because `_item` includes `tell_block`
    // and other multi-line-body shapes that GLR can prefer over the
    // single-statement alternative.
    if_simple_statement: ($) =>
      prec.right(
        1,
        seq(
          field("keyword", $.keyword_if),
          field("condition", $._expression),
          $.keyword_then,
          $.inline_marker,
          field("then_action", choice(
            $.return_statement,
            $.exit_statement,
            $.continue_statement,
            $.error_statement,
            $.log_statement,
            $.set_statement,
            $.copy_statement,
            $.command_call,
            $.tell_simple_statement
          ))
        )
      ),

    keyword_if: ($) => token(ci("if")),
    keyword_then: ($) => token(ci("then")),

    else_if_clause: ($) =>
      seq(
        $.keyword_else_if,
        field("condition", $._expression),
        $.keyword_then,
        repeat($._item)
      ),

    keyword_else_if: ($) => token(seq(ci("else"), /[ \t]+/, ci("if"))),

    else_clause: ($) => seq($.keyword_else, repeat($._item)),

    keyword_else: ($) => token(ci("else")),

    // ==================== REPEAT BLOCK ====================

    // Repeat block: repeat ... end repeat
    repeat_block: ($) =>
      prec.right(
        seq(
          field("keyword", $.keyword_repeat),
          optional($._repeat_clause),
          repeat($._item),
          $.keyword_end,
          optional(token(ci("repeat")))
        )
      ),

    keyword_repeat: ($) => token(ci("repeat")),

    _repeat_clause: ($) =>
      choice(
        seq(
          token(ci("with")),
          $.identifier,
          token(ci("from")),
          $._expression,
          token(ci("to")),
          $._expression,
          optional(seq(token(ci("by")), $._expression))
        ),
        seq(token(ci("with")), $.identifier, token(ci("in")), $._expression),
        seq(token(ci("while")), $._expression),
        seq(token(ci("until")), $._expression),
        seq($._expression, token(ci("times")))
      ),

    // ==================== TRY BLOCK ====================

    // Try block: try ... on error [errMsg] [number errNum] ... end try
    try_block: ($) =>
      prec.right(
        seq(
          field("keyword", $.keyword_try),
          repeat($._item),
          optional($.error_handler),
          $.keyword_end,
          optional(token(ci("try")))
        )
      ),

    keyword_try: ($) => token(ci("try")),

    error_handler: ($) =>
      prec.right(
        1,
        seq(
          $.keyword_on_error,
          optional($.error_parameters),
          repeat($._item)
        )
      ),

    error_parameters: ($) =>
      prec(
        2,
        choice(
          seq($._name_ref, optional(seq(token(ci("number")), $._name_ref))),
          seq(token(ci("number")), $._name_ref)
        )
      ),

    keyword_on_error: ($) => token(seq(ci("on"), /[ \t]+/, ci("error"))),

    // ==================== CONSIDERING/IGNORING BLOCKS ====================

    // Considering block: `considering A, B [but ignoring C, D] ... end considering`
    considering_block: ($) =>
      prec.right(
        seq(
          field("keyword", $.keyword_considering),
          $.text_attribute,
          repeat(seq(token(choice(",", ci("and"))), $.text_attribute)),
          optional($.but_ignoring_clause),
          repeat($._item),
          $.keyword_end,
          optional(token(ci("considering")))
        )
      ),

    keyword_considering: ($) => token(ci("considering")),

    // Ignoring block: `ignoring A, B [but considering C, D] ... end ignoring`
    ignoring_block: ($) =>
      prec.right(
        seq(
          field("keyword", $.keyword_ignoring),
          $.text_attribute,
          repeat(seq(token(choice(",", ci("and"))), $.text_attribute)),
          optional($.but_considering_clause),
          repeat($._item),
          $.keyword_end,
          optional(token(ci("ignoring")))
        )
      ),

    keyword_ignoring: ($) => token(ci("ignoring")),

    // `but ignoring X` and `but considering X` modifiers on the head of a
    // `considering` or `ignoring` block.
    but_ignoring_clause: ($) =>
      seq(
        token(seq(ci("but"), /[ \t]+/, ci("ignoring"))),
        $.text_attribute,
        repeat(seq(token(choice(",", ci("and"))), $.text_attribute))
      ),

    but_considering_clause: ($) =>
      seq(
        token(seq(ci("but"), /[ \t]+/, ci("considering"))),
        $.text_attribute,
        repeat(seq(token(choice(",", ci("and"))), $.text_attribute))
      ),

    text_attribute: ($) =>
      token(
        choice(
          ci("case"),
          ci("diacriticals"),
          ci("hyphens"),
          ci("punctuation"),
          ci("white space"),
          seq(ci("numeric"), /[ \t]+/, ci("strings")),
          ci("expansion"),
          seq(ci("application"), /[ \t]+/, ci("responses"))
        )
      ),

    // ==================== TIMEOUT BLOCK ====================

    // With timeout block: with timeout [of] N seconds ... end timeout
    timeout_block: ($) =>
      prec.right(
        seq(
          field("keyword", $.keyword_with_timeout),
          optional(token(ci("of"))),
          $._expression,
          token(ci("seconds")),
          repeat($._item),
          $.keyword_end,
          optional(token(ci("timeout")))
        )
      ),

    keyword_with_timeout: ($) => token(seq(ci("with"), /[ \t]+/, ci("timeout"))),

    // ==================== TRANSACTION BLOCK ====================

    // With transaction block: `with transaction [<session>] ... end transaction`
    // Bundles Apple events into a single atomic operation for apps that
    // support transactional updates (rare; used in database-style scripting).
    transaction_block: ($) =>
      prec.right(
        seq(
          field("keyword", $.keyword_with_transaction),
          optional(field("session", $._expression)),
          repeat($._item),
          $.keyword_end,
          optional(token(ci("transaction")))
        )
      ),

    keyword_with_transaction: ($) => token(seq(ci("with"), /[ \t]+/, ci("transaction"))),

    // ==================== USING TERMS FROM BLOCK ====================

    // Using terms from block: using terms from application "X" ... end using terms from
    using_terms_block: ($) =>
      prec.right(
        seq(
          field("keyword", $.keyword_using_terms_from),
          field("source", $._expression),
          repeat($._item),
          $.keyword_end,
          optional(token(seq(ci("using"), /[ \t]+/, ci("terms"), /[ \t]+/, ci("from"))))
        )
      ),

    keyword_using_terms_from: ($) => token(seq(ci("using"), /[ \t]+/, ci("terms"), /[ \t]+/, ci("from"))),

    // ==================== USE STATEMENTS ====================

    // Use statement — covers every form documented in the AppleScript
    // Language Guide and macosxautomation.com:
    //   `use AppleScript version "2.4"`
    //   `use scripting additions`
    //   `use framework "Foundation"`
    //   `use application "Finder"`
    //   `use script "MyLibrary"`
    //   `use Safari: application "Safari" version "7.0" without importing`
    // The optional `<alias>:` prefix binds the imported resource to a name;
    // the optional `version "X"` requests a minimum version; the optional
    // `with importing` / `without importing` controls terminology import.
    use_statement: ($) =>
      seq(
        $.keyword_use,
        optional(seq(field("alias", $.identifier), ":")),
        choice(
          seq(token(ci("AppleScript")), optional($.use_version_clause)),
          seq(token(ci("framework")), $.string, optional($.use_version_clause), optional($.use_importing_clause)),
          seq(token(ci("scripting")), token(ci("additions"))),
          seq(token(ci("application")), $.string, optional($.use_version_clause), optional($.use_importing_clause)),
          seq(token(ci("script")), $.string, optional($.use_version_clause), optional($.use_importing_clause))
        )
      ),

    use_version_clause: ($) => seq(token(ci("version")), $.string),

    use_importing_clause: ($) =>
      token(choice(
        seq(ci("with"), /[ \t]+/, ci("importing")),
        seq(ci("without"), /[ \t]+/, ci("importing"))
      )),

    keyword_use: ($) => token(ci("use")),

    // ==================== DECLARATIONS ====================

    // Property declaration
    property_declaration: ($) =>
      seq(
        $.keyword_property,
        field("name", $._name_ref),
        ":",
        field("value", $._expression)
      ),

    // Accepts the full word `property` or the short form `prop`, both of
    // which the AppleScript compiler treats as identical.
    keyword_property: ($) => token(choice(ci("property"), ci("prop"))),

    // Global declaration
    global_declaration: ($) =>
      seq(
        $.keyword_global,
        $._name_ref,
        repeat(seq(",", $._name_ref))
      ),

    keyword_global: ($) => token(ci("global")),

    // Local declaration
    local_declaration: ($) =>
      seq(
        $.keyword_local,
        $._name_ref,
        repeat(seq(",", $._name_ref))
      ),

    keyword_local: ($) => token(ci("local")),

    // ==================== STATEMENTS ====================

    // Set statement
    // `set <target> to <value>`. Target can be a multi-word property name
    // (`folder actions enabled`, `current view`). Value may be either an
    // expression or a command call (`set MyPath to path to me`).
    set_statement: ($) =>
      seq(
        $.keyword_set,
        field("variable", choice($._expression, $.compound_name)),
        token(ci("to")),
        field("value", choice($._expression, $.command_call))
      ),

    keyword_set: ($) => token(ci("set")),

    // Copy statement
    copy_statement: ($) =>
      seq(
        $.keyword_copy,
        field("value", $._expression),
        token(ci("to")),
        field("variable", $._expression)
      ),

    keyword_copy: ($) => token(ci("copy")),

    // Return statement
    return_statement: ($) =>
      prec.right(
        seq(
          $.keyword_return,
          optional($._expression)
        )
      ),

    keyword_return: ($) => token(ci("return")),

    // Error statement: error "message" number N
    error_statement: ($) =>
      prec.right(
        seq(
          $.keyword_error,
          optional($._expression),
          optional(seq(token(ci("number")), $._expression)),
          optional(seq(token(ci("from")), $._expression)),
          optional(seq(token(ci("to")), $._expression)),
          optional(seq(token(ci("partial")), token(ci("result")), $._expression))
        )
      ),

    keyword_error: ($) => token(ci("error")),

    // Exit statement
    exit_statement: ($) =>
      prec.right(
        seq(
          $.keyword_exit,
          optional(token(ci("repeat")))
        )
      ),

    keyword_exit: ($) => token(ci("exit")),

    // Continue statement. Two forms:
    //   `continue` — bare; rarely used (loop control in some scripts).
    //   `continue <command_call>` — delegate to the inherited/parent
    //     handler. Common idiom inside `using terms from` and script
    //     objects with a `parent` clause:
    //         on activate
    //             continue activate
    //         end activate
    //     The argument is a full command_call so labelled parameters and
    //     `with`/`without` flags pass through.
    continue_statement: ($) =>
      prec.right(seq($.keyword_continue, optional($.command_call))),

    keyword_continue: ($) => token(ci("continue")),

    // Log statement
    log_statement: ($) =>
      seq(
        $.keyword_log,
        $._expression
      ),

    keyword_log: ($) => token(ci("log")),

    // ==================== COMMAND CALLS ====================

    // Common AppleScript commands. Argument can be either an expression or
    // a multi-word `compound_name` so `path to home folder` and similar
    // app-dictionary references parse without spilling tokens.
    command_call: ($) =>
      prec.right(
        seq(
          field("command", $.command_name),
          optional(field("argument", choice($._expression, $.compound_name))),
          repeat(choice($.command_parameter, $.command_flag))
        )
      ),

    // Boolean flag parameter — `with multiple selections allowed`,
    // `without invisibles`, `with hidden answer`. No value expression; the
    // flag itself is the entire parameter. Listed before `command_parameter`
    // in `command_call`'s repeat so the longest-match lexer favours it.
    command_flag: ($) =>
      field("name", $.command_flag_name),

    command_flag_name: ($) =>
      token(
        choice(
          seq(ci("with"), /[ \t]+/, ci("multiple"), /[ \t]+/, ci("selections"), /[ \t]+/, ci("allowed")),
          seq(ci("with"), /[ \t]+/, ci("empty"), /[ \t]+/, ci("selection"), /[ \t]+/, ci("allowed")),
          seq(ci("with"), /[ \t]+/, ci("hidden"), /[ \t]+/, ci("answer")),
          seq(ci("with"), /[ \t]+/, ci("administrator"), /[ \t]+/, ci("privileges")),
          seq(ci("with"), /[ \t]+/, ci("data")),
          seq(ci("with"), /[ \t]+/, ci("replacing")),
          seq(ci("with"), /[ \t]+/, ci("transaction")),
          seq(ci("with"), /[ \t]+/, ci("invisibles")),
          seq(ci("without"), /[ \t]+/, ci("multiple"), /[ \t]+/, ci("selections"), /[ \t]+/, ci("allowed")),
          seq(ci("without"), /[ \t]+/, ci("empty"), /[ \t]+/, ci("selection"), /[ \t]+/, ci("allowed")),
          seq(ci("without"), /[ \t]+/, ci("hidden"), /[ \t]+/, ci("answer")),
          seq(ci("without"), /[ \t]+/, ci("invisibles")),
          seq(ci("without"), /[ \t]+/, ci("transaction")),
          seq(ci("without"), /[ \t]+/, ci("replacing"))
        )
      ),

    command_name: ($) =>
      token(
        choice(
          // Standard additions
          seq(ci("display"), /[ \t]+/, ci("dialog")),
          seq(ci("display"), /[ \t]+/, ci("alert")),
          seq(ci("display"), /[ \t]+/, ci("notification")),
          seq(ci("choose"), /[ \t]+/, ci("file")),
          seq(ci("choose"), /[ \t]+/, ci("folder")),
          seq(ci("choose"), /[ \t]+/, ci("from"), /[ \t]+/, ci("list")),
          seq(ci("choose"), /[ \t]+/, ci("color")),
          seq(ci("do"), /[ \t]+/, ci("shell"), /[ \t]+/, ci("script")),
          seq(ci("run"), /[ \t]+/, ci("script")),
          seq(ci("load"), /[ \t]+/, ci("script")),
          seq(ci("store"), /[ \t]+/, ci("script")),
          seq(ci("path"), /[ \t]+/, ci("to")),
          seq(ci("info"), /[ \t]+/, ci("for")),
          seq(ci("list"), /[ \t]+/, ci("folder")),
          seq(ci("list"), /[ \t]+/, ci("disks")),
          seq(ci("system"), /[ \t]+/, ci("info")),
          seq(ci("system"), /[ \t]+/, ci("attribute")),
          // `current date` is intentionally not listed here — it's modelled
          // as a built-in expression (`current_date`) so it can participate
          // in arithmetic (`current date + 5 * days`).
          seq(ci("time"), /[ \t]+/, ci("to"), /[ \t]+/, ci("GMT")),
          seq(ci("random"), /[ \t]+/, ci("number")),
          seq(ci("round")),
          seq(ci("read")),
          seq(ci("write")),
          seq(ci("open"), /[ \t]+/, ci("for"), /[ \t]+/, ci("access")),
          seq(ci("close"), /[ \t]+/, ci("access")),
          seq(ci("get"), /[ \t]+/, ci("eof")),
          seq(ci("set"), /[ \t]+/, ci("eof")),
          seq(ci("clipboard"), /[ \t]+/, ci("info")),
          seq(ci("set"), /[ \t]+/, ci("the"), /[ \t]+/, ci("clipboard"), /[ \t]+/, ci("to")),
          seq(ci("the"), /[ \t]+/, ci("clipboard")),
          seq(ci("ASCII"), /[ \t]+/, ci("number")),
          seq(ci("ASCII"), /[ \t]+/, ci("character")),
          seq(ci("offset")),
          seq(ci("summarize")),
          seq(ci("beep")),
          seq(ci("delay")),
          seq(ci("say")),
          // Application commands
          ci("activate"),
          ci("launch"),
          ci("quit"),
          ci("reopen"),
          ci("run"),
          ci("open"),
          ci("close"),
          ci("save"),
          ci("delete"),
          ci("duplicate"),
          ci("make"),
          ci("move"),
          ci("count"),
          ci("get"),
          ci("print"),
          // Image Events / Finder commands
          ci("rotate"),
          ci("scale"),
          ci("crop"),
          ci("flip"),
          ci("pad"),
          ci("embed"),
          ci("unembed"),
          ci("convert"),
          ci("download"),
          ci("upload"),
          ci("send"),
          ci("receive"),
          ci("eject"),
          ci("mount")
        )
      ),

    // NOTE: `command_parameter` can attach across a newline (e.g.
    //     display dialog "X"
    //         default answer ""
    // the indented second line binds back to the first as a parameter).
    // The TOKEN-level multi-word gluing was fixed in v1.5.0 (`\s+` →
    // `[ \t]+` inside multi-word `token(...)` rules), but the RULE-level
    // `repeat($.command_parameter)` still skips over newlines via `extras`.
    // Whether this is a bug or a feature depends on context — Apple's own
    // formatter often wraps long command calls this way without a `¬`.
    // Documented here so a future maintainer doesn't spend an hour
    // rediscovering it.
    //
    // Named parameters for commands: with title "X", buttons {"OK"}, etc.
    // The value may be an expression, a multi-word `compound_name`, or a
    // bare `command_call` such as `default location path to desktop folder`.
    command_parameter: ($) =>
      seq(
        field("name", $.parameter_name),
        field("value", choice($._expression, $.compound_name, $.command_call))
      ),

    parameter_name: ($) =>
      token(
        choice(
          // Common parameter names
          seq(ci("with"), /[ \t]+/, ci("title")),
          seq(ci("with"), /[ \t]+/, ci("prompt")),
          seq(ci("with"), /[ \t]+/, ci("icon")),
          seq(ci("with"), /[ \t]+/, ci("properties")),
          seq(ci("without"), /[ \t]+/, ci("hidden"), /[ \t]+/, ci("answer")),
          seq(ci("default"), /[ \t]+/, ci("answer")),
          seq(ci("default"), /[ \t]+/, ci("button")),
          seq(ci("default"), /[ \t]+/, ci("color")),
          seq(ci("default"), /[ \t]+/, ci("name")),
          seq(ci("default"), /[ \t]+/, ci("location")),
          seq(ci("default"), /[ \t]+/, ci("items")),
          seq(ci("giving"), /[ \t]+/, ci("up"), /[ \t]+/, ci("after")),
          // Image Events parameter names: `rotate X to angle N`,
          // `scale X by factor N`, `pad X with pad color C`, etc.
          seq(ci("to"), /[ \t]+/, ci("angle")),
          seq(ci("by"), /[ \t]+/, ci("factor")),
          seq(ci("to"), /[ \t]+/, ci("size")),
          seq(ci("with"), /[ \t]+/, ci("pad"), /[ \t]+/, ci("color")),
          ci("buttons"),
          ci("using"),
          ci("at"),
          ci("to"),
          ci("from"),
          ci("for"),
          ci("in"),
          ci("with"),
          ci("without"),
          ci("as"),
          ci("by"),
          ci("thru"),
          ci("through"),
          ci("before"),
          ci("after"),
          ci("instead"), seq(/[ \t]+/, ci("of")),
          ci("into"),
          ci("onto"),
          ci("between"),
          ci("against"),
          ci("above"),
          ci("below"),
          ci("aside"), seq(/[ \t]+/, ci("from")),
          ci("around"),
          ci("beside"),
          ci("beneath"),
          ci("under"),
          ci("over"),
          ci("named"),
          seq(ci("starting"), /[ \t]+/, ci("at")),
          seq(ci("multiple"), /[ \t]+/, ci("selections"), /[ \t]+/, ci("allowed")),
          seq(ci("empty"), /[ \t]+/, ci("selection"), /[ \t]+/, ci("allowed")),
          seq(ci("of"), /[ \t]+/, ci("type")),
          seq(ci("invisibles"))
        )
      ),

    // ==================== EXPRESSIONS ====================

    // Expressions. `the` is decorative in AppleScript (`set the x to the name
    // of the file`); it may appear before any noun phrase. We consume an
    // optional `the` at the start of every expression so it doesn't have to
    // be sprinkled through every other rule.
    _expression: ($) =>
      seq(
        optional($.the_keyword),
        choice(
          $.binary_expression,
          $.unary_expression,
          $.string,
          $.number,
          $.boolean,
          $.missing_value,
          $.null_value,
          $.current_application,
          $.current_date,
          $.me_reference,
          $.it_reference,
          $.its_reference,
          $.result_reference,
          $.list,
          $.record,
          $.parenthesized_expression,
          $.reference,
          $.object_specifier,
          $.property_reference,
          $.index_expression,
          $.range_expression,
          $.coercion_expression,
          $.concatenation,
          $.reference_to_expression,
          $.date_literal,
          $.objc_selector_call,
          $.possessive_expression,
          $.new_specifier,
          $.raw_data,
          $.my_expression,
          $.handler_call,
          $.applescript_constant,
          $.relative_reference,
          $.alias_expression,
          // Piped identifiers are first-class expressions, not just name
          // slots — `set X to |class|`, `display dialog |the message|`,
          // etc. The `_name_ref` helper covers the inverse case (name-only
          // slots like parameter lists where expressions would be wrong).
          $.piped_identifier,
          $.identifier
        )
      ),

    // `alias <expr>` — produces an alias value from a path/string expression.
    // The `alias` token is supplied by the external scanner (see scanner.c);
    // it's only emitted when the next non-whitespace input is NOT `of`, so
    // the property-reference form `alias of theItem` keeps parsing as a
    // plain `property_reference(compound_name(alias), theItem)`.
    alias_expression: ($) =>
      prec.right(seq($.alias_prefix, $._expression)),

    // Relative reference forms from the AppleScript Language Guide:
    //   `<insertion-point> <base>`
    //   `before <ref>`, `after <ref>`, `behind <ref>`,
    //   `in front of <ref>`, `in back of <ref>`
    // Example: `move word 1 to before paragraph 3`, `paragraph after word 99`.
    // Note: `beginning` and `end` are insertion-point specifier prefixes
    // (handled by `specifier_prefix`); the words below stand alone before a
    // base expression.
    relative_reference: ($) =>
      prec.right(seq(
        $.relative_position,
        $._expression
      )),

    relative_position: ($) =>
      token(
        choice(
          seq(ci("in"), /[ \t]+/, ci("front"), /[ \t]+/, ci("of")),
          seq(ci("in"), /[ \t]+/, ci("back"), /[ \t]+/, ci("of")),
          ci("before"),
          ci("after"),
          ci("behind")
        )
      ),

    // AppleScript's built-in constants: scalar values (`pi`), whitespace
    // characters (`space`, `tab`, `return`, `linefeed`, `quote`, `null`),
    // weekday names (`Monday`–`Sunday`), month names (`January`–`December`),
    // and time-unit constants used in date arithmetic (`seconds`, `minutes`,
    // `hours`, `days`, `weeks`). Listing these explicitly makes them
    // highlight as constants rather than plain identifiers.
    applescript_constant: ($) =>
      token(
        choice(
          // Scalar constants
          ci("pi"),
          // Whitespace and character constants. `return` is intentionally
          // omitted — it's already the return-statement keyword and adding it
          // here creates a lexer collision.
          ci("space"),
          ci("tab"),
          ci("linefeed"),
          ci("quote"),
          // `null` is already covered by `null_value`; keep that one
          // canonical and don't list it here.
          // Days of week
          ci("Monday"), ci("Tuesday"), ci("Wednesday"), ci("Thursday"),
          ci("Friday"), ci("Saturday"), ci("Sunday"),
          // Months
          ci("January"), ci("February"), ci("March"), ci("April"),
          ci("May"), ci("June"), ci("July"), ci("August"),
          ci("September"), ci("October"), ci("November"), ci("December")
          // Time-unit names (`seconds`, `minutes`, `hours`, `days`, `weeks`)
          // are deliberately NOT in this list. They appear in two contexts:
          //   1. Date arithmetic — `set t to current date + 2 * hours`. Here
          //      they parse as plain `identifier`, which is sufficient for
          //      highlighting and outline; nothing is lost by not naming them.
          //   2. `with timeout of 30 seconds` — that block has its own
          //      dedicated `seconds` keyword token inside `timeout_block`,
          //      unaffected by this list.
          // Listing them as `applescript_constant` blocks them from being
          // used as property names (e.g. `hours of theDate`, which is a
          // common pattern when poking at date records).
        )
      ),

    // Handler call as expression: `f()`, `userPicksFolder()`, `f(x, y)`.
    // Distinct from `parenthesized_expression` so the trailing `()` glues
    // tightly to the identifier instead of becoming an orphan node.
    handler_call: ($) =>
      prec(11, seq(
        // Callee can be a plain or piped identifier — `f()`,
        // `userPicksFolder()`, `|length|()` (ASObjC no-arg method via
        // a piped selector name).
        choice($.identifier, $.piped_identifier),
        token.immediate("("),
        optional(seq(
          choice($._expression, $.command_call),
          repeat(seq(",", choice($._expression, $.command_call)))
        )),
        ")"
      )),

    // `my <expr>` — script self-reference, used to call own handlers / refer
    // to own properties from inside a tell block: `my resolve_conflicts(x)`.
    // Bumped precedence so the leading `my` keyword wins over the bare-
    // identifier path through `_expression`.
    my_expression: ($) =>
      prec.right(10, seq($.keyword_my, $._expression)),

    keyword_my: ($) => token(ci("my")),

    // AppleScript raw data literal: «class fold», «data utxt201C», etc.
    // Used in decompiled scripts for special types/strings. Built as a token
    // sequence rather than a single regex because tree-sitter's lexer
    // generation can drop tokens whose regex includes non-ASCII anchor
    // characters.
    raw_data: ($) =>
      token(seq("«", /[A-Za-z0-9 ]+/, "»")),

    the_keyword: ($) => token(ci("the")),

    // `new <element_type>` — the argument shape used by `make`, e.g.
    // `make new folder at … with properties {…}` and `make new document`.
    new_specifier: ($) =>
      prec.right(seq(
        token(ci("new")),
        choice($.element_type, $.identifier)
      )),

    // Possessive accessor: `x's y` — common in modern AppleScript and
    // dominant in ASObjC (`current application's NSString`). The right side
    // is `compound_name` so multi-word app-dictionary property names
    // (`AppleScript's text item delimiters`) parse as a single accessor.
    // Higher precedence than binary operators so `x's y + z` is `(x's y) + z`.
    possessive_expression: ($) =>
      prec.left(
        7,
        seq($._expression, $.possessive, $.compound_name)
      ),

    // ObjC bridge method call: `receiver's selector:arg [label:arg ...]`.
    // Slightly higher precedence than plain possessive so the `:arg` tail wins
    // when present. The bare receiverless form (`sortList:myList`) isn't
    // modeled here because it collides with record-entry syntax inside `{}`.
    objc_selector_call: ($) =>
      prec.left(
        8,
        seq(
          $._expression,
          $.possessive,
          $.identifier,
          ":",
          $._expression,
          repeat(seq($.identifier, ":", $._expression))
        )
      ),

    possessive: ($) => token("'s"),

    // `a reference to <expr>` — three-word prefix that wraps an expression
    // as a live reference. Common in ASObjC: `property NS : a reference to current application's NSString`.
    // Use a single multi-word token so a bare `a` identifier (`a or b`) is unaffected.
    reference_to_expression: ($) =>
      prec.right(seq(
        token(seq(ci("a"), /[ \t]+/, ci("reference"), /[ \t]+/, ci("to"))),
        $._expression
      )),

    // Date literal: `date "Saturday, January 1, 2000 at 12:00:00 AM"`
    date_literal: ($) =>
      seq(token(ci("date")), $.string),

    // Parens can wrap a command call when used as a value:
    // `(path to home folder)`, `(do shell script "uname -m")`. They can also
    // serve as the argument list of a handler call: `f(x, y, z)`. Otherwise
    // plain expressions.
    parenthesized_expression: ($) =>
      seq(
        "(",
        optional(seq(
          choice($.command_call, $._expression),
          repeat(seq(",", choice($.command_call, $._expression)))
        )),
        ")"
      ),

    // List literal — items can be plain expressions or multi-word application
    // constants like `Eight channel` (Image Events' colorspace names).
    list: ($) =>
      seq(
        "{",
        optional(seq($._list_item, repeat(seq(",", $._list_item)))),
        "}"
      ),

    _list_item: ($) => choice($._expression, $.compound_name),

    record: ($) => seq("{", $.record_entry, repeat(seq(",", $.record_entry)), "}"),

    // Record entry. Both key AND value may be multi-word, since
    // AppleScript app dictionaries use such forms freely:
    //   {file name: x, disclosure triangle: y}   ← multi-word keys
    //   {repetition method: start after completion}  ← multi-word value
    // The value form is NOT valid generic AppleScript per `osacompile`;
    // it only resolves when the receiving app's dictionary defines
    // those words as enumeration constants. This is lenient editor-time
    // parsing: OmniFocus / Mail / Calendar scripts use this pattern
    // heavily and we'd rather highlight them right than be strict.
    record_entry: ($) =>
      seq($.compound_name, ":", choice($._expression, $.compound_name)),

    reference: ($) =>
      seq(
        $.keyword_application,
        $.string
      ),

    keyword_application: ($) => token(ci("application")),

    // ==================== BINARY EXPRESSIONS ====================

    // Binary operators with precedence
    binary_expression: ($) =>
      choice(
        // Comparison operators (lowest precedence)
        prec.left(1, seq($._expression, $.comparison_operator, $._expression)),
        // Logical operators
        prec.left(2, seq($._expression, $.logical_operator, $._expression)),
        // Arithmetic operators
        prec.left(3, seq($._expression, $.additive_operator, $._expression)),
        prec.left(4, seq($._expression, $.multiplicative_operator, $._expression)),
        // Exponentiation (right associative, highest precedence)
        prec.right(5, seq($._expression, "^", $._expression)),
        // Postfix `exists` predicate: `folder X exists`
        prec.left(1, seq($._expression, token(ci("exists"))))
      ),

    // Comparison operators with all the synonyms documented in the AppleScript
    // Language Guide (Operators Reference). AppleScript provides multiple
    // English-like spellings for each comparison; we accept them all so real
    // scripts highlight consistently.
    comparison_operator: ($) =>
      token(
        choice(
          // Symbol forms
          "=",
          "≠",
          "/=",
          "<",
          ">",
          "≤",
          "<=",
          "≥",
          ">=",
          // Equality
          ci("is equal to"),
          ci("is equal"),
          ci("equal to"),
          ci("equals"),
          ci("equal"),
          ci("is not equal to"),
          ci("is not equal"),
          ci("isn't equal to"),
          ci("isn't equal"),
          ci("does not equal"),
          ci("doesn't equal"),
          // Ordering (less than)
          ci("is less than"),
          ci("less than"),
          ci("is not greater than or equal to"),
          ci("is not greater than or equal"),
          ci("isn't greater than or equal to"),
          ci("isn't greater than or equal"),
          // Ordering (less than or equal)
          ci("is less than or equal to"),
          ci("is less than or equal"),
          ci("less than or equal to"),
          ci("less than or equal"),
          ci("is not greater than"),
          ci("isn't greater than"),
          // Ordering (greater than)
          ci("is greater than"),
          ci("greater than"),
          ci("is not less than or equal to"),
          ci("is not less than or equal"),
          ci("isn't less than or equal to"),
          ci("isn't less than or equal"),
          // Ordering (greater than or equal)
          ci("is greater than or equal to"),
          ci("is greater than or equal"),
          ci("greater than or equal to"),
          ci("greater than or equal"),
          ci("is not less than"),
          ci("isn't less than"),
          // String ordering (text-only)
          ci("comes before"),
          ci("does not come before"),
          ci("doesn't come before"),
          ci("comes after"),
          ci("does not come after"),
          ci("doesn't come after"),
          // Generic `is` / `is not` — must come AFTER the `is equal/less/...`
          // variants above so the longer form wins via longest-match lexing.
          ci("is not"),
          ci("isn't"),
          ci("is"),
          // Containment
          ci("contains"),
          ci("does not contain"),
          ci("doesn't contain"),
          ci("is contained by"),
          ci("is not contained by"),
          ci("isn't contained by"),
          ci("is in"),
          ci("is not in"),
          ci("isn't in"),
          // Text matching
          ci("starts with"),
          ci("begins with"),
          ci("start with"),
          ci("begin with"),
          ci("ends with"),
          ci("end with"),
          ci("does not start with"),
          ci("does not begin with"),
          ci("does not end with"),
          ci("doesn't start with"),
          ci("doesn't begin with"),
          ci("doesn't end with")
        )
      ),

    logical_operator: ($) =>
      token(choice(ci("and"), ci("or"))),

    additive_operator: ($) => token(choice("+", "-")),

    multiplicative_operator: ($) =>
      token(choice("*", "/", "÷", ci("mod"), ci("div"))),

    // Unary operators
    unary_expression: ($) =>
      prec.right(6, seq($.unary_operator, $._expression)),

    unary_operator: ($) => token(choice(ci("not"), "-")),

    // String concatenation
    concatenation: ($) =>
      prec.left(2, seq($._expression, "&", $._expression)),

    // ==================== OBJECT SPECIFIERS ====================

    // Object specifier: `window 1 of application "Finder"`, also `every word`
    // (no `of` tail). Optional trailing `whose | where` filter:
    // `every file of home whose size > 1000`.
    // The `_expression` slot accepts compound_name so multi-word element types
    // (`UI element`, `static text`) parse cleanly.
    object_specifier: ($) =>
      prec.left(
        3,
        seq(
          $.specifier_prefix,
          choice($._expression, $.compound_name),
          optional(seq(token(ci("of")), $._expression)),
          optional($.whose_clause)
        )
      ),

    // Filter clause for object specifiers: `every file whose name ends with ".txt"`
    whose_clause: ($) =>
      prec.right(seq(
        token(choice(ci("whose"), ci("where"))),
        $._expression
      )),

    specifier_prefix: ($) =>
      token(
        choice(
          // Insertion points used by `make new X at <ins> of <container>`:
          seq(ci("end"), /[ \t]+/, ci("of")),
          seq(ci("beginning"), /[ \t]+/, ci("of")),
          ci("first"),
          ci("second"),
          ci("third"),
          ci("fourth"),
          ci("fifth"),
          ci("sixth"),
          ci("seventh"),
          ci("eighth"),
          ci("ninth"),
          ci("tenth"),
          ci("last"),
          ci("front"),
          ci("back"),
          ci("middle"),
          ci("any"),
          ci("some"),
          ci("every")
        )
      ),

    // Property reference: `name of theFile`, also multi-word app-dictionary
    // properties like `current view of window`, `name extension of theFile`,
    // `folder actions enabled`. Both sides allow multi-word names so list
    // items like `millions of colors plus` parse as a single reference.
    property_reference: ($) =>
      prec.left(
        3,
        seq(
          $.compound_name,
          token(ci("of")),
          choice($._expression, $.compound_name)
        )
      ),

    // A 1–6-word name. Each word may be an `identifier` or an `element_type`
    // — common dictionary names like `Folder Action scripts folder` and
    // long enum values like `two hundred fifty six colors` need this. Higher
    // precedence than the bare-identifier path so when `of` follows, the
    // multi-word interpretation wins via the explicit conflict declaration.
    compound_name: ($) =>
      prec.right(seq(
        choice($._name_ref, $.element_type),
        optional(choice($._name_ref, $.element_type)),
        optional(choice($._name_ref, $.element_type)),
        optional(choice($._name_ref, $.element_type)),
        optional(choice($._name_ref, $.element_type)),
        optional(choice($._name_ref, $.element_type))
      )),

    // Index expression: `item 1`, `window 2`, `paragraph 3 of foo`,
    // `script ScriptName of folder action FolderName`. The leading
    // element accepts `keyword_script` as an alternative so command
    // arguments like `delete script X` parse — the lexer always emits
    // `keyword_script` for the bare word `script`, so element_type
    // never sees it.
    index_expression: ($) =>
      prec.left(
        4,
        seq(
          choice($.element_type, $.keyword_script),
          $._expression,
          optional(seq(token(ci("of")), $._expression))
        )
      ),

    element_type: ($) =>
      token(
        choice(
          // Common single-word element types and their plurals (AppleScript
          // accepts both forms with the same meaning in range expressions).
          ci("item"), ci("items"),
          ci("word"), ci("words"),
          ci("character"), ci("characters"),
          ci("paragraph"), ci("paragraphs"),
          ci("line"), ci("lines"),
          ci("window"), ci("windows"),
          ci("document"), ci("documents"),
          ci("file"), ci("files"),
          ci("folder"), ci("folders"),
          ci("disk"), ci("disks"),
          ci("process"), ci("processes"),
          ci("button"), ci("buttons"),
          ci("menu"), ci("menus"),
          ci("row"), ci("rows"),
          ci("column"), ci("columns"),
          ci("cell"), ci("cells"),
          // Common multi-word element types from Finder, System Events,
          // and Image Events dictionaries that real scripts use freely.
          seq(ci("text"), /[ \t]+/, ci("item")),
          seq(ci("menu"), /[ \t]+/, ci("item")),
          seq(ci("text"), /[ \t]+/, ci("field")),
          seq(ci("application"), /[ \t]+/, ci("file")),
          seq(ci("application"), /[ \t]+/, ci("process")),
          seq(ci("folder"), /[ \t]+/, ci("action")),
          seq(ci("script"), /[ \t]+/, ci("file")),
          ci("attachment"),
          seq(ci("outgoing"), /[ \t]+/, ci("message")),
          seq(ci("incoming"), /[ \t]+/, ci("message")),
          seq(ci("list"), /[ \t]+/, ci("view"), /[ \t]+/, ci("options")),
          seq(ci("container"), /[ \t]+/, ci("window")),
          seq(ci("information"), /[ \t]+/, ci("window")),
          seq(ci("document"), /[ \t]+/, ci("file")),
          seq(ci("scroll"), /[ \t]+/, ci("bar")),
          seq(ci("scroll"), /[ \t]+/, ci("area")),
          seq(ci("static"), /[ \t]+/, ci("text")),
          seq(ci("UI"), /[ \t]+/, ci("element")),
          seq(ci("menu"), /[ \t]+/, ci("bar")),
          seq(ci("menu"), /[ \t]+/, ci("bar"), /[ \t]+/, ci("item")),
          seq(ci("tool"), /[ \t]+/, ci("bar")),
          seq(ci("title"), /[ \t]+/, ci("bar")),
          seq(ci("status"), /[ \t]+/, ci("bar")),
          seq(ci("text"), /[ \t]+/, ci("area")),
          seq(ci("color"), /[ \t]+/, ci("well")),
          seq(ci("combo"), /[ \t]+/, ci("box")),
          seq(ci("check"), /[ \t]+/, ci("box")),
          seq(ci("radio"), /[ \t]+/, ci("button")),
          seq(ci("radio"), /[ \t]+/, ci("group")),
          seq(ci("pop"), /[ \t]+/, ci("up"), /[ \t]+/, ci("button")),
          seq(ci("disclosure"), /[ \t]+/, ci("triangle")),
          seq(ci("incrementor"), /[ \t]+/, ci("button"))
        )
      ),

    // Range expression: `items 1 thru 5`, `characters 3 through 10 of X`.
    range_expression: ($) =>
      prec.left(
        3,
        seq(
          $.element_type,
          $._expression,
          $.range_operator,
          $._expression,
          optional(seq(token(ci("of")), $._expression))
        )
      ),

    range_operator: ($) => token(choice(ci("thru"), ci("through"))),

    // Coercion: x as text, y as integer
    coercion_expression: ($) =>
      prec.left(
        1,
        seq(
          $._expression,
          token(ci("as")),
          $.type_specifier
        )
      ),

    type_specifier: ($) =>
      token(
        choice(
          ci("text"),
          ci("string"),
          ci("integer"),
          ci("real"),
          ci("number"),
          ci("boolean"),
          ci("list"),
          ci("record"),
          ci("date"),
          ci("file"),
          ci("alias"),
          seq(ci("POSIX"), /[ \t]+/, ci("file")),
          seq(ci("POSIX"), /[ \t]+/, ci("path")),
          ci("class"),
          ci("constant"),
          ci("script"),
          seq(ci("Unicode"), /[ \t]+/, ci("text")),
          seq(ci("styled"), /[ \t]+/, ci("text")),
          ci("data"),
          ci("reference"),
          ci("ref"),  // short form of `reference`
          ci("anything"),
          seq(ci("list"), /[ \t]+/, ci("of"), /[ \t]+/, ci("text")),
          seq(ci("list"), /[ \t]+/, ci("of"), /[ \t]+/, ci("integer")),
          seq(ci("list"), /[ \t]+/, ci("of"), /[ \t]+/, ci("number"))
        )
      ),

    // ==================== SPECIAL REFERENCES ====================

    current_application: ($) => token(seq(ci("current"), /[ \t]+/, ci("application"))),

    // `current date` — built-in expression returning the current date object.
    current_date: ($) => token(seq(ci("current"), /[ \t]+/, ci("date"))),

    me_reference: ($) => token(ci("me")),

    it_reference: ($) => token(ci("it")),

    // `its` — possessive companion to `it`, used in property chains
    // (`its name`, `its size of file`). Treated as a special reference for
    // highlighting; distinct from `it` to preserve the semantic difference.
    its_reference: ($) => token(ci("its")),

    result_reference: ($) => token(ci("result")),

    null_value: ($) => token(ci("null")),

    // ==================== LITERALS ====================

    // String with escape sequences. The whole literal is a single `token`
    // so `extras` (whitespace, comments, line-continuation `¬`) cannot be
    // inserted between the opening quote and characters inside. Without this,
    // a string like `"--XXXX"` would have `--XXXX` consumed as a comment.
    string: ($) =>
      token(
        seq(
          '"',
          repeat(choice(
            seq("\\", /./),
            /[^"\\]/
          )),
          '"'
        )
      ),

    // Numeric literal. Plain integer / real / exponential, OR an ordinal
    // form (`1st`, `2nd`, `23rd`, `101st`, `11th`) used as a synonym for
    // ordinal positional words (`first`, `second`, ...).
    number: ($) =>
      token(
        choice(
          // Ordinal suffix forms: any integer followed by st/nd/rd/th (case
          // insensitive). Listed BEFORE the plain numeric pattern so the
          // longer ordinal token wins via longest-match lexing.
          /-?\d+(st|nd|rd|th|ST|ND|RD|TH)/,
          /-?\d+(\.\d+)?(E[+-]?\d+)?/,
          // Leading-dot form: `.5`, `-.25`. Common in window-manager
          // scripts that compute screen fractions; AppleScript accepts it.
          /-?\.\d+(E[+-]?\d+)?/
        )
      ),

    boolean: ($) => token(choice(ci("true"), ci("false"))),

    missing_value: ($) => token(seq(ci("missing"), /[ \t]+/, ci("value"))),

    // ==================== IDENTIFIERS & COMMENTS ====================

    identifier: ($) => /[a-zA-Z_][a-zA-Z0-9_]*/,

    // _name_ref: anywhere a plain $.identifier appears as a NAME slot
    // (parameter, property declaration, variable in global/local/error).
    // Piped identifiers ARE expressions, so for slots that already accept
    // $._expression they fall through there; this helper is only for the
    // name-only slots where $._expression would be wrong (e.g. parameter
    // lists, property names — you don't want a string literal there).
    _name_ref: ($) => choice($.identifier, $.piped_identifier),

    // Line comments only. Block comments `(* ... *)` are handled by the
    // external scanner so they can respect strings and nest.
    comment: ($) =>
      token(
        choice(
          seq("--", /.*/),
          // `#!` listed before `#` so the shebang form wins via longest-
          // match lexing when both could fire on the first line.
          seq("#!", /.*/),
          seq("#", /.*/)
        )
      ),
  },
});
