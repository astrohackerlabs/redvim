; AppleScript syntax highlighting for tree-sitter / Helix
;
; Helix uses first-match-wins: more specific patterns must come BEFORE the
; generic `(identifier) @variable` catch-all at the bottom of this file.

; ---------------------------------------------------------------------------
; Comments
; ---------------------------------------------------------------------------
(comment) @comment.line
(block_comment) @comment.block

; ---------------------------------------------------------------------------
; Literals
; ---------------------------------------------------------------------------
(string) @string
(raw_data) @string.special
(number) @constant.numeric

(boolean) @constant.builtin.boolean

[
  (missing_value)
  (null_value)
  (applescript_constant)
  (current_date)
] @constant.builtin

; ---------------------------------------------------------------------------
; Built-in references
; ---------------------------------------------------------------------------
[
  (it_reference)
  (its_reference)
  (me_reference)
  (current_application)
  (result_reference)
] @variable.builtin

; ---------------------------------------------------------------------------
; Handlers (functions)
; ---------------------------------------------------------------------------
(handler_definition
  name: [(identifier) (command_name) (folder_action_event)] @function)

; First child of a handler call is the handler name; arguments are hoisted
; siblings, so anchor to the first child only.
(handler_call
  . (identifier) @function.call)

; Application / Standard-Additions commands: `display dialog`, `do shell script`
(command_name) @function.builtin
(folder_action_event) @function.builtin

; ---------------------------------------------------------------------------
; Parameters
; ---------------------------------------------------------------------------
(parameter_list
  [(identifier) (piped_identifier)] @variable.parameter)

(labeled_parameter
  label: (identifier) @variable.parameter)
(labeled_parameter
  name: [(identifier) (piped_identifier)] @variable.parameter)

(command_parameter
  name: (parameter_name) @variable.parameter)
(command_flag
  name: (command_flag_name) @variable.parameter)

; ---------------------------------------------------------------------------
; Properties / record keys
; ---------------------------------------------------------------------------
(property_declaration
  name: [(identifier) (piped_identifier)] @variable.other.member)

(record_entry
  (compound_name) @variable.other.member)

; ---------------------------------------------------------------------------
; Types
; ---------------------------------------------------------------------------
(element_type) @type.builtin
(type_specifier) @type

; ---------------------------------------------------------------------------
; Operators
; ---------------------------------------------------------------------------
[
  (comparison_operator)
  (logical_operator)
] @keyword.operator

[
  (additive_operator)
  (multiplicative_operator)
  (unary_operator)
  (range_operator)
  (possessive)
] @operator

"&" @operator

; ---------------------------------------------------------------------------
; Specifiers
; ---------------------------------------------------------------------------
[
  (specifier_prefix)
  (relative_position)
  (the_keyword)
] @keyword

; ---------------------------------------------------------------------------
; Keywords — control flow
; ---------------------------------------------------------------------------
[
  (keyword_if)
  (keyword_else)
  (keyword_else_if)
  (keyword_then)
] @keyword.control.conditional

(keyword_repeat) @keyword.control.repeat
(keyword_return) @keyword.control.return

[
  (keyword_try)
  (keyword_error)
  (keyword_on_error)
] @keyword.control.exception

(keyword_use) @keyword.control.import

[
  (keyword_exit)
  (keyword_continue)
] @keyword.control

; ---------------------------------------------------------------------------
; Keywords — declarations / statements / blocks
; ---------------------------------------------------------------------------
[
  (keyword_tell)
  (keyword_end)
  (keyword_on)
  (keyword_function)
  (keyword_script)
  (keyword_to)
  (keyword_handler_to)
  (keyword_set)
  (keyword_copy)
  (keyword_global)
  (keyword_local)
  (keyword_property)
  (keyword_log)
  (keyword_my)
  (keyword_application)
  (keyword_considering)
  (keyword_ignoring)
  (keyword_using_terms_from)
  (keyword_with_timeout)
  (keyword_with_transaction)
] @keyword

; ---------------------------------------------------------------------------
; Punctuation
; ---------------------------------------------------------------------------
["(" ")" "{" "}"] @punctuation.bracket
["," ":"] @punctuation.delimiter

; ---------------------------------------------------------------------------
; Catch-all — must stay last (first-match-wins)
; ---------------------------------------------------------------------------
[
  (identifier)
  (piped_identifier)
] @variable
