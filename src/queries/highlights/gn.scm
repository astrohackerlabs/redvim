; Includes

"import" @include

; Conditionals

[
  "if"
  "else"
] @conditional

; Repeats

"foreach" @repeat

; Operators

[
  "="
  "+="
  "-="
  "!"
  "+"
  "-"
  "<"
  "<="
  ">"
  ">="
  "=="
  "!="
  "&&"
  "||"
] @operator

; Variables

(identifier) @variable

; Functions

(call_expression function: (identifier) @function.call)

; Fields

(scope_access field: (identifier) @property)

; Literals

(string) @string

(escape_sequence) @string.escape

(expansion (identifier) @variable.parameter)

(integer) @number

(expansion (hex)) @string.escape

(boolean) @boolean

; Punctuation

[ "{" "}" "[" "]" "(" ")" ] @punctuation.bracket

[
  "."
  ","
] @punctuation.delimiter

(expansion ["$" "${" "}"] @punctuation.delimiter)

; Comments

(comment) @comment

(assignment_statement . (identifier) @property)
